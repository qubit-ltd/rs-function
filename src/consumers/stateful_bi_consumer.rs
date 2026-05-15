/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # BiConsumer Types
//!
//! Provides bi-consumer interface implementations for operations accepting
//! two input parameters without returning a result.
//!
//! It is similar to the `FnMut(&T, &U)` trait in the standard library.
//!
//! This module provides a unified `BiConsumer` trait and three concrete
//! implementations based on different ownership models:
//!
//! - **`BoxStatefulBiConsumer<T, U>`**: Box-based single ownership for one-time use
//! - **`ArcStatefulBiConsumer<T, U>`**: Arc<Mutex<>>-based thread-safe shared
//!   ownership
//! - **`RcStatefulBiConsumer<T, U>`**: Rc<RefCell<>>-based single-threaded shared
//!   ownership
//!
//! # Design Philosophy
//!
//! BiConsumer uses `FnMut(&T, &U)` semantics: can modify its own state but
//! does NOT modify input values.
//!
//! Suitable for statistics, accumulation, and event processing scenarios
//! involving two parameters.
//!
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::consumers::{
    bi_consumer_once::BoxBiConsumerOnce,
    macros::{
        impl_box_conditional_consumer,
        impl_box_consumer_methods,
        impl_conditional_consumer_clone,
        impl_conditional_consumer_conversions,
        impl_conditional_consumer_debug_display,
        impl_consumer_clone,
        impl_consumer_common_methods,
        impl_consumer_debug_display,
        impl_shared_conditional_consumer,
        impl_shared_consumer_methods,
    },
};
use crate::macros::{
    impl_arc_conversions,
    impl_box_conversions,
    impl_closure_trait,
    impl_rc_conversions,
};
use crate::predicates::bi_predicate::{
    ArcBiPredicate,
    BiPredicate,
    BoxBiPredicate,
    RcBiPredicate,
};

mod box_stateful_bi_consumer;
pub use box_stateful_bi_consumer::BoxStatefulBiConsumer;
mod rc_stateful_bi_consumer;
pub use rc_stateful_bi_consumer::RcStatefulBiConsumer;
mod arc_stateful_bi_consumer;
pub use arc_stateful_bi_consumer::ArcStatefulBiConsumer;
mod fn_stateful_bi_consumer_ops;
pub use fn_stateful_bi_consumer_ops::FnStatefulBiConsumerOps;
mod box_conditional_stateful_bi_consumer;
pub use box_conditional_stateful_bi_consumer::BoxConditionalStatefulBiConsumer;
mod arc_conditional_stateful_bi_consumer;
pub use arc_conditional_stateful_bi_consumer::ArcConditionalStatefulBiConsumer;
mod rc_conditional_stateful_bi_consumer;
pub use rc_conditional_stateful_bi_consumer::RcConditionalStatefulBiConsumer;

// =======================================================================
// 1. BiConsumer Trait - Unified BiConsumer Interface
// =======================================================================

