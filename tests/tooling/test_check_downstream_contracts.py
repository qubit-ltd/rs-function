import importlib.util
import tempfile
import unittest
import os
import sys
import time
from pathlib import Path

SCRIPT = Path(__file__).resolve().parents[2] / "scripts/check-downstream-contracts.py"
spec = importlib.util.spec_from_file_location("check_downstream_contracts", SCRIPT)
checker = importlib.util.module_from_spec(spec)
spec.loader.exec_module(checker)


class MetadataTests(unittest.TestCase):
    def setUp(self):
        self.expected = {name: Path(f"/siblings/{name}/Cargo.toml") for name in checker.TARGETS}
        self.good = {"packages": [
            {"name": name, "source": None, "manifest_path": str(path)}
            for name, path in self.expected.items()
        ]}

    def test_accepts_exact_local_packages(self):
        self.assertEqual([], checker.check_metadata(self.good, self.expected))

    def test_rejects_registry_fallback(self):
        self.good["packages"][0]["source"] = "registry+https://example.invalid"
        self.assertTrue(checker.check_metadata(self.good, self.expected))

    def test_rejects_wrong_path_duplicate_and_missing(self):
        self.good["packages"][0]["manifest_path"] = "/wrong/Cargo.toml"
        self.good["packages"].append(dict(self.good["packages"][0]))
        self.good["packages"].pop(1)
        self.assertGreaterEqual(len(checker.check_metadata(self.good, self.expected)), 2)

    def test_function_source_uses_current_checkout(self):
        roots = checker.checkout_roots(Path("/siblings"), Path("/candidate-function"))
        self.assertEqual(roots["qubit-function"], Path("/candidate-function"))

    def test_manifest_rewrite_uses_crate_directories_metadata_uses_manifest_files(self):
        roots = checker.checkout_roots(Path("/siblings"), Path("/candidate-function"))
        expected = checker.expected_manifests(roots)
        original = ('[dev-dependencies]\n'
                    'qubit-function = "^0.18"\n'
                    'qubit-executor = { path = "../../", version = "^0.9" }\n'
                    'qubit-rayon-executor = "^0.9"\n'
                    'qubit-tokio-executor = "^0.12"\n'
                    'qubit-thread-pool = "^0.12"\n')
        converted = checker.rewrite_manifest(original, roots)
        import tomllib
        parsed = tomllib.loads(converted)
        for name, root in roots.items():
            self.assertEqual(Path(parsed["dev-dependencies"][name]["path"]), root)
            self.assertEqual(expected[name], root / "Cargo.toml")


class ManifestTests(unittest.TestCase):
    def test_rewrites_known_fixture_without_touching_original(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            manifest = root / "Cargo.toml"
            original = ('[dev-dependencies]\nqubit-executor = { path = "../../", version = "^0.9" }\n'
                        'qubit-function = "^0.18"\nqubit-rayon-executor = { version = "^0.9" }\n'
                        'qubit-thread-pool = { version = "^0.12" }\n'
                        'qubit-tokio-executor = { path = "../../../rs-tokio-executor", version = "^0.12" }\n'
                        '\n[patch.crates-io]\nqubit-executor = { path = "../../../rs-executor" }\n')
            manifest.write_text(original)
            changed = checker.rewrite_manifest(original, {name: root / name for name in checker.TARGETS})
            self.assertIn('qubit-function = { version = "^0.18", path = ', changed)
            self.assertIn('[patch.crates-io]', changed)
            self.assertEqual(original, manifest.read_text())

    def test_rejects_unknown_dependency_keys(self):
        original = ('[dev-dependencies]\n'
                    'qubit-function = { version = "^0.18", features = ["full"] }\n')
        original += ''.join(f'{name} = "^1"\n' for name in checker.TARGETS if name != "qubit-function")
        with self.assertRaisesRegex(ValueError, "unsupported keys"):
            checker.rewrite_manifest(original, {name: Path("/sibling/Cargo.toml") for name in checker.TARGETS})


class LockTests(unittest.TestCase):
    def test_allows_only_target_registry_entries_to_be_replaced(self):
        original = {"package": [
            {"name": "qubit-function", "version": "0.18.4", "source": "registry+crates-io"},
            {"name": "qubit-rayon-executor", "version": "0.9.0", "source": "registry+crates-io"},
            {"name": "unrelated", "version": "1.0.0", "source": "registry+crates-io"},
        ]}
        candidate = {"package": [
            {"name": "unrelated", "version": "1.0.0", "source": "registry+crates-io"},
        ]}
        expected = [
            ("qubit-function", "0.18.4", "registry+crates-io"),
            ("qubit-rayon-executor", "0.9.0", "registry+crates-io"),
        ]
        replacements, unrelated = checker.compare_registry_lock_changes(original, candidate)
        self.assertEqual(replacements, expected)
        self.assertEqual(unrelated, [])

        candidate["package"][0]["version"] = "1.1.0"
        _, unrelated = checker.compare_registry_lock_changes(original, candidate)
        self.assertEqual(unrelated, [
            ("unrelated", "1.0.0", "registry+crates-io"),
            ("unrelated", "1.1.0", "registry+crates-io"),
        ])


class ProcessTests(unittest.TestCase):
    def test_failure_code_and_timeout_reap_process_group(self):
        with tempfile.TemporaryDirectory() as temp:
            log = Path(temp) / "command.log"
            ready = Path(temp) / "child-ready"
            pid_file = Path(temp) / "child.pid"
            marker = Path(temp) / "child-survived"
            child = ("import os,signal,time,pathlib; signal.signal(signal.SIGTERM, signal.SIG_IGN); "
                     "pathlib.Path(%r).write_text(str(os.getpid())); pathlib.Path(%r).touch(); "
                     "time.sleep(6.5); pathlib.Path(%r).write_text('alive')") % (str(pid_file), str(ready), str(marker))
            wait_for_ready = f"while not pathlib.Path({str(ready)!r}).exists():\n time.sleep(.01)"
            parent = ("import pathlib,subprocess,sys,time; subprocess.Popen([sys.executable,'-c',%r]); "
                      "exec(%r); time.sleep(20)") % (child, wait_for_ready)
            with self.assertRaises(TimeoutError):
                checker.run([sys.executable, "-c", parent], log, 1, os.environ.copy())
            time.sleep(0.75)
            self.assertFalse(marker.exists(), "a child process survived timeout cleanup")
            child_pid = int(pid_file.read_text())
            proc_status = Path(f"/proc/{child_pid}/stat")
            if proc_status.exists():
                self.assertEqual(proc_status.read_text().split()[2], "Z", "descendant is still running")
            code, _ = checker.run([sys.executable, "-c", "raise SystemExit(7)"], log, 2, os.environ.copy())
            self.assertEqual(code, 7)


if __name__ == "__main__":
    unittest.main()
