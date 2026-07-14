# Callback Object Consistency Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make callback wrapper naming, diagnostics, semantic composition, name propagation, mutable shared invocation, feature documentation, and API documentation consistent without expanding the public callback-family surface.

**Architecture:** Reuse the crate-private `CallbackMetadata` and existing constructor/name/debug macros instead of introducing a public abstraction. Apply the approved preserve-or-clear name contract at wrapper construction sites, keep Conditional builders metadata-free, and locate closure blanket implementations beside their semantic traits so feature selection cannot change trait semantics.

**Tech Stack:** Rust 1.85+, Cargo feature matrix, `parking_lot`, integration tests under `tests/`, rustdoc, shell CI scripts.

## Global Constraints

- Public API-size reduction is out of scope.
- Do not add a public naming trait or expose `CallbackMetadata`.
- Do not add metadata to Conditional intermediate builders.
- Keep feature names and dependencies exactly: `default = []`, `rc = []`, `once = []`, `stateful = ["dep:parking_lot"]`, `full = ["rc", "once", "stateful"]`.
- Box combinators consume `self`; Rc and Arc combinators borrow the left side and capture the right side by value.
- Arc stateless composition requires `Send + Sync + 'static`; Arc stateful composition requires `Send + 'static`.
- Preserve names through `map`, `map_err`, `not`, and `reversed`; clear names through `and_then`, logical composition, `zip`, `filter`, and `when/or_else`.
- Use TDD: observe each behavior test fail before changing production code.
- Keep all tests under `tests/`; do not add inline `#[cfg(test)]` modules.
- Do not create, delete, move, or rename Rust source files.
- Do not stage, commit, push, or otherwise publish changes without explicit user authorization.

---

### Task 1: Add `with_name` to the shared naming contract

**Files:**
- Modify: `tests/callback_wrapper_contract_tests.rs`
- Modify: `src/macros/common_name_methods.rs`

**Interfaces:**
- Consumes: existing `CallbackMetadata::{name,set_name,clear_name}` and all final wrappers already invoking `impl_common_name_methods!`.
- Produces: `pub fn with_name(mut self, name: &str) -> Self` on every currently nameable final wrapper.

- [ ] **Step 1: Add compile-and-behavior tests for representative Box, Rc, Arc, and once wrappers**

```rust
let callback = BoxFunction::new(|value: &i32| *value).with_name("identity");
assert_eq!(callback.name(), Some("identity"));

let original = ArcFunction::new_with_name("original", |value: &i32| *value);
let renamed = original.clone().with_name("renamed");
assert_eq!(original.name(), Some("original"));
assert_eq!(renamed.name(), Some("renamed"));
```

- [ ] **Step 2: Run the focused contract test and verify RED**

Run: `cargo test --test callback_wrapper_contract_tests with_name`

Expected: compilation fails because `with_name` does not exist.

- [ ] **Step 3: Extend `impl_common_name_methods!` with the consuming builder method**

```rust
#[doc = concat!("Sets the name of this ", $type_desc, " and returns it.")]
#[inline]
pub fn with_name(mut self, name: &str) -> Self {
    self.metadata.set_name(name);
    self
}
```

- [ ] **Step 4: Re-run the focused test and verify GREEN**

Run: `cargo test --test callback_wrapper_contract_tests with_name`

Expected: all selected tests pass.

### Task 2: Add metadata and diagnostics to Comparator wrappers

**Files:**
- Modify: `tests/comparator/box_comparator_tests.rs`
- Modify: `tests/comparator/rc_comparator_tests.rs`
- Modify: `tests/comparator/arc_comparator_tests.rs`
- Modify: `src/comparator/box_comparator.rs`
- Modify: `src/comparator/rc_comparator.rs`
- Modify: `src/comparator/arc_comparator.rs`

