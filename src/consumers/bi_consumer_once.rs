/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # BiConsumerOnce Types
//!
//! Provides one-time bi-consumer interface implementations for operations
//! accepting two input parameters without returning a result.
//!
//! It is similar to the `FnOnce(&T, &U)` trait in the standard library.
//!
//! This module provides a unified `BiConsumerOnce` trait and one concrete
//! implementation:
//!
//! - **`BoxBiConsumerOnce<T, U>`**: Box-based single ownership
//!   implementation
//!
//! # Why No Arc/Rc Variants?
//!
//! Unlike reusable [`BiConsumer`](crate::consumers::BiConsumer)
//! implementations, this module does **not** provide `ArcBiConsumerOnce` or
//! `RcBiConsumerOnce` implementations. This is a design decision based on the
//! fact that `FnOnce` semantics require single ownership at the call site,
//! while `Arc` and `Rc` are meant to preserve shared ownership across clones.
//!
//! # Design Philosophy
//!
//! BiConsumerOnce uses `FnOnce(&T, &U)` semantics: for truly one-time
//! consumption operations.
//!
//! Unlike BiConsumer, BiConsumerOnce consumes itself on first call. Suitable
//! for initialization callbacks, cleanup callbacks, etc.
//!
use crate::{
    consumers::macros::{
        impl_box_conditional_consumer,
        impl_box_consumer_methods,
        impl_conditional_consumer_debug_display,
        impl_consumer_common_methods,
        impl_consumer_debug_display,
    },
    macros::{
        impl_box_once_conversions,
        impl_closure_once_trait,
    },
    predicates::bi_predicate::{
        BiPredicate,
        BoxBiPredicate,
    },
};

// ==========================================================================
// Type Aliases
// ==========================================================================

/// Type alias for bi-consumer once function signature.
type BiConsumerOnceFn<T, U> = dyn FnOnce(&T, &U);

mod box_bi_consumer_once;
pub use box_bi_consumer_once::BoxBiConsumerOnce;
mod fn_bi_consumer_once_ops;
pub use fn_bi_consumer_once_ops::FnBiConsumerOnceOps;
mod box_conditional_bi_consumer_once;
pub use box_conditional_bi_consumer_once::BoxConditionalBiConsumerOnce;

// =======================================================================
// 1. BiConsumerOnce Trait - Unified Interface
// =======================================================================

