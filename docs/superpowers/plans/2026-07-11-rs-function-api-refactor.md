# rs-function API Refactor Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use
> superpowers:subagent-driven-development (recommended) or
> superpowers:executing-plans to implement this plan task-by-task. Steps use
> checkbox (`- [ ]`) syntax for tracking.

**Goal:** Refactor `qubit-function` so semantic traits contain only intrinsic
operations, wrappers use `Type::new(impl SemanticTrait)`, misleading
`ArcFoo`/`RcFoo` conversions disappear, optional API costs are feature-gated,
and the public documentation describes the remaining concurrency contracts.

**Architecture:** Semantic traits remain the stable behavior layer and retain
closure blanket implementations. `Box`/`Rc`/`Arc` wrappers become explicit
type-erasure boundaries whose constructors adapt semantic trait values to the
existing callable storage. Cargo features gate ownership, invocation-count,
stateful, and combinator cost centers without gating the baseline behavior
traits.

**Tech Stack:** Rust 1.94, Cargo features, `parking_lot`, rustdoc/doctest,
project integration tests, rs-ci shell tooling.

## Global Constraints

- Preserve every existing semantic family; freeze expansion but do not remove
  families as part of this refactor.
- Retain blanket implementations from `Fn`, `FnMut`, and `FnOnce` to semantic
  traits.
- Core semantic traits expose only their intrinsic call operation.
- Use `Type::new(impl SemanticTrait)` as the only wrapper construction API;
  do not add `from`, `wrap`, or `from_predicate` variants.
- Remove `ArcFoo -> RcFoo` and `RcFoo -> ArcFoo` conversions rather than
  retaining adapter views.
- Keep baseline APIs available with `default = []`.
- Do not add a wildcard prelude.
- All new or changed public methods require English rustdoc.
- Tests remain under `tests/`; no source-local test modules.
- Commit messages are in English and grouped by intent.

---

### Task 1: Lock the desired constructor and ambiguity behavior with tests

**Files:**

- Modify: `tests/predicates/predicate_tests.rs`
- Modify: `tests/functions/function_tests.rs`
- Modify: `tests/consumers/consumer_tests.rs`
- Modify: `tests/tasks/callable_tests.rs`
- Modify: `tests/tasks/runnable_tests.rs`
- Modify: `tests/reexports_tests.rs`

**Interfaces:**

- Consumes: existing `Predicate`, `Function`, `Consumer`, `Callable`, and
  `Runnable` implementations.
- Produces: executable examples proving custom semantic implementations can be
  passed to `BoxFoo::new` and `ArcFoo::new`; compile-fail rustdoc proving
  ambiguous closure conversion methods are no longer the recommended API.

- [ ] **Step 1: Add failing constructor tests**

  Add focused tests such as:

  ```rust
  struct Positive;

  impl Predicate<i32> for Positive {
      fn test(&self, value: &i32) -> bool {
          *value > 0
      }
  }

  #[test]
  fn test_new_accepts_custom_predicate() {
      let predicate = BoxPredicate::new(Positive);
      assert!(predicate.test(&1));
      assert!(!predicate.test(&0));
  }
  ```

  Mirror the behavior for an immutable function/consumer and a mutable
  callable/runnable, including the `Send`/`Sync` constraints of Arc wrappers.

- [ ] **Step 2: Run targeted tests and verify RED**

  Run:

  ```bash
  cargo +1.94.0 test --test mod test_new_accepts_custom -- --nocapture
  ```

  Expected: compilation fails because current constructors require native
  callable traits rather than semantic traits.

- [ ] **Step 3: Add export-path expectations**

  Update `tests/reexports_tests.rs` so root and first-level semantic module
  paths remain valid while no test relies on deeper physical module paths.

### Task 2: Reduce semantic traits and move blanket implementations to core

**Files:**

