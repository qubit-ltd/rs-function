/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # StatefulTransformer Types
//!
//! Provides Rust implementations of stateful transformer traits for stateful value
//! transformation. StatefulTransformers consume input values (taking ownership) and
//! produce output values while allowing internal state modification. This is
//! analogous to `FnMut(T) -> R` in Rust's standard library.
//!
//! This module provides the `StatefulTransformer<T, R>` trait and three implementations:
//!
//! - [`BoxStatefulTransformer`]: Single ownership, not cloneable
//! - [`ArcStatefulTransformer`]: Thread-safe shared ownership, cloneable
//! - [`RcStatefulTransformer`]: Single-threaded shared ownership, cloneable
//!
//! # Author
//!
//! Haixing Hu
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use parking_lot::Mutex;

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
use crate::transformers::{
    macros::{
        impl_box_conditional_transformer,
        impl_box_transformer_methods,
        impl_conditional_transformer_clone,
        impl_conditional_transformer_debug_display,
        impl_shared_conditional_transformer,
        impl_shared_transformer_methods,
        impl_transformer_clone,
        impl_transformer_common_methods,
        impl_transformer_constant_method,
        impl_transformer_debug_display,
    },
    transformer_once::BoxTransformerOnce,
};

mod box_stateful_transformer;
pub use box_stateful_transformer::BoxStatefulTransformer;
mod rc_stateful_transformer;
pub use rc_stateful_transformer::RcStatefulTransformer;
mod arc_stateful_transformer;
pub use arc_stateful_transformer::ArcStatefulTransformer;
mod fn_stateful_transformer_ops;
pub use fn_stateful_transformer_ops::FnStatefulTransformerOps;
mod box_conditional_stateful_transformer;
pub use box_conditional_stateful_transformer::BoxConditionalStatefulTransformer;
mod rc_conditional_stateful_transformer;
pub use rc_conditional_stateful_transformer::RcConditionalStatefulTransformer;
mod arc_conditional_stateful_transformer;
pub use arc_conditional_stateful_transformer::ArcConditionalStatefulTransformer;

// ============================================================================
// Core Trait
// ============================================================================

/// StatefulTransformer trait - transforms values from type T to type R with state
///
/// Defines the behavior of a stateful transformation: converting a value
/// of type `T` to a value of type `R` by consuming the input while
/// allowing modification of internal state. This is analogous to
/// `FnMut(T) -> R` in Rust's standard library.
///
/// # Type Parameters
///
/// * `T` - The type of the input value (consumed)
/// * `R` - The type of the output value
///
/// # Author
///
/// Haixing Hu
pub trait StatefulTransformer<T, R> {
    /// Applies the transformation to the input value to produce an output value
    ///
    /// # Parameters
    ///
    /// * `input` - The input value to transform (consumed)
    ///
    /// # Returns
    ///
    /// The transformed output value
    fn apply(&mut self, input: T) -> R;

    /// Converts to BoxStatefulTransformer
    ///
    /// **⚠️ Consumes `self`**: The original transformer becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns `BoxStatefulTransformer<T, R>`
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in a `BoxStatefulTransformer` by
    /// creating a new closure that calls `self.apply()`. This is a lightweight
    /// adapter, but it is not strictly zero-cost.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{StatefulTransformer, BoxStatefulTransformer};
    ///
    /// struct CustomTransformer {
    ///     multiplier: i32,
    /// }
    ///
    /// impl StatefulTransformer<i32, i32> for CustomTransformer {
    ///     fn apply(&mut self, input: i32) -> i32 {
    ///         self.multiplier += 1;
    ///         input * self.multiplier
    ///     }
    /// }
    ///
    /// let transformer = CustomTransformer { multiplier: 0 };
    /// let mut boxed = transformer.into_box();
    /// assert_eq!(boxed.apply(10), 10);  // 10 * 1
    /// assert_eq!(boxed.apply(10), 20);  // 10 * 2
    /// ```
    fn into_box(self) -> BoxStatefulTransformer<T, R>
    where
        Self: Sized + 'static,
    {
        let mut transformer = self;
        BoxStatefulTransformer::new(move |t| transformer.apply(t))
    }

    /// Converts to RcStatefulTransformer
    ///
    /// **⚠️ Consumes `self`**: The original transformer becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns `RcStatefulTransformer<T, R>`
    ///
    /// # Default Implementation
    ///
    /// The default implementation first converts to `BoxStatefulTransformer` using
    /// `into_box()`, then wraps it in `RcStatefulTransformer`. Specific implementations
    /// may override this for better efficiency.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{StatefulTransformer, RcStatefulTransformer};
    ///
    /// struct CustomTransformer {
    ///     multiplier: i32,
    /// }
    ///
    /// impl StatefulTransformer<i32, i32> for CustomTransformer {
    ///     fn apply(&mut self, input: i32) -> i32 {
    ///         self.multiplier += 1;
    ///         input * self.multiplier
    ///     }
    /// }
    ///
    /// let transformer = CustomTransformer { multiplier: 0 };
    /// let mut rc_transformer = transformer.into_rc();
    /// assert_eq!(rc_transformer.apply(10), 10);  // 10 * 1
    /// assert_eq!(rc_transformer.apply(10), 20);  // 10 * 2
    /// ```
    fn into_rc(self) -> RcStatefulTransformer<T, R>
    where
        Self: Sized + 'static,
    {
        let mut transformer = self;
        RcStatefulTransformer::new(move |t| transformer.apply(t))
    }

