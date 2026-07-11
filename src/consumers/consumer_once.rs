// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # ConsumerOnce Types
//!
//! Provides implementations of one-time consumer interfaces for executing
//! one-time operations that accept a single input parameter but return no
//! result.
//!
//! It is similar to the `FnOnce(&T)` trait in the standard library.
//!
//! This module provides a unified `ConsumerOnce` trait and one concrete
//! implementation:
//!
//! - **`BoxConsumerOnce<T>`**: Box-based single ownership implementation
//!
//! # Why No Arc/Rc Variants?
//!
//! Unlike reusable [`Consumer`](crate::consumers::Consumer) implementations,
//! this module does **not** provide `ArcConsumerOnce` or `RcConsumerOnce`
//! implementations. This is a design decision based on the fact that `FnOnce`
//! semantics require single ownership at the call site, while `Arc` and `Rc`
//! are meant to preserve shared ownership across clones.
//!
//! # Design Philosophy
//!
//! ConsumerOnce uses `FnOnce(&T)` semantics for truly one-time consumption
//! operations.
//!
//! Unlike Consumer, ConsumerOnce consumes itself on first call. Suitable for
//! initialization callbacks, cleanup callbacks, and similar scenarios.

use crate::consumers::macros::{
    impl_box_conditional_consumer,
    impl_box_consumer_methods,
    impl_conditional_consumer_debug_display,
    impl_consumer_common_methods,
    impl_consumer_debug_display,
};
use crate::macros::impl_closure_once_trait;
use crate::predicates::predicate::{
    BoxPredicate,
    Predicate,
};

mod box_consumer_once;
pub use box_consumer_once::BoxConsumerOnce;
#[cfg(feature = "combinators")]
mod fn_consumer_once_ops;
#[cfg(feature = "combinators")]
pub use fn_consumer_once_ops::FnConsumerOnceOps;
mod box_conditional_consumer_once;
#[cfg(not(feature = "combinators"))]
pub(crate) use box_conditional_consumer_once::BoxConditionalConsumerOnce;
#[cfg(feature = "combinators")]
pub use box_conditional_consumer_once::BoxConditionalConsumerOnce;

// ============================================================================
// 1. ConsumerOnce Trait - Unified ConsumerOnce Interface
// ============================================================================

/// ConsumerOnce trait - Unified one-time consumer interface
///
/// It is similar to the `FnOnce(&T)` trait in the standard library.
///
/// Defines the core behavior of all one-time consumer types. Similar to
/// consumers implementing `FnOnce(&T)`, executes operations that accept a value
/// reference but return no result (only side effects), consuming itself in the
/// process.
///
/// # Automatic Implementation
///
/// - All closures implementing `FnOnce(&T)`
/// - `BoxConsumerOnce<T>`
///
/// # Features
///
/// - **Unified Interface**: All consumer types share the same `accept` method
///   signature
/// - **Automatic Implementation**: Closures automatically implement this trait
///   with zero overhead
/// - **Type Conversion**: Can be converted to BoxConsumerOnce
/// - **Generic Programming**: Write functions that work with any one-time
///   consumer type
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
///     l.lock().expect("mutex should not be poisoned").push(*x);
/// });
/// apply_consumer(box_con, &5);
/// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);
/// ```
pub trait ConsumerOnce<T> {
    /// Execute one-time consumption operation
    ///
    /// Executes an operation on the given reference. The operation typically
    /// reads the input value or produces side effects, but does not modify
    /// the input value itself. Consumes self.
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
}
