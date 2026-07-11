// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
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
//! - **`BoxStatefulBiConsumer<T, U>`**: Box-based single ownership for one-time
//!   use
//! - **`ArcStatefulBiConsumer<T, U>`**: Arc<Mutex<>>-based thread-safe shared
//!   ownership
//! - **`RcStatefulBiConsumer<T, U>`**: Rc<RefCell<>>-based single-threaded
//!   shared ownership
//!
//! # Design Philosophy
//!
//! BiConsumer uses `FnMut(&T, &U)` semantics: can modify its own state but
//! does NOT modify input values.
//!
//! Suitable for statistics, accumulation, and event processing scenarios
//! involving two parameters.
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
use crate::predicates::bi_predicate::{ArcBiPredicate, BiPredicate, BoxBiPredicate};
#[cfg(feature = "rc")]
use crate::predicates::bi_predicate::RcBiPredicate;

mod box_stateful_bi_consumer;
pub use box_stateful_bi_consumer::BoxStatefulBiConsumer;
#[cfg(feature = "rc")]
mod rc_stateful_bi_consumer;
#[cfg(feature = "rc")]
pub use rc_stateful_bi_consumer::RcStatefulBiConsumer;
mod arc_stateful_bi_consumer;
pub use arc_stateful_bi_consumer::ArcStatefulBiConsumer;
#[cfg(feature = "combinators")]
mod fn_stateful_bi_consumer_ops;
#[cfg(feature = "combinators")]
pub use fn_stateful_bi_consumer_ops::FnStatefulBiConsumerOps;
mod box_conditional_stateful_bi_consumer;
#[cfg(not(feature = "combinators"))]
pub(crate) use box_conditional_stateful_bi_consumer::BoxConditionalStatefulBiConsumer;
#[cfg(feature = "combinators")]
pub use box_conditional_stateful_bi_consumer::BoxConditionalStatefulBiConsumer;
mod arc_conditional_stateful_bi_consumer;
#[cfg(not(feature = "combinators"))]
pub(crate) use arc_conditional_stateful_bi_consumer::ArcConditionalStatefulBiConsumer;
#[cfg(feature = "combinators")]
pub use arc_conditional_stateful_bi_consumer::ArcConditionalStatefulBiConsumer;
#[cfg(feature = "rc")]
mod rc_conditional_stateful_bi_consumer;
#[cfg(feature = "rc")]
#[cfg(not(feature = "combinators"))]
pub(crate) use rc_conditional_stateful_bi_consumer::RcConditionalStatefulBiConsumer;
#[cfg(all(feature = "rc", feature = "combinators"))]
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
/// - `BoxStatefulBiConsumer<T, U>`, `ArcStatefulBiConsumer<T, U>`,
///   `RcStatefulBiConsumer<T, U>`
///
/// # Features
///
/// - **Unified Interface**: All bi-consumer types share the same `accept`
///   method signature
/// - **Automatic Implementation**: Closures automatically implement this trait
///   with zero overhead
/// - **Type Conversions**: Easy conversion between ownership models
/// - **Generic Programming**: Write functions accepting any bi-consumer type
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
}