/// BiConsumerOnce trait - Unified one-time bi-consumer interface
///
/// It is similar to the `FnOnce(&T, &U)` trait in the standard library.
///
/// Defines core behavior for all one-time bi-consumer types. Similar to a
/// bi-consumer implementing `FnOnce(&T, &U)`, performs operations
/// accepting two value references but returning no result (side effects
/// only), consuming itself in the process.
///
/// # Automatic Implementations
///
/// - All closures implementing `FnOnce(&T, &U)`
/// - `BoxBiConsumerOnce<T, U>`
///
/// # Features
///
/// - **Unified Interface**: All bi-consumer types share the same `accept`
///   method signature
/// - **Automatic Implementation**: Closures automatically implement this
///   trait with zero overhead
/// - **Type Conversions**: Can convert to BoxBiConsumerOnce
/// - **Generic Programming**: Write functions accepting any one-time
///   bi-consumer type
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BiConsumerOnce, BoxBiConsumerOnce};
/// use std::sync::{Arc, Mutex};
///
/// fn apply_consumer<C: BiConsumerOnce<i32, i32>>(
///     consumer: C,
///     a: &i32,
///     b: &i32
/// ) {
///     consumer.accept(a, b);
/// }
///
/// let log = Arc::new(Mutex::new(Vec::new()));
/// let l = log.clone();
/// let box_con = BoxBiConsumerOnce::new(move |x: &i32, y: &i32| {
///     l.lock().expect("mutex should not be poisoned").push(*x + *y);
/// });
/// apply_consumer(box_con, &5, &3);
/// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8]);
/// ```
///
pub trait BiConsumerOnce<T, U> {
    /// Performs the one-time consumption operation
    ///
    /// Executes an operation on the given two references. The operation
    /// typically reads input values or produces side effects, but does not
    /// modify the input values themselves. Consumes self.
    ///
    /// # Parameters
    ///
    /// * `first` - Reference to the first value to consume
    /// * `second` - Reference to the second value to consume
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BiConsumerOnce, BoxBiConsumerOnce};
    ///
    /// let consumer = BoxBiConsumerOnce::new(|x: &i32, y: &i32| {
    ///     println!("Sum: {}", x + y);
    /// });
    /// consumer.accept(&5, &3);
    /// ```
    fn accept(self, first: &T, second: &U);

    /// Converts to BoxBiConsumerOnce
    ///
    /// **⚠️ Consumes `self`**: Original consumer becomes unavailable after
    /// calling this method.
    ///
    /// # Returns
    ///
    /// Returns the wrapped `BoxBiConsumerOnce<T, U>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::BiConsumerOnce;
    /// use std::sync::{Arc, Mutex};
    ///
    /// let log = Arc::new(Mutex::new(Vec::new()));
    /// let l = log.clone();
    /// let closure = move |x: &i32, y: &i32| {
    ///     l.lock().expect("mutex should not be poisoned").push(*x + *y);
    /// };
    /// let box_consumer = closure.into_box();
    /// box_consumer.accept(&5, &3);
    /// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8]);
    /// ```
    fn into_box(self) -> BoxBiConsumerOnce<T, U>
    where
        Self: Sized + 'static,
    {
        BoxBiConsumerOnce::new(move |t, u| self.accept(t, u))
    }

    /// Converts to a closure
    ///
    /// **⚠️ Consumes `self`**: Original consumer becomes unavailable after
    /// calling this method.
    ///
    /// Converts the one-time bi-consumer to a closure usable with standard
    /// library methods requiring `FnOnce`.
    ///
    /// # Returns
    ///
    /// Returns a closure implementing `FnOnce(&T, &U)`
    fn into_fn(self) -> impl FnOnce(&T, &U)
    where
        Self: Sized + 'static,
    {
        move |t, u| self.accept(t, u)
    }

    /// Convert to BoxBiConsumerOnce without consuming self
    ///
    /// **⚠️ Requires Clone**: This method requires `Self` to implement
    /// `Clone`. Clones the current bi-consumer and then converts the clone
    /// to a `BoxBiConsumerOnce`.
    ///
    /// # Returns
    ///
    /// Returns the wrapped `BoxBiConsumerOnce<T, U>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::BiConsumerOnce;
    /// use std::sync::{Arc, Mutex};
    ///
    /// let log = Arc::new(Mutex::new(Vec::new()));
    /// let l = log.clone();
    /// let closure = move |x: &i32, y: &i32| {
    ///     l.lock().expect("mutex should not be poisoned").push(*x + *y);
    /// };
    /// let box_consumer = closure.to_box();
    /// box_consumer.accept(&5, &3);
    /// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8]);
    /// ```
    fn to_box(&self) -> BoxBiConsumerOnce<T, U>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_box()
    }

    /// Convert to closure without consuming self
    ///
    /// **⚠️ Requires Clone**: This method requires `Self` to implement
    /// `Clone`. Clones the current bi-consumer and then converts the clone
    /// to a closure.
    ///
    /// # Returns
    ///
    /// Returns a closure implementing `FnOnce(&T, &U)`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::BiConsumerOnce;
    /// use std::sync::{Arc, Mutex};
    ///
    /// let log = Arc::new(Mutex::new(Vec::new()));
    /// let l = log.clone();
    /// let closure = move |x: &i32, y: &i32| {
    ///     l.lock().expect("mutex should not be poisoned").push(*x + *y);
    /// };
    /// let func = closure.to_fn();
    /// func(&5, &3);
    /// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8]);
    /// ```
    fn to_fn(&self) -> impl FnOnce(&T, &U)
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_fn()
    }
}
