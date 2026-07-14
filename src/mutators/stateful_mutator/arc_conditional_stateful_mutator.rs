// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
// qubit-style: allow explicit-imports
//! Defines the `ArcConditionalStatefulMutator` public type.

use super::{
    ArcPredicate,
    ArcStatefulMutator,
    Predicate,
    StatefulMutator,
    impl_conditional_mutator_clone,
    impl_conditional_mutator_debug_display,
    impl_shared_conditional_mutator,
};

// ============================================================================
// 9. ArcConditionalStatefulMutator - Arc-based Conditional Stateful Mutator
// ============================================================================

/// Arc-based conditional stateful mutator.
///
/// A thread-safe conditional stateful mutator that only executes when a
/// predicate is satisfied. Uses `ArcStatefulMutator` and `ArcPredicate` for
/// shared ownership across threads.
///
/// This type is typically created by calling `ArcStatefulMutator::when()` and
/// works with `or_else()` to create if-then-else logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send + Sync`, safe for concurrent use
/// - **Conditional Execution**: Only mutates when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
///
/// # Examples
///
/// ```rust
/// use qubit_function::{ArcStatefulMutator, StatefulMutator};
///
/// let mut calls = 0;
/// let conditional = ArcStatefulMutator::new(move |x: &mut i32| {
///     calls += 1;
///     *x += calls;
/// })
///     .when(|x: &i32| *x > 0);
///
/// let conditional_clone = conditional.clone();
///
/// let mut value = 5;
/// let mut m = conditional;
/// m.apply(&mut value);
/// assert_eq!(value, 6);
/// ```
///
/// # Locking and reentrancy
///
/// When the wrapped stateful callback executes, the underlying
/// `parking_lot::Mutex` remains locked until that callback returns.
/// Synchronous re-entry through the same shared state deadlocks. The mutex is
/// not poisoned after a panic, and mutations completed before a panic are not
/// rolled back.
pub struct ArcConditionalStatefulMutator<T> {
    pub(super) mutator: ArcStatefulMutator<T>,
    pub(super) predicate: ArcPredicate<T>,
}

// Generate shared conditional mutator methods (and_then, or_else, conversions)
impl_shared_conditional_mutator!(
    ArcConditionalStatefulMutator<T>,
    ArcStatefulMutator,
    StatefulMutator,
    into_arc,
    Send + Sync + 'static
);

impl<T> StatefulMutator<T> for ArcConditionalStatefulMutator<T> {
    fn apply(&mut self, value: &mut T) {
        if self.predicate.test(value) {
            self.mutator.apply(value);
        }
    }
}

// Generate Clone trait implementation for conditional mutator
impl_conditional_mutator_clone!(ArcConditionalStatefulMutator<T>);

// Generate Debug and Display trait implementations for conditional mutator
impl_conditional_mutator_debug_display!(ArcConditionalStatefulMutator<T>);