- Modify: `src/consumers/{consumer,bi_consumer,consumer_once,bi_consumer_once,stateful_consumer,stateful_bi_consumer}.rs`
- Modify: `src/functions/{function,bi_function,mutating_function,bi_mutating_function,function_once,bi_function_once,mutating_function_once,bi_mutating_function_once,stateful_function,stateful_mutating_function}.rs`
- Modify: `src/mutators/{mutator,mutator_once,stateful_mutator}.rs`
- Modify: `src/predicates/{predicate,bi_predicate,stateful_predicate,stateful_bi_predicate}.rs`
- Modify: `src/suppliers/{supplier,supplier_once,stateful_supplier}.rs`
- Modify: `src/tasks/{callable,callable_once,callable_with,runnable,runnable_once,runnable_with}.rs`
- Modify: `src/testers/{tester,stateful_tester}.rs`
- Modify: `src/transformers/{transformer,transformer_once,bi_transformer,bi_transformer_once,stateful_transformer,stateful_bi_transformer}.rs`
- Modify: `src/macros/closure_trait.rs`
- Modify: `src/macros/closure_once_trait.rs`
- Modify: `src/macros/mod.rs`
- Modify: `Cargo.toml`

**Interfaces:**

- Consumes: intrinsic methods (`accept`, `apply`, `test`, `get`, `call`,
  `run`, `transform`, `compare`).
- Produces: traits containing only intrinsic methods and local blanket impls
  that do not depend on any ownership wrapper module.

- [ ] **Step 1: Remove conversion methods from one representative trait**

  Reduce `Predicate<T>` to:

  ```rust
  pub trait Predicate<T> {
      fn test(&self, value: &T) -> bool;
  }

  impl<T, F> Predicate<T> for F
  where
      F: Fn(&T) -> bool,
  {
      #[inline]
      fn test(&self, value: &T) -> bool {
          self(value)
      }
  }
  ```

  Place analogous blanket impls beside every semantic trait instead of in
  `arc_*` files.

- [ ] **Step 2: Run the predicate target and verify expected failures**

  Run:

  ```bash
  cargo +1.94.0 test --test mod predicates::predicate_tests --no-run
  ```

  Expected: existing conversion-method tests and wrapper implementations fail,
  identifying the migration surface.

- [ ] **Step 3: Apply the same intrinsic-only rule to all families**

  Remove `into_*`, `to_*`, and cross-family conversion helpers from semantic
  trait definitions. Keep combinators only on explicitly imported extension
  traits or concrete conditional wrappers.

- [ ] **Step 4: Simplify closure macros and dependency usage**

  Remove wrapper-name synthesis and conversion generation from
  `closure_trait.rs` and `closure_once_trait.rs`. Remove `pastey` from
  `Cargo.toml` when `rg 'paste::|pastey' src Cargo.toml` returns no remaining
  production use.

- [ ] **Step 5: Verify the library compiles far enough to expose wrapper work**

  Run:

  ```bash
  cargo +1.94.0 check --lib
  ```

  Expected: failures are limited to wrapper constructor/conversion impls, not
  missing blanket implementations.

### Task 3: Make wrapper constructors accept semantic trait implementations

**Files:**

- Modify: `src/macros/common_new_methods.rs`
- Modify: `src/consumers/macros/consumer_common_methods.rs`
- Modify: `src/functions/macros/function_common_methods.rs`
- Modify: `src/mutators/macros/mutator_common_methods.rs`
- Modify: `src/predicates/macros/predicate_common_methods.rs`
- Modify: `src/suppliers/macros/supplier_common_methods.rs`
- Modify: `src/transformers/macros/transformer_common_methods.rs`
- Modify: wrapper files under `src/{consumers,functions,mutators,predicates,suppliers,tasks,testers,transformers}/**/{box,rc,arc}_*.rs`
- Modify: corresponding tests under `tests/{consumers,functions,mutators,predicates,suppliers,tasks,testers,transformers}/`

**Interfaces:**

- Consumes: semantic traits from Task 2.
- Produces: `new`, `new_with_name`, and `new_with_optional_name` constructors
  accepting a semantic implementation and adapting it to existing callable
  storage.

- [ ] **Step 1: Extend the shared constructor macro**

  Add a semantic-adapter form that accepts a generic semantic bound, call
  arguments, and storage wrapper expression. Generated constructors must adapt
  with `move |args| source.intrinsic_method(args)` and preserve names.

