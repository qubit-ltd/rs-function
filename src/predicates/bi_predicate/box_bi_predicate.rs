// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxBiPredicate` public type.

use std::ops::Not;

use super::ALWAYS_FALSE_NAME;
use super::ALWAYS_TRUE_NAME;
use super::BiPredicateFn;
use crate::BiPredicate;
use crate::predicates::macros::impl_box_predicate_methods;
use crate::predicates::macros::impl_predicate_common_methods;
use crate::predicates::macros::impl_predicate_debug_display;

/// A Box-based bi-predicate with single ownership.
///
/// This type is suitable when the bi-predicate needs a stable single-owner
/// type but does not need to be cloned or shared. Composition
/// methods consume `self`, reflecting the single-ownership model.
///
/// # Type Parameters
///
/// * `T` - The type of the first value tested by the predicate.
/// * `U` - The type of the second value tested by the predicate.
///
/// # Examples
///
/// ```rust
/// # {
/// use qubit_function::{BiPredicate, BoxBiPredicate};
///
/// let pred = BoxBiPredicate::new(|x: &i32, y: &i32| x + y > 0);
/// assert!(pred.test(&5, &3));
///
/// // Chaining consumes the bi-predicate
/// let combined = pred.and(BoxBiPredicate::new(|x: &i32, y: &i32| x > y));
/// assert!(combined.test(&10, &5));
/// # }
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxBiPredicate<T, U> {
    /// The wrapped callback implementation.
    pub(super) function: Box<BiPredicateFn<T, U>>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

impl<T, U> BoxBiPredicate<T, U> {
    // Generates: new(), new_with_name(), name(), set_name(), always_true(),
    // always_false()
    impl_predicate_common_methods!(
        BoxBiPredicate<T, U>,
        semantic (BiPredicate<T, U> + 'static),
        |predicate| move |first: &T, second: &U| predicate.test(first, second),
        |f| Box::new(f)
    );

    // Generates: and(), or(), nand(), xor(), nor()
    impl_box_predicate_methods!(BoxBiPredicate<T, U>);
}

impl<T, U> Not for BoxBiPredicate<T, U>
where
    T: 'static,
    U: 'static,
{
    /// The boxed bi-predicate type produced by logical negation.
    type Output = BoxBiPredicate<T, U>;

    /// Returns a boxed predicate that negates this predicate's result.
    ///
    /// The callback and its diagnostic metadata are moved into the returned
    /// wrapper, preserving the original name while transferring ownership.
    ///
    /// # Returns
    ///
    /// A boxed predicate that returns the logical negation of this predicate.
    fn not(self) -> Self::Output {
        let metadata = self.metadata;
        let function = self.function;
        BoxBiPredicate::new_with_metadata(move |first: &T, second: &U| !function(first, second), metadata)
    }
}

// Generates: impl Debug for BoxBiPredicate<T, U> and impl Display for
// BoxBiPredicate<T, U>
impl_predicate_debug_display!(BoxBiPredicate<T, U>);

impl<T, U> BiPredicate<T, U> for BoxBiPredicate<T, U> {
    /// Evaluates the wrapped callback for the two borrowed inputs.
    ///
    /// The callback is invoked through shared references; any interior
    /// mutability or external side effects come from the callback itself.
    ///
    /// # Parameters
    ///
    /// * `first` - The first value passed to the callback.
    /// * `second` - The second value passed to the callback.
    ///
    /// # Returns
    ///
    /// The boolean result returned by the wrapped callback.
    #[inline(always)]
    fn test(&self, first: &T, second: &U) -> bool {
        (self.function)(first, second)
    }
}