    /// Converts to ArcStatefulTransformer
    ///
    /// **⚠️ Consumes `self`**: The original transformer becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns `ArcStatefulTransformer<T, R>`
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in an `ArcStatefulTransformer` by creating
    /// a new closure that calls `self.apply()`. Note that this requires `self`
    /// to implement `Send` due to Arc's thread-safety requirements.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{StatefulTransformer, ArcStatefulTransformer};
    ///
    /// struct CustomTransformer {
    ///     multiplier: i32,
    /// }
    ///
    /// impl StatefulTransformer<i32, i32> for CustomTransformer {
    ///     fn apply(&mut self, input: i32) -> i32 {
    ///         self.multiplier += 1;
    ///         input * self.multiplier
    ///     }
    /// }
    ///
    /// let transformer = CustomTransformer { multiplier: 0 };
    /// let mut arc_transformer = transformer.into_arc();
    /// assert_eq!(arc_transformer.apply(10), 10);  // 10 * 1
    /// assert_eq!(arc_transformer.apply(10), 20);  // 10 * 2
    /// ```
    fn into_arc(self) -> ArcStatefulTransformer<T, R>
    where
        Self: Sized + Send + 'static,
    {
        let mut transformer = self;
        ArcStatefulTransformer::new(move |t| transformer.apply(t))
    }

    /// Converts to a closure implementing `FnMut(T) -> R`
    ///
    /// **⚠️ Consumes `self`**: The original transformer becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns an implementation of `FnMut(T) -> R`
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a new closure that calls `self.apply()`.
    /// Specific implementations may override this for better efficiency.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{StatefulTransformer, BoxStatefulTransformer};
    ///
    /// let transformer = BoxStatefulTransformer::new(|x: i32| x * 2);
    /// let mut closure = transformer.into_fn();
    /// assert_eq!(closure(10), 20);
    /// assert_eq!(closure(15), 30);
    /// ```
    fn into_fn(self) -> impl FnMut(T) -> R
    where
        Self: Sized + 'static,
    {
        let mut transformer = self;
        move |t| transformer.apply(t)
    }

    /// Converts transformer to a mutable closure (`FnMut`) with an explicit
    /// method name.
    ///
    /// This is a naming alias of [`StatefulTransformer::into_fn`] to make the
    /// mutability of the returned closure explicit.
    fn into_mut_fn(self) -> impl FnMut(T) -> R
    where
        Self: Sized + 'static,
    {
        self.into_fn()
    }

    /// Converts to `BoxTransformerOnce`.
    ///
    /// This method has a default implementation that wraps the
    /// transformer in a `BoxTransformerOnce`. Custom implementations
    /// can override this method for optimization purposes.
    ///
    /// # Returns
    ///
    /// A new `BoxTransformerOnce<T, R>` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{StatefulTransformer, TransformerOnce};
    ///
    /// let closure = |x: i32| x * 2;
    /// let once = closure.into_once();
    /// assert_eq!(once.apply(5), 10);
    /// ```
    fn into_once(self) -> BoxTransformerOnce<T, R>
    where
        Self: Sized + 'static,
    {
        let mut transformer = self;
        BoxTransformerOnce::new(move |t| transformer.apply(t))
    }

    /// Non-consuming conversion to `BoxStatefulTransformer`.
    ///
    /// Default implementation requires `Self: Clone` and wraps a cloned
    /// instance in a `RefCell` so the returned transformer can mutate state
    /// across calls.
    fn to_box(&self) -> BoxStatefulTransformer<T, R>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_box()
    }

    /// Non-consuming conversion to `RcStatefulTransformer`.
    ///
    /// Default implementation clones `self` into an `Rc<RefCell<_>>` so the
    /// resulting transformer can be shared within a single thread.
    fn to_rc(&self) -> RcStatefulTransformer<T, R>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_rc()
    }

    /// Non-consuming conversion to `ArcStatefulTransformer` (thread-safe).
    ///
    /// Default implementation requires `Self: Clone + Send + Sync` and wraps
    /// the cloned instance in `Arc<Mutex<_>>` so it can be used across
    /// threads.
    fn to_arc(&self) -> ArcStatefulTransformer<T, R>
    where
        Self: Sized + Clone + Send + 'static,
    {
        self.clone().into_arc()
    }

    /// Non-consuming conversion to a closure (`FnMut(T) -> R`).
    ///
    /// Default implementation clones `self` into a `RefCell` and returns a
    /// closure that calls `apply` on the interior mutable value.
    fn to_fn(&self) -> impl FnMut(T) -> R
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_fn()
    }

    /// Non-consuming conversion to a mutable closure (`FnMut`) with an explicit
    /// method name.
    ///
    /// This is a naming alias of [`StatefulTransformer::to_fn`] and preserves
    /// the same clone-based behavior.
    fn to_mut_fn(&self) -> impl FnMut(T) -> R
    where
        Self: Sized + Clone + 'static,
    {
        self.to_fn()
    }

    /// Creates a `BoxTransformerOnce` from a cloned transformer
    ///
    /// Uses `Clone` to obtain an owned copy and converts it into a
    /// `BoxTransformerOnce`. Requires `Self: Clone`. Custom implementations
    /// can override this for better performance.
    fn to_once(&self) -> BoxTransformerOnce<T, R>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_once()
    }
}
