# Semantic Callback Objects Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan.

**Goal:** Refactor `rs-function` into an unambiguous semantic callback-object library: keep domain traits and Box/Rc/Arc wrappers, make wrapper chaining a baseline capability, remove every `Fn*Ops` API, consolidate callback metadata, use explicit imports, correct documentation contracts, and pin CI to the checked-out `rs-ci` revision.

**Architecture:** Keep the existing semantic-family layout and public wrapper families. Traits contain only invocation behavior; concrete wrappers own construction, naming, ownership conversion, and composition. A crate-private `CallbackMetadata` stores names as `Option<Arc<str>>`. Stateful wrappers keep their existing `Rc<RefCell<_>>` and `Arc<parking_lot::Mutex<_>>` semantics. No compatibility shim or deprecated `Fn*Ops` facade is retained.

**Tech Stack:** Rust 1.94, Cargo feature matrix, compile-fixture contract tests, rustdoc/doctests, GitHub Actions reusable workflows, repository-provided `align-ci.sh` and `ci-check.sh`.

## Global Constraints

- Work only on branch `refactor/semantic-callback-objects`, created from clean `dev-starfish`.
- Breaking API changes and file deletion are authorized; commits are not authorized. Do not commit unless the user separately grants permission.
- Follow `~/.codex/specs/general.mdc`, `git.mdc`, `rust-coding.mdc`, `rust-comment.mdc`, `rust-test.mdc`, and `shell.mdc`.
- Use tests to expose each behavioral change before changing implementation.
- Preserve meaningful behavior assertions from deleted `Fn*Ops` tests by moving them to concrete-wrapper tests.
- Do not change the package version or sibling crates in this plan.
- Use `apply_patch` for source and documentation edits. Use formatters only for mechanical formatting.
- At each checkpoint inspect `git diff --check` and `git status --short`; do not alter unrelated user changes.

---

### Task 1: Replace the feature/API compile contract with the approved contract

**Files:**

- Modify: `tests/feature_contract_tests.rs`
- Modify: `.rs-ci-cargo-matrix.json`

**Step 1: Write failing baseline composition fixtures**

Replace the tests that expect composition to be absent without `combinators` with fixtures proving that wrapper composition is baseline functionality:

```rust
#[test]
fn test_baseline_accepts_box_consumer_when() {
    let output = compile_consumer(
        &[],
        r#"
use qubit_function::BoxConsumer;

fn main() {
    let consumer = BoxConsumer::new(|_: &i32| {});
    let _conditional = consumer.when(|value: &i32| *value > 0);
}
"#,
    );

    assert!(output.status.success(), "{}", cargo_diagnostics(&output));
}
```

Add equivalent baseline fixtures for `BoxRunnable::then_callable` and `BoxRunnableWith::then_callable_with`. Keep Once task chaining under `features = ["once"]`.

**Step 2: Write failing negative fixtures for the removed extension layer**

Add compile failures for:

- `use qubit_function::*;` followed by a raw closure `.when(...)`, expecting `no method named` rather than multiple applicable items;
- root import `qubit_function::FnTesterOps`, expecting unresolved import;
- deep import `qubit_function::testers::tester::fn_tester_ops::FnTesterOps`, expecting the module to be absent;
- wrapper chaining under `use qubit_function::*;`, expecting success and therefore proving glob import no longer introduces ambiguity.

Use minimal fixtures so the asserted diagnostic is caused only by the intended API contract.

**Step 3: Run the contract tests and confirm RED**

Run:

```bash
cargo +1.94.0 test --test feature_contract_tests --no-default-features
```

Expected: the new wrapper-composition tests fail because composition is still gated, and removal tests fail because `Fn*Ops` is still exported.

**Step 4: Define the target feature matrix**

Change `.rs-ci-cargo-matrix.json` to these checks:

- `baseline`: test, doc;
- `rc`: test;
- `once`: test;
- `stateful`: test;
- `rc-stateful`: test;
- `once-stateful`: test;
- `full`: test, doc, clippy.

