/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
// qubit-style: allow explicit-imports
//! Defines the `ArcConditionalStatefulMutator` public type.

use super::{
    ArcPredicate,
    ArcStatefulMutator,
    BoxStatefulMutator,
    Predicate,
    RcStatefulMutator,
    StatefulMutator,
    impl_conditional_mutator_clone,
    impl_conditional_mutator_conversions,
    impl_conditional_mutator_debug_display,
    impl_shared_conditional_mutator,
};

// ============================================================================
// 9. ArcConditionalMutator - Arc-based Conditional Mutator
// ============================================================================

/// ArcConditionalMutator struct
///
/// A thread-safe conditional mutator that only executes when a predicate is
/// satisfied. Uses `ArcMutator` and `ArcPredicate` for shared ownership across
/// threads.
///
/// This type is typically created by calling `ArcMutator::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
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
/// use qubit_function::{Mutator, ArcMutator};
///
/// let conditional = ArcMutator::new(|x: &mut i32| *x *= 2)
///     .when(|x: &i32| *x > 0);
///
/// let conditional_clone = conditional.clone();
///
/// let mut value = 5;
/// let mut m = conditional;
/// m.apply(&mut value);
/// assert_eq!(value, 10);
/// ```
///
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

    // Generates: into_box(), into_rc(), into_fn()
    impl_conditional_mutator_conversions!(BoxStatefulMutator<T>, RcStatefulMutator, FnMut);
}

// Generate Clone trait implementation for conditional mutator
impl_conditional_mutator_clone!(ArcConditionalStatefulMutator<T>);

// Generate Debug and Display trait implementations for conditional mutator
impl_conditional_mutator_debug_display!(ArcConditionalStatefulMutator<T>);
