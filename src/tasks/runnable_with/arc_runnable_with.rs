// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
// qubit-style: allow explicit-imports
//! Defines the `ArcRunnableWith` public type.

use std::sync::Arc;

use parking_lot::Mutex;

use crate::{
    functions::macros::impl_function_debug_display,
    macros::{
        impl_common_name_methods,
        impl_common_new_methods,
    },
    tasks::runnable_with::RunnableWith,
};

type ArcRunnableWithFn<T, E> =
    Arc<Mutex<dyn FnMut(&mut T) -> Result<(), E> + Send>>;

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
pub struct ArcRunnableWith<T, E> {
    /// The stateful closure executed by this runnable.
    pub(super) function: ArcRunnableWithFn<T, E>,
    /// The optional name of this runnable.
    pub(super) name: Option<String>,
}

impl<T, E> Clone for ArcRunnableWith<T, E> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            function: Arc::clone(&self.function),
            name: self.name.clone(),
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
        (self.function.lock())(input)
    }
}

impl_function_debug_display!(ArcRunnableWith<T, E>);
