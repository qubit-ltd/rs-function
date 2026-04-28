/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # MutatorOnce Types
//!
//! Provides Java-style one-time `Mutator` interface implementations for performing
//! operations that consume self and modify the input value.
//!
//! It is similar to the `FnOnce(&mut T)` trait in the standard library.
//!
//! This module provides a unified `MutatorOnce` trait and a Box-based single
//! ownership implementation:
//!
//! - **`BoxMutatorOnce<T>`**: Box-based single ownership implementation for
//!   one-time use scenarios
//!
//! # Design Philosophy
//!
//! The key difference between `MutatorOnce` and `Mutator`:
//!
//! - **Mutator**: `&mut self`, can be called multiple times, uses `FnMut(&mut T)`
//! - **MutatorOnce**: `self`, can only be called once, uses `FnOnce(&mut T)`
//!
//! ## MutatorOnce vs Mutator
//!
//! | Feature | Mutator | MutatorOnce |
//! |---------|---------|-------------|
//! | **Self Parameter** | `&mut self` | `self` |
//! | **Call Count** | Multiple | Once |
//! | **Closure Type** | `FnMut(&mut T)` | `FnOnce(&mut T)` |
//! | **Use Cases** | Repeatable modifications | One-time resource transfers, init callbacks |
//!
//! # Why MutatorOnce?
//!
//! Core value of MutatorOnce:
//!
//! 1. **Store FnOnce closures**: Allows moving captured variables
//! 2. **Delayed execution**: Store in data structures, execute later
//! 3. **Resource transfer**: Suitable for scenarios requiring ownership transfer
//!
//! # Why Only Box Variant?
//!
//! - **Arc/Rc conflicts with FnOnce semantics**: FnOnce can only be called once,
//!   while shared ownership implies multiple references
//! - **Box is perfect match**: Single ownership aligns perfectly with one-time
//!   call semantics
//!
//! # Use Cases
//!
//! ## BoxMutatorOnce
//!
//! - Post-initialization callbacks (moving data)
//! - Resource transfer (moving Vec, String, etc.)
//! - One-time complex operations (requiring moved capture variables)
//!
//! # Examples
//!
//! ## Basic Usage
//!
//! ```rust
//! use qubit_function::{BoxMutatorOnce, MutatorOnce};
//!
//! let data = vec![1, 2, 3];
//! let mutator = BoxMutatorOnce::new(move |x: &mut Vec<i32>| {
//!     x.extend(data); // Move data
//! });
//!
//! let mut target = vec![0];
//! mutator.apply(&mut target);
//! assert_eq!(target, vec![0, 1, 2, 3]);
//! ```
//!
//! ## Method Chaining
//!
//! ```rust
//! use qubit_function::{BoxMutatorOnce, MutatorOnce};
//!
//! let data1 = vec![1, 2];
//! let data2 = vec![3, 4];
//!
//! let chained = BoxMutatorOnce::new(move |x: &mut Vec<i32>| {
//!     x.extend(data1);
//! })
//! .and_then(move |x: &mut Vec<i32>| {
//!     x.extend(data2);
//! });
//!
//! let mut target = vec![0];
//! chained.apply(&mut target);
//! assert_eq!(target, vec![0, 1, 2, 3, 4]);
//! ```
//!
//! ## Initialization Callback
//!
//! ```rust
//! use qubit_function::{BoxMutatorOnce, MutatorOnce};
//!
//! struct Initializer {
//!     on_complete: Option<BoxMutatorOnce<Vec<i32>>>,
//! }
//!
//! impl Initializer {
//!     fn new<F>(callback: F) -> Self
//!     where
//!         F: FnOnce(&mut Vec<i32>) + 'static
//!     {
//!         Self {
//!             on_complete: Some(BoxMutatorOnce::new(callback))
//!         }
//!     }
//!
//!     fn run(mut self, data: &mut Vec<i32>) {
//!         // Execute initialization logic
//!         data.push(42);
//!
//!         // Call callback
//!         if let Some(callback) = self.on_complete.take() {
//!             callback.apply(data);
//!         }
//!     }
//! }
//!
//! let data_to_add = vec![1, 2, 3];
//! let init = Initializer::new(move |x| {
//!     x.extend(data_to_add); // Move data_to_add
//! });
//!
//! let mut result = Vec::new();
//! init.run(&mut result);
//! assert_eq!(result, vec![42, 1, 2, 3]);
//! ```
//!
//! # Author
//!
//! Haixing Hu
use crate::macros::{
    impl_box_once_conversions,
    impl_closure_once_trait,
};
use crate::mutators::macros::{
    impl_box_conditional_mutator,
    impl_box_mutator_methods,
    impl_conditional_mutator_debug_display,
    impl_mutator_common_methods,
    impl_mutator_debug_display,
};
use crate::predicates::predicate::{
    BoxPredicate,
    Predicate,
};

mod box_mutator_once;
pub use box_mutator_once::BoxMutatorOnce;
mod fn_mutator_once_ops;
pub use fn_mutator_once_ops::FnMutatorOnceOps;
mod box_conditional_mutator_once;
pub use box_conditional_mutator_once::BoxConditionalMutatorOnce;

