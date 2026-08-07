// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `ArcConditionalMutator` public type.

use crate::ArcMutator;
use crate::ArcPredicate;
use crate::Mutator;
use crate::Predicate;
use crate::mutators::macros::impl_conditional_mutator_clone;
use crate::mutators::macros::impl_conditional_mutator_debug_display;
use crate::mutators::macros::impl_shared_conditional_mutator;

// ============================================================================
// 10. ArcConditionalMutator - Arc-based Conditional Mutator
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
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct ArcConditionalMutator<T> {
    /// The wrapped mutator callback.
    pub(super) mutator: ArcMutator<T>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: ArcPredicate<T>,
}

// Generate shared conditional mutator methods (and_then, or_else, conversions)
impl_shared_conditional_mutator!(
    ArcConditionalMutator<T>,
    ArcMutator,
    Mutator,
    callback_bounds = (Send + Sync + 'static)
);

impl<T> Mutator<T> for ArcConditionalMutator<T> {
    fn apply(&self, value: &mut T) {
        if self.predicate.test(value) {
            self.mutator.apply(value);
        }
    }
}

// Generate Clone trait implementation for conditional mutator
impl_conditional_mutator_clone!(ArcConditionalMutator<T>);

// Generate Debug and Display trait implementations for conditional mutator
impl_conditional_mutator_debug_display!(ArcConditionalMutator<T>);
