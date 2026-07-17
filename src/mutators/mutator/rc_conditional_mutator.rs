// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcConditionalMutator` public type.

use {
    crate::Mutator,
    crate::Predicate,
    crate::RcMutator,
    crate::RcPredicate,
    crate::mutators::macros::impl_conditional_mutator_clone,
    crate::mutators::macros::impl_conditional_mutator_debug_display,
    crate::mutators::macros::impl_shared_conditional_mutator,
};

// ============================================================================
// 9. RcConditionalMutator - Rc-based Conditional Mutator
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
/// - **Ownership Cost**: `Rc` uses non-atomic reference counting; `Arc` uses
///   atomic reference counting, and neither wrapper locks callback invocation
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
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct RcConditionalMutator<T> {
    /// The wrapped mutator callback.
    pub(super) mutator: RcMutator<T>,
    /// The predicate controlling conditional execution.
    pub(super) predicate: RcPredicate<T>,
}

// Generate shared conditional mutator methods (and_then, or_else)
impl_shared_conditional_mutator!(
    RcConditionalMutator<T>,
    RcMutator,
    Mutator,
    callback_bounds = ('static)
);

impl<T> Mutator<T> for RcConditionalMutator<T> {
    fn apply(&self, value: &mut T) {
        if self.predicate.test(value) {
            self.mutator.apply(value);
        }
    }
}

// Generate Clone trait implementation for conditional mutator
impl_conditional_mutator_clone!(RcConditionalMutator<T>);

// Generate Debug and Display trait implementations for conditional mutator
impl_conditional_mutator_debug_display!(RcConditionalMutator<T>);