**Interfaces:**
- Consumes: `CallbackMetadata`, `impl_common_name_methods!`, and the constructor behavior of existing wrappers.
- Produces: `new_with_name`, `new_with_optional_name`, `name`, `set_name`, `clear_name`, `with_name`, `Debug`, and `Display` on all three Comparator wrappers.

- [ ] **Step 1: Add metadata and formatting tests for Box, Rc, and Arc**

```rust
let unnamed = BoxComparator::new(|a: &i32, b: &i32| a.cmp(b));
assert_eq!(unnamed.name(), None);
assert_eq!(format!("{unnamed}"), "BoxComparator");

let named = ArcComparator::new_with_name("ascending", |a: &i32, b: &i32| a.cmp(b));
assert_eq!(named.name(), Some("ascending"));
assert_eq!(format!("{named:?}"), "ArcComparator { name: Some(\"ascending\") }");
assert_eq!(format!("{named}"), "ArcComparator(ascending)");
```

Also verify `new_with_optional_name`, `set_name`, `clear_name`, and clone-independent renaming for Rc and Arc.

- [ ] **Step 2: Run Comparator tests and verify RED**

Run: `cargo test --test integration_tests comparator::`

Expected: compilation fails on the new naming and formatting API.

- [ ] **Step 3: Add metadata-aware constructors and name methods**

Each struct gains `metadata: CallbackMetadata`; `new` delegates to unnamed metadata, and the two named constructors preserve the existing semantic-trait bounds.

```rust
pub fn new_with_name<F>(name: &str, source: F) -> Self
where
    F: Comparator<T> + 'static,
{
    Self {
        function: Box::new(move |left, right| source.compare(left, right)),
        metadata: CallbackMetadata::named(name),
    }
}
```

Use the corresponding `Send + Sync + 'static` bound for Arc and invoke `impl_common_name_methods!("comparator")` in each implementation.

- [ ] **Step 4: Implement `Debug` and `Display` without formatting the callback**

```rust
impl<T> std::fmt::Debug for BoxComparator<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("BoxComparator").field("name", &self.name()).finish()
    }
}
```

Display emits `Type(name)` when named and `Type` otherwise; Rc and Arc use their concrete type names.

- [ ] **Step 5: Re-run Comparator tests and verify GREEN**

Run: `cargo test --test integration_tests comparator::`

Expected: all Comparator tests pass.

### Task 3: Add metadata and diagnostics to Tester and StatefulTester wrappers

**Files:**
- Modify: `tests/testers/tester/box_tester_tests.rs`
- Modify: `tests/testers/tester/rc_tester_tests.rs`
- Modify: `tests/testers/tester/arc_tester_tests.rs`
- Modify: `tests/testers/stateful_tester_tests.rs`
- Modify: `src/testers/tester/box_tester.rs`
- Modify: `src/testers/tester/rc_tester.rs`
- Modify: `src/testers/tester/arc_tester.rs`
- Modify: `src/testers/stateful_tester/box_stateful_tester.rs`
- Modify: `src/testers/stateful_tester/rc_stateful_tester.rs`
- Modify: `src/testers/stateful_tester/arc_stateful_tester.rs`

**Interfaces:**
- Consumes: Task 1 naming macro and `CallbackMetadata`.
- Produces: the same constructor, naming, clone-isolation, `Debug`, and `Display` contract as Task 2 for Tester and StatefulTester.

- [ ] **Step 1: Add failing metadata and formatting tests for all six shared/owned variants**

Use the same assertions as Comparator with concrete names such as `BoxTester`, `RcTester`, and `ArcStatefulTester`; include a mutating StatefulTester closure to prove metadata does not alter invocation semantics.

- [ ] **Step 2: Run focused tests and verify RED**

Run: `cargo test --test integration_tests testers::`

Expected: compilation fails on missing metadata APIs and formatting implementations.

- [ ] **Step 3: Add metadata-aware fields, constructors, and name methods**

Use `Tester` bounds for stateless wrappers and `StatefulTester` bounds for mutable wrappers. Preserve current Box/Rc/Arc storage and feature gates.

