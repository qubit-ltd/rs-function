// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `ArcStatefulMutatingFunction` public type.

use std::sync::Arc;

use parking_lot::Mutex;

use crate::ArcConditionalStatefulMutatingFunction;
use crate::Function;
use crate::Predicate;
use crate::StatefulFunction;
use crate::StatefulMutatingFunction;
use crate::functions::macros::impl_function_clone;
use crate::functions::macros::impl_function_common_methods;
use crate::functions::macros::impl_function_debug_display;
use crate::functions::macros::impl_function_identity_method;
use crate::functions::macros::impl_shared_function_methods;

// =======================================================================
// 5. ArcStatefulMutatingFunction - Thread-Safe Shared Ownership
// =======================================================================

/// Erases a stateful mutable callback behind shared, synchronized ownership.
///
/// # Type Parameters
///
/// * `T` - The type of the callback's mutable input.
/// * `R` - The type returned by the callback.
type ArcStatefulMutatingFunctionFn<T, R> = Arc<Mutex<dyn FnMut(&mut T) -> R + Send + 'static>>;

/// ArcStatefulMutatingFunction struct
///
/// A stateful mutating function implementation based on
/// `Arc<Mutex<dyn FnMut(&mut T) -> R + Send>>` for thread-safe shared
/// ownership scenarios. This type allows the function to be safely shared
/// and used across multiple threads.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send + Sync`, safe for concurrent use
/// - **Stateful**: Can modify captured environment (uses `FnMut`)
/// - **Chainable**: Method chaining via `&self` (non-consuming)
///
/// # Type Parameters
///
/// * `T` - The mutable input type accepted by the callback.
/// * `R` - The result type returned by the callback.
///
/// # Use Cases
///
/// Choose `ArcStatefulMutatingFunction` when:
/// - The function needs to be shared across multiple threads for stateful
///   operations
/// - Concurrent task processing (e.g., thread pools)
/// - Thread safety is required (Send + Sync)
///
/// # Examples
///
/// ```rust
/// use qubit_function::ArcStatefulMutatingFunction;
/// use qubit_function::StatefulMutatingFunction;
///
/// let counter = {
///     let mut count = 0;
///     ArcStatefulMutatingFunction::new(move |x: &mut i32| {
///         count += 1;
///         *x *= 2;
///         count
///     })
/// };
/// let mut clone = counter.clone();
///
/// let mut value = 5;
/// assert_eq!(clone.apply(&mut value), 1);
/// assert_eq!(value, 10);
/// ```
///
/// # Locking and reentrancy
///
/// Each call acquires a `parking_lot::Mutex` and holds it while the user
/// callback runs. Synchronous re-entry through the same shared state
/// deadlocks. The mutex is not poisoned after a panic, and mutations completed
/// before a panic are not rolled back.
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct ArcStatefulMutatingFunction<T, R> {
    /// The wrapped callback implementation.
    pub(super) function: ArcStatefulMutatingFunctionFn<T, R>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

impl<T, R> ArcStatefulMutatingFunction<T, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(),
    // set_name()
    impl_function_common_methods!(
        ArcStatefulMutatingFunction<T, R>,
        (FnMut(&mut T) -> R + Send + 'static),
        |f| Arc::new(Mutex::new(f))
    );

    // Generates: when(), and_then(), compose()
    impl_shared_function_methods!(
        ArcStatefulMutatingFunction<T, R>,
        ArcConditionalStatefulMutatingFunction,
        ArcPredicate,
        Function,  // chains a non-mutating function after this mutating function
        predicate_bounds = (Send + Sync + 'static),
        chained_bounds = (Send + 'static)
    );

    /// Chains a stateful function after this mutating function.
    ///
    /// The source is borrowed and its shared callback handle is cloned into
    /// the returned wrapper. Each call first mutates the input and produces an
    /// intermediate result, then passes a shared reference to that result to
    /// `after`. Clones of the returned wrapper share both callback states.
    /// The chained callback must be `Send` so the result remains usable across
    /// threads; it need not be `Sync`. The returned wrapper has no diagnostic
    /// name; use [`Self::with_name`] to assign one. The source lock is released
    /// before the chained callback runs. Reentrant calls through the same
    /// callback can deadlock, and completed mutations are not rolled back
    /// after a panic.
    ///
    /// # Type Parameters
    ///
    /// - `S`: The result type returned by the chained function.
    /// - `F`: The stateful function type.
    ///
    /// # Parameters
    ///
    /// - `after`: The stateful function to call with each intermediate result.
    ///
    /// # Returns
    ///
    /// A new stateful mutating function that returns the chained result.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::ArcStatefulMutatingFunction;
    /// use qubit_function::StatefulMutatingFunction;
    ///
    /// let mut source_calls = 0;
    /// let source = ArcStatefulMutatingFunction::new(move |value: &mut i32| {
    ///     source_calls += 1;
    ///     *value += source_calls;
    ///     *value
    /// });
    /// let mut calls = 0;
    /// let mut pipeline = source.and_then_stateful(move |value: &i32| {
    ///     calls += 1;
    ///     *value + calls
    /// });
    /// let mut input = 5;
    /// assert_eq!(pipeline.apply(&mut input), 7);
    /// assert_eq!(pipeline.apply(&mut input), 10);
    /// ```
    pub fn and_then_stateful<S, F>(&self, mut after: F) -> ArcStatefulMutatingFunction<T, S>
    where
        T: 'static,
        R: 'static,
        S: 'static,
        F: StatefulFunction<R, S> + Send + 'static,
    {
        let mut source = self.clone();
        ArcStatefulMutatingFunction::new(move |input: &mut T| {
            let result = source.apply(input);
            after.apply(&result)
        })
    }
}

// Generates: Clone implementation for ArcStatefulMutatingFunction<T, R>
impl_function_clone!(ArcStatefulMutatingFunction<T, R>);

// Generates: Debug and Display implementations for
// ArcStatefulMutatingFunction<T, R>
impl_function_debug_display!(ArcStatefulMutatingFunction<T, R>);

// Generates: identity() method for ArcStatefulMutatingFunction<T, T>
impl_function_identity_method!(ArcStatefulMutatingFunction<T, T>, mutating, StatefulMutatingFunction);

// Implement StatefulMutatingFunction trait for ArcStatefulMutatingFunction<T,
// R>
impl<T, R> StatefulMutatingFunction<T, R> for ArcStatefulMutatingFunction<T, R> {
    #[inline]
    fn apply(&mut self, t: &mut T) -> R {
        let mut function = self.function.lock();
        function(t)
    }
}
