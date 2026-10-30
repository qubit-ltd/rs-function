// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `ArcRunnableWith` public type.

use std::sync::Arc;

use parking_lot::Mutex;

use crate::functions::macros::impl_function_debug_display;
use crate::macros::impl_common_name_methods;
use crate::macros::impl_common_new_methods;
use crate::tasks::runnable_with::RunnableWith;

/// Shares a mutable callback behind a mutex and requires it to be `Send`.
///
/// The callback receives mutable input and returns `Result<(), E>`. Cloned
/// handles invoke the same callback state, with the mutex serializing access.
type ArcRunnableWithFn<T, E> = Arc<Mutex<dyn FnMut(&mut T) -> Result<(), E> + Send>>;

/// Thread-safe shared runnable with mutable input.
///
/// `ArcRunnableWith<T, E>` stores an
/// `Arc<Mutex<dyn FnMut(&mut T) -> Result<(), E> + Send>>`.
/// Clones share callback state but keep independently mutable name metadata.
///
/// # Locking and reentrancy
///
/// Each call acquires a `parking_lot::Mutex` and holds it while the user
/// callback runs. Synchronous re-entry through the same shared wrapper
/// deadlocks. The mutex is not poisoned after a panic, and mutations completed
/// before a panic are not rolled back.
///
/// # Type Parameters
///
/// * `T` - The mutable input type accepted by the callback.
/// * `E` - The error type returned when the callback fails.
///
/// # Examples
///
/// ```
/// use qubit_function::tasks::runnable_with::ArcRunnableWith;
/// use qubit_function::tasks::runnable_with::RunnableWith;
///
/// let mut runnable = ArcRunnableWith::new(|value: &mut i32| {
///     *value += 1;
///     Ok::<(), ()>(())
/// });
/// let mut value = 1;
/// runnable.run_with(&mut value).unwrap();
/// assert_eq!(value, 2);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct ArcRunnableWith<T, E> {
    /// The stateful closure executed by this runnable.
    pub(super) function: ArcRunnableWithFn<T, E>,
    /// The optional name of this runnable.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

impl<T, E> Clone for ArcRunnableWith<T, E> {
    /// Clones the shared callback handle and the independently owned name
    /// metadata.
    #[inline]
    fn clone(&self) -> Self {
        Self {
            function: Arc::clone(&self.function),
            metadata: self.metadata.clone(),
        }
    }
}

impl<T, E> ArcRunnableWith<T, E> {
    impl_common_new_methods!(
        semantic_mut(RunnableWith<T, E> + Send + 'static),
        |source| move |input: &mut T| source.run_with(input),
        |function| Arc::new(Mutex::new(function)),
        "runnable-with"
    );

    impl_common_name_methods!("runnable-with");
}

impl<T, E> RunnableWith<T, E> for ArcRunnableWith<T, E> {
    /// Executes the shared callback while holding its mutex.
    ///
    /// # Parameters
    ///
    /// * `input` - The value the callback may mutate during this invocation.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` when the callback succeeds, or forwards its `Err(E)`.
    ///
    /// # Errors
    ///
    /// Returns the error produced by the wrapped callback. The input may have
    /// been partially mutated before the error was returned.
    #[inline]
    fn run_with(&mut self, input: &mut T) -> Result<(), E> {
        let mut function = self.function.lock();
        function(input)
    }
}

impl_function_debug_display!(ArcRunnableWith<T, E>);