- [ ] **Step 2: Implement Predicate wrappers and verify GREEN**

  Update `BoxPredicate`, `RcPredicate`, and `ArcPredicate`; run:

  ```bash
  cargo +1.94.0 test --test mod predicates::predicate_tests
  ```

  Expected: custom implementation and closure construction tests pass.

- [ ] **Step 3: Implement immutable callback wrappers**

  Migrate Function, Consumer, Predicate, Supplier, Transformer, Tester, and
  Comparator wrapper constructors, preserving `Send + Sync` bounds on Arc.

- [ ] **Step 4: Implement mutable callback wrappers**

  Migrate Stateful, Callable, and Runnable wrappers. Box adapters capture a
  mutable source locally; Rc uses `RefCell`; Arc uses `parking_lot::Mutex` and
  retains the existing `Send` contract.

- [ ] **Step 5: Run all constructor-family tests**

  Run:

  ```bash
  cargo +1.94.0 test --all-features --test mod
  ```

  Expected: all migrated constructor tests pass; remaining failures mention
  removed conversions only.

### Task 4: Remove misleading ownership conversions and migrate call sites

**Files:**

- Modify: `src/macros/{arc_conversions,rc_conversions,box_conversions}.rs`
- Modify: all wrapper files invoking those macros under `src/`
- Modify: conversion tests under `tests/macros/`
- Modify: family tests under `tests/`
- Modify: examples under `examples/`
- Modify: rustdoc examples under `src/`

**Interfaces:**

- Consumes: wrapper constructors from Task 3.
- Produces: no `ArcFoo -> RcFoo` or `RcFoo -> ArcFoo` API; explicit
  reconstruction from original semantic values where ownership changes are
  genuinely needed.

- [ ] **Step 1: Add negative API searches**

  Establish the audit command:

  ```bash
  rg '\.(into_rc|to_rc)\(' src tests examples
  rg '\.(into_arc|to_arc)\(' src tests examples
  ```

  Classify each match as raw semantic construction, same-wrapper clone, or
  prohibited cross-wrapper conversion.

- [ ] **Step 2: Remove Arc/Rc cross-conversion macro arms**

  Keep only operations whose names match real storage behavior. Prefer
  inherent `Clone`, constructor calls, or explicit callable extraction over
  adapter closures.

- [ ] **Step 3: Replace downstream construction calls**

  Replace generic `value.into_arc()` and `value.into_box()` with
  `ArcFoo::new(value)` and `BoxFoo::new(value)` at repository and direct
  workspace downstream call sites.

- [ ] **Step 4: Remove matrix-only tests**

  Delete test cases whose only requirement is Arc/Rc conversion symmetry;
  retain tests for same-container cloning, names, invocation behavior, and
  direct construction from semantic implementations.

- [ ] **Step 5: Verify no prohibited conversion remains**

  Run the audit commands and targeted all-feature tests. Expected: no source
  API or production example exposes Arc/Rc cross conversion.

### Task 5: Add Cargo feature layers

**Files:**

- Modify: `Cargo.toml`
- Modify: `src/lib.rs`
- Modify: `src/{consumers,functions,mutators,predicates,suppliers,tasks,testers,transformers}/mod.rs`
- Modify: family module declarations throughout `src/`
- Modify: `tests/mod.rs` and family `tests/*/mod.rs`
- Modify: `tests/reexports_tests.rs`
- Create: `.rs-ci-cargo-matrix.json`

**Interfaces:**

- Consumes: refactored API from Tasks 2-4.
- Produces: baseline plus `rc`, `once`, `stateful`, `combinators`, and `full`
  features; optional `parking_lot`; feature-matrix validation.

- [ ] **Step 1: Add feature-manifest tests and verify RED**

  Extend manifest tests to assert exact feature relationships and the optional
  `parking_lot` dependency. Run the test and expect failure before editing
  `Cargo.toml`.

- [ ] **Step 2: Define Cargo features**

  Add:

  ```toml
  [features]
  default = []
  rc = []
  once = []
  stateful = ["dep:parking_lot"]
  combinators = []
  full = ["rc", "once", "stateful", "combinators"]
  ```

  Mark `parking_lot` optional.