Delete `combinators`, `once-combinators`, and `stateful-combinators` entries.

**Step 5: Checkpoint**

Run:

```bash
git diff --check
git status --short
```

Expected: only the contract test, matrix, and previously approved design/plan documents are changed.

---

### Task 2: Delete `Fn*Ops` and make wrapper composition unconditional

**Files:**

- Modify: `Cargo.toml`
- Modify: `src/lib.rs`
- Modify: family module files under `src/comparator.rs`, `src/consumers/`, `src/functions/`, `src/mutators/`, `src/predicates/`, `src/suppliers/`, `src/testers/`, and `src/transformers/`
- Delete: every source file matching `src/**/fn_*ops.rs`, including `src/mutators/stateful_mutator/fn_mut_stateful_mutator_ops.rs`
- Delete: `src/functions/macros/fn_ops_trait.rs`
- Delete or migrate: every test file matching `tests/**/fn_*ops_tests.rs`, including `fn_mut_stateful_mutator_ops_tests.rs`
- Rename or rewrite: `examples/suppliers/closure_supplier_ops_demo.rs`
- Rename or rewrite: `examples/transformers/fn_bi_transformer_ops_demo.rs`
- Rename or rewrite: `examples/transformers/fn_transformer_ops_demo.rs`
- Rename or rewrite: `examples/transformers/fn_transformer_once_ops_demo.rs`

**Step 1: Remove the feature declaration**

Set:

```toml
[features]
default = []
rc = []
once = []
stateful = ["dep:parking_lot"]
full = ["rc", "once", "stateful"]
```

Do not change the package version.

**Step 2: Remove composition feature gates from wrapper code**

Delete `#[cfg(feature = "combinators")]`, `#[cfg_attr(...)]`, and `doc(cfg(...))` gates that control wrapper composition or conditional types. Composition modules and methods must compile whenever their owning wrapper family compiles.

Retain only `rc`, `once`, and `stateful` gates according to ownership and invocation semantics.

**Step 3: Remove extension traits and exports**

Delete all `Fn*Ops` files, their `mod` declarations, and all root/family re-exports. Delete the macro that exists solely to generate those traits.

Verify no Rust source still declares or exports an extension trait:

```bash
rg -n 'Fn[A-Za-z0-9_]*Ops|fn_[a-z0-9_]*ops' src
```

Expected: no matches.

**Step 4: Preserve valuable behavior tests on wrappers**

For each deleted `fn_*ops_tests.rs`, classify assertions:

- delete assertions that only prove a raw closure gained an extension method;
- move short-circuit, ordering, error, naming, conditional, and chaining behavior assertions into the corresponding `box_*_tests.rs`, `rc_*_tests.rs`, or `arc_*_tests.rs`;
- prefer Box for ownership-neutral behavior, adding Rc/Arc coverage only where clone/share behavior differs.

After migration, delete the extension-only test files and remove their `mod` declarations.

**Step 5: Rewrite extension-named examples around explicit wrappers**

Use precise imports and begin chains with the appropriate wrapper constructor. Rename example targets and `Cargo.toml` entries so public examples contain no `fn_*ops` or `closure_*_ops` naming.

**Step 6: Run focused and complete tests**

Run:

```bash
cargo +1.94.0 test --test feature_contract_tests --no-default-features
cargo +1.94.0 test --all-features --all-targets
```

Expected: all contract fixtures pass; all migrated behavior tests pass.

**Step 7: Checkpoint**

Run:

```bash
rg -n 'feature\s*=\s*"combinators"|Fn[A-Za-z0-9_]*Ops|fn_[a-z0-9_]*ops' Cargo.toml src tests examples .rs-ci-cargo-matrix.json
git diff --check
```

Expected: no stale API or feature references and no whitespace errors.

---

### Task 3: Introduce shared callback metadata test-first

**Files:**

