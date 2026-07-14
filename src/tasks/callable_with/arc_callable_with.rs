// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
// qubit-style: allow explicit-imports
//! Defines the `ArcCallableWith` public type.

use std::sync::Arc;

use parking_lot::Mutex;

use crate::{
    functions::macros::impl_function_debug_display,
    macros::{
        impl_common_name_methods,
        impl_common_new_methods,
    },
    tasks::callable_with::CallableWith,
};

type ArcCallableWithFn<T, R, E> =
    Arc<Mutex<dyn FnMut(&mut T) -> Result<R, E> + Send>>;

/// Thread-safe shared callable with mutable input.
///
/// `ArcCallableWith<T, R, E>` stores an
/// `Arc<Mutex<dyn FnMut(&mut T) -> Result<R, E> + Send>>`.
/// # Locking and reentrancy
///
/// Each call acquires a `parking_lot::Mutex` and holds it while the user
/// callback runs. Synchronous re-entry through the same shared wrapper
/// deadlocks. The mutex is not poisoned after a panic, and mutations completed
/// before a panic are not rolled back.
pub struct ArcCallableWith<T, R, E> {
    /// The stateful closure executed by this callable.
    pub(super) function: ArcCallableWithFn<T, R, E>,
    /// The optional name of this callable.
    pub(super) name: Option<String>,
}

impl<T, R, E> Clone for ArcCallableWith<T, R, E> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            function: Arc::clone(&self.function),
            name: self.name.clone(),
        }
    }
}

impl<T, R, E> ArcCallableWith<T, R, E> {
    impl_common_new_methods!(
        semantic_mut(CallableWith<T, R, E> + Send + 'static),
        |source| move |input: &mut T| source.call_with(input),
        |function| Arc::new(Mutex::new(function)),
        "callable-with"
    );

    impl_common_name_methods!("callable-with");
}

impl<T, R, E> CallableWith<T, R, E> for ArcCallableWith<T, R, E> {
    /// Executes the thread-safe callable with mutable input.
    #[inline]
    fn call_with(&mut self, input: &mut T) -> Result<R, E> {
        (self.function.lock())(input)
    }
}

impl_function_debug_display!(ArcCallableWith<T, R, E>);
