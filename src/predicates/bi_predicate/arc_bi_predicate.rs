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
//! Defines the `ArcBiPredicate` public type.

use std::ops::Not;

use super::{
    ALWAYS_FALSE_NAME,
    ALWAYS_TRUE_NAME,
    Arc,
    BiPredicate,
    BoxBiPredicate,
    RcBiPredicate,
    SendSyncBiPredicateFn,
    impl_arc_conversions,
    impl_closure_trait,
    impl_predicate_clone,
    impl_predicate_common_methods,
    impl_predicate_debug_display,
    impl_shared_predicate_methods,
};

/// An Arc-based bi-predicate with thread-safe shared ownership.
///
/// This type is suitable for scenarios where the bi-predicate needs
/// to be shared across threads. Composition methods borrow `&self`,
/// allowing the original bi-predicate to remain usable after
/// composition.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BiPredicate, ArcBiPredicate};
///
/// let pred = ArcBiPredicate::new(|x: &i32, y: &i32| x + y > 0);
/// assert!(pred.test(&5, &3));
///
/// // Original bi-predicate remains usable after composition
/// let combined = pred.and(ArcBiPredicate::new(|x, y| x > y));
/// assert!(pred.test(&5, &3));  // Still works
///
/// // Can be cloned and sent across threads
/// let pred_clone = pred.clone();
/// std::thread::spawn(move || {
///     assert!(pred_clone.test(&10, &5));
/// }).join().expect("thread should not panic");
/// ```
///
pub struct ArcBiPredicate<T, U> {
    pub(super) function: Arc<SendSyncBiPredicateFn<T, U>>,
    pub(super) name: Option<String>,
}

impl<T, U> ArcBiPredicate<T, U> {
    // Generates: new(), new_with_name(), name(), set_name(), always_true(), always_false()
    impl_predicate_common_methods!(
        ArcBiPredicate<T, U>,
        (Fn(&T, &U) -> bool + Send + Sync + 'static),
        |f| Arc::new(f)
    );

    // Generates: and(), or(), nand(), xor(), nor()
    impl_shared_predicate_methods!(
        ArcBiPredicate<T, U>,
        Send + Sync + 'static
    );
}

impl<T, U> Not for ArcBiPredicate<T, U>
where
    T: 'static,
    U: 'static,
{
    type Output = ArcBiPredicate<T, U>;

    fn not(self) -> Self::Output {
        let function = self.function;
        ArcBiPredicate::new(move |first, second| !function(first, second))
    }
}

impl<T, U> Not for &ArcBiPredicate<T, U>
where
    T: 'static,
    U: 'static,
{
    type Output = ArcBiPredicate<T, U>;

    fn not(self) -> Self::Output {
        let function = self.function.clone();
        ArcBiPredicate::new(move |first, second| !function(first, second))
    }
}

// Generates: impl Clone for ArcBiPredicate<T, U>
impl_predicate_clone!(ArcBiPredicate<T, U>);

// Generates: impl Debug for ArcBiPredicate<T, U> and impl Display for ArcBiPredicate<T, U>
impl_predicate_debug_display!(ArcBiPredicate<T, U>);

// Implements BiPredicate trait for ArcBiPredicate<T, U>
impl<T, U> BiPredicate<T, U> for ArcBiPredicate<T, U> {
    fn test(&self, first: &T, second: &U) -> bool {
        (self.function)(first, second)
    }

    // Generates: into_box, into_rc, into_arc, into_fn, to_box, to_rc, to_arc, to_fn
    impl_arc_conversions!(
        ArcBiPredicate<T, U>,
        BoxBiPredicate,
        RcBiPredicate,
        Fn(first: &T, second: &U) -> bool
    );
}

// Blanket implementation for all closures that match
// Fn(&T, &U) -> bool. This provides optimal implementations for
// closures by wrapping them directly into the target type.
impl_closure_trait!(
    BiPredicate<T, U>,
    test,
    Fn(first: &T, second: &U) -> bool
);
