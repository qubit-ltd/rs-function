# rs-function 0.16 Migration Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Release the refactored API as `0.16.0`, make feature boundaries match their documented contracts, repair coverage threshold enforcement upstream, and migrate every direct workspace consumer to the unpublished path dependency.

**Architecture:** Keep the existing semantic-trait and wrapper design. Repair feature behavior at the generated-method and owning-module boundaries, keep the API-family reduction out of scope, and migrate downstream construction to explicit wrapper constructors. Fix the shared coverage implementation in `rs-ci`, publish that commit to `dev-starfish`, `dev`, and `main`, then update the `.rs-ci` gitlink in `rs-function`.

**Tech Stack:** Rust 1.94, Cargo feature matrices, Bash, jq, Python unittest, Git submodules.

## Global Constraints

- `qubit-function` becomes version `0.16.0`.
- Direct downstream crates use `qubit-function = { path = "../rs-function" }` until publication.
- Public API-family reduction and removal are explicitly out of scope.
- Existing uncommitted downstream changes belong to the user and must be preserved.
- Only `rs-ci` is committed and pushed; its commit message is English.
- The `rs-ci` commit must reach `dev-starfish`, `dev`, and `main`, and the checkout must finish on its original branch.
- Production-code changes follow red-green-refactor; documentation and manifest-only edits use direct verification.

---

### Task 1: Repair upstream coverage threshold enforcement

**Files:**
- Modify: `../rs-ci/tests/coverage_script_tests.py`
- Modify: `../rs-ci/coverage.sh`

**Interfaces:**
- Consumes: LLVM coverage JSON file summaries.
- Produces: `check_json_coverage` that rejects a source file whenever its summary violates a configured threshold, independent of segment encoding.

- [ ] **Step 1: Add a regression test**

  Extend the fake Cargo command so the JSON output path receives a supplied fixture. Add a test whose file summary reports region coverage below the threshold while its segment list contains no counted zero segment. Assert that `coverage.sh json` exits non-zero and prints `per-source coverage thresholds failed`.

- [ ] **Step 2: Verify RED**

  Run `python3 -m unittest tests.coverage_script_tests -v` in `rs-ci`.

  Expected: the new regression test fails because the current script reports success.

- [ ] **Step 3: Implement the minimal fix**

  Remove `$has_uncovered_region` and enforce the three summary comparisons directly:

  ```jq
  select(
      (($summary.functions.count > 0) and ($summary.functions.percent < $min_functions))
      or (($summary.lines.count > 0) and ($summary.lines.percent <= $min_lines))
      or (($summary.regions.count > 0) and ($summary.regions.percent <= $min_regions))
  )
  ```

- [ ] **Step 4: Verify GREEN**

  Run `python3 -m unittest discover -s tests -v` and `bash -n coverage.sh`.

  Expected: all tests pass and Bash syntax validation succeeds.

- [ ] **Step 5: Commit and synchronize authorized branches**

  Fetch first, verify branch ancestry, commit as:

  ```text
  fix(coverage): enforce summary thresholds directly
  ```

  Push the commit to `dev-starfish`, fast-forward `dev` and `main`, push both, and return to the original branch. Stop instead of resolving any conflict.

### Task 2: Update the rs-ci submodule and crate version

**Files:**
- Modify: `.rs-ci` gitlink
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`
- Modify: `README.md`
- Modify: `README.zh_CN.md`
- Modify: `COVERAGE.md`

**Interfaces:**
- Consumes: the pushed `rs-ci/main` coverage fix.
- Produces: `qubit-function 0.16.0` and documentation aligned with actual default thresholds and hosted-CI behavior.

- [ ] **Step 1: Synchronize the submodule**

  Run `./update-submodule.sh` from `rs-function` and verify `.rs-ci` points to the new `origin/main` commit.

- [ ] **Step 2: Change the crate version**

  Set the package and lockfile version to `0.16.0`; update README dependency examples to `0.16`.

- [ ] **Step 3: Align coverage documentation to code**

  Document local defaults as functions `>=100%`, lines `>95%`, regions `>95%`; distinguish local `ci-check.sh` enforcement from hosted reusable-workflow reporting.

- [ ] **Step 4: Verify manifest consistency**

  Run `cargo +1.94.0 metadata --no-deps --format-version 1` and verify it reports `qubit-function 0.16.0`.

### Task 3: Lock feature contracts with failing tests

**Files:**
- Create: `tests/feature_contract_tests.rs`
- Modify: `.rs-ci-cargo-matrix.json`

**Interfaces:**
- Consumes: baseline, `rc`, `stateful`, and `combinators` feature selections.
- Produces: regression coverage for stable `Debug`/`Display` implementations and for combinator API absence without the feature.

- [ ] **Step 1: Add compile-time trait assertions**

  Add baseline assertions for `BoxCallable`, `BoxCallableWith`, and `BoxRunnableWith`; add conditional assertions for their Rc and Arc counterparts.

- [ ] **Step 2: Verify RED**

  Run `cargo +1.94.0 test --no-default-features --test feature_contract_tests`.

  Expected: compilation fails because baseline Box types do not currently implement `Debug` and `Display`.

- [ ] **Step 3: Add a nested Cargo compile-fail contract**

  Build a temporary consumer crate with `default-features = false`; assert that `BoxConsumer::when` and the deep `FnTesterOps` path fail without `combinators`, while the corresponding program succeeds with `combinators`.

- [ ] **Step 4: Verify the combinator RED state**

  Run only the negative contract test.

  Expected: it fails because baseline currently exposes the combinator API.

- [ ] **Step 5: Exercise contracts in the feature matrix**

  Run tests, not only checks, for baseline, `rc`, `stateful`, and `combinators` matrix entries.

### Task 4: Repair feature ownership and warnings

**Files:**
- Modify: `src/tasks/{callable,callable_with,runnable_with}/{box,rc,arc}_*.rs`
- Modify: generated combinator macros under `src/{consumers,functions,mutators,predicates,suppliers,transformers}/macros/`
- Modify: hand-written combinators under `src/{comparator,predicates,tasks,testers}/`
- Modify: `src/testers/{tester,stateful_tester}.rs`
- Modify: stateful family imports that use `RefCell` only with `rc`
- Modify: duplicated `rc` attributes in task and supplier modules

**Interfaces:**
- Consumes: feature-contract tests from Task 3.
- Produces: baseline APIs independent of unrelated features; combinator methods available only with `combinators`; warning-free individual feature builds.

- [ ] **Step 1: Move `Debug` and `Display` implementations**

  Put each Box/Rc/Arc implementation in the file that owns that type and remove cross-type implementations from Arc modules.

- [ ] **Step 2: Gate generated and hand-written combinators**

  Apply `#[cfg(feature = "combinators")]` to generated `when`, `and_then`, logical, comparison, mapping, filtering, and zipping methods and to equivalent hand-written methods. Make tester implementation modules private and gate their extension modules.

