/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Consumer Types
//!
//! Provides implementations of consumer interfaces for executing operations
//! that accept a single input parameter but return no result.
//!
//! It is similar to the `FnMut(&T)` trait in the standard library.
//!
//! This module provides a unified `Consumer` trait and three concrete
//! implementations based on different ownership models:
//!
//! - **`BoxStatefulConsumer<T>`**: Box-based single ownership implementation for
//!   one-time use scenarios
//! - **`ArcStatefulConsumer<T>`**: Thread-safe shared ownership implementation
//!   based on Arc<Mutex<>>
//! - **`RcStatefulConsumer<T>`**: Single-threaded shared ownership implementation
//!   based on Rc<RefCell<>>
//!
//! # Design Philosophy
//!
//! Consumer uses `FnMut(&T)` semantics, allowing modification of its own state
//! but not the input value.
//!
//! Suitable for statistics, accumulation, event handling, and other scenarios.
//!
//! # Author
//!
//! Haixing Hu
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::consumers::consumer_once::BoxConsumerOnce;
use crate::consumers::macros::{
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
};
use crate::macros::{
    impl_arc_conversions,
    impl_box_conversions,
    impl_closure_trait,
    impl_rc_conversions,
};
use crate::predicates::predicate::{
    ArcPredicate,
    BoxPredicate,
    Predicate,
    RcPredicate,
};

mod box_stateful_consumer;
pub use box_stateful_consumer::BoxStatefulConsumer;
mod rc_stateful_consumer;
pub use rc_stateful_consumer::RcStatefulConsumer;
mod arc_stateful_consumer;
pub use arc_stateful_consumer::ArcStatefulConsumer;
mod fn_stateful_consumer_ops;
pub use fn_stateful_consumer_ops::FnStatefulConsumerOps;
mod box_conditional_stateful_consumer;
pub use box_conditional_stateful_consumer::BoxConditionalStatefulConsumer;
mod arc_conditional_stateful_consumer;
pub use arc_conditional_stateful_consumer::ArcConditionalStatefulConsumer;
mod rc_conditional_stateful_consumer;
pub use rc_conditional_stateful_consumer::RcConditionalStatefulConsumer;

// ============================================================================
// 1. Consumer Trait - Unified Consumer Interface
// ============================================================================

