/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # ConsumerOnce Types
//!
//! Provides implementations of one-time consumer interfaces for executing one-time operations
//! that accept a single input parameter but return no result.
//!
//! It is similar to the `FnOnce(&T)` trait in the standard library.
//!
//! This module provides a unified `ConsumerOnce` trait and one concrete implementation:
//!
//! - **`BoxConsumerOnce<T>`**: Box-based single ownership implementation
//!
//! # Why No Arc/Rc Variants?
//!
//! Unlike reusable [`Consumer`](crate::consumers::Consumer) implementations,
//! this module does **not** provide `ArcConsumerOnce` or `RcConsumerOnce`
//! implementations. This is a design decision based on the fact that `FnOnce`
//! semantics are fundamentally incompatible with shared ownership. See design
//! docs for details.
//!
//! # Design Philosophy
//!
//! ConsumerOnce uses `FnOnce(&T)` semantics for truly one-time consumption operations.
//!
//! Unlike Consumer, ConsumerOnce consumes itself on first call. Suitable for initialization
//! callbacks, cleanup callbacks, and similar scenarios.
//!
//! # Author
//!
//! Haixing Hu

use crate::consumers::macros::{
    impl_box_conditional_consumer,
    impl_box_consumer_methods,
    impl_conditional_consumer_debug_display,
    impl_consumer_common_methods,
    impl_consumer_debug_display,
};
use crate::macros::{
    impl_box_once_conversions,
    impl_closure_once_trait,
};
use crate::predicates::predicate::{
    BoxPredicate,
    Predicate,
};

mod box_consumer_once;
pub use box_consumer_once::BoxConsumerOnce;
mod fn_consumer_once_ops;
pub use fn_consumer_once_ops::FnConsumerOnceOps;
mod box_conditional_consumer_once;
pub use box_conditional_consumer_once::BoxConditionalConsumerOnce;

// ============================================================================
// 1. ConsumerOnce Trait - Unified ConsumerOnce Interface
// ============================================================================

