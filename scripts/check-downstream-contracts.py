#!/usr/bin/env python3
"""Run rs-executor adapter contracts against the five local candidate checkouts."""
from __future__ import annotations
import argparse, hashlib, json, os, re, shutil, signal, subprocess, sys, tempfile, time, tomllib, threading
from pathlib import Path

TARGETS = {
    "qubit-function": "rs-function", "qubit-executor": "rs-executor",
    "qubit-rayon-executor": "rs-rayon-executor", "qubit-tokio-executor": "rs-tokio-executor",
    "qubit-thread-pool": "rs-thread-pool",
}


def checkout_roots(sibling_root: Path, function_root: Path) -> dict[str, Path]:
    roots = {name: sibling_root / path for name, path in TARGETS.items()}
    roots["qubit-function"] = function_root
    return roots


def expected_manifests(roots: dict[str, Path]) -> dict[str, Path]:
    return {name: root / "Cargo.toml" for name, root in roots.items()}


def compare_registry_lock_changes(original: dict, candidate: dict):
    def registry_packages(lock):
        return {
            (package.get("name"), package.get("version"), package.get("source"))
            for package in lock.get("package", [])
            if str(package.get("source", "")).startswith("registry+")
        }

    before = registry_packages(original)
    after = registry_packages(candidate)
    removed, added = before - after, after - before
    replacements = sorted(entry for entry in removed if entry[0] in TARGETS)
    unrelated = sorted(entry for entry in (removed | added) if entry not in replacements)
    return replacements, unrelated


def toml_string(value: str) -> str:
    """Encode a TOML basic string while keeping non-BMP Unicode scalar values."""
    return json.dumps(value, ensure_ascii=False)


def check_metadata(metadata, expected):
    errors = []
    packages = metadata.get("packages") if isinstance(metadata, dict) else None
    if not isinstance(packages, list):
        return ["metadata must contain a packages array"]
    for name, manifest in expected.items():
        found = [p for p in packages if isinstance(p, dict) and p.get("name") == name]
        if len(found) != 1:
            errors.append(f"{name}: expected exactly one package, found {len(found)}")
            continue
        package = found[0]
        if package.get("source") is not None:
            errors.append(f"{name}: expected local source (null), got {package.get('source')!r}")
        actual = package.get("manifest_path")
        if not isinstance(actual, str) or Path(actual).resolve() != manifest.resolve():
            errors.append(f"{name}: expected manifest {manifest.resolve()}, got {actual!r}")
    return errors


def rewrite_manifest(original: str, paths: dict[str, Path]) -> str:
    """Rewrite the known fixture manifest or fail closed."""
    try:
        data = tomllib.loads(original)
    except tomllib.TOMLDecodeError as error:
        raise ValueError(f"invalid fixture Cargo.toml: {error}") from error
    dependencies = data.get("dev-dependencies", {})
    if set(TARGETS) - set(dependencies):
        raise ValueError("fixture dev-dependencies do not contain all five expected packages")
    lines, section, replaced = [], "", set()
    patched = set()
    for line in original.splitlines(keepends=True):
        if line.strip().startswith("[") and line.strip().endswith("]"):
            section = line.strip()[1:-1]
        match = re.match(r'^(\s*)([A-Za-z0-9_-]+)\s*=\s*(.*?)(\r?\n)?$', line)
        if section == "patch.crates-io" and match and match.group(2) in TARGETS:
            name = match.group(2)
            line = f'{match.group(1)}{name} = {{ path = {toml_string(str(paths[name]))} }}{match.group(4) or ""}'
            patched.add(name)
        if section == "dev-dependencies" and match and match.group(2) in TARGETS:
            name, value = match.group(2), dependencies[match.group(2)]
            version = value if isinstance(value, str) else value.get("version") if isinstance(value, dict) else None
            if not isinstance(version, str):
                raise ValueError(f"unsupported fixture dependency declaration for {name}")
            if isinstance(value, dict) and set(value) - {"version", "path"}:
                raise ValueError(f"unsupported keys for fixture dependency {name}: {sorted(set(value) - {'version', 'path'})}")
            line = f'{match.group(1)}{name} = {{ version = {toml_string(version)}, path = {toml_string(str(paths[name]))} }}{match.group(4) or ""}'
            replaced.add(name)
        lines.append(line)
    if replaced != set(TARGETS):
        raise ValueError(f"could not safely rewrite dependencies: found {sorted(replaced)}")
    result = "".join(lines)
    if "[patch.crates-io]" not in original:
        result += "\n[patch.crates-io]\n"
    for name in TARGETS:
        if name not in patched:
            result += f'{name} = {{ path = {toml_string(str(paths[name]))} }}\n'
    try:
        tomllib.loads(result)
    except tomllib.TOMLDecodeError as error:
        raise ValueError(f"rewritten fixture Cargo.toml is invalid: {error}") from error
    return result