- Create: `src/metadata.rs`
- Modify: `src/lib.rs`
- Modify: `src/macros/common_new_methods.rs`
- Modify: `src/macros/common_name_methods.rs`
- Modify: family clone and Debug/Display macros under `src/{consumers,functions,mutators,predicates,suppliers,transformers}/macros/`
- Modify: task wrapper files under `src/tasks/` that implement naming, clone, Debug, or Display without those family macros
- Modify: representative wrapper tests in `tests/consumers/consumer/`, `tests/predicates/predicate/`, `tests/tasks/`, and stateful wrapper test directories

**Step 1: Add failing metadata behavior tests through public wrappers**

For one Box, one Rc, one Arc, one Stateful Rc/Arc, and one task wrapper, assert:

- unnamed `name()` is `None`;
- `with_name`, `set_name`, and `clear_name` update the public name;
- clone keeps the same visible name;
- Debug and Display retain the established output contract;
- changing one clone's metadata does not unexpectedly rename another clone unless the current public contract intentionally shares it.

Run the focused tests and record the current contract before changing storage.

**Step 2: Add the crate-private value object**

Implement:

```rust
use std::sync::Arc;

#[derive(Clone, Default)]
pub(crate) struct CallbackMetadata {
    name: Option<Arc<str>>,
}
```

Add crate-private methods needed by the macros: construct unnamed/named metadata, return `Option<&str>`, set from `impl Into<Arc<str>>` or an equivalent non-leaking input, and clear.

Declare `pub(crate) mod metadata;` in `src/lib.rs`; do not publicly export the type.

**Step 3: Adapt common construction and naming macros**

Change the common macros to initialize and mutate `metadata` rather than a `name: Option<String>` field. Keep public signatures source-simple (`impl Into<String>` or `&str` as currently appropriate) and return `Option<&str>` from `name()`.

**Step 4: Migrate wrappers by family**

In this order, replace `name: Option<String>` with `metadata: CallbackMetadata` and update Clone/Debug/Display construction:

1. comparator and predicates;
2. consumers and mutators;
3. functions and transformers;
4. suppliers, testers, and tasks;
5. conditional and stateful variants.

After each family, run its integration tests, for example:

```bash
cargo +1.94.0 test --all-features --test predicates_tests
```

If the repository uses one top-level integration target rather than a family target, run the corresponding exact test target discovered by `cargo test --all-features --all-targets --no-run` and `cargo test -- --list`.

**Step 5: Verify migration completeness**

Run:

```bash
rg -n 'name:\s*Option<String>' src
cargo +1.94.0 test --all-features --all-targets
```

Expected: no old name storage and all tests pass.

---

### Task 4: Consolidate wrapper internals without changing ownership semantics

**Files:**

- Modify: `src/macros/common_new_methods.rs`
- Modify: `src/macros/common_name_methods.rs`
- Modify: family method/clone/debug macros under:
  - `src/consumers/macros/`
  - `src/functions/macros/`
  - `src/mutators/macros/`
  - `src/predicates/macros/`
  - `src/suppliers/macros/`
  - `src/transformers/macros/`
- Modify: concrete conditional wrappers under the same family directories
- Modify: stateful wrappers under `src/**/stateful_*/`
- Modify: corresponding tests under `tests/`

**Step 1: Add or retain ownership-sensitive tests before consolidation**

Ensure tests cover:

- Box composition consumes the wrapper where its signature promises ownership transfer;
- Rc/Arc composition from `&self` leaves the source usable;
- predicate `and`, `or`, `nand`, and `nor` short-circuit; `xor` evaluates both operands;
- conditional `when(...).or_else(...)` executes exactly one branch;
- errors are propagated without running later stages;
- Once calls consume their wrapper;
- Stateful clones share one callback state;
- Rc reentry panics through `RefCell`; Arc reentry behavior is documented but is not tested by hanging the suite;
- state changes performed before panic remain visible.

Run the affected tests and confirm the assertions pass before refactoring.

**Step 2: Extract only behaviorally identical macro kernels**

