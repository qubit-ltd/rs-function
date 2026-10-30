// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcStatefulMutatingFunction` public type.

use std::cell::RefCell;
use std::rc::Rc;

use crate::Function;
use crate::Predicate;
use crate::RcConditionalStatefulMutatingFunction;
use crate::StatefulFunction;
use crate::StatefulMutatingFunction;
use crate::functions::macros::impl_function_clone;
use crate::functions::macros::impl_function_common_methods;
use crate::functions::macros::impl_function_debug_display;
use crate::functions::macros::impl_function_identity_method;
use crate::functions::macros::impl_shared_function_methods;

/// Erases the callback type while preserving its mutable call behavior.
///
/// The `Rc<RefCell<_>>` wrapper allows shared ownership on one thread; calling
/// through a reentrant mutable borrow panics.
///
/// # Type Parameters
///
/// * `T` - The mutable input accepted by the callback.
/// * `R` - The value returned by the callback.
pub(super) type RcStatefulMutatingFunctionFn<T, R> = Rc<RefCell<dyn FnMut(&mut T) -> R>>;

// =======================================================================
// 4. RcStatefulMutatingFunction - Single-Threaded Shared Ownership
// =======================================================================

/// A stateful mutating function implementation based on
/// `Rc<RefCell<dyn FnMut(&mut T) -> R>>` for single-threaded shared
/// ownership scenarios. Clones share the callback through `Rc`, and each call
/// checks its mutable borrow through `RefCell`.
///
/// # Type Parameters
///
/// * `T` - The type of the mutable input accepted by the callback.
/// * `R` - The result type returned by the callback.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Stateful**: Can modify captured environment (uses `FnMut`)
/// - **Chainable**: Method chaining via `&self` (non-consuming)
/// - **Callback sharing**: Uses `Rc` and checks mutable callback borrows with
///   `RefCell`; it does not use a mutex.
///
/// # Use Cases
///
/// Choose `RcStatefulMutatingFunction` when:
/// - The function needs to be shared within a single thread for stateful
///   operations
/// - Thread safety is not required
/// - A single-threaded callback sharing mechanism is appropriate
///
/// # Examples
///
/// ```rust
/// use qubit_function::{StatefulMutatingFunction,
///                       RcStatefulMutatingFunction};
///
/// let counter = {
///     let mut count = 0;
///     RcStatefulMutatingFunction::new(move |x: &mut i32| {
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
/// # Borrowing and reentrancy
///
/// Each call holds a mutable `RefCell` borrow while the user callback runs.
/// Synchronous re-entry through the same shared wrapper panics with a borrow
/// error. Mutations completed before a panic are not rolled back.
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct RcStatefulMutatingFunction<T, R> {
    /// The wrapped callback implementation.
    pub(super) function: RcStatefulMutatingFunctionFn<T, R>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

impl<T, R> RcStatefulMutatingFunction<T, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(),
    // set_name()
    impl_function_common_methods!(
        RcStatefulMutatingFunction<T, R>,
        (FnMut(&mut T) -> R + 'static),
        |f| Rc::new(RefCell::new(f))
    );

    // Generates: when(), and_then(), compose()
    impl_shared_function_methods!(
        RcStatefulMutatingFunction<T, R>,
        RcConditionalStatefulMutatingFunction,
        RcPredicate,
        Function,  // chains a non-mutating function after this mutating function
        predicate_bounds = ('static),
        chained_bounds = ('static)
    );

    /// Chains a stateful function after this mutating function.
    ///
    /// The source is borrowed and its shared callback handle is cloned into
    /// the returned wrapper. Each call first mutates the input and produces an
    /// intermediate result, then passes a shared reference to that result to
    /// `after`. Clones of the returned wrapper share both callback states.
    /// The returned wrapper has no diagnostic name; use [`Self::with_name`] to
    /// assign one. Reentrant calls through the same callback can panic because
    /// `RefCell` rejects overlapping mutable borrows. Completed mutations are
    /// not rolled back after a panic.
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
    /// use qubit_function::RcStatefulMutatingFunction;
    /// use qubit_function::StatefulMutatingFunction;
    ///
    /// let mut source_calls = 0;
    /// let source = RcStatefulMutatingFunction::new(move |value: &mut i32| {
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
    pub fn and_then_stateful<S, F>(&self, mut after: F) -> RcStatefulMutatingFunction<T, S>
    where
        T: 'static,
        R: 'static,
        S: 'static,
        F: StatefulFunction<R, S> + 'static,
    {
        let mut source = self.clone();
        RcStatefulMutatingFunction::new(move |input: &mut T| {
            let result = source.apply(input);
            after.apply(&result)
        })
    }
}

// Generates: Clone implementation for RcStatefulMutatingFunction<T, R>
impl_function_clone!(RcStatefulMutatingFunction<T, R>);

// Generates: Debug and Display implementations for
// RcStatefulMutatingFunction<T, R>
impl_function_debug_display!(RcStatefulMutatingFunction<T, R>);

// Generates: identity() method for RcStatefulMutatingFunction<T, T>
impl_function_identity_method!(RcStatefulMutatingFunction<T, T>, mutating, StatefulMutatingFunction);

// Implement StatefulMutatingFunction trait for RcStatefulMutatingFunction<T, R>
impl<T, R> StatefulMutatingFunction<T, R> for RcStatefulMutatingFunction<T, R> {
    #[inline]
    fn apply(&mut self, t: &mut T) -> R {
        let mut function = self.function.borrow_mut();
        function(t)
    }
}
