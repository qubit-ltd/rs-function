/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # MutatingFunctionOnce Types
//!
//! Provides Java-like one-time `MutatingFunction` interface implementations
//! for performing operations that consume self, accept a mutable reference,
//! and return a result.
//!
//! It is similar to the `FnOnce(&mut T) -> R` trait in the standard library.
//!
//! This module provides a unified `MutatingFunctionOnce` trait and a
//! Box-based single ownership implementation:
//!
//! - **`BoxMutatingFunctionOnce<T, R>`**: Box-based single ownership
//!   implementation for one-time use scenarios
//!
//! # Design Philosophy
//!
//! The key difference between `MutatingFunctionOnce` and
//! `MutatingFunction`:
//!
//! - **MutatingFunction**: `&self`, can be called multiple times, uses
//!   `Fn(&mut T) -> R`
//! - **MutatingFunctionOnce**: `self`, can only be called once, uses
//!   `FnOnce(&mut T) -> R`
//!
//! ## MutatingFunctionOnce vs MutatingFunction
//!
//! | Feature | MutatingFunction | MutatingFunctionOnce |
//! |---------|------------------|----------------------|
//! | **Self Parameter** | `&self` | `self` |
//! | **Call Count** | Multiple | Once |
//! | **Closure Type** | `Fn(&mut T) -> R` | `FnOnce(&mut T) -> R` |
//! | **Use Cases** | Repeatable operations | One-time resource
//! transfers |
//!
//! # Why MutatingFunctionOnce?
//!
//! Core value of MutatingFunctionOnce:
//!
//! 1. **Store FnOnce closures**: Allows moving captured variables
//! 2. **Delayed execution**: Store in data structures, execute later
//! 3. **Resource transfer**: Suitable for scenarios requiring ownership
//!    transfer
//! 4. **Return results**: Unlike MutatorOnce, returns information about the
//!    operation
//!
//! # Why Only Box Variant?
//!
//! - **Arc/Rc conflicts with FnOnce semantics**: FnOnce can only be called
//!   once, while shared ownership implies multiple references
//! - **Box is perfect match**: Single ownership aligns perfectly with
//!   one-time call semantics
//!
//! # Use Cases
//!
//! ## BoxMutatingFunctionOnce
//!
//! - Post-initialization callbacks (moving data, returning status)
//! - Resource transfer with result (moving Vec, returning old value)
//! - One-time complex operations (requiring moved capture variables)
//! - Validation with fixes (fix data once, return validation result)
//!
//! # Examples
//!
//! ## Basic Usage
//!
//! ```rust
//! use qubit_function::{BoxMutatingFunctionOnce, MutatingFunctionOnce};
//!
//! let data = vec![1, 2, 3];
//! let func = BoxMutatingFunctionOnce::new(move |x: &mut Vec<i32>| {
//!     let old_len = x.len();
//!     x.extend(data); // Move data
//!     old_len
//! });
//!
//! let mut target = vec![0];
//! let old_len = func.apply(&mut target);
//! assert_eq!(old_len, 1);
//! assert_eq!(target, vec![0, 1, 2, 3]);
//! ```
//!
//! ## Method Chaining
//!
//! ```rust
//! use qubit_function::{BoxMutatingFunctionOnce, MutatingFunctionOnce};
//!
//! let data1 = vec![1, 2];
//!
//! let chained = BoxMutatingFunctionOnce::new(move |x: &mut Vec<i32>| {
//!     x.extend(data1);
//!     x.len()
//! })
//! .and_then(|len: &usize| len + 2);
//!
//! let mut target = vec![0];
//! let final_len = chained.apply(&mut target);
//! assert_eq!(final_len, 5);
//! assert_eq!(target, vec![0, 1, 2]);
//! ```
//!
//! ## Validation Pattern
//!
//! ```rust
//! use qubit_function::{BoxMutatingFunctionOnce, MutatingFunctionOnce};
//!
//! struct Data {
//!     value: i32,
//! }
//!
//! let validator = BoxMutatingFunctionOnce::new(|data: &mut Data| {
//!     if data.value < 0 {
//!         data.value = 0;
//!         Err("Fixed negative value")
//!     } else {
//!         Ok("Valid")
//!     }
//! });
//!
//! let mut data = Data { value: -5 };
//! let result = validator.apply(&mut data);
//! assert_eq!(data.value, 0);
//! assert!(result.is_err());
//! ```
//!
//! # Author
//!
//! Haixing Hu
use crate::functions::{
    function_once::FunctionOnce,
    macros::{
        impl_box_conditional_function,
        impl_box_function_methods,
        impl_conditional_function_debug_display,
        impl_fn_ops_trait,
        impl_function_common_methods,
        impl_function_debug_display,
        impl_function_identity_method,
    },
};
use crate::macros::{
    impl_box_once_conversions,
    impl_closure_once_trait,
};
use crate::predicates::predicate::{
    BoxPredicate,
    Predicate,
};

mod box_mutating_function_once;
pub use box_mutating_function_once::BoxMutatingFunctionOnce;
mod box_conditional_mutating_function_once;
pub use box_conditional_mutating_function_once::BoxConditionalMutatingFunctionOnce;
mod fn_mutating_function_once_ops;
pub use fn_mutating_function_once_ops::FnMutatingFunctionOnceOps;

// =======================================================================
// 1. MutatingFunctionOnce Trait - One-time Function Interface
// =======================================================================

