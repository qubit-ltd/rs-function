// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxBiPredicate` public type.

use std::ops::Not;

use {
    super::ALWAYS_FALSE_NAME,
    super::ALWAYS_TRUE_NAME,
    super::BiPredicateFn,
    crate::BiPredicate,
    crate::predicates::macros::impl_box_predicate_methods,
    crate::predicates::macros::impl_predicate_common_methods,
    crate::predicates::macros::impl_predicate_debug_display,
};

/// A Box-based bi-predicate with single ownership.
///
/// This type is suitable when the bi-predicate needs a stable single-owner
/// type but does not need to be cloned or shared. Composition
/// methods consume `self`, reflecting the single-ownership model.
///
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
pub struct BoxBiPredicate<T, U> {
    pub(super) function: Box<BiPredicateFn<T, U>>,
    pub(super) metadata: crate::callback_metadata::CallbackMetadata,
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
    type Output = BoxBiPredicate<T, U>;

    fn not(self) -> Self::Output {
        BoxBiPredicate::new(move |first: &T, second: &U| {
            !(self.function)(first, second)
        })
    }
}

// Generates: impl Debug for BoxBiPredicate<T, U> and impl Display for
// BoxBiPredicate<T, U>
impl_predicate_debug_display!(BoxBiPredicate<T, U>);

impl<T, U> BiPredicate<T, U> for BoxBiPredicate<T, U> {
    fn test(&self, first: &T, second: &U) -> bool {
        (self.function)(first, second)
    }
}
