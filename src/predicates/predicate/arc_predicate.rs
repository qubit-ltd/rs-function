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
//! Defines the `ArcPredicate` public type.

#![allow(unused_imports)]

use super::*;

/// An Arc-based predicate with thread-safe shared ownership.
///
/// This type is suitable for scenarios where the predicate needs to be
/// shared across threads. Composition methods borrow `&self`, allowing the
/// original predicate to remain usable after composition.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{Predicate, ArcPredicate};
///
/// let pred = ArcPredicate::new(|x: &i32| *x > 0);
/// assert!(pred.test(&5));
///
/// // Original predicate remains usable after composition
/// let combined = pred.and(ArcPredicate::new(|x| x % 2 == 0));
/// assert!(pred.test(&5));  // Still works
///
/// // Can be cloned and sent across threads
/// let pred_clone = pred.clone();
/// std::thread::spawn(move || {
///     assert!(pred_clone.test(&10));
/// }).join().unwrap();
/// ```
///
pub struct ArcPredicate<T> {
    pub(super) function: Arc<dyn Fn(&T) -> bool + Send + Sync>,
    pub(super) name: Option<String>,
}

impl<T> ArcPredicate<T> {
    // Generates: new(), new_with_name(), name(), set_name(), always_true(), always_false()
    impl_predicate_common_methods!(
        ArcPredicate<T>,
        (Fn(&T) -> bool + Send + Sync + 'static),
        |f| Arc::new(f)
    );

    // Generates: and(), or(), not(), nand(), xor(), nor()
    impl_shared_predicate_methods!(ArcPredicate<T>, Send + Sync + 'static);
}

// Generates: impl Clone for ArcPredicate<T>
impl_predicate_clone!(ArcPredicate<T>);

// Generates: impl Debug for ArcPredicate<T> and impl Display for ArcPredicate<T>
impl_predicate_debug_display!(ArcPredicate<T>);

// Implements Predicate trait for ArcPredicate<T>
impl<T> Predicate<T> for ArcPredicate<T> {
    fn test(&self, value: &T) -> bool {
        (self.function)(value)
    }

    // Generates: into_box, into_rc, into_arc, into_fn, to_box, to_rc, to_arc, to_fn
    impl_arc_conversions!(
        ArcPredicate<T>,
        BoxPredicate,
        RcPredicate,
        Fn(t: &T) -> bool
    );
}

// Blanket implementation for all closures that match Fn(&T) -> bool
impl_closure_trait!(
    Predicate<T>,
    test,
    Fn(value: &T) -> bool
);
