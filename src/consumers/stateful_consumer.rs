// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
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
//! - **`BoxStatefulConsumer<T>`**: Box-based single ownership implementation
//!   for one-time use scenarios
//! - **`ArcStatefulConsumer<T>`**: Thread-safe shared ownership implementation
//!   based on Arc<Mutex<>>
//! - **`RcStatefulConsumer<T>`**: Single-threaded shared ownership
//!   implementation based on Rc<RefCell<>>
//!
//! # Design Philosophy
//!
//! Consumer uses `FnMut(&T)` semantics, allowing modification of its own state
//! but not the input value.
//!
//! Suitable for statistics, accumulation, event handling, and other scenarios.
use std::cell::RefCell;
#[cfg(feature = "rc")]
use std::rc::Rc;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::consumers::macros::{
    impl_box_conditional_consumer, impl_box_consumer_methods, impl_conditional_consumer_clone, impl_conditional_consumer_debug_display,
    impl_consumer_clone, impl_consumer_common_methods, impl_consumer_debug_display,
    impl_shared_conditional_consumer, impl_shared_consumer_methods,
};
use crate::macros::{ impl_closure_trait,
};
use crate::predicates::predicate::{ArcPredicate, BoxPredicate, Predicate};
#[cfg(feature = "rc")]
use crate::predicates::predicate::RcPredicate;

mod box_stateful_consumer;
pub use box_stateful_consumer::BoxStatefulConsumer;
#[cfg(feature = "rc")]
mod rc_stateful_consumer;
#[cfg(feature = "rc")]
pub use rc_stateful_consumer::RcStatefulConsumer;
mod arc_stateful_consumer;
pub use arc_stateful_consumer::ArcStatefulConsumer;
#[cfg(feature = "combinators")]
mod fn_stateful_consumer_ops;
#[cfg(feature = "combinators")]
pub use fn_stateful_consumer_ops::FnStatefulConsumerOps;
mod box_conditional_stateful_consumer;
#[cfg(not(feature = "combinators"))]
pub(crate) use box_conditional_stateful_consumer::BoxConditionalStatefulConsumer;
#[cfg(feature = "combinators")]
pub use box_conditional_stateful_consumer::BoxConditionalStatefulConsumer;
mod arc_conditional_stateful_consumer;
#[cfg(not(feature = "combinators"))]
pub(crate) use arc_conditional_stateful_consumer::ArcConditionalStatefulConsumer;
#[cfg(feature = "combinators")]
pub use arc_conditional_stateful_consumer::ArcConditionalStatefulConsumer;
#[cfg(feature = "rc")]
mod rc_conditional_stateful_consumer;
#[cfg(feature = "rc")]
#[cfg(not(feature = "combinators"))]
pub(crate) use rc_conditional_stateful_consumer::RcConditionalStatefulConsumer;
#[cfg(all(feature = "rc", feature = "combinators"))]
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
/// - `BoxStatefulConsumer<T>`, `ArcStatefulConsumer<T>`,
///   `RcStatefulConsumer<T>`
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
///     l.lock().expect("mutex should not be poisoned").push(*x);
/// });
/// apply_consumer(&mut box_con, &5);
/// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);
/// ```
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
}