// ============================================================================
// 1. MutatorOnce Trait - One-time Mutator Interface
// ============================================================================

/// MutatorOnce trait - One-time mutator interface
///
/// Defines the core behavior of all one-time mutator types. Performs operations
/// that consume self and modify the input value.
///
/// This trait is automatically implemented by:
/// - All closures implementing `FnOnce(&mut T)`
/// - `BoxMutatorOnce<T>`
///
/// # Design Rationale
///
/// This trait provides a unified abstraction for one-time mutation operations.
/// The key difference from `Mutator`:
/// - `Mutator` uses `&mut self`, can be called multiple times
/// - `MutatorOnce` uses `self`, can only be called once
///
/// # Features
///
/// - **Unified Interface**: All one-time mutators share the same `mutate`
///   method signature
/// - **Automatic Implementation**: Closures automatically implement this
///   trait with zero overhead
/// - **Type Conversions**: Provides `into_box` method for type conversion
/// - **Generic Programming**: Write functions that work with any one-time
///   mutator type
///
/// # Examples
///
/// ## Generic Function
///
/// ```rust
/// use qubit_function::{MutatorOnce, BoxMutatorOnce};
///
/// fn apply_once<M: MutatorOnce<Vec<i32>>>(
///     mutator: M,
///     initial: Vec<i32>
/// ) -> Vec<i32> {
///     let mut val = initial;
///     mutator.apply(&mut val);
///     val
/// }
///
/// let data = vec![1, 2, 3];
/// let mutator = BoxMutatorOnce::new(move |x: &mut Vec<i32>| {
///     x.extend(data);
/// });
/// let result = apply_once(mutator, vec![0]);
/// assert_eq!(result, vec![0, 1, 2, 3]);
/// ```
///
/// ## Type Conversion
///
/// ```rust
/// use qubit_function::MutatorOnce;
///
/// let data = vec![1, 2, 3];
/// let closure = move |x: &mut Vec<i32>| x.extend(data);
/// let box_mutator = closure.into_box();
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait MutatorOnce<T> {
    /// Performs the one-time mutation operation
    ///
    /// Consumes self and executes an operation on the given mutable reference.
    /// The operation typically modifies the input value or produces side effects,
    /// and can only be called once.
    ///
    /// # Parameters
    ///
    /// * `value` - A mutable reference to the value to be mutated
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{MutatorOnce, BoxMutatorOnce};
    ///
    /// let data = vec![1, 2, 3];
    /// let mutator = BoxMutatorOnce::new(move |x: &mut Vec<i32>| {
    ///     x.extend(data);
    /// });
    ///
    /// let mut target = vec![0];
    /// mutator.apply(&mut target);
    /// assert_eq!(target, vec![0, 1, 2, 3]);
    /// ```
    fn apply(self, value: &mut T);

    /// Converts to `BoxMutatorOnce` (consuming)
    ///
    /// Consumes `self` and returns an owned `BoxMutatorOnce<T>`. The default
    /// implementation simply wraps the consuming `apply(self, &mut T)` call
    /// in a `Box<dyn FnOnce(&mut T)>`. Types that can provide a cheaper or
    /// identity conversion (for example `BoxMutatorOnce` itself) should
    /// override this method.
    ///
    /// # Note
    ///
    /// - This method consumes the source value.
    /// - Implementors may return `self` directly when `Self` is already a
    ///   `BoxMutatorOnce<T>` to avoid the extra wrapper allocation.
    fn into_box(self) -> BoxMutatorOnce<T>
    where
        Self: Sized + 'static,
    {
        BoxMutatorOnce::new(move |t| self.apply(t))
    }

    /// Converts to a consuming closure `FnOnce(&mut T)`
    ///
    /// Consumes `self` and returns a closure that, when invoked, calls
    /// `apply(self, &mut T)`. This is the default, straightforward
    /// implementation; types that can produce a more direct function pointer
    /// or avoid additional captures may override it.
    fn into_fn(self) -> impl FnOnce(&mut T)
    where
        Self: Sized + 'static,
    {
        move |t| self.apply(t)
    }

    /// Non-consuming adapter to `BoxMutatorOnce`
    ///
    /// Creates a `BoxMutatorOnce<T>` that does not consume `self`. The default
    /// implementation requires `Self: Clone` and clones the receiver for the
    /// stored closure; the clone is consumed when the boxed mutator is invoked.
    /// Types that can provide a zero-cost adapter (for example clonable
    /// closures) should override this method to avoid unnecessary allocations.
    fn to_box(&self) -> BoxMutatorOnce<T>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_box()
    }

    /// Non-consuming adapter to a callable `FnOnce(&mut T)`
    ///
    /// Returns a closure that does not consume `self`. The default requires
    /// `Self: Clone` and clones `self` for the captured closure; the clone is
    /// consumed when the returned closure is invoked. Implementors may provide
    /// more efficient adapters for specific types.
    fn to_fn(&self) -> impl FnOnce(&mut T)
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_fn()
    }
}
