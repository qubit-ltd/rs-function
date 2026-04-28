/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # BiTransformer Types
//!
//! Provides Rust implementations of bi-transformer traits for type conversion
//! and value transformation with two inputs. BiTransformers consume two input
//! values (taking ownership) and produce an output value.
//!
//! This module provides the `BiTransformer<T, U, R>` trait and three
//! implementations:
//!
//! - [`BoxBiTransformer`]: Single ownership, not cloneable
//! - [`ArcBiTransformer`]: Thread-safe shared ownership, cloneable
//! - [`RcBiTransformer`]: Single-threaded shared ownership, cloneable
//!
//! # Author
//!
//! Haixing Hu
use std::rc::Rc;
use std::sync::Arc;

use crate::macros::{
    impl_arc_conversions,
    impl_box_conversions,
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
    transformer::Transformer,
};

mod box_bi_transformer;
pub use box_bi_transformer::BoxBiTransformer;
mod rc_bi_transformer;
pub use rc_bi_transformer::RcBiTransformer;
mod arc_bi_transformer;
pub use arc_bi_transformer::ArcBiTransformer;
mod fn_bi_transformer_ops;
pub use fn_bi_transformer_ops::FnBiTransformerOps;
mod binary_operator;
pub use binary_operator::BinaryOperator;
mod box_binary_operator;
pub use box_binary_operator::BoxBinaryOperator;
mod arc_binary_operator;
pub use arc_binary_operator::ArcBinaryOperator;
mod rc_binary_operator;
pub use rc_binary_operator::RcBinaryOperator;
mod box_conditional_bi_transformer;
pub use box_conditional_bi_transformer::BoxConditionalBiTransformer;
mod rc_conditional_bi_transformer;
pub use rc_conditional_bi_transformer::RcConditionalBiTransformer;
mod arc_conditional_bi_transformer;
pub use arc_conditional_bi_transformer::ArcConditionalBiTransformer;

// ============================================================================
// Core Trait
// ============================================================================

/// BiTransformer trait - transforms two values to produce a result
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
pub trait BiTransformer<T, U, R> {
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
    fn apply(&self, first: T, second: U) -> R;

    /// Converts to BoxBiTransformer
    ///
    /// **⚠️ Consumes `self`**: The original bi-transformer becomes unavailable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in a `Box` and creates a
    /// `BoxBiTransformer`. Types can override this method to provide more
    /// efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `BoxBiTransformer<T, U, R>`
    fn into_box(self) -> BoxBiTransformer<T, U, R>
    where
        Self: Sized + 'static,
    {
        BoxBiTransformer::new(move |x, y| self.apply(x, y))
    }

    /// Converts to RcBiTransformer
    ///
    /// **⚠️ Consumes `self`**: The original bi-transformer becomes unavailable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in an `Rc` and creates an
    /// `RcBiTransformer`. Types can override this method to provide more
    /// efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `RcBiTransformer<T, U, R>`
    fn into_rc(self) -> RcBiTransformer<T, U, R>
    where
        Self: Sized + 'static,
    {
        RcBiTransformer::new(move |x, y| self.apply(x, y))
    }

    /// Converts to ArcBiTransformer
    ///
    /// **⚠️ Consumes `self`**: The original bi-transformer becomes unavailable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in an `Arc` and creates an
    /// `ArcBiTransformer`. Types can override this method to provide more
    /// efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `ArcBiTransformer<T, U, R>`
    fn into_arc(self) -> ArcBiTransformer<T, U, R>
    where
        Self: Sized + Send + Sync + 'static,
    {
        ArcBiTransformer::new(move |x, y| self.apply(x, y))
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
    /// Returns a closure that implements `Fn(T, U) -> R`
    fn into_fn(self) -> impl Fn(T, U) -> R
    where
        Self: Sized + 'static,
    {
        move |t, u| self.apply(t, u)
    }

    /// Convert to BiTransformerOnce
    ///
    /// **⚠️ Consumes `self`**: The original bi-transformer will be unavailable after calling this method.
    ///
    /// Converts a reusable bi-transformer to a one-time bi-transformer that consumes itself on use.
    /// This enables passing `BiTransformer` to functions that require `BiTransformerOnce`.
    ///
    /// # Returns
    ///
    /// Returns a `BoxBiTransformerOnce<T, U, R>`
    fn into_once(self) -> BoxBiTransformerOnce<T, U, R>
    where
        Self: Sized + 'static,
    {
        BoxBiTransformerOnce::new(move |t, u| self.apply(t, u))
    }

    /// Non-consuming conversion to `BoxBiTransformer` using `&self`.
    ///
    /// Default implementation clones `self` and delegates to `into_box`.
    fn to_box(&self) -> BoxBiTransformer<T, U, R>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_box()
    }

    /// Non-consuming conversion to `RcBiTransformer` using `&self`.
    ///
    /// Default implementation clones `self` and delegates to `into_rc`.
    fn to_rc(&self) -> RcBiTransformer<T, U, R>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_rc()
    }

    /// Non-consuming conversion to `ArcBiTransformer` using `&self`.
    ///
    /// Default implementation clones `self` and delegates to `into_arc`.
    fn to_arc(&self) -> ArcBiTransformer<T, U, R>
    where
        Self: Sized + Clone + Send + Sync + 'static,
    {
        self.clone().into_arc()
    }

    /// Non-consuming conversion to a boxed function using `&self`.
    ///
    /// Returns a `Box<dyn Fn(T, U) -> R>` that clones `self` and calls
    /// `apply` inside the boxed closure.
    fn to_fn(&self) -> impl Fn(T, U) -> R
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_fn()
    }

    /// Convert to BiTransformerOnce without consuming self
    ///
    /// **⚠️ Requires Clone**: This method requires `Self` to implement `Clone`.
    /// Clones the current bi-transformer and converts the clone to a one-time bi-transformer.
    ///
    /// # Returns
    ///
    /// Returns a `BoxBiTransformerOnce<T, U, R>`
    fn to_once(&self) -> BoxBiTransformerOnce<T, U, R>
    where
        Self: Clone + 'static,
    {
        self.clone().into_once()
    }
}