/// Consumer trait - Unified consumer interface
///
/// Defines the core behavior of all consumer types. Similar to Java's
/// `Consumer<T>` interface, executes operations that accept a value but return
/// no result (side effects only).
///
/// It is similar to the `FnMut(&T)` trait in the standard library.
///
/// Consumer can modify its own state (such as accumulation, counting), but
/// should not modify the consumed value itself.
///
/// # Automatic Implementation
///
/// - All closures implementing `FnMut(&T)`
/// - `BoxStatefulConsumer<T>`, `ArcStatefulConsumer<T>`, `RcStatefulConsumer<T>`
///
/// # Features
///
/// - **Unified Interface**: All consumer types share the same `accept` method
///   signature
/// - **Automatic Implementation**: Closures automatically implement this trait
///   with zero overhead
/// - **Type Conversion**: Easy conversion between different ownership models
/// - **Generic Programming**: Write functions that work with any consumer type
///
/// # Examples
///
/// ```rust
/// use qubit_function::{Consumer, StatefulConsumer, BoxStatefulConsumer, ArcStatefulConsumer};
/// use std::sync::{Arc, Mutex};
///
/// fn apply_consumer<C: StatefulConsumer<i32>>(consumer: &mut C, value: &i32) {
///     consumer.accept(value);
/// }
///
/// // Works with any consumer type
/// let log = Arc::new(Mutex::new(Vec::new()));
/// let l = log.clone();
/// let mut box_con = BoxStatefulConsumer::new(move |x: &i32| {
///     l.lock().unwrap().push(*x);
/// });
/// apply_consumer(&mut box_con, &5);
/// assert_eq!(*log.lock().unwrap(), vec![5]);
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait StatefulConsumer<T> {
    /// Execute consumption operation
    ///
    /// Performs an operation on the given reference. The operation typically
    /// reads the input value or produces side effects, but does not modify the
    /// input value itself. Can modify the consumer's own state.
    ///
    /// # Parameters
    ///
    /// * `value` - Reference to the value to be consumed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Consumer, StatefulConsumer, BoxStatefulConsumer};
    ///
    /// let mut consumer = BoxStatefulConsumer::new(|x: &i32| println!("{}", x));
    /// let value = 5;
    /// consumer.accept(&value);
    /// ```
    fn accept(&mut self, value: &T);

    /// Convert to BoxStatefulConsumer
    ///
    /// **⚠️ Consumes `self`**: The original consumer will be unavailable after
    /// calling this method.
    ///
    /// Converts the current consumer to `BoxStatefulConsumer<T>`.
    ///
    /// # Ownership
    ///
    /// This method **consumes** the consumer (takes ownership of `self`).
    /// After calling this method, the original consumer is no longer available.
    ///
    /// **Tip**: For cloneable consumers ([`ArcStatefulConsumer`], [`RcStatefulConsumer`]),
    /// if you need to preserve the original object, you can call `.clone()`
    /// first.
    ///
    /// # Return Value
    ///
    /// Returns the wrapped `BoxStatefulConsumer<T>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::Consumer;
    /// use std::sync::{Arc, Mutex};
    ///
    /// let log = Arc::new(Mutex::new(Vec::new()));
    /// let l = log.clone();
    /// let closure = move |x: &i32| {
    ///     l.lock().unwrap().push(*x);
    /// };
    /// let mut box_consumer = closure.into_box();
    /// box_consumer.accept(&5);
    /// assert_eq!(*log.lock().unwrap(), vec![5]);
    /// ```
    fn into_box(self) -> BoxStatefulConsumer<T>
    where
        Self: Sized + 'static,
    {
        let mut consumer = self;
        BoxStatefulConsumer::new(move |t| consumer.accept(t))
    }

    /// Convert to RcStatefulConsumer
    ///
    /// **⚠️ Consumes `self`**: The original consumer will be unavailable after
    /// calling this method.
    ///
    /// # Return Value
    ///
    /// Returns the wrapped `RcStatefulConsumer<T>`
    fn into_rc(self) -> RcStatefulConsumer<T>
    where
        Self: Sized + 'static,
    {
        let mut consumer = self;
        RcStatefulConsumer::new(move |t| consumer.accept(t))
    }

    /// Convert to ArcStatefulConsumer
    ///
    /// **⚠️ Consumes `self`**: The original consumer will be unavailable after
    /// calling this method.
    ///
    /// # Return Value
    ///
    /// Returns the wrapped `ArcStatefulConsumer<T>`
    fn into_arc(self) -> ArcStatefulConsumer<T>
    where
        Self: Sized + Send + 'static,
    {
        let mut consumer = self;
        ArcStatefulConsumer::new(move |t| consumer.accept(t))
    }

    /// Convert to closure
    ///
    /// **⚠️ Consumes `self`**: The original consumer will be unavailable after
    /// calling this method.
    ///
    /// Converts the consumer to a closure that can be used directly in standard
    /// library functions requiring `FnMut`.
    ///
    /// # Return Value
    ///
    /// Returns a closure implementing `FnMut(&T)`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Consumer, StatefulConsumer, RcStatefulConsumer};
    /// use std::sync::{Arc, Mutex};
    ///
    /// let log = Arc::new(Mutex::new(Vec::new()));
    /// let l = log.clone();
    /// let mut consumer = RcStatefulConsumer::new(move |x: &i32| {
    ///     l.lock().unwrap().push(*x);
    /// });
    /// let mut func = consumer.into_fn();
    /// func(&5);
    /// assert_eq!(*log.lock().unwrap(), vec![5]);
    /// ```
    fn into_fn(self) -> impl FnMut(&T)
    where
        Self: Sized + 'static,
    {
        let mut consumer = self;
        move |t| consumer.accept(t)
    }

    /// Convert to ConsumerOnce
    ///
    /// **⚠️ Consumes `self`**: The original consumer will be unavailable after calling this method.
    ///
    /// Converts a reusable stateful consumer to a one-time consumer that consumes itself on use.
    /// This enables passing `StatefulConsumer` to functions that require `ConsumerOnce`.
    ///
    /// # Returns
    ///
    /// Returns a `BoxConsumerOnce<T>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Consumer, ConsumerOnce, StatefulConsumer, BoxStatefulConsumer};
    ///
    /// fn takes_once<C: ConsumerOnce<i32>>(consumer: C, value: &i32) {
    ///     consumer.accept(value);
    /// }
    ///
    /// let consumer = BoxStatefulConsumer::new(|x: &i32| println!("{}", x));
    /// takes_once(consumer.into_once(), &5);
    /// ```
    fn into_once(self) -> BoxConsumerOnce<T>
    where
        Self: Sized + 'static,
    {
        BoxConsumerOnce::new(move |t| {
            let mut consumer = self;
            consumer.accept(t);
        })
    }

    /// Convert to BoxStatefulConsumer
    ///
    /// **⚠️ Requires Clone**: The original consumer must implement Clone.
    ///
    /// Converts the current consumer to `BoxStatefulConsumer<T>` by cloning it first.
    ///
    /// # Ownership
    ///
    /// This method does **not consume** the consumer. It clones the consumer and
    /// then converts the clone to `BoxStatefulConsumer<T>`. The original consumer remains
    /// available after calling this method.
    ///
    /// # Return Value
    ///
    /// Returns the wrapped `BoxStatefulConsumer<T>` from the clone
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Consumer, StatefulConsumer, ArcStatefulConsumer};
    /// use std::sync::{Arc, Mutex};
    ///
    /// let log = Arc::new(Mutex::new(Vec::new()));
    /// let l = log.clone();
    /// let mut consumer = ArcStatefulConsumer::new(move |x: &i32| {
    ///     l.lock().unwrap().push(*x);
    /// });
    /// let mut box_consumer = consumer.to_box();
    /// box_consumer.accept(&5);
    /// assert_eq!(*log.lock().unwrap(), vec![5]);
    /// // Original consumer still usable
    /// consumer.accept(&3);
    /// assert_eq!(*log.lock().unwrap(), vec![5, 3]);
    /// ```
    fn to_box(&self) -> BoxStatefulConsumer<T>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_box()
    }

    /// Convert to RcStatefulConsumer
    ///
    /// **⚠️ Requires Clone**: The original consumer must implement Clone.
    ///
    /// Converts the current consumer to `RcStatefulConsumer<T>` by cloning it first.
    ///
    /// # Ownership
    ///
    /// This method does **not consume** the consumer. It clones the consumer and
    /// then converts the clone to `RcStatefulConsumer<T>`. The original consumer remains
    /// available after calling this method.
    ///
    /// # Return Value
    ///
    /// Returns the wrapped `RcStatefulConsumer<T>` from the clone
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Consumer, StatefulConsumer, ArcStatefulConsumer};
    /// use std::sync::{Arc, Mutex};
    ///
    /// let log = Arc::new(Mutex::new(Vec::new()));
    /// let l = log.clone();
    /// let mut consumer = ArcStatefulConsumer::new(move |x: &i32| {
    ///     l.lock().unwrap().push(*x);
    /// });
    /// let mut rc_consumer = consumer.to_rc();
    /// rc_consumer.accept(&5);
    /// assert_eq!(*log.lock().unwrap(), vec![5]);
    /// // Original consumer still usable
    /// consumer.accept(&3);
    /// assert_eq!(*log.lock().unwrap(), vec![5, 3]);
    /// ```
    fn to_rc(&self) -> RcStatefulConsumer<T>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_rc()
    }

    /// Convert to ArcStatefulConsumer
    ///
    /// **⚠️ Requires Clone + Send**: The original consumer must implement
    /// Clone + Send.
    ///
    /// Converts the current consumer to `ArcStatefulConsumer<T>` by cloning it first.
    ///
    /// # Ownership
    ///
    /// This method does **not consume** the consumer. It clones the consumer and
    /// then converts the clone to `ArcStatefulConsumer<T>`. The original consumer remains
    /// available after calling this method.
    ///
    /// # Return Value
    ///
    /// Returns the wrapped `ArcStatefulConsumer<T>` from the clone
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Consumer, StatefulConsumer, ArcStatefulConsumer};
    /// use std::sync::{Arc, Mutex};
    ///
    /// let log = Arc::new(Mutex::new(Vec::new()));
    /// let l = log.clone();
    /// let mut consumer = ArcStatefulConsumer::new(move |x: &i32| {
    ///     l.lock().unwrap().push(*x);
    /// });
    /// let mut arc_consumer = consumer.to_arc();
    /// arc_consumer.accept(&5);
    /// assert_eq!(*log.lock().unwrap(), vec![5]);
    /// // Original consumer still usable
    /// consumer.accept(&3);
    /// assert_eq!(*log.lock().unwrap(), vec![5, 3]);
    /// ```
    fn to_arc(&self) -> ArcStatefulConsumer<T>
    where
        Self: Sized + Clone + Send + 'static,
    {
        self.clone().into_arc()
    }

    /// Convert to closure
    ///
    /// **⚠️ Requires Clone**: The original consumer must implement Clone.
    ///
    /// Converts the consumer to a closure that can be used directly in standard
    /// library functions requiring `FnMut`.
    ///
    /// # Ownership
    ///
    /// This method does **not consume** the consumer. It clones the consumer and
    /// then converts the clone to a closure. The original consumer remains
    /// available after calling this method.
    ///
    /// # Return Value
    ///
    /// Returns a closure implementing `FnMut(&T)` from the clone
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Consumer, StatefulConsumer, RcStatefulConsumer};
    /// use std::sync::{Arc, Mutex};
    ///
    /// let log = Arc::new(Mutex::new(Vec::new()));
    /// let l = log.clone();
    /// let mut consumer = RcStatefulConsumer::new(move |x: &i32| {
    ///     l.lock().unwrap().push(*x);
    /// });
    /// {
    ///     let mut func = consumer.to_fn();
    ///     func(&5);
    /// }
    /// assert_eq!(*log.lock().unwrap(), vec![5]);
    /// // Original consumer still usable
    /// consumer.accept(&3);
    /// assert_eq!(*log.lock().unwrap(), vec![5, 3]);
    /// ```
    fn to_fn(&self) -> impl FnMut(&T)
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_fn()
    }

    /// Convert to ConsumerOnce without consuming self
    ///
    /// **⚠️ Requires Clone**: This method requires `Self` to implement `Clone`.
    /// Clones the current consumer and converts the clone to a one-time consumer.
    ///
    /// # Returns
    ///
    /// Returns a `BoxConsumerOnce<T>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Consumer, ConsumerOnce, StatefulConsumer, RcStatefulConsumer};
    ///
    /// fn takes_once<C: ConsumerOnce<i32>>(consumer: C, value: &i32) {
    ///     consumer.accept(value);
    /// }
    ///
    /// let consumer = RcStatefulConsumer::new(|x: &i32| println!("{}", x));
    /// takes_once(consumer.to_once(), &5);
    /// ```
    fn to_once(&self) -> BoxConsumerOnce<T>
    where
        Self: Clone + 'static,
    {
        self.clone().into_once()
    }
}
