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

/// The erased callback representation used by this implementation.
type ArcRunnableWithFn<T, E> = Arc<Mutex<dyn FnMut(&mut T) -> Result<(), E> + Send>>;

/// Thread-safe shared runnable with mutable input.
///
/// `ArcRunnableWith<T, E>` stores an
/// `Arc<Mutex<dyn FnMut(&mut T) -> Result<(), E> + Send>>`.
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
/// use qubit_function::tasks::runnable_with::{ArcRunnableWith, RunnableWith};
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
    /// Executes the thread-safe runnable with mutable input.
    #[inline]
    fn run_with(&mut self, input: &mut T) -> Result<(), E> {
        let mut function = self.function.lock();
        function(input)
    }
}

impl_function_debug_display!(ArcRunnableWith<T, E>);