def _cleanup_process_group(process):
    """Terminate the command's session, escalating even if its leader exits."""
    errors = []
    previous_sigint_handler = None
    if threading.current_thread() is threading.main_thread():
        previous_sigint_handler = signal.signal(signal.SIGINT, signal.SIG_IGN)

    try:
        try:
            os.killpg(process.pid, signal.SIGTERM)
        except ProcessLookupError:
            pass
        else:
            deadline = time.monotonic() + 5
            while True:
                try:
                    os.killpg(process.pid, 0)
                except ProcessLookupError:
                    break
                remaining = deadline - time.monotonic()
                if remaining <= 0:
                    try:
                        os.killpg(process.pid, signal.SIGKILL)
                    except ProcessLookupError:
                        pass
                    break
                time.sleep(min(0.05, remaining))
    except BaseException as error:
        errors.append(error)
    finally:
        try:
            process.wait(timeout=5)
        except BaseException as error:
            errors.append(error)
        if previous_sigint_handler is not None:
            try:
                signal.signal(signal.SIGINT, previous_sigint_handler)
            except BaseException as error:
                errors.append(error)

    if errors:
        if any(not isinstance(error, Exception) for error in errors):
            raise BaseExceptionGroup("process-group cleanup failed", errors)
        raise ExceptionGroup("process-group cleanup failed", errors)