/// BiConsumer trait - Unified bi-consumer interface
///
/// Defines core behavior for all bi-consumer types. Similar to Java's
/// `BiConsumer<T, U>` interface, performs operations accepting two values
/// but returning no result (side effects only).
///
/// It is similar to the `FnMut(&T, &U)` trait in the standard library.
///
/// BiConsumer can modify its own state (e.g., accumulate, count) but
/// should NOT modify the consumed values themselves.
///
/// # Automatic Implementations
///
/// - All closures implementing `FnMut(&T, &U)`
/// - `BoxStatefulBiConsumer<T, U>`, `ArcStatefulBiConsumer<T, U>`, `RcStatefulBiConsumer<T, U>`
///
/// # Features
///
/// - **Unified Interface**: All bi-consumer types share the same `accept`
///   method signature
/// - **Automatic Implementation**: Closures automatically implement this
///   trait with zero overhead
/// - **Type Conversions**: Easy conversion between ownership models
/// - **Generic Programming**: Write functions accepting any bi-consumer
///   type
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BiConsumer, BoxStatefulBiConsumer, StatefulBiConsumer};
/// use std::cell::RefCell;
/// use std::rc::Rc;
///
/// fn apply_bi_consumer<C: StatefulBiConsumer<i32, i32>>(
///     consumer: &mut C,
///     a: &i32,
///     b: &i32
/// ) {
///     consumer.accept(a, b);
/// }
///
/// // Works with any bi-consumer type
/// let log = Rc::new(RefCell::new(Vec::new()));
/// let l = log.clone();
/// let mut box_con = BoxStatefulBiConsumer::new(move |x: &i32, y: &i32| {
///     l.borrow_mut().push(*x + *y);
/// });
/// apply_bi_consumer(&mut box_con, &5, &3);
/// assert_eq!(*log.borrow(), vec![8]);
/// ```
///
pub trait StatefulBiConsumer<T, U> {
    /// Performs the consumption operation
    ///
    /// Executes an operation on the given two references. The operation
    /// typically reads input values or produces side effects, but does not
    /// modify the input values themselves. Can modify the consumer's own
    /// state.
    ///
    /// # Parameters
    ///
    /// * `first` - Reference to the first value to consume
    /// * `second` - Reference to the second value to consume
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BiConsumer, BoxStatefulBiConsumer, StatefulBiConsumer};
    /// use std::sync::{Arc, Mutex};
    ///
    /// let log = Arc::new(Mutex::new(Vec::new()));
    /// let l = log.clone();
    /// let mut consumer = BoxStatefulBiConsumer::new(move |x: &i32, y: &i32| {
    ///     l.lock().expect("mutex should not be poisoned").push(*x + *y);
    /// });
    /// consumer.accept(&5, &3);
    /// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8]);
    /// ```
    fn accept(&mut self, first: &T, second: &U);

    /// Converts to BoxStatefulBiConsumer
    ///
    /// **⚠️ Consumes `self`**: Original consumer becomes unavailable after
    /// calling this method.
    ///
    /// Converts the current bi-consumer to `BoxStatefulBiConsumer<T, U>`.
    ///
    /// # Ownership
    ///
    /// This method **consumes** the consumer (takes ownership of `self`).
    /// After calling, the original consumer is no longer available.
    ///
    /// **Tip**: For cloneable consumers ([`ArcStatefulBiConsumer`],
    /// [`RcStatefulBiConsumer`]), call `.clone()` first if you need to keep the
    /// original.
    ///
    /// # Returns
    ///
    /// Returns the wrapped `BoxStatefulBiConsumer<T, U>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BiConsumer, StatefulBiConsumer};
    /// use std::sync::{Arc, Mutex};
    ///
    /// let log = Arc::new(Mutex::new(Vec::new()));
    /// let l = log.clone();
    /// let closure = move |x: &i32, y: &i32| {
    ///     l.lock().expect("mutex should not be poisoned").push(*x + *y);
    /// };
    /// let mut box_consumer = StatefulBiConsumer::into_box(closure);
    /// box_consumer.accept(&5, &3);
    /// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8]);
    /// ```
    fn into_box(self) -> BoxStatefulBiConsumer<T, U>
    where
        Self: Sized + 'static,
    {
        let mut consumer = self;
        BoxStatefulBiConsumer::new(move |t, u| consumer.accept(t, u))
    }

    /// Converts to RcStatefulBiConsumer
    ///
    /// **⚠️ Consumes `self`**: Original consumer becomes unavailable after
    /// calling this method.
    ///
    /// # Returns
    ///
    /// Returns the wrapped `RcStatefulBiConsumer<T, U>`
    fn into_rc(self) -> RcStatefulBiConsumer<T, U>
    where
        Self: Sized + 'static,
    {
        let mut consumer = self;
        RcStatefulBiConsumer::new(move |t, u| consumer.accept(t, u))
    }

    /// Converts to ArcStatefulBiConsumer
    ///
    /// **⚠️ Consumes `self`**: Original consumer becomes unavailable after
    /// calling this method.
    ///
    /// # Returns
    ///
    /// Returns the wrapped `ArcStatefulBiConsumer<T, U>`
    fn into_arc(self) -> ArcStatefulBiConsumer<T, U>
    where
        Self: Sized + Send + 'static,
    {
        let mut consumer = self;
        ArcStatefulBiConsumer::new(move |t, u| consumer.accept(t, u))
    }

    /// Converts bi-consumer to a closure
    ///
    /// **⚠️ Consumes `self`**: Original consumer becomes unavailable after
    /// calling this method.
    ///
    /// Converts the bi-consumer to a closure usable with standard library
    /// methods requiring `FnMut`.
    ///
    /// # Returns
    ///
    /// Returns a closure implementing `FnMut(&T, &U)`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BiConsumer, ArcStatefulBiConsumer, StatefulBiConsumer};
    /// use std::sync::{Arc, Mutex};
    ///
    /// let log = Arc::new(Mutex::new(Vec::new()));
    /// let l = log.clone();
    /// let mut consumer = ArcStatefulBiConsumer::new(move |x: &i32, y: &i32| {
    ///     l.lock().expect("mutex should not be poisoned").push(*x + *y);
    /// });
    /// let mut func = consumer.into_fn();
    /// func(&5, &3);
    /// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8]);
    /// ```
    fn into_fn(self) -> impl FnMut(&T, &U)
    where
        Self: Sized + 'static,
    {
        let mut consumer = self;
        move |t, u| consumer.accept(t, u)
    }

    /// Convert to BiConsumerOnce
    ///
    /// **⚠️ Consumes `self`**: The original consumer will be unavailable after calling this method.
    ///
    /// Converts a reusable stateful bi-consumer to a one-time consumer that consumes itself on use.
    /// This enables passing `StatefulBiConsumer` to functions that require `BiConsumerOnce`.
    ///
    /// # Returns
    ///
    /// Returns a `BoxBiConsumerOnce<T, U>`
    fn into_once(self) -> BoxBiConsumerOnce<T, U>
    where
        Self: Sized + 'static,
    {
        BoxBiConsumerOnce::new(move |t, u| {
            let mut consumer = self;
            consumer.accept(t, u);
        })
    }

    /// Converts to BoxStatefulBiConsumer (non-consuming)
    ///
    /// **⚠️ Requires Clone**: Original consumer must implement Clone.
    ///
    /// Converts the current bi-consumer to `BoxStatefulBiConsumer<T, U>` by cloning
    /// it first.
    ///
    /// # Ownership
    ///
    /// This method does **not consume** the consumer. It clones the consumer
    /// and then converts the clone to `BoxStatefulBiConsumer<T, U>`. The original
    /// consumer remains available after calling this method.
    ///
    /// # Returns
    ///
    /// Returns the wrapped `BoxStatefulBiConsumer<T, U>` from the clone
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BiConsumer, ArcStatefulBiConsumer, StatefulBiConsumer};
    /// use std::sync::{Arc, Mutex};
    ///
    /// let log = Arc::new(Mutex::new(Vec::new()));
    /// let l = log.clone();
    /// let mut consumer = ArcStatefulBiConsumer::new(move |x: &i32, y: &i32| {
    ///     l.lock().expect("mutex should not be poisoned").push(*x + *y);
    /// });
    /// let mut box_consumer = consumer.to_box();
    /// box_consumer.accept(&5, &3);
    /// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8]);
    /// // Original consumer still usable
    /// consumer.accept(&2, &1);
    /// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8, 3]);
    /// ```
    fn to_box(&self) -> BoxStatefulBiConsumer<T, U>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_box()
    }

    /// Converts to RcStatefulBiConsumer (non-consuming)
    ///
    /// **⚠️ Requires Clone**: Original consumer must implement Clone.
    ///
    /// Converts the current bi-consumer to `RcStatefulBiConsumer<T, U>` by cloning
    /// it first.
    ///
    /// # Ownership
    ///
    /// This method does **not consume** the consumer. It clones the consumer
    /// and then converts the clone to `RcStatefulBiConsumer<T, U>`. The original
    /// consumer remains available after calling this method.
    ///
    /// # Returns
    ///
    /// Returns the wrapped `RcStatefulBiConsumer<T, U>` from the clone
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BiConsumer, ArcStatefulBiConsumer, StatefulBiConsumer};
    /// use std::sync::{Arc, Mutex};
    ///
    /// let log = Arc::new(Mutex::new(Vec::new()));
    /// let l = log.clone();
    /// let mut consumer = ArcStatefulBiConsumer::new(move |x: &i32, y: &i32| {
    ///     l.lock().expect("mutex should not be poisoned").push(*x + *y);
    /// });
    /// let mut rc_consumer = consumer.to_rc();
    /// rc_consumer.accept(&5, &3);
    /// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8]);
    /// // Original consumer still usable
    /// consumer.accept(&2, &1);
    /// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8, 3]);
    /// ```
    fn to_rc(&self) -> RcStatefulBiConsumer<T, U>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_rc()
    }

    /// Converts to ArcStatefulBiConsumer (non-consuming)
    ///
    /// **⚠️ Requires Clone + Send**: Original consumer must implement Clone +
    /// Send.
    ///
    /// Converts the current bi-consumer to `ArcStatefulBiConsumer<T, U>` by cloning
    /// it first.
    ///
    /// # Ownership
    ///
    /// This method does **not consume** the consumer. It clones the consumer
    /// and then converts the clone to `ArcStatefulBiConsumer<T, U>`. The original
    /// consumer remains available after calling this method.
    ///
    /// # Returns
    ///
    /// Returns the wrapped `ArcStatefulBiConsumer<T, U>` from the clone
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BiConsumer, ArcStatefulBiConsumer, StatefulBiConsumer};
    /// use std::sync::{Arc, Mutex};
    ///
    /// let log = Arc::new(Mutex::new(Vec::new()));
    /// let l = log.clone();
    /// let mut consumer = ArcStatefulBiConsumer::new(move |x: &i32, y: &i32| {
    ///     l.lock().expect("mutex should not be poisoned").push(*x + *y);
    /// });
    /// let mut arc_consumer = consumer.to_arc();
    /// arc_consumer.accept(&5, &3);
    /// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8]);
    /// // Original consumer still usable
    /// consumer.accept(&2, &1);
    /// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8, 3]);
    /// ```
    fn to_arc(&self) -> ArcStatefulBiConsumer<T, U>
    where
        Self: Sized + Clone + Send + 'static,
    {
        self.clone().into_arc()
    }

    /// Converts to closure (non-consuming)
    ///
    /// **⚠️ Requires Clone**: Original consumer must implement Clone.
    ///
    /// Converts the consumer to a closure that can be used directly in
    /// standard library functions requiring `FnMut`.
    ///
    /// # Ownership
    ///
    /// This method does **not consume** the consumer. It clones the consumer
    /// and then converts the clone to a closure. The original consumer
    /// remains available after calling this method.
    ///
    /// # Returns
    ///
    /// Returns a closure implementing `FnMut(&T, &U)` from the clone
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BiConsumer, ArcStatefulBiConsumer, StatefulBiConsumer};
    /// use std::sync::{Arc, Mutex};
    ///
    /// let log = Arc::new(Mutex::new(Vec::new()));
    /// let l = log.clone();
    /// let mut consumer = ArcStatefulBiConsumer::new(move |x: &i32, y: &i32| {
    ///     l.lock().expect("mutex should not be poisoned").push(*x + *y);
    /// });
    /// {
    ///     let mut func = consumer.to_fn();
    ///     func(&5, &3);
    ///     assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8]);
    /// }
    /// // Original consumer still usable
    /// consumer.accept(&2, &1);
    /// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8, 3]);
    /// ```
    fn to_fn(&self) -> impl FnMut(&T, &U)
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_fn()
    }

    /// Convert to BiConsumerOnce without consuming self
    ///
    /// **⚠️ Requires Clone**: This method requires `Self` to implement `Clone`.
    /// Clones the current consumer and converts the clone to a one-time consumer.
    ///
    /// # Returns
    ///
    /// Returns a `BoxBiConsumerOnce<T, U>`
    fn to_once(&self) -> BoxBiConsumerOnce<T, U>
    where
        Self: Clone + 'static,
    {
        self.clone().into_once()
    }
}
