/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # BiMutatingFunction Types
//!
//! Provides Rust implementations of bi-mutating-function traits for performing
//! operations that accept two mutable references and return a result.
//!
//! It is similar to the `Fn(&mut T, &mut U) -> R` trait in the standard library.
//!
//! This module provides the `BiMutatingFunction<T, U, R>` trait and three
//! implementations:
//!
//! - [`BoxBiMutatingFunction`]: Single ownership, not cloneable
//! - [`ArcBiMutatingFunction`]: Thread-safe shared ownership, cloneable
//! - [`RcBiMutatingFunction`]: Single-threaded shared ownership, cloneable
//!
//! # Author
//!
//! Haixing Hu

use std::rc::Rc;
use std::sync::Arc;

use crate::functions::{
    bi_mutating_function_once::BoxBiMutatingFunctionOnce,
    macros::{
        impl_box_conditional_function,
        impl_box_function_methods,
        impl_conditional_function_clone,
        impl_conditional_function_debug_display,
        impl_function_clone,
        impl_function_common_methods,
        impl_function_constant_method,
        impl_function_debug_display,
        impl_shared_conditional_function,
        impl_shared_function_methods,
    },
    mutating_function::MutatingFunction,
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

mod box_bi_mutating_function;
pub use box_bi_mutating_function::BoxBiMutatingFunction;
mod rc_bi_mutating_function;
pub use rc_bi_mutating_function::RcBiMutatingFunction;
mod arc_bi_mutating_function;
pub use arc_bi_mutating_function::ArcBiMutatingFunction;
mod fn_bi_mutating_function_ops;
pub use fn_bi_mutating_function_ops::FnBiMutatingFunctionOps;
mod box_binary_mutating_function;
pub use box_binary_mutating_function::BoxBinaryMutatingFunction;
mod arc_binary_mutating_function;
pub use arc_binary_mutating_function::ArcBinaryMutatingFunction;
mod rc_binary_mutating_function;
pub use rc_binary_mutating_function::RcBinaryMutatingFunction;
mod box_conditional_bi_mutating_function;
pub use box_conditional_bi_mutating_function::BoxConditionalBiMutatingFunction;
mod rc_conditional_bi_mutating_function;
pub use rc_conditional_bi_mutating_function::RcConditionalBiMutatingFunction;
mod arc_conditional_bi_mutating_function;
pub use arc_conditional_bi_mutating_function::ArcConditionalBiMutatingFunction;

// ============================================================================
// Core Trait
// ============================================================================

/// BiMutatingFunction trait - performs operations on two mutable references
///
/// Defines the behavior of a bi-mutating-function: computing a value of type `R`
/// from mutable references to types `T` and `U`, potentially modifying both inputs.
/// This is analogous to `Fn(&mut T, &mut U) -> R` in Rust's standard library.
///
/// # Type Parameters
///
/// * `T` - The type of the first input value (mutable reference)
/// * `U` - The type of the second input value (mutable reference)
/// * `R` - The type of the output value
///
/// # Author
///
/// Haixing Hu
pub trait BiMutatingFunction<T, U, R> {
    /// Applies the bi-mutating-function to two mutable references and returns a result
    ///
    /// # Parameters
    ///
    /// * `first` - Mutable reference to the first input value
    /// * `second` - Mutable reference to the second input value
    ///
    /// # Returns
    ///
    /// The computed output value
    fn apply(&self, first: &mut T, second: &mut U) -> R;

    /// Converts to BoxBiMutatingFunction
    ///
    /// **⚠️ Consumes `self`**: The original bi-function becomes
    /// unavailable after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in a `Box` and creates a
    /// `BoxBiMutatingFunction`. Types can override this method to provide more
    /// efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `BoxBiMutatingFunction<T, U, R>`
    fn into_box(self) -> BoxBiMutatingFunction<T, U, R>
    where
        Self: Sized + 'static,
    {
        BoxBiMutatingFunction::new(move |t, u| self.apply(t, u))
    }

    /// Converts to RcBiMutatingFunction
    ///
    /// **⚠️ Consumes `self`**: The original bi-function becomes
    /// unavailable after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in an `Rc` and creates an
    /// `RcBiMutatingFunction`. Types can override this method to provide more
    /// efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `RcBiMutatingFunction<T, U, R>`
    fn into_rc(self) -> RcBiMutatingFunction<T, U, R>
    where
        Self: Sized + 'static,
    {
        RcBiMutatingFunction::new(move |t, u| self.apply(t, u))
    }

    /// Converts to ArcBiMutatingFunction
    ///
    /// **⚠️ Consumes `self`**: The original bi-function becomes
    /// unavailable after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in an `Arc` and creates
    /// an `ArcBiMutatingFunction`. Types can override this method to provide
    /// more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `ArcBiMutatingFunction<T, U, R>`
    fn into_arc(self) -> ArcBiMutatingFunction<T, U, R>
    where
        Self: Sized + Send + Sync + 'static,
    {
        ArcBiMutatingFunction::new(move |t, u| self.apply(t, u))
    }

    /// Converts bi-mutating-function to a closure
    ///
    /// **⚠️ Consumes `self`**: The original bi-function becomes
    /// unavailable after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a closure that captures `self`
    /// and calls its `apply` method. Types can override this method
    /// to provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns a closure that implements `Fn(&mut T, &mut U) -> R`
    fn into_fn(self) -> impl Fn(&mut T, &mut U) -> R
    where
        Self: Sized + 'static,
    {
        move |t, u| self.apply(t, u)
    }

    /// Converts to BiMutatingFunctionOnce
    ///
    /// **⚠️ Consumes `self`**: The original bi-function becomes unavailable after calling this method.
    ///
    /// Converts a reusable bi-mutating-function to a one-time bi-mutating-function that consumes itself on use.
    /// This enables passing `BiMutatingFunction` to functions that require `BiMutatingFunctionOnce`.
    ///
    /// # Returns
    ///
    /// Returns a `BoxBiMutatingFunctionOnce<T, U, R>`
    fn into_once(self) -> BoxBiMutatingFunctionOnce<T, U, R>
    where
        Self: Sized + 'static,
    {
        BoxBiMutatingFunctionOnce::new(move |t, u| self.apply(t, u))
    }

    /// Non-consuming conversion to `BoxBiMutatingFunction` using `&self`.
    ///
    /// Default implementation clones `self` and delegates to `into_box`.
    fn to_box(&self) -> BoxBiMutatingFunction<T, U, R>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_box()
    }

    /// Non-consuming conversion to `RcBiMutatingFunction` using `&self`.
    ///
    /// Default implementation clones `self` and delegates to `into_rc`.
    fn to_rc(&self) -> RcBiMutatingFunction<T, U, R>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_rc()
    }

    /// Non-consuming conversion to `ArcBiMutatingFunction` using `&self`.
    ///
    /// Default implementation clones `self` and delegates to `into_arc`.
    fn to_arc(&self) -> ArcBiMutatingFunction<T, U, R>
    where
        Self: Sized + Clone + Send + Sync + 'static,
    {
        self.clone().into_arc()
    }

    /// Non-consuming conversion to a boxed function using `&self`.
    ///
    /// Returns a `Box<dyn Fn(&mut T, &mut U) -> R>` that clones `self` and calls
    /// `apply` inside the boxed closure.
    fn to_fn(&self) -> impl Fn(&mut T, &mut U) -> R
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_fn()
    }

    /// Convert to BiMutatingFunctionOnce without consuming self
    ///
    /// **⚠️ Requires Clone**: This method requires `Self` to implement `Clone`.
    /// Clones the current bi-function and converts the clone to a one-time bi-function.
    ///
    /// # Returns
    ///
    /// Returns a `BoxBiMutatingFunctionOnce<T, U, R>`
    fn to_once(&self) -> BoxBiMutatingFunctionOnce<T, U, R>
    where
        Self: Clone + 'static,
    {
        self.clone().into_once()
    }
}
