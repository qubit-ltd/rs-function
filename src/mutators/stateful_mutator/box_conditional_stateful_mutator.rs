/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `BoxConditionalStatefulMutator` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// 7. BoxConditionalMutator - Box-based Conditional Mutator
// ============================================================================

/// BoxConditionalMutator struct
///
/// A conditional mutator that only executes when a predicate is satisfied.
/// Uses `BoxMutator` and `BoxPredicate` for single ownership semantics.
///
/// This type is typically created by calling `BoxMutator::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes `self` on use
/// - **Conditional Execution**: Only mutates when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
/// - **Implements Mutator**: Can be used anywhere a `Mutator` is expected
///
/// # Examples
///
/// ## Basic Conditional Execution
///
/// ```rust
/// use qubit_function::{Mutator, BoxMutator};
///
/// let mutator = BoxMutator::new(|x: &mut i32| *x *= 2);
/// let mut conditional = mutator.when(|x: &i32| *x > 0);
///
/// let mut positive = 5;
/// conditional.apply(&mut positive);
/// assert_eq!(positive, 10); // Executed
///
/// let mut negative = -5;
/// conditional.apply(&mut negative);
/// assert_eq!(negative, -5); // Not executed
/// ```
///
/// ## With or_else Branch
///
/// ```rust
/// use qubit_function::{Mutator, BoxMutator};
///
/// let mut mutator = BoxMutator::new(|x: &mut i32| *x *= 2)
///     .when(|x: &i32| *x > 0)
///     .or_else(|x: &mut i32| *x -= 1);
///
/// let mut positive = 5;
/// mutator.apply(&mut positive);
/// assert_eq!(positive, 10); // when branch executed
///
/// let mut negative = -5;
/// mutator.apply(&mut negative);
/// assert_eq!(negative, -6); // or_else branch executed
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxConditionalStatefulMutator<T> {
    pub(super) mutator: BoxStatefulMutator<T>,
    pub(super) predicate: BoxPredicate<T>,
}

// Generate box conditional mutator methods (and_then, or_else)
impl_box_conditional_mutator!(
    BoxConditionalStatefulMutator<T>,
    BoxStatefulMutator,
    StatefulMutator
);

impl<T> StatefulMutator<T> for BoxConditionalStatefulMutator<T> {
    fn apply(&mut self, value: &mut T) {
        if self.predicate.test(value) {
            self.mutator.apply(value);
        }
    }

    // Generates: into_box(), into_rc(), into_fn()
    impl_conditional_mutator_conversions!(BoxStatefulMutator<T>, RcStatefulMutator, FnMut);
}

// Generate Debug and Display trait implementations for conditional mutator
impl_conditional_mutator_debug_display!(BoxConditionalStatefulMutator<T>);
