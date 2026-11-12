#!/usr/bin/env python3
"""Run rs-executor adapter contracts against the five local candidate checkouts."""
from __future__ import annotations
import argparse, hashlib, json, os, re, shutil, signal, subprocess, sys, tempfile, time, tomllib
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
            line = f'{match.group(1)}{name} = {{ path = {json.dumps(str(paths[name]))} }}{match.group(4) or ""}'
            patched.add(name)
        if section == "dev-dependencies" and match and match.group(2) in TARGETS:
            name, value = match.group(2), dependencies[match.group(2)]
            version = value if isinstance(value, str) else value.get("version") if isinstance(value, dict) else None
            if not isinstance(version, str):
                raise ValueError(f"unsupported fixture dependency declaration for {name}")
            if isinstance(value, dict) and set(value) - {"version", "path"}:
                raise ValueError(f"unsupported keys for fixture dependency {name}: {sorted(set(value) - {'version', 'path'})}")
            line = f'{match.group(1)}{name} = {{ version = {json.dumps(version)}, path = {json.dumps(str(paths[name]))} }}{match.group(4) or ""}'
            replaced.add(name)
        lines.append(line)
    if replaced != set(TARGETS):
        raise ValueError(f"could not safely rewrite dependencies: found {sorted(replaced)}")
    result = "".join(lines)
    if "[patch.crates-io]" not in original:
        result += "\n[patch.crates-io]\n"
    for name in TARGETS:
        if name not in patched:
            result += f'{name} = {{ path = {json.dumps(str(paths[name]))} }}\n'
    return result


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
            except subprocess.TimeoutExpired:
                os.killpg(process.pid, signal.SIGTERM)
                # Keep the process group alive through the full grace period:
                # the leader may exit on SIGTERM while descendants ignore it.
                time.sleep(5)
                try:
                    os.killpg(process.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                process.wait()
                raise TimeoutError(f"command timed out after {timeout}s: {command[0]}")
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
            def registry_packages(doc):
                return {(p.get("name"), p.get("version"), p.get("source"))
                        for p in doc.get("package", []) if str(p.get("source", "")).startswith("registry+")}
            changed_registry = sorted(registry_packages(original_locked) ^ registry_packages(final_locked))
            lines.append(f"- Temporary Cargo.lock SHA-256: `{hashlib.sha256(lock.read_bytes()).hexdigest()}`")
            lines.append(f"- Registry package lock changes: `{changed_registry or 'none'}`")
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
    except Exception as error:
        lines += ["", f"Result: FAIL — {type(error).__name__}: {error}", ""]
        report.write_text("\n".join(lines), encoding="utf-8")
        print(f"FAIL: {error}; report: {report}", file=sys.stderr)
        return 1

if __name__ == "__main__":
    raise SystemExit(main())