- [ ] **Step 3: Gate modules and re-exports**

  Keep core traits and high-frequency Box/stateless Arc wrappers baseline.
  Gate Rc with `rc`, once families with `once`, shared mutable/stateful types
  with `stateful`, and conditional/extension APIs with `combinators`. Use
  `cfg(all(...))` only for genuine intersections.

- [ ] **Step 4: Gate tests and examples**

  Apply matching `#[cfg(feature = ...)]` module declarations and Cargo
  `required-features` entries so default builds do not compile unavailable
  examples while `--all-features` retains full coverage.

- [ ] **Step 5: Add feature matrix configuration**

  Configure checks for baseline, each individual feature, important
  intersections (`rc,stateful`, `once,combinators`), and `full`.

- [ ] **Step 6: Run feature checks**

  Run:

  ```bash
  cargo +1.94.0 check --no-default-features
  cargo +1.94.0 check --features rc
  cargo +1.94.0 check --features once
  cargo +1.94.0 check --features stateful
  cargo +1.94.0 check --features combinators
  cargo +1.94.0 check --features full
  ```

  Expected: every command passes without warnings.

### Task 6: Stabilize public paths and concurrency documentation

**Files:**

- Modify: `src/lib.rs`
- Modify: semantic `src/*/mod.rs` files
- Modify: shared mutable wrapper files under `src/`
- Modify: `tests/reexports_tests.rs`
- Modify: `README.md`
- Modify: `README.zh_CN.md`

**Interfaces:**

- Consumes: feature-gated modules from Task 5.
- Produces: stable root/first-level semantic paths, private physical modules,
  accurate concurrency/rustdoc contracts, evidence-calibrated README copy.

- [ ] **Step 1: Make physical implementation modules private**

  Preserve crate-root and first-level semantic re-exports. Change nested
  physical modules from `pub mod` to private `mod` where no stable namespace is
  required.

- [ ] **Step 2: Update re-export tests**

  Assert only supported root and first-level paths. Do not expose implementation
  file names as compatibility promises.

- [ ] **Step 3: Add shared mutable rustdoc contracts**

  For Arc wrappers document mutex serialization, full-callback critical
  sections, synchronous re-entry deadlock, and non-poisoning behavior. For Rc
  wrappers document `RefCell` borrow panic on synchronous re-entry. Apply the
  same contract to shared Callable/Runnable wrappers.

- [ ] **Step 4: Revise README performance wording**

  Replace unsupported “High-Performance Concurrency” and “Zero-Cost
  Abstractions” claims with “Thread-safe callback adapters” and “Ergonomic
  callback abstractions”; explain allocation, dispatch, reference-count, and
  lock costs.

- [ ] **Step 5: Verify documentation**

  Run:

  ```bash
  RUSTDOCFLAGS="-D warnings" cargo +1.94.0 doc --all-features --no-deps
  cargo +1.94.0 test --all-features --doc
  ```

  Expected: rustdoc and doctests pass with no warnings.

### Task 7: Final migration, formatting, and CI

**Files:**

- Modify: any directly dependent workspace crate that still uses removed
  conversion methods, limited to the 13 dependencies listed in the design
  review.
- Modify: files changed automatically by project formatting/lint fixes.

**Interfaces:**

- Consumes: completed refactor.
- Produces: a clean all-feature build, passing downstream checks, and passing
  project CI scripts.

- [ ] **Step 1: Check direct downstream crates**

  Run targeted `cargo check --all-targets --all-features` in every direct
  workspace dependency and replace removed calls with explicit wrapper
  constructors.

- [ ] **Step 2: Run the required alignment script**

  Run:

  ```bash
  ./align-ci.sh
  ```

  Expected: formatting and Clippy alignment complete successfully.

- [ ] **Step 3: Run the required full CI script**

  Run:

  ```bash
  ./ci-check.sh
  ```

  Expected: all 11 CI stages pass, including feature matrix, package, coverage,
  documentation, and audit checks.

- [ ] **Step 4: Review and group commits**

  Inspect `git status` and `git diff`; create English commits grouped as API,
  features, documentation, downstream migrations, and CI fixes. Do not push.