/// ConsumerOnce trait - Unified one-time consumer interface
///
/// It is similar to the `FnOnce(&T)` trait in the standard library.
///
/// Defines the core behavior of all one-time consumer types. Similar to consumers
/// implementing `FnOnce(&T)`, executes operations that accept a value reference but
/// return no result (only side effects), consuming itself in the process.
///
/// # Automatic Implementation
///
/// - All closures implementing `FnOnce(&T)`
/// - `BoxConsumerOnce<T>`
///
/// # Features
///
/// - **Unified Interface**: All consumer types share the same `accept` method signature
/// - **Automatic Implementation**: Closures automatically implement this trait with zero overhead
/// - **Type Conversion**: Can be converted to BoxConsumerOnce
/// - **Generic Programming**: Write functions that work with any one-time consumer type
///
/// # Examples
///
/// ```rust
/// use qubit_function::{ConsumerOnce, BoxConsumerOnce};
/// use std::sync::{Arc, Mutex};
///
/// fn apply_consumer<C: ConsumerOnce<i32>>(consumer: C, value: &i32) {
///     consumer.accept(value);
/// }
///
/// let log = Arc::new(Mutex::new(Vec::new()));
/// let l = log.clone();
/// let box_con = BoxConsumerOnce::new(move |x: &i32| {
///     l.lock().unwrap().push(*x);
/// });
/// apply_consumer(box_con, &5);
/// assert_eq!(*log.lock().unwrap(), vec![5]);
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait ConsumerOnce<T> {
    /// Execute one-time consumption operation
    ///
    /// Executes an operation on the given reference. The operation typically reads
    /// the input value or produces side effects, but does not modify the input
    /// value itself. Consumes self.
    ///
    /// # Parameters
    ///
    /// * `value` - Reference to the value to be consumed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{ConsumerOnce, BoxConsumerOnce};
    ///
    /// let consumer = BoxConsumerOnce::new(|x: &i32| println!("{}", x));
    /// consumer.accept(&5);
    /// ```
    fn accept(self, value: &T);

    /// Convert to BoxConsumerOnce
    ///
    /// **⚠️ Consumes `self`**: The original consumer will be unavailable after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in a `BoxConsumerOnce` by calling
    /// `accept` on the consumer. Types can override this method to provide more
    /// efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns the wrapped `BoxConsumerOnce<T>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::ConsumerOnce;
    /// use std::sync::{Arc, Mutex};
    ///
    /// let log = Arc::new(Mutex::new(Vec::new()));
    /// let l = log.clone();
    /// let closure = move |x: &i32| {
    ///     l.lock().unwrap().push(*x);
    /// };
    /// let box_consumer = closure.into_box();
    /// box_consumer.accept(&5);
    /// assert_eq!(*log.lock().unwrap(), vec![5]);
    /// ```
    fn into_box(self) -> BoxConsumerOnce<T>
    where
        Self: Sized + 'static,
    {
        BoxConsumerOnce::new(move |t| self.accept(t))
    }

    /// Convert to closure
    ///
    /// **⚠️ Consumes `self`**: The original consumer will be unavailable after calling this method.
    ///
    /// Converts a one-time consumer to a closure that can be used directly in places
    /// where the standard library requires `FnOnce`.
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a closure that captures `self` and calls
    /// its `accept` method. Types can override this method to provide more efficient
    /// conversions.
    ///
    /// # Returns
    ///
    /// Returns a closure implementing `FnOnce(&T)`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::ConsumerOnce;
    /// use std::sync::{Arc, Mutex};
    ///
    /// let log = Arc::new(Mutex::new(Vec::new()));
    /// let l = log.clone();
    /// let closure = move |x: &i32| {
    ///     l.lock().unwrap().push(*x * 2);
    /// };
    /// let func = closure.into_fn();
    /// func(&5);
    /// assert_eq!(*log.lock().unwrap(), vec![10]);
    /// ```
    fn into_fn(self) -> impl FnOnce(&T)
    where
        Self: Sized + 'static,
    {
        move |t| self.accept(t)
    }

    /// Convert to BoxConsumerOnce without consuming self
    ///
    /// **⚠️ Requires Clone**: This method requires `Self` to implement
    /// `Clone`. Clones the current consumer and wraps it in a
    /// `BoxConsumerOnce`.
    ///
    /// # Default Implementation
    ///
    /// The default implementation clones `self` and then calls
    /// `into_box()` on the clone. Types can override this method to
    /// provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns the wrapped `BoxConsumerOnce<T>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::ConsumerOnce;
    /// use std::sync::{Arc, Mutex};
    ///
    /// let log = Arc::new(Mutex::new(Vec::new()));
    /// let l = log.clone();
    /// let closure = move |x: &i32| {
    ///     l.lock().unwrap().push(*x);
    /// };
    /// let box_consumer = closure.to_box();
    /// box_consumer.accept(&5);
    /// assert_eq!(*log.lock().unwrap(), vec![5]);
    /// ```
    fn to_box(&self) -> BoxConsumerOnce<T>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_box()
    }

    /// Convert to closure without consuming self
    ///
    /// **⚠️ Requires Clone**: This method requires `Self` to implement
    /// `Clone`. Clones the current consumer and then converts the clone
    /// to a closure.
    ///
    /// # Default Implementation
    ///
    /// The default implementation clones `self` and then calls
    /// `into_fn()` on the clone. Types can override this method to
    /// provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns a closure implementing `FnOnce(&T)`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::ConsumerOnce;
    /// use std::sync::{Arc, Mutex};
    ///
    /// let log = Arc::new(Mutex::new(Vec::new()));
    /// let l = log.clone();
    /// let closure = move |x: &i32| {
    ///     l.lock().unwrap().push(*x * 2);
    /// };
    /// let func = closure.to_fn();
    /// func(&5);
    /// assert_eq!(*log.lock().unwrap(), vec![10]);
    /// ```
    fn to_fn(&self) -> impl FnOnce(&T)
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_fn()
    }
}
