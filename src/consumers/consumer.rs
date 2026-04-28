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
//! Provides implementations of non-mutating consumer interfaces for executing
//! operations that neither modify their own state nor modify input values.
//!
//! It is similar to the `Fn(&T)` trait in the standard library.
//!
//! This module provides a unified `Consumer` trait and three concrete
//! implementations based on different ownership models:
//!
//! - **`BoxConsumer<T>`**: Box-based single ownership implementation
//! - **`ArcConsumer<T>`**: Arc-based thread-safe shared ownership
//!   implementation
//! - **`RcConsumer<T>`**: Rc-based single-threaded shared ownership
//!   implementation
//!
//! # Design Philosophy
//!
//! Consumer uses `Fn(&T)` semantics: it is invoked through `&self` and receives
//! shared references to input values.
//!
//! Suitable for pure observation, logging, notification and other scenarios.
//! Compared to `StatefulConsumer`, `Consumer` does not require wrapper-level
//! interior mutability (`Mutex`/`RefCell`), making it more efficient and easier
//! to share.
//!
//! # Author
//!
//! Haixing Hu

use std::rc::Rc;
use std::sync::Arc;

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

mod box_consumer;
pub use box_consumer::BoxConsumer;
mod rc_consumer;
pub use rc_consumer::RcConsumer;
mod arc_consumer;
pub use arc_consumer::ArcConsumer;
mod fn_consumer_ops;
pub use fn_consumer_ops::FnConsumerOps;
mod box_conditional_consumer;
pub use box_conditional_consumer::BoxConditionalConsumer;
mod rc_conditional_consumer;
pub use rc_conditional_consumer::RcConditionalConsumer;
mod arc_conditional_consumer;
pub use arc_conditional_consumer::ArcConditionalConsumer;

// ============================================================================
// 1. Consumer Trait - Unified Consumer Interface
// ============================================================================

