// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # BiConsumerOnce Types
//!
//! Provides one-time bi-consumer interface implementations for operations
//! accepting two input parameters without returning a result.
//!
//! It is similar to the `FnOnce(&T, &U)` trait in the standard library.
//!
//! This module provides a unified `BiConsumerOnce` trait and one concrete
//! implementation:
//!
//! - **`BoxBiConsumerOnce<T, U>`**: Box-based single ownership implementation
//!
//! # Why No Arc/Rc Variants?
//!
//! Unlike reusable [`BiConsumer`](crate::consumers::BiConsumer)
//! implementations, this module does **not** provide `ArcBiConsumerOnce` or
//! `RcBiConsumerOnce` implementations. This is a design decision based on the
//! fact that `FnOnce` semantics require single ownership at the call site,
//! while `Arc` and `Rc` are meant to preserve shared ownership across clones.
//!
//! # Design Philosophy
//!
//! BiConsumerOnce uses `FnOnce(&T, &U)` semantics: for truly one-time
//! consumption operations.
//!
//! Unlike BiConsumer, BiConsumerOnce consumes itself on first call. Suitable
//! for initialization callbacks, cleanup callbacks, etc.
#[cfg(feature = "combinators")]
use crate::{
    consumers::macros::{
        impl_box_conditional_consumer,
        impl_conditional_consumer_debug_display,
    },
    predicates::bi_predicate::{
        BiPredicate,
        BoxBiPredicate,
    },
};
use crate::{
    consumers::macros::{
        impl_box_consumer_methods,
        impl_consumer_common_methods,
        impl_consumer_debug_display,
    },
    macros::impl_closure_once_trait,
};

// ==========================================================================
// Type Aliases
// ==========================================================================

/// Type alias for bi-consumer once function signature.
type BiConsumerOnceFn<T, U> = dyn FnOnce(&T, &U);

mod box_bi_consumer_once;
pub use box_bi_consumer_once::BoxBiConsumerOnce;
#[cfg(feature = "combinators")]
mod fn_bi_consumer_once_ops;
#[cfg(feature = "combinators")]
pub use fn_bi_consumer_once_ops::FnBiConsumerOnceOps;
#[cfg(feature = "combinators")]
mod box_conditional_bi_consumer_once;
#[cfg(feature = "combinators")]
pub use box_conditional_bi_consumer_once::BoxConditionalBiConsumerOnce;

// =======================================================================
// 1. BiConsumerOnce Trait - Unified Interface
// =======================================================================

/// BiConsumerOnce trait - Unified one-time bi-consumer interface
///
/// It is similar to the `FnOnce(&T, &U)` trait in the standard library.
///
/// Defines core behavior for all one-time bi-consumer types. Similar to a
/// bi-consumer implementing `FnOnce(&T, &U)`, performs operations
/// accepting two value references but returning no result (side effects
/// only), consuming itself in the process.
///
/// # Automatic Implementations
///
/// - All closures implementing `FnOnce(&T, &U)`
/// - `BoxBiConsumerOnce<T, U>`
///
/// # Features
///
/// - **Unified Interface**: All bi-consumer types share the same `accept`
///   method signature
/// - **Automatic Implementation**: Closures implement this trait directly,
///   without allocating an adapter
/// - **Generic Programming**: Write functions accepting any one-time
///   bi-consumer type
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BiConsumerOnce, BoxBiConsumerOnce};
/// use std::sync::{Arc, Mutex};
///
/// fn apply_consumer<C: BiConsumerOnce<i32, i32>>(
///     consumer: C,
///     a: &i32,
///     b: &i32
/// ) {
///     consumer.accept(a, b);
/// }
///
/// let log = Arc::new(Mutex::new(Vec::new()));
/// let l = log.clone();
/// let box_con = BoxBiConsumerOnce::new(move |x: &i32, y: &i32| {
///     l.lock().expect("mutex should not be poisoned").push(*x + *y);
/// });
/// apply_consumer(box_con, &5, &3);
/// assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8]);
/// ```
pub trait BiConsumerOnce<T, U> {
    /// Performs the one-time consumption operation
    ///
    /// Executes an operation on the given two references. The operation
    /// typically reads input values or produces side effects, but does not
    /// modify the input values themselves. Consumes self.
    ///
    /// # Parameters
    ///
    /// * `first` - Reference to the first value to consume
    /// * `second` - Reference to the second value to consume
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BiConsumerOnce, BoxBiConsumerOnce};
    ///
    /// let consumer = BoxBiConsumerOnce::new(|x: &i32, y: &i32| {
    ///     println!("Sum: {}", x + y);
    /// });
    /// consumer.accept(&5, &3);
    /// ```
    fn accept(self, first: &T, second: &U);
}
