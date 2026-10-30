# Code Coverage Guide

This project uses `cargo-llvm-cov` for local and CI coverage collection and
reporting. The project entry point is `./.infra/bin/coverage.sh`; it is intended
for contributor checks and is excluded from published crates.

## Dependencies

Install `cargo-llvm-cov`, the LLVM tools component for the active Rust toolchain,
and `jq`:

```bash
cargo install cargo-llvm-cov
rustup component add llvm-tools-preview
```

The collection tool uses `cargo-llvm-cov` to collect coverage and check
thresholds. After collection succeeds, the shared report script exports the
reports and uses `jq` to read configuration and build the summary files.

## Quick Start

```bash
./.infra/bin/coverage.sh --help
./.infra/bin/coverage.sh
```

The help output lists the collection options accepted by the underlying
`rs-infra-coverage collect` command, including `--output` and
`--ignore-thresholds`. The wrapper runs the report export after collection; if
help succeeds without producing coverage data, the export step can still fail
because there are no `.profraw` files.

The tool checks selected source files using aggregate covered and total counts,
not a separate threshold for every file. A project's explicit configuration in
`.infra/coverage/coverage.json` takes precedence. Missing thresholds use these
tool defaults:

- Lines: `> 90%`
- Functions: `>= 95%`
- Regions: `> 85%`

This project currently has no explicit coverage configuration, so these tool
defaults apply. Configure project policy in `.infra/coverage/coverage.json`
rather than through undocumented `MIN_*` environment variables.

## Collection and Reports

The wrapper first runs `rs-infra-coverage collect`. It writes the raw LLVM JSON
report to `target/infra/coverage/raw.json` by default and checks thresholds. You
can choose another raw report path with the supported `--output <PATH>` option.
If collection or threshold checking fails, the wrapper stops before report
export.

After successful collection, the shared report script uses `cargo llvm-cov
report` to export these files:

- JSON: `coverage.json`
- LCOV: `lcov.info`
- Text: `coverage.txt`
- Aggregate JSON summary: `ci-summary.json`
- Coverage badge data: `coverage-badge.json`
- Markdown summary: `coverage-summary.md`
- HTML: `target/llvm-cov/html/index.html`
- Cobertura XML: `target/llvm-cov/cobertura.xml`

The HTML report is generated on disk; the project wrapper does not request that
a browser open automatically.

## Direct `cargo llvm-cov` Usage

Direct commands are useful for ad hoc inspection. They do not run the project's
collection tool or its configured threshold checks:

```bash
cargo llvm-cov clean
cargo llvm-cov --html --open
cargo llvm-cov --lcov --output-path target/llvm-cov/lcov.info
cargo llvm-cov --json --output-path target/llvm-cov/coverage.json
cargo llvm-cov --cobertura --output-path target/llvm-cov/cobertura.xml
```

The `integration_tests` target runs with the default feature set. To run the
specific filtered test below, enable the `rc` feature and pass the test path
filter after `--`:

```bash
cargo llvm-cov --features rc --html --open --test integration_tests -- testers::tester_2_tests::tests::test_always_true
```

The argument after `--` is a libtest test-path filter string, not a Cargo test
target name.

## Compile-Contract Cache

Compile-contract tests keep their Cargo build cache at
`<checkout>/target/compile-contract/<feature_key>`. Their source fixtures remain
in temporary directories and are removed after each invocation; the
feature-specific target directory is a persistent, reusable cache. Setting
`CARGO_TARGET_DIR` for the checkout does not change this helper cache location.
Normal test runs do not clear the shared cache. If cleanup is needed, target
only the confirmed compile-contract cache under the current checkout; the helper
does not scan historical `/tmp` leftovers.

## Exclusions

`.llvm-cov.toml` excludes test, benchmark, and example files from reports:

- `tests/*`
- `benches/*`
- `examples/*`

The collection tool also filters Cargo registry, rustup, and sibling workspace
crates so reports cover this crate's source files.

## CI

The repository workflow delegates to the reusable `rs-infra-ci` workflow on
`main`. That workflow runs `./.infra/bin/coverage.sh` without extra collection
arguments. The collection tool enforces thresholds from the project
configuration or its defaults; because collection runs before export, a
collection or threshold failure prevents report export and artifact upload. On
success, CI publishes the generated reports as the `coverage-reports` artifact
and adds a summary to the job. The reusable workflow and coverage tool follow
their upstream `main` revisions, so their exact behavior can change over time.

## Common Issues

If `cargo-llvm-cov` is missing, install it and add `llvm-tools-preview` to the
active toolchain. Install `jq` as well: the report export and summary stages use
it even though threshold checking is performed during collection.

If coverage data looks stale, use the supported cleanup subcommand directly:

```bash
cargo llvm-cov clean
```

This cleans the current checkout's cargo-llvm-cov data; it does not remove the
separate compile-contract cache.

## References

- [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov)
- [LLVM Coverage Mapping](https://llvm.org/docs/CoverageMappingFormat.html)