/// Consumer trait - Unified non-mutating consumer interface
///
/// It is similar to the `Fn(&T)` trait in the standard library.
///
/// Defines the core behavior of all non-mutating consumer types. The API uses
/// `&self` and shared input references, so callers can use a consumer without
/// granting mutable access to the consumer wrapper or input value.
///
/// # Auto-implementation
///
/// - All closures implementing `Fn(&T)`
/// - `BoxConsumer<T>`, `ArcConsumer<T>`,
///   `RcConsumer<T>`
///
/// # Features
///
/// - **Unified Interface**: All non-mutating consumer types share the same `accept`
///   method signature
/// - **Auto-implementation**: Closures automatically implement this trait with
///   zero overhead
/// - **Type Conversion**: Easy conversion between different ownership models
/// - **Generic Programming**: Write functions that work with any non-mutating
///   consumer type
/// - **No Wrapper Interior Mutability**: No need for Mutex or RefCell in the
///   wrapper, making shared ownership more efficient
///
/// # Examples
///
/// ```rust
/// use qubit_function::{Consumer, BoxConsumer};
///
/// fn apply_consumer<C: Consumer<i32>>(consumer: &C, value: &i32) {
///     consumer.accept(value);
/// }
///
/// let box_con = BoxConsumer::new(|x: &i32| {
///     println!("Value: {}", x);
/// });
/// apply_consumer(&box_con, &5);
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait Consumer<T> {
    /// Execute non-mutating consumption operation
    ///
    /// Performs an operation on the given reference. The operation typically
    /// reads input values or produces side effects, but neither modifies the
    /// input value nor the consumer's own state.
    ///
    /// # Parameters
    ///
    /// * `value` - Reference to the value to consume
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Consumer, BoxConsumer};
    ///
    /// let consumer = BoxConsumer::new(|x: &i32| println!("{}", x));
    /// consumer.accept(&5);
    /// ```
    fn accept(&self, value: &T);

    /// Convert to BoxConsumer
    ///
    /// **⚠️ Consumes `self`**: The original consumer will be unavailable after
    /// calling this method.
    ///
    /// # Returns
    ///
    /// Returns the wrapped `BoxConsumer<T>`
    fn into_box(self) -> BoxConsumer<T>
    where
        Self: Sized + 'static,
    {
        BoxConsumer::new(move |t| self.accept(t))
    }

    /// Convert to RcConsumer
    ///
    /// **⚠️ Consumes `self`**: The original consumer will be unavailable after
    /// calling this method.
    ///
    /// # Returns
    ///
    /// Returns the wrapped `RcConsumer<T>`
    fn into_rc(self) -> RcConsumer<T>
    where
        Self: Sized + 'static,
    {
        RcConsumer::new(move |t| self.accept(t))
    }

    /// Convert to ArcConsumer
    ///
    /// **⚠️ Consumes `self`**: The original consumer will be unavailable after
    /// calling this method.
    ///
    /// # Returns
    ///
    /// Returns the wrapped `ArcConsumer<T>`
    fn into_arc(self) -> ArcConsumer<T>
    where
        Self: Sized + Send + Sync + 'static,
    {
        ArcConsumer::new(move |t| self.accept(t))
    }

    /// Convert to closure
    ///
    /// **⚠️ Consumes `self`**: The original consumer will be unavailable after
    /// calling this method.
    ///
    /// Converts a non-mutating consumer to a closure that can be used directly in
    /// places where the standard library requires `Fn`.
    ///
    /// # Returns
    ///
    /// Returns a closure implementing `Fn(&T)`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Consumer, BoxConsumer};
    ///
    /// let consumer = BoxConsumer::new(|x: &i32| {
    ///     println!("Value: {}", x);
    /// });
    /// let func = consumer.into_fn();
    /// func(&5);
    /// ```
    fn into_fn(self) -> impl Fn(&T)
    where
        Self: Sized + 'static,
    {
        move |t| self.accept(t)
    }

    /// Convert to ConsumerOnce
    ///
    /// **⚠️ Consumes `self`**: The original consumer will be unavailable after calling this method.
    ///
    /// Converts a reusable non-mutating consumer to a one-time consumer that consumes itself on use.
    /// This enables passing `Consumer` to functions that require `ConsumerOnce`.
    ///
    /// # Returns
    ///
    /// Returns a `BoxConsumerOnce<T>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Consumer, ConsumerOnce, BoxConsumer};
    ///
    /// fn takes_once<C: ConsumerOnce<i32>>(consumer: C, value: &i32) {
    ///     consumer.accept(value);
    /// }
    ///
    /// let consumer = BoxConsumer::new(|x: &i32| println!("{}", x));
    /// takes_once(consumer.into_once(), &5);
    /// ```
    fn into_once(self) -> BoxConsumerOnce<T>
    where
        Self: Sized + 'static,
    {
        BoxConsumerOnce::new(move |t| self.accept(t))
    }

    /// Non-consuming conversion to `BoxConsumer`
    ///
    /// **⚠️ Does NOT consume `self`**: This method clones `self` and returns a
    /// boxed non-mutating consumer that calls the cloned consumer. Requires
    /// `Self: Clone` so it can be called through an immutable reference.
    ///
    /// # Returns
    ///
    /// Returns the wrapped `BoxConsumer<T>`
    fn to_box(&self) -> BoxConsumer<T>
    where
        Self: Clone + 'static,
    {
        self.clone().into_box()
    }

    /// Non-consuming conversion to `RcConsumer`
    ///
    /// **⚠️ Does NOT consume `self`**: Clones `self` and returns an
    /// `RcConsumer` that forwards to the cloned consumer. Requires
    /// `Self: Clone`.
    ///
    /// # Returns
    ///
    /// Returns the wrapped `RcConsumer<T>`
    fn to_rc(&self) -> RcConsumer<T>
    where
        Self: Clone + 'static,
    {
        self.clone().into_rc()
    }

    /// Non-consuming conversion to `ArcConsumer`
    ///
    /// **⚠️ Does NOT consume `self`**: Clones `self` and returns an
    /// `ArcConsumer`. Requires `Self: Clone + Send + Sync` so the result
    /// is thread-safe.
    ///
    /// # Returns
    ///
    /// Returns the wrapped `ArcConsumer<T>`
    fn to_arc(&self) -> ArcConsumer<T>
    where
        Self: Clone + Send + Sync + 'static,
    {
        self.clone().into_arc()
    }

    /// Non-consuming conversion to a boxed closure
    ///
    /// **⚠️ Does NOT consume `self`**: Returns a closure which calls a cloned
    /// copy of the consumer. Requires `Self: Clone`.
    ///
    /// # Returns
    ///
    /// Returns a closure implementing `Fn(&T)` which forwards to the cloned
    /// consumer.
    fn to_fn(&self) -> impl Fn(&T)
    where
        Self: Clone + 'static,
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
    /// use qubit_function::{Consumer, ConsumerOnce, ArcConsumer};
    ///
    /// fn takes_once<C: ConsumerOnce<i32>>(consumer: C, value: &i32) {
    ///     consumer.accept(value);
    /// }
    ///
    /// let consumer = ArcConsumer::new(|x: &i32| println!("{}", x));
    /// takes_once(consumer.to_once(), &5);
    /// ```
    fn to_once(&self) -> BoxConsumerOnce<T>
    where
        Self: Clone + 'static,
    {
        self.clone().into_once()
    }
}
