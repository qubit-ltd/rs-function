/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # StatefulBiTransformer Types
//!
//! Provides Rust implementations of stateful bi-transformer traits for type
//! conversion and value transformation with two inputs. StatefulBiTransformers
//! consume two input values (taking ownership) and produce an output value.
//!
//! This module provides the `StatefulBiTransformer<T, U, R>` trait and three
//! implementations:
//!
//! - [`BoxStatefulBiTransformer`]: Single ownership, not cloneable
//! - [`ArcStatefulBiTransformer`]: Thread-safe shared ownership, cloneable
//! - [`RcStatefulBiTransformer`]: Single-threaded shared ownership, cloneable
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
    impl_closure_trait,
    impl_rc_conversions,
};
use crate::predicates::bi_predicate::{
    ArcBiPredicate,
    BiPredicate,
    BoxBiPredicate,
    RcBiPredicate,
};
use crate::transformers::{
    bi_transformer_once::BoxBiTransformerOnce,
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
    stateful_transformer::StatefulTransformer,
};

mod box_stateful_bi_transformer;
pub use box_stateful_bi_transformer::BoxStatefulBiTransformer;
mod rc_stateful_bi_transformer;
pub use rc_stateful_bi_transformer::RcStatefulBiTransformer;
mod arc_stateful_bi_transformer;
pub use arc_stateful_bi_transformer::ArcStatefulBiTransformer;
mod fn_stateful_bi_transformer_ops;
pub use fn_stateful_bi_transformer_ops::FnStatefulBiTransformerOps;
mod stateful_binary_operator;
pub use stateful_binary_operator::StatefulBinaryOperator;
mod box_stateful_binary_operator;
pub use box_stateful_binary_operator::BoxStatefulBinaryOperator;
mod arc_stateful_binary_operator;
pub use arc_stateful_binary_operator::ArcStatefulBinaryOperator;
mod rc_stateful_binary_operator;
pub use rc_stateful_binary_operator::RcStatefulBinaryOperator;
mod box_conditional_stateful_bi_transformer;
pub use box_conditional_stateful_bi_transformer::BoxConditionalStatefulBiTransformer;
mod rc_conditional_stateful_bi_transformer;
pub use rc_conditional_stateful_bi_transformer::RcConditionalStatefulBiTransformer;
mod arc_conditional_stateful_bi_transformer;
pub use arc_conditional_stateful_bi_transformer::ArcConditionalStatefulBiTransformer;

// ============================================================================
// Core Trait
// ============================================================================

/// StatefulBiTransformer trait - transforms two values to produce a result
///
/// Defines the behavior of a bi-transformation: converting two values of types
/// `T` and `U` to a value of type `R` by consuming the inputs. This is
/// analogous to `Fn(T, U) -> R` in Rust's standard library.
///
/// # Type Parameters
///
/// * `T` - The type of the first input value (consumed)
/// * `U` - The type of the second input value (consumed)
/// * `R` - The type of the output value
///
/// # Author
///
/// Haixing Hu
pub trait StatefulBiTransformer<T, U, R> {
    /// Transforms two input values to produce an output value
    ///
    /// # Parameters
    ///
    /// * `first` - The first input value to transform (consumed)
    /// * `second` - The second input value to transform (consumed)
    ///
    /// # Returns
    ///
    /// The transformed output value
    fn apply(&mut self, first: T, second: U) -> R;

    /// Converts to BoxStatefulBiTransformer
    ///
    /// **⚠️ Consumes `self`**: The original bi-transformer becomes unavailable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in a `Box` and creates a
    /// `BoxStatefulBiTransformer`. Types can override this method to provide more
    /// efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `BoxStatefulBiTransformer<T, U, R>`
    fn into_box(self) -> BoxStatefulBiTransformer<T, U, R>
    where
        Self: Sized + 'static,
    {
        let mut trans = self;
        BoxStatefulBiTransformer::new(move |x, y| trans.apply(x, y))
    }

