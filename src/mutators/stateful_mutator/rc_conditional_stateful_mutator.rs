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
//! Defines the `RcConditionalStatefulMutator` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// 8. RcConditionalMutator - Rc-based Conditional Mutator
// ============================================================================

/// RcConditionalMutator struct
///
/// A single-threaded conditional mutator that only executes when a predicate is
/// satisfied. Uses `RcMutator` and `RcPredicate` for shared ownership within a
/// single thread.
///
/// This type is typically created by calling `RcMutator::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Conditional Execution**: Only mutates when predicate returns `true`
/// - **No Lock Overhead**: More efficient than `ArcConditionalMutator`
///
/// # Examples
///
/// ```rust
/// use qubit_function::{Mutator, RcMutator};
///
/// let conditional = RcMutator::new(|x: &mut i32| *x *= 2)
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
pub struct RcConditionalStatefulMutator<T> {
    pub(super) mutator: RcStatefulMutator<T>,
    pub(super) predicate: RcPredicate<T>,
}

// Generate shared conditional mutator methods (and_then, or_else, conversions)
impl_shared_conditional_mutator!(
    RcConditionalStatefulMutator<T>,
    RcStatefulMutator,
    StatefulMutator,
    into_rc,
    'static
);

impl<T> StatefulMutator<T> for RcConditionalStatefulMutator<T> {
    fn apply(&mut self, value: &mut T) {
        if self.predicate.test(value) {
            self.mutator.apply(value);
        }
    }

    // Generates: into_box(), into_rc(), into_fn()
    impl_conditional_mutator_conversions!(BoxStatefulMutator<T>, RcStatefulMutator, FnMut);
}

// Generate Clone trait implementation for conditional mutator
impl_conditional_mutator_clone!(RcConditionalStatefulMutator<T>);

// Generate Debug and Display trait implementations for conditional mutator
impl_conditional_mutator_debug_display!(RcConditionalStatefulMutator<T>);