/// MutatingFunctionOnce trait - One-time mutating function interface
///
/// It is similar to the `FnOnce(&mut T) -> R` trait in the standard library.
///
/// Defines the core behavior of all one-time mutating function types.
/// Performs operations that consume self, accept a mutable reference,
/// potentially modify it, and return a result.
///
/// This trait is automatically implemented by:
/// - All closures implementing `FnOnce(&mut T) -> R`
/// - `BoxMutatingFunctionOnce<T, R>`
///
/// # Design Rationale
///
/// This trait provides a unified abstraction for one-time mutating function
/// operations. The key difference from `MutatingFunction`:
/// - `MutatingFunction` uses `&self`, can be called multiple times
/// - `MutatingFunctionOnce` uses `self`, can only be called once
///
/// # Features
///
/// - **Unified Interface**: All one-time mutating functions share the same
///   `apply` method signature
/// - **Automatic Implementation**: Closures automatically implement this
///   trait with zero overhead
/// - **Type Conversions**: Provides `into_box` method for type conversion
/// - **Generic Programming**: Write functions that work with any one-time
///   mutating function type
///
/// # Examples
///
/// ## Generic Function
///
/// ```rust
/// use qubit_function::{MutatingFunctionOnce, BoxMutatingFunctionOnce};
///
/// fn apply<F: MutatingFunctionOnce<Vec<i32>, usize>>(
///     func: F,
///     initial: Vec<i32>
/// ) -> (Vec<i32>, usize) {
///     let mut val = initial;
///     let result = func.apply(&mut val);
///     (val, result)
/// }
///
/// let data = vec![1, 2, 3];
/// let func = BoxMutatingFunctionOnce::new(move |x: &mut Vec<i32>| {
///     let old_len = x.len();
///     x.extend(data);
///     old_len
/// });
/// let (vec, old_len) = apply(func, vec![0]);
/// assert_eq!(vec, vec![0, 1, 2, 3]);
/// assert_eq!(old_len, 1);
/// ```
///
/// ## Type Conversion
///
/// ```rust
/// use qubit_function::MutatingFunctionOnce;
///
/// let data = vec![1, 2, 3];
/// let closure = move |x: &mut Vec<i32>| {
///     let old_len = x.len();
///     x.extend(data);
///     old_len
/// };
/// let box_func = closure.into_box();
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait MutatingFunctionOnce<T, R> {
    /// Performs the one-time mutating function operation
    ///
    /// Consumes self and executes an operation on the given mutable
    /// reference, potentially modifying it, and returns a result. The
    /// operation can only be called once.
    ///
    /// # Parameters
    ///
    /// * `t - A mutable reference to the input value
    ///
    /// # Returns
    ///
    /// The computed result value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{MutatingFunctionOnce,
    ///                       BoxMutatingFunctionOnce};
    ///
    /// let data = vec![1, 2, 3];
    /// let func = BoxMutatingFunctionOnce::new(move |x: &mut Vec<i32>| {
    ///     let old_len = x.len();
    ///     x.extend(data);
    ///     old_len
    /// });
    ///
    /// let mut target = vec![0];
    /// let old_len = func.apply(&mut target);
    /// assert_eq!(old_len, 1);
    /// assert_eq!(target, vec![0, 1, 2, 3]);
    /// ```
    fn apply(self, t: &mut T) -> R;

    /// Converts to `BoxMutatingFunctionOnce` (consuming)
    ///
    /// Consumes `self` and returns an owned `BoxMutatingFunctionOnce<T, R>`.
    /// The default implementation simply wraps the consuming
    /// `apply(self, &mut T)` call in a `Box<dyn FnOnce(&mut T) -> R>`.
    /// Types that can provide a cheaper or identity conversion (for example
    /// `BoxMutatingFunctionOnce` itself) should override this method.
    ///
    /// # Note
    ///
    /// - This method consumes the source value.
    /// - Implementors may return `self` directly when `Self` is already a
    ///   `BoxMutatingFunctionOnce<T, R>` to avoid the extra wrapper
    ///   allocation.
    fn into_box(self) -> BoxMutatingFunctionOnce<T, R>
    where
        Self: Sized + 'static,
    {
        BoxMutatingFunctionOnce::new(move |t| self.apply(t))
    }

    /// Converts to a consuming closure `FnOnce(&mut T) -> R`
    ///
    /// Consumes `self` and returns a closure that, when invoked, calls
    /// `apply(self, &mut T)`. This is the default, straightforward
    /// implementation; types that can produce a more direct function pointer
    /// or avoid additional captures may override it.
    fn into_fn(self) -> impl FnOnce(&mut T) -> R
    where
        Self: Sized + 'static,
    {
        move |t| self.apply(t)
    }

    /// Non-consuming adapter to `BoxMutatingFunctionOnce`
    ///
    /// Creates a `BoxMutatingFunctionOnce<T, R>` that does not consume
    /// `self`. The default implementation requires `Self: Clone` and clones
    /// the receiver for the stored closure; the clone is consumed when the
    /// boxed function is invoked. Types that can provide a zero-cost adapter
    /// (for example clonable closures) should override this method to avoid
    /// unnecessary allocations.
    fn to_box(&self) -> BoxMutatingFunctionOnce<T, R>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_box()
    }

    /// Non-consuming adapter to a callable `FnOnce(&mut T) -> R`
    ///
    /// Returns a closure that does not consume `self`. The default requires
    /// `Self: Clone` and clones `self` for the captured closure; the clone is
    /// consumed when the returned closure is invoked. Implementors may
    /// provide more efficient adapters for specific types.
    fn to_fn(&self) -> impl FnOnce(&mut T) -> R
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_fn()
    }
}
