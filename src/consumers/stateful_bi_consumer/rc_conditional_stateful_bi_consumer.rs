/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `RcConditionalStatefulBiConsumer` public type.

#![allow(unused_imports)]

use super::*;

// =======================================================================
// 9. RcConditionalStatefulBiConsumer - Rc-based Conditional BiConsumer
// =======================================================================

/// RcConditionalStatefulBiConsumer struct
///
/// A single-threaded conditional bi-consumer that only executes when a predicate is
/// satisfied. Uses `RcStatefulBiConsumer` and `RcBiPredicate` for shared ownership within a
/// single thread.
///
/// This type is typically created by calling `RcStatefulBiConsumer::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Conditional Execution**: Only consumes when predicate returns `true`
/// - **No Lock Overhead**: More efficient than `ArcConditionalStatefulBiConsumer`
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BiConsumer, RcStatefulBiConsumer, StatefulBiConsumer};
/// use std::rc::Rc;
/// use std::cell::RefCell;
///
/// let log = Rc::new(RefCell::new(Vec::new()));
/// let l = log.clone();
/// let conditional = RcStatefulBiConsumer::new(move |x: &i32, y: &i32| {
///     l.borrow_mut().push(*x + *y);
/// }).when(|x: &i32, y: &i32| *x > 0 && *y > 0);
///
/// let conditional_clone = conditional.clone();
///
/// let mut value = 5;
/// let mut m = conditional;
/// m.accept(&value, &3);
/// assert_eq!(*log.borrow(), vec![8]);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct RcConditionalStatefulBiConsumer<T, U> {
    pub(super) consumer: RcStatefulBiConsumer<T, U>,
    pub(super) predicate: RcBiPredicate<T, U>,
}

// Use macro to generate and_then and or_else methods
impl_shared_conditional_consumer!(
    RcConditionalStatefulBiConsumer<T, U>,
    RcStatefulBiConsumer,
    StatefulBiConsumer,
    into_rc,
    'static
);

impl<T, U> StatefulBiConsumer<T, U> for RcConditionalStatefulBiConsumer<T, U> {
    fn accept(&mut self, first: &T, second: &U) {
        if self.predicate.test(first, second) {
            self.consumer.accept(first, second);
        }
    }

    // Generates: into_box(), into_rc(), into_fn()
    impl_conditional_consumer_conversions!(
        BoxStatefulBiConsumer<T, U>,
        RcStatefulBiConsumer,
        FnMut
    );
}

// Use macro to generate Clone implementation
impl_conditional_consumer_clone!(RcConditionalStatefulBiConsumer<T, U>);

// Use macro to generate Debug and Display implementations
impl_conditional_consumer_debug_display!(RcConditionalStatefulBiConsumer<T, U>);