Consolidate repeated constructor, naming, clone, and formatting behavior. Conditional implementation may use private kernels, but keep public wrapper return types and ownership signatures explicit.

Do not unify Box, Rc, Arc, Stateful, or Once storage into a higher-order generic container. Do not introduce a second public stateful invocation trait.

**Step 3: Keep lock/borrow scopes visible**

In Stateful Rc/Arc invocation code, bind the `RefCell` borrow or `parking_lot::Mutex` guard in the invocation method so the held scope is obvious and documented. Do not clone or release the underlying callback before invoking it.

**Step 4: Verify behavior after each family**

Run:

```bash
cargo +1.94.0 test --all-features --all-targets
cargo +1.94.0 test --all-features --doc
```

Expected: behavior and doctests pass without adding compatibility APIs.

---

### Task 5: Replace parent-module import preludes with explicit imports

**Files:**

- Modify: all concrete Rust files under `src/comparator/`, `src/consumers/`, `src/functions/`, `src/mutators/`, `src/predicates/`, `src/suppliers/`, `src/tasks/`, `src/testers/`, and `src/transformers/`
- Modify: the aggregate modules `src/comparator.rs` and `src/{consumers,functions,mutators,predicates,suppliers,tasks,testers,transformers}/mod.rs`

**Step 1: Capture the current violations**

Run:

```bash
rg -n '^use super::\{' src
rg -n 'qubit-style: allow explicit-imports' src
```

Save the result in the working log, not in the repository.

**Step 2: Refactor imports family by family**

For each concrete file:

- import `std` names directly from `std::...`;
- import `parking_lot` names directly when stateful code needs them;
- import crate types via explicit `crate::...` paths;
- use `super::...` only for a type actually defined by the direct parent family module;
- remove parent module imports that exist only to feed child modules;
- remove `qubit-style: allow explicit-imports` comments unless a concrete, documented exception remains.

Run `cargo +1.94.0 check --all-features --all-targets` after each top-level family.

**Step 3: Verify import hygiene**

Run:

```bash
rg -n 'qubit-style: allow explicit-imports' src
./.rs-ci/style-check.sh
cargo +1.94.0 clippy --all-features --all-targets -- -D warnings
```

Expected: no blanket allow comments; style and Clippy pass. Any remaining `use super::{...}` must name only direct-parent family items and be individually reviewed.

---

### Task 6: Complete the English/Chinese documentation contract audit

**Files:**

- Modify: `README.md`
- Modify: `README.zh_CN.md`
- Modify: rustdoc in `src/**/*.rs`
- Modify: examples under `examples/`
- Modify: example declarations in `Cargo.toml`

**Step 1: Inventory stale and over-strong claims**

Run:

```bash
rg -n 'Fn[A-Za-z0-9_]*Ops|combinators|no overhead|zero.overhead|one.time use|pure|side.effect free|deterministic|into_mut_fn|to_mut_fn|into_fn|to_fn|same concrete type|无开销|纯函数|确定性|单次使用' README.md README.zh_CN.md src examples Cargo.toml
```

Classify every match against the approved design checklist.

**Step 2: Rewrite the crate positioning and API examples**

Make both READMEs say that the crate provides semantic traits plus storable/nameable/shareable/composable callback objects. Show precise root imports and start composition with an explicit Box/Rc/Arc wrapper.

Document the actual costs and boundaries:

- Box allocation and dynamic dispatch;
- Rc single-thread sharing;
- Arc thread-safe shared ownership;
- Stateful Rc borrow and Arc lock held during invocation;
- reentry outcomes;
- Once-only consumption;
- composition-specific `'static` constraints where present.

Keep English and Chinese feature tables and examples equivalent.

**Step 3: Audit rustdoc family by family**

Remove claims that `Fn` proves purity, determinism, repeatability, or absence of side effects. Correct ownership language per method signature. Remove deleted conversions and extension-trait links. Restrict “same type” claims to the concrete wrapper methods where true.

**Step 4: Give examples exact feature requirements**

