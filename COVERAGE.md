# Code Coverage Guide

This project uses `cargo-llvm-cov` for local and CI coverage reporting.
The coverage script is intended for contributor checks and is excluded from
published crates.

## Dependencies

Install the coverage tool and its LLVM component:

```bash
cargo install cargo-llvm-cov
rustup component add llvm-tools-preview
```

The `json` and `all` formats also require `jq` because the script validates
per-file thresholds from the generated JSON report.

## Quick Start

Use `coverage.sh` for normal checks:

```bash
./coverage.sh              # Generate HTML and open it in a browser
./coverage.sh text         # Print a text report to the terminal
./coverage.sh lcov         # Generate LCOV
./coverage.sh json         # Generate JSON and enforce thresholds
./coverage.sh cobertura    # Generate Cobertura XML
./coverage.sh all          # Run tests once and generate all report formats
./coverage.sh all --clean  # Clean old coverage data first
./coverage.sh help         # Show all options
```

`json` and `all` enforce the default local thresholds for every source file:

- Functions: `100%`
- Lines: `> 95%`
- Regions: `> 95%`

The thresholds can be overridden for stricter checks:

```bash
MIN_FUNCTION_COVERAGE=100 MIN_LINE_COVERAGE=98 MIN_REGION_COVERAGE=98 ./coverage.sh json
```

## Report Locations

Generated reports are written under `target/llvm-cov`:

- HTML: `target/llvm-cov/html/index.html`
- LCOV: `target/llvm-cov/lcov.info`
- JSON: `target/llvm-cov/coverage.json`
- Cobertura: `target/llvm-cov/cobertura.xml`
- Text: `target/llvm-cov/coverage.txt` (`all` only)

## How `all` Works

`./coverage.sh all` runs tests once with `cargo llvm-cov --no-report`, then
uses `cargo llvm-cov report` to generate HTML, LCOV, JSON, Cobertura, and text
reports from the same coverage data. This avoids repeated test execution.

## Direct `cargo llvm-cov` Usage

Direct commands are useful for ad hoc inspection, but they do not enforce this
project's per-file thresholds:

```bash
cargo llvm-cov clean
cargo llvm-cov --html --open
cargo llvm-cov --lcov --output-path target/llvm-cov/lcov.info
cargo llvm-cov --json --output-path target/llvm-cov/coverage.json
cargo llvm-cov --cobertura --output-path target/llvm-cov/cobertura.xml
```

To filter tests, pass normal test filters after `--`:

```bash
cargo llvm-cov --html --open --test tester_tests -- test_always_true
```

## Exclusions

`.llvm-cov.toml` excludes test, benchmark, and example files from reports:

- `tests/*`
- `benches/*`
- `examples/*`

`coverage.sh` also filters Cargo registry, rustup, and sibling workspace crates
so reports only cover this crate's source files.

## CI

The reusable GitHub Actions workflow runs `./coverage.sh all` with
`COVERAGE_ENFORCE_THRESHOLDS=0`, publishes coverage artifacts, and reports the
aggregate result without applying per-source thresholds. The local
`./ci-check.sh` command runs `./coverage.sh json` with threshold enforcement
enabled by default.

## Common Issues

If `cargo-llvm-cov` is missing, install it and add `llvm-tools-preview` to the
active toolchain.

If `json` or `all` fails before threshold checking, install `jq`.

If coverage data looks stale, run:

```bash
./coverage.sh json --clean
```

## References

- [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov)
- [LLVM Coverage Mapping](https://llvm.org/docs/CoverageMappingFormat.html)