def run(command, log, timeout, env, stderr_log=None):
    log.write_text("# $ " + " ".join(map(str, command)) + "\n", encoding="utf-8")
    start = time.monotonic()
    stderr_stream = stderr_log.open("w", encoding="utf-8") if stderr_log else None
    try:
        with log.open("a", encoding="utf-8") as stream:
            process = subprocess.Popen(command, stdout=stream,
                                       stderr=stderr_stream if stderr_stream else subprocess.STDOUT,
                                       env=env, start_new_session=True)
            try:
                code = process.wait(timeout=timeout)
            except subprocess.TimeoutExpired as original:
                try:
                    _cleanup_process_group(process)
                except BaseException as cleanup_error:
                    raise BaseExceptionGroup(
                        "command timed out and process-group cleanup failed",
                        [original, cleanup_error],
                    )
                raise TimeoutError(f"command timed out after {timeout}s: {command[0]}") from original
            except BaseException as original:
                try:
                    _cleanup_process_group(process)
                except BaseException as cleanup_error:
                    raise BaseExceptionGroup(
                        "command failed and process-group cleanup failed",
                        [original, cleanup_error],
                    )
                raise
    finally:
        if stderr_stream:
            stderr_stream.close()
    return code, round(time.monotonic() - start, 3)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--sibling-root", required=True, type=Path)
    parser.add_argument("--report-dir", type=Path)
    parser.add_argument("--timeout", type=int, default=1200)
    args = parser.parse_args()
    sibling_root = args.sibling_root.resolve()
    function_root = Path(__file__).resolve().parents[1]
    roots = checkout_roots(sibling_root, function_root)
    report_dir = args.report_dir.resolve() if args.report_dir else Path(tempfile.mkdtemp(prefix="rs-function-downstream-report-")).resolve()
    if any(report_dir == root.resolve() or root.resolve() in report_dir.parents
           for root in [function_root, sibling_root, *(sibling_root / p for p in TARGETS.values())]):
        parser.error("--report-dir must be outside all five checkouts")
    if args.timeout <= 0:
        parser.error("--timeout must be positive")
    if args.report_dir and report_dir.exists() and any(report_dir.iterdir()):
        parser.error("--report-dir must be absent or empty")
    report_dir.mkdir(parents=True, exist_ok=True)
    report = report_dir / "downstream-contracts.md"
    lines = ["# Candidate downstream contract audit", "", f"Sibling root: `{sibling_root}`", ""]
    expected = expected_manifests(roots)
    try:
        for name, root in roots.items():
            manifest = root / "Cargo.toml"
            if not root.is_dir() or not manifest.is_file():
                raise ValueError(f"required checkout or Cargo.toml missing: {root}")
            head = subprocess.run(["git", "-C", str(root), "rev-parse", "HEAD"], check=True, text=True, capture_output=True).stdout.strip()
            status = subprocess.run(["git", "-C", str(root), "status", "--porcelain=v1"], check=True, text=True, capture_output=True).stdout
            diff = subprocess.run(["git", "-C", str(root), "diff", "--stat", "HEAD"], check=True, text=True, capture_output=True).stdout.strip()
            untracked = subprocess.run(["git", "-C", str(root), "ls-files", "--others", "--exclude-standard"], check=True, text=True, capture_output=True).stdout.strip()
            summary = "; ".join(part for part in (diff, f"untracked: {untracked}" if untracked else "") if part) or "clean"
            lines.append(f"- {name}: `{head}`, dirty={bool(status)}; status=`{status.strip() or 'clean'}`; diff summary=`{summary}`")
        env = os.environ.copy()
        with tempfile.TemporaryDirectory(prefix="rs-function-downstream-") as temporary:
            temp = Path(temporary)
            fixture = temp / "adapter_contracts"
            shutil.copytree(sibling_root / "rs-executor/tests/adapter_contracts", fixture, ignore=shutil.ignore_patterns("target", ".infra"))
            manifest = fixture / "Cargo.toml"
            original = manifest.read_text(encoding="utf-8")
            manifest.write_text(rewrite_manifest(original, roots), encoding="utf-8")
            lock = fixture / "Cargo.lock"
            original_lock = lock.read_bytes()
            original_locked = tomllib.loads(original_lock.decode("utf-8"))
            env["CARGO_TARGET_DIR"] = str(temp / "target")
            metadata_cmd = ["cargo", "+1.94.0", "metadata", "--format-version", "1", "--manifest-path", str(manifest)]
            for index, extra in enumerate(([], ["--locked"]), 1):
                log = report_dir / f"metadata-{index}.log"
                code, elapsed = run(metadata_cmd + extra, log, args.timeout, env,
                                    report_dir / f"metadata-{index}.stderr.log")
                if code:
                    raise RuntimeError(f"metadata command failed with exit {code}; see {log}")
                raw = log.read_text(encoding="utf-8").split("\n", 1)[1]
                metadata = json.loads(raw)
                (report_dir / f"metadata-{index}.json").write_text(json.dumps(metadata, indent=2) + "\n", encoding="utf-8")
                errors = check_metadata(metadata, expected)
                if errors:
                    raise RuntimeError("source validation failed: " + "; ".join(errors))
                lines.append(f"- `cargo +1.94.0 metadata {'--locked' if extra else '(resolve)'}`: exit 0, {elapsed}s; five exact local sources verified.")
            final_locked = tomllib.loads(lock.read_text(encoding="utf-8"))
            target_replacements, changed_registry = compare_registry_lock_changes(original_locked, final_locked)
            lines.append(f"- Temporary Cargo.lock SHA-256: `{hashlib.sha256(lock.read_bytes()).hexdigest()}`")
            lines.append(f"- Target registry entries replaced by local path patches: `{target_replacements or 'none'}`")
            lines.append(f"- Unrelated registry package lock changes: `{changed_registry or 'none'}`")
            if changed_registry:
                raise RuntimeError("Cargo.lock changed unrelated registry dependencies: " + repr(changed_registry))
            code, elapsed = run(["cargo", "+1.94.0", "test", "--locked", "--manifest-path", str(manifest)], report_dir / "cargo-test.log", args.timeout, env)
            lines.append(f"- `cargo +1.94.0 test --locked`: exit {code}, {elapsed}s")
            if code:
                raise RuntimeError(f"adapter contract tests failed with exit {code}; see cargo-test.log")
        lines += ["", "Result: PASS", ""]
        report.write_text("\n".join(lines), encoding="utf-8")
        print(f"PASS; report: {report}")
        return 0
    except KeyboardInterrupt:
        lines += ["", "Result: INTERRUPTED", ""]
        report.write_text("\n".join(lines), encoding="utf-8")
        print(f"INTERRUPTED; report: {report}", file=sys.stderr)
        return 130
    except Exception as error:
        lines += ["", f"Result: FAIL — {type(error).__name__}: {error}", ""]
        report.write_text("\n".join(lines), encoding="utf-8")
        print(f"FAIL: {error}; report: {report}", file=sys.stderr)
        return 1

if __name__ == "__main__":
    raise SystemExit(main())
