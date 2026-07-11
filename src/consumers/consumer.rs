// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
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

#[cfg(feature = "rc")]
use std::rc::Rc;
use std::sync::Arc;

use crate::consumers::macros::{
    impl_box_conditional_consumer,
    impl_box_consumer_methods,
    impl_conditional_consumer_clone,
    impl_conditional_consumer_debug_display,
    impl_consumer_clone,
    impl_consumer_common_methods,
    impl_consumer_debug_display,
    impl_shared_conditional_consumer,
    impl_shared_consumer_methods,
};
use crate::macros::impl_closure_trait;
#[cfg(feature = "rc")]
use crate::predicates::predicate::RcPredicate;
use crate::predicates::predicate::{
    ArcPredicate,
    BoxPredicate,
    Predicate,
};

mod box_consumer;
pub use box_consumer::BoxConsumer;
#[cfg(feature = "rc")]
mod rc_consumer;
#[cfg(feature = "rc")]
pub use rc_consumer::RcConsumer;
mod arc_consumer;
pub use arc_consumer::ArcConsumer;
#[cfg(feature = "combinators")]
mod fn_consumer_ops;
#[cfg(feature = "combinators")]
pub use fn_consumer_ops::FnConsumerOps;
mod box_conditional_consumer;
#[cfg(not(feature = "combinators"))]
pub(crate) use box_conditional_consumer::BoxConditionalConsumer;
#[cfg(feature = "combinators")]
pub use box_conditional_consumer::BoxConditionalConsumer;
#[cfg(feature = "rc")]
mod rc_conditional_consumer;
#[cfg(feature = "rc")]
#[cfg(not(feature = "combinators"))]
pub(crate) use rc_conditional_consumer::RcConditionalConsumer;
#[cfg(all(feature = "rc", feature = "combinators"))]
pub use rc_conditional_consumer::RcConditionalConsumer;
mod arc_conditional_consumer;
#[cfg(not(feature = "combinators"))]
pub(crate) use arc_conditional_consumer::ArcConditionalConsumer;
#[cfg(feature = "combinators")]
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
/// - `BoxConsumer<T>`, `ArcConsumer<T>`, `RcConsumer<T>`
///
/// # Features
///
/// - **Unified Interface**: All non-mutating consumer types share the same
///   `accept` method signature
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
}
