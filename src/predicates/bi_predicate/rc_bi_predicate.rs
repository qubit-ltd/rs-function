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
//! Defines the `RcBiPredicate` public type.

use std::ops::Not;

use super::{
    ALWAYS_FALSE_NAME,
    ALWAYS_TRUE_NAME,
    BiPredicate,
    BiPredicateFn,
    BoxBiPredicate,
    Rc,
    impl_predicate_clone,
    impl_predicate_common_methods,
    impl_predicate_debug_display,
    impl_rc_conversions,
    impl_shared_predicate_methods,
};

/// An Rc-based bi-predicate with single-threaded shared ownership.
///
/// This type is suitable for scenarios where the bi-predicate needs
/// to be reused in a single-threaded context. Composition methods
/// borrow `&self`, allowing the original bi-predicate to remain
/// usable after composition.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BiPredicate, RcBiPredicate};
///
/// let pred = RcBiPredicate::new(|x: &i32, y: &i32| x + y > 0);
/// assert!(pred.test(&5, &3));
///
/// // Original bi-predicate remains usable after composition
/// let combined = pred.and(RcBiPredicate::new(|x, y| x > y));
/// assert!(pred.test(&5, &3));  // Still works
/// ```
///
pub struct RcBiPredicate<T, U> {
    pub(super) function: Rc<BiPredicateFn<T, U>>,
    pub(super) name: Option<String>,
}

impl<T, U> RcBiPredicate<T, U> {
    // Generates: new(), new_with_name(), name(), set_name(), always_true(), always_false()
    impl_predicate_common_methods!(
        RcBiPredicate<T, U>,
        (Fn(&T, &U) -> bool + 'static),
        |f| Rc::new(f)
    );

    // Generates: and(), or(), nand(), xor(), nor()
    impl_shared_predicate_methods!(RcBiPredicate<T, U>, 'static);
}

impl<T, U> Not for RcBiPredicate<T, U>
where
    T: 'static,
    U: 'static,
{
    type Output = RcBiPredicate<T, U>;

    fn not(self) -> Self::Output {
        let function = self.function;
        RcBiPredicate::new(move |first, second| !function(first, second))
    }
}

impl<T, U> Not for &RcBiPredicate<T, U>
where
    T: 'static,
    U: 'static,
{
    type Output = RcBiPredicate<T, U>;

    fn not(self) -> Self::Output {
        let function = self.function.clone();
        RcBiPredicate::new(move |first, second| !function(first, second))
    }
}

// Generates: impl Clone for RcBiPredicate<T, U>
impl_predicate_clone!(RcBiPredicate<T, U>);

// Generates: impl Debug for RcBiPredicate<T, U> and impl Display for RcBiPredicate<T, U>
impl_predicate_debug_display!(RcBiPredicate<T, U>);

// Implements BiPredicate trait for RcBiPredicate<T, U>
impl<T, U> BiPredicate<T, U> for RcBiPredicate<T, U> {
    fn test(&self, first: &T, second: &U) -> bool {
        (self.function)(first, second)
    }

    // Generates: into_box(), into_rc(), into_fn(), to_box(), to_rc(), to_fn()
    impl_rc_conversions!(
        RcBiPredicate<T, U>,
        BoxBiPredicate,
        Fn(first: &T, second: &U) -> bool
    );
}