- [ ] **Step 3: Clean feature-specific imports and duplicate attributes**

  Gate `RefCell` imports with `rc` where appropriate and collapse consecutive duplicate `#[cfg(feature = "rc")]` attributes to one.

- [ ] **Step 4: Verify GREEN**

  Run the feature-contract tests and `./.rs-ci/cargo-feature-check.sh run-all`.

  Expected: all combinations pass with no warnings.

### Task 5: Correct documentation and remove stale test scaffolding

**Files:**
- Modify: affected Rust documentation under `src/`
- Modify: affected examples under `examples/`
- Modify: legacy test files under `tests/`

**Interfaces:**
- Consumes: actual method signatures and allocation/dispatch behavior.
- Produces: documentation that matches code and tests without obsolete empty conversion modules.

- [ ] **Step 1: Correct factual documentation errors**

  Fix the malformed `SupplierOnce` reference, borrowed-input descriptions, Supplier/StatefulSupplier tables, closure blanket-implementation docs, unsupported `10x` and zero-overhead claims, and obsolete conversion references.

- [ ] **Step 2: Remove empty stale test modules**

  Delete empty `conversion`, `to_*`, and obsolete trait-default test modules while preserving all active tests and test files.

- [ ] **Step 3: Verify documentation and tests**

  Run rustdoc with warnings denied, doctests, and the all-feature test suite.

### Task 6: Migrate all direct downstream crates

**Files:**
- Modify: `../rs-{batch,cas,dcl,execution-services,executor,http,progress,rayon-batch,rayon-executor,retry,task,thread-pool,tokio-executor}/Cargo.toml`
- Modify: their `Cargo.lock` files
- Modify: explicit construction call sites in `rs-batch`, `rs-retry`, and `rs-dcl`

**Interfaces:**
- Consumes: local `../rs-function` version `0.16.0`.
- Produces: 13 downstream crates that compile against the unpublished path dependency.

- [ ] **Step 1: Establish RED**

  Run patched checks against the current API and retain the known missing-method/import failures as the migration baseline.

- [ ] **Step 2: Change dependencies to relative paths**

  Use exactly:

  ```toml
  qubit-function = { path = "../rs-function" }
  ```

  Add `features = ["stateful"]` only where shared mutable task wrappers require it.

- [ ] **Step 3: Replace removed conversion calls**

  Use `BoxConsumer::new`, `ArcConsumer::new`, `ArcTester::new`, `ArcBiConsumer::new`, `ArcBiFunction::new`, and the matching explicit constructors required by each field type.

- [ ] **Step 4: Refresh lockfiles without discarding user changes**

  Let Cargo update only the resolution needed by each edited manifest, then inspect each diff to confirm existing dirty changes remain present.

- [ ] **Step 5: Verify GREEN**

  Run `cargo check --all-targets --all-features` in all 13 downstream crates.

  Expected: every crate passes.

### Task 7: Final verification and handoff

**Files:**
- Verify all modified repositories; do not commit or push non-`rs-ci` repositories.

**Interfaces:**
- Consumes: Tasks 1-6.
- Produces: evidence-backed status, repository diffs, and remaining risks.

- [ ] **Step 1: Run rs-function project checks**

  Run feature matrix, formatting, Clippy, rustdoc, tests, package validation, and the repaired coverage command.

- [ ] **Step 2: Re-run all downstream checks**

  Confirm 13/13 pass against relative paths.

- [ ] **Step 3: Audit Git state**

  Confirm the rs-ci branches and remote refs contain the same coverage commit, rs-ci is back on its original branch, rs-function points at that main commit, and no unrelated user changes were discarded.
