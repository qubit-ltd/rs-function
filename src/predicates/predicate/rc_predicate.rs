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
//! Defines the `RcPredicate` public type.

#![allow(unused_imports)]

use super::*;

/// An Rc-based predicate with single-threaded shared ownership.
///
/// This type is suitable for scenarios where the predicate needs to be
/// reused in a single-threaded context. Composition methods borrow `&self`,
/// allowing the original predicate to remain usable after composition.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{Predicate, RcPredicate};
///
/// let pred = RcPredicate::new(|x: &i32| *x > 0);
/// assert!(pred.test(&5));
///
/// // Original predicate remains usable after composition
/// let combined = pred.and(RcPredicate::new(|x| x % 2 == 0));
/// assert!(pred.test(&5));  // Still works
/// ```
///
pub struct RcPredicate<T> {
    pub(super) function: Rc<dyn Fn(&T) -> bool>,
    pub(super) name: Option<String>,
}

impl<T> RcPredicate<T> {
    // Generates: new(), new_with_name(), name(), set_name(), always_true(), always_false()
    impl_predicate_common_methods!(RcPredicate<T>, (Fn(&T) -> bool + 'static), |f| Rc::new(f));

    // Generates: and(), or(), not(), nand(), xor(), nor()
    impl_shared_predicate_methods!(RcPredicate<T>, 'static);
}

// Generates: impl Clone for RcPredicate<T>
impl_predicate_clone!(RcPredicate<T>);

// Generates: impl Debug for RcPredicate<T> and impl Display for RcPredicate<T>
impl_predicate_debug_display!(RcPredicate<T>);

// Implements Predicate trait for RcPredicate<T>
impl<T> Predicate<T> for RcPredicate<T> {
    fn test(&self, value: &T) -> bool {
        (self.function)(value)
    }

    // Generates: into_box(), into_rc(), into_fn(), to_box(), to_rc(), to_fn()
    impl_rc_conversions!(
        RcPredicate<T>,
        BoxPredicate,
        Fn(t: &T) -> bool
    );
}
