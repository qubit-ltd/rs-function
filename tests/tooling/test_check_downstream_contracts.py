import importlib.util
import json
import signal
import subprocess
import tempfile
import unittest
import os
import unittest.mock
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
    def test_rewritten_manifest_round_trips_non_bmp_paths(self):
        import tomllib
        with tempfile.TemporaryDirectory() as temp:
            unicode_root = Path(temp) / "candidate-🧪-路径"
            unicode_root.mkdir()
            paths = {name: unicode_root / directory for name, directory in checker.TARGETS.items()}
            original = ('[dev-dependencies]\n'
                        'qubit-function = "^0.18"\nqubit-executor = "^0.9"\n'
                        'qubit-rayon-executor = "^0.9"\nqubit-tokio-executor = "^0.12"\n'
                        'qubit-thread-pool = "^0.12"\n')
            parsed = tomllib.loads(checker.rewrite_manifest(original, paths))
            for name, crate_root in paths.items():
                self.assertEqual(Path(parsed["dev-dependencies"][name]["path"]), crate_root)
                self.assertEqual(Path(parsed["patch"]["crates-io"][name]["path"]), crate_root)

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
    def test_keyboard_interrupt_cleans_group_before_reraise(self):
        class FakeProcess:
            pid = 12345
            waits = 0

            def wait(self, timeout=None):
                self.waits += 1
                if self.waits == 1:
                    raise KeyboardInterrupt()
                return -signal.SIGKILL

        with tempfile.TemporaryDirectory() as temp:
            log = Path(temp) / "command.log"
            stderr_log = Path(temp) / "command.stderr.log"
            fake = FakeProcess()
            original_handler = checker.signal.getsignal(signal.SIGINT)
            opened_stderr = []
            original_open = Path.open

            def track_open(path, *args, **kwargs):
                stream = original_open(path, *args, **kwargs)
                if path == stderr_log:
                    opened_stderr.append(stream)
                return stream

            with unittest.mock.patch.object(checker.subprocess, "Popen", return_value=fake), \
                 unittest.mock.patch.object(checker.os, "killpg", side_effect=[None, ProcessLookupError()]) as killpg, \
                 unittest.mock.patch.object(checker.time, "sleep"), \
                 unittest.mock.patch.object(Path, "open", autospec=True, side_effect=track_open):
                with self.assertRaises(KeyboardInterrupt):
                    checker.run(["fake"], log, 2, os.environ.copy(), stderr_log)
            self.assertEqual(killpg.call_args_list[0].args, (12345, checker.signal.SIGTERM))
            self.assertEqual(fake.waits, 2)
            self.assertEqual(len(opened_stderr), 1)
            self.assertTrue(opened_stderr[0].closed)
            self.assertIs(checker.signal.getsignal(signal.SIGINT), original_handler)

    def test_creation_failure_does_not_signal_process_group(self):
        with tempfile.TemporaryDirectory() as temp:
            with unittest.mock.patch.object(checker.subprocess, "Popen", side_effect=OSError("spawn failed")), \
                 unittest.mock.patch.object(checker.os, "killpg") as killpg:
                with self.assertRaisesRegex(OSError, "spawn failed"):
                    checker.run(["fake"], Path(temp) / "command.log", 2, os.environ.copy())
            killpg.assert_not_called()

    def test_disappeared_group_does_not_hide_original_exception(self):
        class FakeProcess:
            pid = 12346
            waits = 0

            def wait(self, timeout=None):
                self.waits += 1
                if self.waits == 1:
                    raise KeyboardInterrupt()
                return -signal.SIGTERM

        with tempfile.TemporaryDirectory() as temp:
            with unittest.mock.patch.object(checker.subprocess, "Popen", return_value=FakeProcess()), \
                 unittest.mock.patch.object(checker.os, "killpg", side_effect=ProcessLookupError()):
                with self.assertRaises(KeyboardInterrupt):
                    checker.run(["fake"], Path(temp) / "command.log", 2, os.environ.copy())

    def test_cleanup_error_keeps_original_exception_context(self):
        class FakeProcess:
            pid = 12347
            waits = 0

            def wait(self, timeout=None):
                self.waits += 1
                if self.waits == 1:
                    raise KeyboardInterrupt()
                return -signal.SIGTERM

        with tempfile.TemporaryDirectory() as temp:
            with unittest.mock.patch.object(checker.subprocess, "Popen", return_value=FakeProcess()), \
                 unittest.mock.patch.object(checker.os, "killpg", side_effect=PermissionError("denied")):
                with self.assertRaises(BaseExceptionGroup) as raised:
                    checker.run(["fake"], Path(temp) / "command.log", 2, os.environ.copy())
            self.assertIsInstance(raised.exception.exceptions[0], KeyboardInterrupt)
            cleanup_errors = raised.exception.exceptions[1].exceptions
            self.assertIsInstance(cleanup_errors[0], PermissionError)

    @unittest.skipUnless(os.name == "posix", "requires POSIX process groups and SIGINT")
    def test_sigint_reaps_descendant_and_second_sigint_is_deferred(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            bin_dir = root / "bin"
            bin_dir.mkdir()
            sibling_root = root / "siblings"
            sibling_root.mkdir()
            for directory in checker.TARGETS.values():
                (sibling_root / directory).mkdir()
                (sibling_root / directory / "Cargo.toml").write_text("[package]\nname = 'fixture'\n", encoding="utf-8")
            fixture = sibling_root / checker.TARGETS["qubit-executor"] / "tests" / "adapter_contracts"
            fixture.mkdir(parents=True)
            (fixture / "Cargo.toml").write_text(
                "[package]\nname = 'adapter_contracts'\nversion = '0.1.0'\nedition = '2024'\n\n"
                "[dev-dependencies]\n"
                + "\n".join(f"{name} = '^0.1'" for name in checker.TARGETS)
                + "\n",
                encoding="utf-8",
            )
            (fixture / "Cargo.lock").write_text("version = 4\n\n[[package]]\nname = 'adapter_contracts'\nversion = '0.1.0'\n", encoding="utf-8")
            ready = root / "ready"
            leader_pid_file = root / "leader.pid"
            descendant_pid_file = root / "descendant.pid"
            marker = root / "survived"
            report_dir = root / "report"
            expected = checker.expected_manifests(
                checker.checkout_roots(sibling_root, SCRIPT.resolve().parents[1])
            )
            metadata = json.dumps({"packages": [
                {"name": name, "source": None, "manifest_path": str(path)}
                for name, path in expected.items()
            ]})
            cargo = bin_dir / "cargo"
            cargo.write_text("""#!/usr/bin/env python3
import json, os, pathlib, subprocess, sys, time
args = sys.argv[1:]
if 'metadata' in args:
    print(os.environ['FAKE_CARGO_METADATA'])
    raise SystemExit(0)
pathlib.Path(os.environ['FAKE_CARGO_LEADER_PID']).write_text(str(os.getpid()))
descendant = (
    "import os,pathlib,signal,time; signal.signal(signal.SIGTERM, signal.SIG_IGN); "
    "pathlib.Path(os.environ['FAKE_CARGO_DESCENDANT_PID']).write_text(str(os.getpid())); "
    "pathlib.Path(os.environ['FAKE_CARGO_READY']).touch(); time.sleep(7); "
    "pathlib.Path(os.environ['FAKE_CARGO_MARKER']).write_text('alive')"
)
subprocess.Popen([sys.executable, '-c', descendant])
time.sleep(30)
""", encoding="utf-8")
            cargo.chmod(0o755)
            git = bin_dir / "git"
            git.write_text(
                "#!/usr/bin/env python3\nimport sys\n"
                "if sys.argv[-2] == 'rev-parse': print('0123456789abcdef0123456789abcdef01234567')\n",
                encoding="utf-8",
            )
            git.chmod(0o755)
            env = os.environ.copy()
            env["PATH"] = str(bin_dir) + os.pathsep + env["PATH"]
            env.update({
                "FAKE_CARGO_METADATA": metadata,
                "FAKE_CARGO_LEADER_PID": str(leader_pid_file),
                "FAKE_CARGO_DESCENDANT_PID": str(descendant_pid_file),
                "FAKE_CARGO_READY": str(ready),
                "FAKE_CARGO_MARKER": str(marker),
            })
            process = subprocess.Popen(
                [sys.executable, str(SCRIPT), "--sibling-root", str(sibling_root),
                 "--report-dir", str(report_dir)],
                env=env, start_new_session=True,
            )
            leader_pid = None
            descendant_pid = None
            try:
                deadline = time.monotonic() + 10
                while not ready.exists() or not leader_pid_file.exists() or not descendant_pid_file.exists():
                    if process.poll() is not None:
                        self.fail(f"checker exited before readiness with {process.returncode}")
                    if time.monotonic() >= deadline:
                        self.fail("controlled Cargo process tree did not become ready")
                    time.sleep(0.01)
                leader_pid = int(leader_pid_file.read_text())
                descendant_pid = int(descendant_pid_file.read_text())
                os.kill(process.pid, signal.SIGINT)
                time.sleep(0.05)
                os.kill(process.pid, signal.SIGINT)
                self.assertEqual(process.wait(timeout=10), 130)
                report = (report_dir / "downstream-contracts.md").read_text()
                self.assertIn("Result: INTERRUPTED", report)
                self.assertFalse(Path(f"/proc/{leader_pid}").exists(), "direct child was not waited and reaped")
                self.assertFalse(marker.exists(), "a descendant survived SIGINT cleanup")
                proc_status = Path(f"/proc/{descendant_pid}/stat")
                if proc_status.exists():
                    self.assertEqual(proc_status.read_text().split()[2], "Z", "descendant is still running")
            finally:
                if process.poll() is None:
                    os.killpg(process.pid, signal.SIGKILL)
                    process.wait(timeout=5)
                if leader_pid is not None:
                    try:
                        os.killpg(leader_pid, signal.SIGKILL)
                    except ProcessLookupError:
                        pass

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
