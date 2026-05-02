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
//! Provides non-mutating bi-consumer interface implementations for operations
//! that accept two input parameters without modifying their own state or
//! the input values.
//!
//! It is similar to the `Fn(&T, &U)` trait in the standard library.
//!
//! This module provides a unified `BiConsumer` trait and three
//! concrete implementations based on different ownership models:
//!
//! - **`BoxBiConsumer<T, U>`**: Box-based single ownership
//! - **`ArcBiConsumer<T, U>`**: Arc-based thread-safe shared
//!   ownership
//! - **`RcBiConsumer<T, U>`**: Rc-based single-threaded shared
//!   ownership
//!
//! # Design Philosophy
//!
//! BiConsumer uses `Fn(&T, &U)` semantics: it is invoked through `&self` and
//! receives shared references to both input values.
//!
//! Suitable for pure observation, logging, and notification scenarios with two
//! parameters. Compared to `StatefulBiConsumer`, `BiConsumer` does not require
//! wrapper-level interior mutability (`Mutex`/`RefCell`), making it more
//! efficient and easier to share.
//!
use std::rc::Rc;
use std::sync::Arc;

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

// ==========================================================================
// Type Aliases
// ==========================================================================

/// Type alias for non-mutating bi-consumer function signature.
type BiConsumerFn<T, U> = dyn Fn(&T, &U);

/// Type alias for thread-safe non-mutating bi-consumer function signature.
type ThreadSafeBiConsumerFn<T, U> = dyn Fn(&T, &U) + Send + Sync;

mod box_bi_consumer;
pub use box_bi_consumer::BoxBiConsumer;
mod rc_bi_consumer;
pub use rc_bi_consumer::RcBiConsumer;
mod arc_bi_consumer;
pub use arc_bi_consumer::ArcBiConsumer;
mod fn_bi_consumer_ops;
pub use fn_bi_consumer_ops::FnBiConsumerOps;
mod box_conditional_bi_consumer;
pub use box_conditional_bi_consumer::BoxConditionalBiConsumer;
mod arc_conditional_bi_consumer;
pub use arc_conditional_bi_consumer::ArcConditionalBiConsumer;
mod rc_conditional_bi_consumer;
pub use rc_conditional_bi_consumer::RcConditionalBiConsumer;

// =======================================================================
// 1. BiConsumer Trait - Unified Interface
// =======================================================================