- [ ] **Step 4: Add concrete `Debug` and `Display` implementations**

Formatting must exactly match Task 2 and must not add trait bounds to callback result types.

- [ ] **Step 5: Re-run focused tests and verify GREEN**

Run: `cargo test --test integration_tests testers::`

Expected: all Tester and StatefulTester tests pass.

### Task 4: Generalize Comparator, Tester, and StatefulTester combinators

**Files:**
- Modify: `tests/comparator/{box_comparator_tests,rc_comparator_tests,arc_comparator_tests}.rs`
- Modify: `tests/testers/tester/{box_tester_tests,rc_tester_tests,arc_tester_tests}.rs`
- Modify: `tests/testers/stateful_tester_tests.rs`
- Modify: `src/comparator/{box_comparator,rc_comparator,arc_comparator}.rs`
- Modify: `src/testers/tester/{box_tester,rc_tester,arc_tester}.rs`
- Modify: `src/testers/stateful_tester/{box_stateful_tester,rc_stateful_tester,arc_stateful_tester}.rs`

**Interfaces:**
- Consumes: `Comparator<T>`, `Tester`, and `StatefulTester` semantic traits.
- Produces: generic right-hand operands with the exact ownership and thread-safety bounds from the approved design.

- [ ] **Step 1: Add tests using closures and custom non-wrapper semantic types**

```rust
struct Natural;
impl Comparator<i32> for Natural {
    fn compare(&self, left: &i32, right: &i32) -> Ordering { left.cmp(right) }
}

let chained = RcComparator::new(|_: &i32, _: &i32| Ordering::Equal)
    .then_comparing(Natural);
assert_eq!(chained.compare(&1, &2), Ordering::Less);
```

For Tester and StatefulTester cover custom objects plus closures. Verify left Rc/Arc values remain callable; explicitly pass `right.clone()` when the right wrapper must remain usable. Retain existing short-circuit tests for `and`, `or`, `nand`, `nor`, and two-sided evaluation for `xor`.

- [ ] **Step 2: Run the focused tests and verify RED**

Run: `cargo test --test integration_tests comparator:: testers::`

Expected: current concrete-wrapper parameter types reject custom semantic implementations.

- [ ] **Step 3: Implement the generic Comparator signatures**

```rust
pub fn then_comparing<C>(&self, other: C) -> Self
where
    T: 'static,
    C: Comparator<T> + 'static,
```

Box uses `self` and `Comparator<T> + 'static`; Arc adds `Send + Sync`. Capture `other` by value and call `other.compare(a, b)` only when the first comparison is equal.

- [ ] **Step 4: Implement generic Tester and StatefulTester signatures**

Apply the same pattern to `and`, `or`, `nand`, `xor`, and `nor`. Arc Tester adds `Send + Sync`; Arc StatefulTester adds only `Send` because its callback is serialized by `Mutex`.

- [ ] **Step 5: Re-run focused tests and verify GREEN**

Run: `cargo test --test integration_tests comparator:: testers::`

Expected: generic composition and all existing truth-table tests pass.

### Task 5: Enforce the name-propagation contract

**Files:**
- Modify: existing integration tests under `tests/functions/`, `tests/predicates/`, `tests/suppliers/`, `tests/tasks/`, `tests/transformers/`, `tests/consumers/`, `tests/mutators/`, `tests/comparator/`, and `tests/testers/` that correspond to the affected wrappers.
- Modify: `src/tasks/callable/{box_callable,rc_callable,arc_callable}.rs`
- Modify: `src/tasks/callable_with/{box_callable_with,rc_callable_with,arc_callable_with}.rs`
- Modify: `src/tasks/callable_once/{box_callable_once,local_box_callable_once}.rs`
- Modify: `src/comparator/{box_comparator,rc_comparator,arc_comparator}.rs`
- Modify: family combinator macros under `src/functions/macros/`, `src/predicates/macros/`, `src/suppliers/macros/`, `src/transformers/macros/`, `src/consumers/macros/`, and `src/mutators/macros/` only where current construction violates the approved contract.