In `Cargo.toml`, remove the blanket `required-features = ["full"]` where an example only needs baseline behavior. Assign only `rc`, `once`, and/or `stateful` when the example actually imports those APIs.

**Step 5: Verify documentation contracts**

Run:

```bash
cargo +1.94.0 test --all-features --doc
cargo +1.94.0 doc --all-features --no-deps
rg -n 'Fn[A-Za-z0-9_]*Ops|feature\s*=\s*"combinators"|into_mut_fn|to_mut_fn|into_fn|to_fn' README.md README.zh_CN.md src examples Cargo.toml
```

Expected: doctests/docs pass and removed contracts have no stale references.

---

### Task 7: Pin and minimize the GitHub Actions caller

**Files:**

- Modify: `.github/workflows/ci.yml`

**Step 1: Verify the reusable workflow contract at the checked-out submodule revision**

Inspect `.rs-ci/.github/workflows/rust-ci.yml` and its called workflows for declared secrets and required permissions. Record which caller permissions are actually necessary for pull-request reporting and Pages/coverage publication.

**Step 2: Pin the workflow**

Replace the mutable ref with the current `.rs-ci` submodule SHA:

```yaml
jobs:
  rust-ci:
    uses: qubit-ltd/rs-ci/.github/workflows/rust-ci.yml@4912370cd7529f2a3bbb7086fdcaf704eb75247d
```

Remove `secrets: inherit` unless the reusable workflow declares and consumes a specific secret. If a secret is genuinely required, pass only that named secret.

**Step 3: Minimize permissions based on evidence**

Keep only caller permissions required by the pinned workflow. Do not remove `pull-requests: write`, `pages: write`, or `id-token: write` if inspection proves a called job requires it; explain retained write permissions in a YAML comment only if the reason is not self-evident.

**Step 4: Validate workflow syntax and alignment**

Run the repository's available workflow/style checks through:

```bash
./.rs-ci/style-check.sh
git diff --check .github/workflows/ci.yml
```

Expected: valid style, immutable workflow ref, and no broad secret inheritance.

---

### Task 8: Final repository-wide verification and cleanup

**Files:**

- Modify only files identified by verification failures.

**Step 1: Run structural residue checks**

Run:

```bash
rg -n 'Fn[A-Za-z0-9_]*Ops|fn_[a-z0-9_]*ops|feature\s*=\s*"combinators"|name:\s*Option<String>|qubit-style: allow explicit-imports' Cargo.toml src tests examples README.md README.zh_CN.md .rs-ci-cargo-matrix.json
git diff --check
```

Expected: no stale extension API, feature, old metadata storage, or blanket import exception.

**Step 2: Run CI-aligned formatting/fixing exactly as requested**

Run:

```bash
./align-ci.sh
```

Expected: formatting and Clippy fix phase succeeds. Inspect its diff and ensure it contains only mechanical changes consistent with this refactor.

**Step 3: Run the full CI mirror exactly as requested**

Run:

```bash
./ci-check.sh
```

Expected: all format, Clippy, style, debug/release build, tests, docs, README version, feature-matrix, package, coverage, and audit steps pass.

**Step 4: Re-run residue and worktree checks after automated fixes**

Run:

```bash
rg -n 'Fn[A-Za-z0-9_]*Ops|fn_[a-z0-9_]*ops|feature\s*=\s*"combinators"|name:\s*Option<String>|qubit-style: allow explicit-imports' Cargo.toml src tests examples README.md README.zh_CN.md .rs-ci-cargo-matrix.json
git diff --check
git status --short
git diff --stat
```

Expected: residue search is empty, diff checks pass, and the final status contains only intentional uncommitted refactor changes and the approved design/plan documents.

**Step 5: Report completion without committing**

Report:

- branch name;
- major API removals and retained callback-object capabilities;
- metadata/import/documentation/CI outcomes;
- exact results of `align-ci.sh` and `ci-check.sh`;
- uncommitted worktree state and the fact that no commit was made without authorization.
