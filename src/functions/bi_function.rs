/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # BiFunction Types
//!
//! Provides Rust implementations of bi-function traits for computing output values
//! from two input references. BiFunctions borrow input values (not consuming them)
//! and produce output values.
//!
//! It is similar to the `Fn(&T, &U) -> R` trait in the standard library.
//!
//! This module provides the `BiFunction<T, U, R>` trait and three
//! implementations:
//!
//! - [`BoxBiFunction`]: Single ownership, not cloneable
//! - [`ArcBiFunction`]: Thread-safe shared ownership, cloneable
//! - [`RcBiFunction`]: Single-threaded shared ownership, cloneable
//!
//! # Author
//!
//! Haixing Hu
use std::rc::Rc;
use std::sync::Arc;

use crate::functions::{
    bi_function_once::BoxBiFunctionOnce,
    function::Function,
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

mod box_bi_function;
pub use box_bi_function::BoxBiFunction;
mod rc_bi_function;
pub use rc_bi_function::RcBiFunction;
mod arc_bi_function;
pub use arc_bi_function::ArcBiFunction;
mod fn_bi_function_ops;
pub use fn_bi_function_ops::FnBiFunctionOps;
mod box_binary_function;
pub use box_binary_function::BoxBinaryFunction;
mod arc_binary_function;
pub use arc_binary_function::ArcBinaryFunction;
mod rc_binary_function;
pub use rc_binary_function::RcBinaryFunction;
mod box_conditional_bi_function;
pub use box_conditional_bi_function::BoxConditionalBiFunction;
mod rc_conditional_bi_function;
pub use rc_conditional_bi_function::RcConditionalBiFunction;
mod arc_conditional_bi_function;
pub use arc_conditional_bi_function::ArcConditionalBiFunction;

// ============================================================================
// Core Trait
// ============================================================================

/// BiFunction trait - computes output from two input references
///
/// Defines the behavior of a bi-function: computing a value of type `R`
/// from references to types `T` and `U` without consuming the inputs. This is analogous to
/// `Fn(&T, &U) -> R` in Rust's standard library, similar to Java's `BiFunction<T, U, R>`.
///
/// # Type Parameters
///
/// * `T` - The type of the first input value (borrowed)
/// * `U` - The type of the second input value (borrowed)
/// * `R` - The type of the output value
///
/// # Author
///
/// Haixing Hu
pub trait BiFunction<T, U, R> {
    /// Applies the bi-function to two input references to produce an output value
    ///
    /// # Parameters
    ///
    /// * `first` - Reference to the first input value
    /// * `second` - Reference to the second input value
    ///
    /// # Returns
    ///
    /// The computed output value
    fn apply(&self, first: &T, second: &U) -> R;

    /// Converts to BoxBiFunction
    ///
    /// **⚠️ Consumes `self`**: The original bi-function becomes
    /// unavailable after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in a `Box` and creates a
    /// `BoxBiFunction`. Types can override this method to provide more
    /// efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `BoxBiFunction<T, U, R>`
    fn into_box(self) -> BoxBiFunction<T, U, R>
    where
        Self: Sized + 'static,
    {
        BoxBiFunction::new(move |t, u| self.apply(t, u))
    }

    /// Converts to RcBiFunction
    ///
    /// **⚠️ Consumes `self`**: The original bi-function becomes
    /// unavailable after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in an `Rc` and creates an
    /// `RcBiFunction`. Types can override this method to provide more
    /// efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `RcBiFunction<T, U, R>`
    fn into_rc(self) -> RcBiFunction<T, U, R>
    where
        Self: Sized + 'static,
    {
        RcBiFunction::new(move |t, u| self.apply(t, u))
    }

    /// Converts to ArcBiFunction
    ///
    /// **⚠️ Consumes `self`**: The original bi-function becomes
    /// unavailable after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in an `Arc` and creates
    /// an `ArcBiFunction`. Types can override this method to provide
    /// more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `ArcBiFunction<T, U, R>`
    fn into_arc(self) -> ArcBiFunction<T, U, R>
    where
        Self: Sized + Send + Sync + 'static,
    {
        ArcBiFunction::new(move |t, u| self.apply(t, u))
    }

    /// Converts bi-function to a closure
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
    /// Returns a closure that implements `Fn(&T, &U) -> R`
    fn into_fn(self) -> impl Fn(&T, &U) -> R
    where
        Self: Sized + 'static,
    {
        move |t, u| self.apply(t, u)
    }

    /// Converts to BiFunctionOnce
    ///
    /// **⚠️ Consumes `self`**: The original bi-function becomes unavailable after calling this method.
    ///
    /// Converts a reusable bi-function to a one-time bi-function that consumes itself on use.
    /// This enables passing `BiFunction` to functions that require `BiFunctionOnce`.
    ///
    /// # Returns
    ///
    /// Returns a `BoxBiFunctionOnce<T, U, R>`
    fn into_once(self) -> BoxBiFunctionOnce<T, U, R>
    where
        Self: Sized + 'static,
    {
        BoxBiFunctionOnce::new(move |t, u| self.apply(t, u))
    }

    /// Non-consuming conversion to `BoxBiFunction` using `&self`.
    ///
    /// Default implementation clones `self` and delegates to `into_box`.
    fn to_box(&self) -> BoxBiFunction<T, U, R>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_box()
    }

    /// Non-consuming conversion to `RcBiFunction` using `&self`.
    ///
    /// Default implementation clones `self` and delegates to `into_rc`.
    fn to_rc(&self) -> RcBiFunction<T, U, R>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_rc()
    }

    /// Non-consuming conversion to `ArcBiFunction` using `&self`.
    ///
    /// Default implementation clones `self` and delegates to `into_arc`.
    fn to_arc(&self) -> ArcBiFunction<T, U, R>
    where
        Self: Sized + Clone + Send + Sync + 'static,
    {
        self.clone().into_arc()
    }

    /// Non-consuming conversion to a boxed function using `&self`.
    ///
    /// Returns a `Box<dyn Fn(&T, &U) -> R>` that clones `self` and calls
    /// `apply` inside the boxed closure.
    fn to_fn(&self) -> impl Fn(&T, &U) -> R
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_fn()
    }

    /// Convert to BiFunctionOnce without consuming self
    ///
    /// **⚠️ Requires Clone**: This method requires `Self` to implement `Clone`.
    /// Clones the current bi-function and converts the clone to a one-time bi-function.
    ///
    /// # Returns
    ///
    /// Returns a `BoxBiFunctionOnce<T, U, R>`
    fn to_once(&self) -> BoxBiFunctionOnce<T, U, R>
    where
        Self: Clone + 'static,
    {
        self.clone().into_once()
    }
}