**Interfaces:**
- Consumes: Task 1 `with_name` and metadata-aware wrappers from Tasks 2–3.
- Produces: preserve through `map`, `map_err`, `not`, `reversed`; clear through `and_then`, logic, `zip`, `filter`, and `when/or_else`.

- [ ] **Step 1: Add table-driven name-propagation assertions to existing family tests**

```rust
let mapped = BoxCallable::new_with_name("load", || Ok::<_, ()>(1)).map(|value| value + 1);
assert_eq!(mapped.name(), Some("load"));

let chained = BoxCallable::new_with_name("load", || Ok::<_, ()>(1))
    .and_then(|value| Ok(value + 1));
assert_eq!(chained.name(), None);
assert_eq!(chained.with_name("pipeline").name(), Some("pipeline"));
```

Cover every operation class on representative Box/Rc/Arc wrappers and every macro path shared by multiple families. Include final `or_else` results; do not assert or add `name` on Conditional builders.

- [ ] **Step 2: Run affected family tests and verify RED**

Run: `cargo test --test integration_tests`

Expected: assertions identify current preserve/clear inconsistencies.

- [ ] **Step 3: Preserve metadata for single-source transformations**

Move or clone the source metadata into `map`, `map_err`, `not`, and `reversed` result construction. Do not reconstruct these results exclusively through unnamed `new`.

- [ ] **Step 4: Clear metadata for multi-source or branching transformations**

Construct `and_then`, logical combinations, `zip`, `filter`, and `when/or_else` results with unnamed metadata. Keep Conditional structs unchanged.

- [ ] **Step 5: Re-run the integration suite and verify GREEN**

Run: `cargo test --test integration_tests`

Expected: all name-propagation and legacy behavior tests pass.

### Task 6: Relocate closure blanket implementations

**Files:**
- Modify: semantic trait modules under `src/consumers/`, `src/functions/`, `src/mutators/`, `src/predicates/`, `src/suppliers/`, `src/testers/`, and `src/transformers/` that correspond to current `impl_closure_trait!` invocations.
- Modify: current wrapper files containing `impl_closure_trait!`, including `arc_*` wrapper files and `src/tasks/runnable.rs`.
- Modify: `tests/feature_contract_tests.rs` only if a focused existing feature assertion must be extended to cover semantic closure implementations.

**Interfaces:**
- Consumes: `impl_closure_trait!` macro and existing semantic trait definitions.
- Produces: feature-independent closure blanket implementations co-located with semantic traits, with no duplicate impls.

- [ ] **Step 1: Add or extend a feature-contract test that uses a closure through a semantic trait with optional wrapper features disabled**

Use the existing feature harness and an assertion equivalent to:

```rust
fn invoke<F: Function<i32, i32>>(function: F) -> i32 { function.apply(&1) }
assert_eq!(invoke(|value: &i32| *value + 1), 2);
```

- [ ] **Step 2: Run the baseline feature contract and verify RED if the selected trait currently depends on an optional wrapper module**

Run: `cargo test --test feature_contract_tests test_baseline_feature_contract`

Expected: the new contract exposes any feature-coupled blanket impl; if the representative trait is already baseline-stable, retain the test as a guard and use the next compile step to detect duplicate/missing impls during relocation.

- [ ] **Step 3: Move each macro invocation beside its trait definition**

Move imports and invocations only; do not change bounds or callback semantics. Remove the old invocation from each wrapper file in the same patch to avoid conflicting implementations.

- [ ] **Step 4: Verify all feature combinations compile**

Run: `cargo test --test feature_contract_tests`

Expected: baseline, rc, once, stateful, mixed, and full contracts pass.

### Task 7: Make shared mutable invocation scope explicit

