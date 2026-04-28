/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `BoxPredicate` public type.

#![allow(unused_imports)]

use super::*;

/// A Box-based predicate with single ownership.
///
/// This type is suitable for one-time use scenarios where the predicate does
/// not need to be cloned or shared. Composition methods consume `self`,
/// reflecting the single-ownership model.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{Predicate, BoxPredicate};
///
/// let pred = BoxPredicate::new(|x: &i32| *x > 0);
/// assert!(pred.test(&5));
///
/// // Chaining consumes the predicate
/// let combined = pred.and(BoxPredicate::new(|x| x % 2 == 0));
/// assert!(combined.test(&4));
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxPredicate<T> {
    pub(super) function: Box<dyn Fn(&T) -> bool>,
    pub(super) name: Option<String>,
}

impl<T> BoxPredicate<T> {
    // Generates: new(), new_with_name(), name(), set_name(), always_true(), always_false()
    impl_predicate_common_methods!(BoxPredicate<T>, (Fn(&T) -> bool + 'static), |f| Box::new(f));

    // Generates: and(), or(), not(), nand(), xor(), nor()
    impl_box_predicate_methods!(BoxPredicate<T>);
}

// Generates: impl Debug for BoxPredicate<T> and impl Display for BoxPredicate<T>
impl_predicate_debug_display!(BoxPredicate<T>);

// Implements Predicate trait for BoxPredicate<T>
impl<T> Predicate<T> for BoxPredicate<T> {
    fn test(&self, value: &T) -> bool {
        (self.function)(value)
    }

    // Generates: into_box(), into_rc(), into_fn()
    impl_box_conversions!(
        BoxPredicate<T>,
        RcPredicate,
        Fn(&T) -> bool
    );
}