/// BiConsumer trait - Unified non-mutating bi-consumer interface
///
/// It is similar to the `Fn(&T, &U)` trait in the standard library.
///
/// Defines core behavior for all non-mutating bi-consumer types. The API uses
/// `&self` and shared input references, so callers can use a bi-consumer
/// without granting mutable access to the consumer wrapper or input values.
///
/// # Automatic Implementations
///
/// - All closures implementing `Fn(&T, &U)`
/// - `BoxBiConsumer<T, U>`, `ArcBiConsumer<T, U>`,
///   `RcBiConsumer<T, U>`
///
/// # Features
///
/// - **Unified Interface**: All non-mutating bi-consumer types share the same
///   `accept` method signature
/// - **Automatic Implementation**: Closures automatically implement this
///   trait with zero overhead
/// - **Type Conversions**: Easy conversion between ownership models
/// - **Generic Programming**: Write functions accepting any non-mutating
///   bi-consumer type
/// - **No Wrapper Interior Mutability**: No need for Mutex or RefCell in the
///   wrapper, making shared ownership more efficient
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BiConsumer, BoxBiConsumer};
///
/// fn apply_consumer<C: BiConsumer<i32, i32>>(
///     consumer: &C,
///     a: &i32,
///     b: &i32
/// ) {
///     consumer.accept(a, b);
/// }
///
/// let box_con = BoxBiConsumer::new(|x: &i32, y: &i32| {
///     println!("Sum: {}", x + y);
/// });
/// apply_consumer(&box_con, &5, &3);
/// ```
///
pub trait BiConsumer<T, U> {
    /// Performs the non-mutating consumption operation
    ///
    /// Executes an operation on the given two references. The operation
    /// typically reads input values or produces side effects, but neither
    /// modifies the input values nor the consumer's own state.
    ///
    /// # Parameters
    ///
    /// * `first` - Reference to the first value to consume
    /// * `second` - Reference to the second value to consume
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BiConsumer, BoxBiConsumer};
    ///
    /// let consumer = BoxBiConsumer::new(|x: &i32, y: &i32| {
    ///     println!("Values: {}, {}", x, y);
    /// });
    /// consumer.accept(&5, &3);
    /// ```
    fn accept(&self, first: &T, second: &U);

    /// Converts to BoxBiConsumer
    ///
    /// **⚠️ Consumes `self`**: Original consumer becomes unavailable after
    /// calling this method.
    ///
    /// # Returns
    ///
    /// Returns the wrapped `BoxBiConsumer<T, U>`
    fn into_box(self) -> BoxBiConsumer<T, U>
    where
        Self: Sized + 'static,
    {
        BoxBiConsumer::new(move |t, u| self.accept(t, u))
    }

    /// Converts to RcBiConsumer
    ///
    /// **⚠️ Consumes `self`**: Original consumer becomes unavailable after
    /// calling this method.
    ///
    /// # Returns
    ///
    /// Returns the wrapped `RcBiConsumer<T, U>`
    fn into_rc(self) -> RcBiConsumer<T, U>
    where
        Self: Sized + 'static,
    {
        RcBiConsumer::new(move |t, u| self.accept(t, u))
    }

    /// Converts to ArcBiConsumer
    ///
    /// **⚠️ Consumes `self`**: Original consumer becomes unavailable after
    /// calling this method.
    ///
    /// # Returns
    ///
    /// Returns the wrapped `ArcBiConsumer<T, U>`
    fn into_arc(self) -> ArcBiConsumer<T, U>
    where
        Self: Sized + Send + Sync + 'static,
    {
        ArcBiConsumer::new(move |t, u| self.accept(t, u))
    }

    /// Converts non-mutating bi-consumer to a closure
    ///
    /// **⚠️ Consumes `self`**: Original consumer becomes unavailable after
    /// calling this method.
    ///
    /// Converts the non-mutating bi-consumer to a closure usable with standard
    /// library methods requiring `Fn`.
    ///
    /// # Returns
    ///
    /// Returns a closure implementing `Fn(&T, &U)`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BiConsumer, BoxBiConsumer};
    ///
    /// let consumer = BoxBiConsumer::new(|x: &i32, y: &i32| {
    ///     println!("Sum: {}", x + y);
    /// });
    /// let func = consumer.into_fn();
    /// func(&5, &3);
    /// ```
    fn into_fn(self) -> impl Fn(&T, &U)
    where
        Self: Sized + 'static,
    {
        move |t, u| self.accept(t, u)
    }

    /// Convert to BiConsumerOnce
    ///
    /// **⚠️ Consumes `self`**: The original consumer will be unavailable after calling this method.
    ///
    /// Converts a reusable non-mutating bi-consumer to a one-time consumer that consumes itself on use.
    /// This enables passing `BiConsumer` to functions that require `BiConsumerOnce`.
    ///
    /// # Returns
    ///
    /// Returns a `BoxBiConsumerOnce<T, U>`
    fn into_once(self) -> BoxBiConsumerOnce<T, U>
    where
        Self: Sized + 'static,
    {
        BoxBiConsumerOnce::new(move |t, u| self.accept(t, u))
    }

    /// Converts to BoxBiConsumer (without consuming self)
    ///
    /// Creates a new `BoxBiConsumer` by cloning the current consumer.
    /// The original consumer remains usable after this call.
    ///
    /// # Returns
    ///
    /// Returns a new `BoxBiConsumer<T, U>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BiConsumer, ArcBiConsumer};
    ///
    /// let consumer = ArcBiConsumer::new(|x: &i32, y: &i32| {
    ///     println!("Sum: {}", x + y);
    /// });
    /// let box_consumer = consumer.to_box();
    /// box_consumer.accept(&5, &3);
    /// // Original consumer still usable
    /// consumer.accept(&10, &20);
    /// ```
    fn to_box(&self) -> BoxBiConsumer<T, U>
    where
        Self: Clone + 'static,
    {
        self.clone().into_box()
    }

    /// Converts to RcBiConsumer (without consuming self)
    ///
    /// Creates a new `RcBiConsumer` by cloning the current consumer.
    /// The original consumer remains usable after this call.
    ///
    /// # Returns
    ///
    /// Returns a new `RcBiConsumer<T, U>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BiConsumer, ArcBiConsumer};
    ///
    /// let consumer = ArcBiConsumer::new(|x: &i32, y: &i32| {
    ///     println!("Sum: {}", x + y);
    /// });
    /// let rc_consumer = consumer.to_rc();
    /// rc_consumer.accept(&5, &3);
    /// // Original consumer still usable
    /// consumer.accept(&10, &20);
    /// ```
    fn to_rc(&self) -> RcBiConsumer<T, U>
    where
        Self: Clone + 'static,
    {
        self.clone().into_rc()
    }

    /// Converts to ArcBiConsumer (without consuming self)
    ///
    /// Creates a new `ArcBiConsumer` by cloning the current consumer.
    /// The original consumer remains usable after this call.
    ///
    /// # Returns
    ///
    /// Returns a new `ArcBiConsumer<T, U>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BiConsumer, ArcBiConsumer};
    ///
    /// let consumer = ArcBiConsumer::new(|x: &i32, y: &i32| {
    ///     println!("Sum: {}", x + y);
    /// });
    /// let arc_consumer = consumer.to_arc();
    /// arc_consumer.accept(&5, &3);
    /// ```
    fn to_arc(&self) -> ArcBiConsumer<T, U>
    where
        Self: Clone + Send + Sync + 'static,
    {
        self.clone().into_arc()
    }

    /// Converts to a closure (without consuming self)
    ///
    /// Creates a new closure by cloning the current consumer.
    /// The original consumer remains usable after this call.
    ///
    /// # Returns
    ///
    /// Returns a closure implementing `Fn(&T, &U)`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BiConsumer, RcBiConsumer};
    ///
    /// let consumer = RcBiConsumer::new(|x: &i32, y: &i32| {
    ///     println!("Sum: {}", x + y);
    /// });
    /// let func = consumer.to_fn();
    /// func(&5, &3);
    /// // Original consumer still usable
    /// consumer.accept(&10, &20);
    /// ```
    fn to_fn(&self) -> impl Fn(&T, &U)
    where
        Self: Clone + 'static,
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
