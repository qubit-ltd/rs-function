// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Consumer Types
//!
//! Provides consumer interfaces for operations invoked through `&self` with a
//! shared input reference. The API does not grant direct mutable access to the
//! wrapper or input, but callbacks may use interior mutability or external
//! side effects.
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
//! Suitable for observation, logging, notification and other scenarios.
//! Compared to `StatefulConsumer`, `Consumer` does not require wrapper-level
//! interior mutability (`Mutex`/`RefCell`), making it more efficient and easier
//! to share.

mod box_consumer;
pub use box_consumer::BoxConsumer;
#[cfg(feature = "rc")]
mod rc_consumer;
#[cfg(feature = "rc")]
pub use rc_consumer::RcConsumer;
mod arc_consumer;
pub use arc_consumer::ArcConsumer;
mod box_conditional_consumer;
pub use box_conditional_consumer::BoxConditionalConsumer;
#[cfg(feature = "rc")]
mod rc_conditional_consumer;
#[cfg(feature = "rc")]
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
/// - `BoxConsumer<T>`, `ArcConsumer<T>`, `RcConsumer<T>`
///
/// # Features
///
/// - **Unified Interface**: All non-mutating consumer types share the same
///   `accept` method signature
/// - **Auto-implementation**: Closures implement this trait directly, without
///   allocating an adapter
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
    /// reads input values or produces side effects. It receives no direct
    /// mutable access to the input or consumer, though interior mutability and
    /// external state remain possible.
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

crate::macros::impl_closure_trait!(
    Consumer<T>,
    accept,
    Fn(value: &T)
);