**Files:**
- Modify: all 15 `Arc<Mutex<FnMut>>` wrapper files found by `self.function.lock()` under `src/`.
- Modify: all 15 `Rc<RefCell<FnMut>>` wrapper files found by `self.function.borrow_mut()` under `src/`.
- Modify: existing stateful and task tests corresponding to those wrappers.

**Interfaces:**
- Consumes: existing `parking_lot::Mutex` and `std::cell::RefCell` wrappers.
- Produces: explicit guard/borrow bindings without changing the lock/borrow lifetime or callback behavior.

- [ ] **Step 1: Confirm existing state mutation and panic/borrow behavior tests pass before the refactor**

Run: `cargo test --test integration_tests stateful`

Expected: all selected tests pass, establishing the behavior baseline.

- [ ] **Step 2: Replace temporary invocation expressions with named bindings**

```rust
let mut function = self.function.lock();
function(value)
```

```rust
let mut function = self.function.borrow_mut();
function(value)
```

Use arity-appropriate arguments and preserve expression return values. Do not shorten the critical section.

- [ ] **Step 3: Run stateful and task tests and verify behavior is unchanged**

Run: `cargo test --test integration_tests stateful`

Run: `cargo test --test integration_tests tasks::`

Expected: all selected tests pass with no warnings.

### Task 8: Correct feature and API documentation

**Files:**
- Modify: `README.md`
- Modify: `README.zh_CN.md`
- Modify: Comparator, Tester, and StatefulTester rustdoc files changed in Tasks 2–4.
- Modify: directly affected combinator rustdoc and `Cargo.toml` feature comments, if present.

**Interfaces:**
- Consumes: final implemented API and approved feature contract.
- Produces: factual English and Chinese documentation aligned with ownership, trait bounds, name propagation, and repeatability.

- [ ] **Step 1: Remove subjective and inaccurate claims**

Remove phrases such as “perfect balance” and “most flexible and elegant”; replace “one-time” descriptions of reusable Box wrappers with “single ownership”.

- [ ] **Step 2: Document exact constructor and combinator contracts**

State that constructors accept semantic trait implementations and closures participate through blanket impls. Document right-side ownership, Arc bounds, preserve/clear name behavior, and final `.with_name(...)` usage.

- [ ] **Step 3: Align English and Chinese feature descriptions**

Explain that `stateful` enables explicit Stateful families plus Mutex-backed Arc tasks, `rc` enables Rc wrappers including RefCell-backed tasks, baseline Box tasks remain `FnMut`, and features represent optional API/dependency cost rather than the complete `FnMut` taxonomy.

- [ ] **Step 4: Run rustdoc and doctests**

Run: `cargo test --doc --all-features`

Run: `cargo doc --all-features --no-deps`

Expected: all doctests pass and documentation builds without warnings.

### Task 9: Format, review, and run full verification

**Files:**
- Review: every modified file.
- Modify: only files that fail formatting, linting, tests, or approved contract checks.

**Interfaces:**
- Consumes: Tasks 1–8.
- Produces: a clean, CI-equivalent implementation ready for user review.

- [ ] **Step 1: Run repository alignment checks**

Run: `./align-ci.sh`

Expected: formatting and repository alignment complete successfully.

- [ ] **Step 2: Inspect the complete working-tree diff**

Run: `git diff --check`

Run: `git status --short`

Run: `git diff --stat`

Expected: no whitespace errors and only approved rs-function files are changed.

- [ ] **Step 3: Run the full CI-equivalent suite**

Run: `./ci-check.sh`

Expected: fmt, Clippy, style, debug/release builds, all tests, doctests, docs, feature matrix, package, coverage, and audit pass.

- [ ] **Step 4: Recheck the final diff after any verification fixes**

Run: `git diff --check && git status --short && git diff --stat`

Expected: the worktree contains only the approved design, plan, source, test, and documentation changes; nothing is staged or committed.
