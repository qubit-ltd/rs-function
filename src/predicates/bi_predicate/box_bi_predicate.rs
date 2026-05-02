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
//! Defines the `BoxBiPredicate` public type.

#![allow(unused_imports)]

use super::*;

/// A Box-based bi-predicate with single ownership.
///
/// This type is suitable for one-time use scenarios where the
/// bi-predicate does not need to be cloned or shared. Composition
/// methods consume `self`, reflecting the single-ownership model.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BiPredicate, BoxBiPredicate};
///
/// let pred = BoxBiPredicate::new(|x: &i32, y: &i32| x + y > 0);
/// assert!(pred.test(&5, &3));
///
/// // Chaining consumes the bi-predicate
/// let combined = pred.and(BoxBiPredicate::new(|x, y| x > y));
/// assert!(combined.test(&10, &5));
/// ```
///
pub struct BoxBiPredicate<T, U> {
    pub(super) function: Box<BiPredicateFn<T, U>>,
    pub(super) name: Option<String>,
}

impl<T, U> BoxBiPredicate<T, U> {
    // Generates: new(), new_with_name(), name(), set_name(), always_true(), always_false()
    impl_predicate_common_methods!(
        BoxBiPredicate<T, U>,
        (Fn(&T, &U) -> bool + 'static),
        |f| Box::new(f)
    );

    // Generates: and(), or(), not(), nand(), xor(), nor()
    impl_box_predicate_methods!(BoxBiPredicate<T, U>);
}

// Generates: impl Debug for BoxBiPredicate<T, U> and impl Display for BoxBiPredicate<T, U>
impl_predicate_debug_display!(BoxBiPredicate<T, U>);

impl<T, U> BiPredicate<T, U> for BoxBiPredicate<T, U> {
    fn test(&self, first: &T, second: &U) -> bool {
        (self.function)(first, second)
    }

    // Generates: into_box(), into_rc(), into_fn()
    impl_box_conversions!(
        BoxBiPredicate<T, U>,
        RcBiPredicate,
        Fn(&T, &U) -> bool
    );
}