    /// Converts to RcStatefulBiTransformer
    ///
    /// **⚠️ Consumes `self`**: The original bi-transformer becomes unavailable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in an `Rc` and creates an
    /// `RcStatefulBiTransformer`. Types can override this method to provide more
    /// efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `RcStatefulBiTransformer<T, U, R>`
    fn into_rc(self) -> RcStatefulBiTransformer<T, U, R>
    where
        Self: Sized + 'static,
    {
        let mut trans = self;
        RcStatefulBiTransformer::new(move |x, y| trans.apply(x, y))
    }

    /// Converts to ArcStatefulBiTransformer
    ///
    /// **⚠️ Consumes `self`**: The original bi-transformer becomes unavailable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in an `Arc` and creates an
    /// `ArcStatefulBiTransformer`. Types can override this method to provide more
    /// efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `ArcStatefulBiTransformer<T, U, R>`
    fn into_arc(self) -> ArcStatefulBiTransformer<T, U, R>
    where
        Self: Sized + Send + 'static,
    {
        let mut trans = self;
        ArcStatefulBiTransformer::new(move |x, y| trans.apply(x, y))
    }

    /// Converts bi-transformer to a closure
    ///
    /// **⚠️ Consumes `self`**: The original bi-transformer becomes unavailable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a closure that captures `self`
    /// and calls its `apply` method. Types can override this method
    /// to provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns a closure that implements `FnMut(T, U) -> R`
    fn into_fn(self) -> impl FnMut(T, U) -> R
    where
        Self: Sized + 'static,
    {
        let mut trans = self;
        move |t, u| trans.apply(t, u)
    }

    /// Converts bi-transformer to a mutable closure (`FnMut`) with an explicit
    /// method name.
    ///
    /// This is a naming alias of [`StatefulBiTransformer::into_fn`] to avoid
    /// confusion with non-stateful `into_fn` methods that typically return `Fn`.
    fn into_mut_fn(self) -> impl FnMut(T, U) -> R
    where
        Self: Sized + 'static,
    {
        self.into_fn()
    }

    /// Converts to BoxBiTransformerOnce
    ///
    /// **⚠️ Consumes `self`**: The original bi-transformer becomes unavailable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in a `Box` and creates a
    /// `BoxBiTransformerOnce`. Types can override this method to provide more
    /// efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `BoxBiTransformerOnce<T, U, R>`
    fn into_once(self) -> BoxBiTransformerOnce<T, U, R>
    where
        Self: Sized + 'static,
    {
        let mut trans = self;
        BoxBiTransformerOnce::new(move |t, u| trans.apply(t, u))
    }

    /// Non-consuming conversion to `BoxStatefulBiTransformer` using `&self`.
    ///
    /// Default implementation clones `self` and delegates to `into_box`.
    fn to_box(&self) -> BoxStatefulBiTransformer<T, U, R>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_box()
    }

    /// Non-consuming conversion to `RcStatefulBiTransformer` using `&self`.
    ///
    /// Default implementation clones `self` and delegates to `into_rc`.
    fn to_rc(&self) -> RcStatefulBiTransformer<T, U, R>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_rc()
    }

    /// Non-consuming conversion to `ArcStatefulBiTransformer` using `&self`.
    ///
    /// Default implementation clones `self` and delegates to `into_arc`.
    fn to_arc(&self) -> ArcStatefulBiTransformer<T, U, R>
    where
        Self: Sized + Clone + Send + 'static,
    {
        self.clone().into_arc()
    }

    /// Non-consuming conversion to a boxed function using `&self`.
    ///
    /// Returns a `Box<dyn FnMut(T, U) -> R>` that clones `self` and calls
    /// `apply` inside the boxed closure.
    fn to_fn(&self) -> impl FnMut(T, U) -> R
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_fn()
    }

    /// Non-consuming conversion to a mutable closure (`FnMut`) with an explicit
    /// method name.
    ///
    /// This is a naming alias of [`StatefulBiTransformer::to_fn`] and keeps the
    /// same clone-based behavior.
    fn to_mut_fn(&self) -> impl FnMut(T, U) -> R
    where
        Self: Sized + Clone + 'static,
    {
        self.to_fn()
    }

    /// Non-consuming conversion to `BoxBiTransformerOnce` using `&self`.
    ///
    /// Default implementation clones `self` and delegates to `into_once`.
    fn to_once(&self) -> BoxBiTransformerOnce<T, U, R>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_once()
    }
}
