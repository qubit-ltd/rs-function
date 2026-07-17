// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `ArcCallable` public type.

use std::sync::Arc;

use parking_lot::Mutex;

use crate::{
    functions::macros::impl_function_debug_display,
    macros::{
        impl_common_name_methods,
        impl_common_new_methods,
    },
    suppliers::supplier::Supplier,
    tasks::callable::Callable,
};

// ============================================================================
// ArcCallable
// ============================================================================

/// Thread-safe callable.
///
/// `ArcCallable<R, E>` stores a `Arc<Mutex<dyn FnMut() -> Result<R, E> +
/// Send>>` and can be called repeatedly across threads.
///
/// # Type Parameters
///
/// * `R` - The success value returned by the computation.
/// * `E` - The error value returned when the computation fails.
/// # Locking and reentrancy
///
/// Each call acquires a `parking_lot::Mutex` and holds it while the user
/// callback runs. Synchronous re-entry through the same shared wrapper
/// deadlocks. The mutex is not poisoned after a panic, and mutations completed
/// before a panic are not rolled back.
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct ArcCallable<R, E> {
    /// The stateful closure executed by this callable.
    pub(super) function: Arc<Mutex<dyn FnMut() -> Result<R, E> + Send>>,
    /// The optional name of this callable.
    pub(super) metadata: crate::callback_metadata::CallbackMetadata,
}

impl<R, E> Clone for ArcCallable<R, E> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            function: Arc::clone(&self.function),
            metadata: self.metadata.clone(),
        }
    }
}

impl<R, E> ArcCallable<R, E> {
    impl_common_new_methods!(
        semantic_mut(Callable<R, E> + Send + 'static),
        |source| move || source.call(),
        |function| Arc::new(Mutex::new(function)),
        "callable"
    );

    /// Creates an `ArcCallable` from a reusable supplier.
    ///
    /// # Parameters
    ///
    /// * `supplier` - The supplier that produces the callable result.
    ///
    /// # Returns
    ///
    /// A new `ArcCallable<R, E>`.
    #[inline]
    pub fn from_supplier<S>(supplier: S) -> Self
    where
        S: Supplier<Result<R, E>> + Send + 'static,
    {
        Self::new(move || supplier.get())
    }

    impl_common_name_methods!("callable");
}

impl<R, E> Callable<R, E> for ArcCallable<R, E> {
    /// Executes the thread-safe callable.
    #[inline]
    fn call(&mut self) -> Result<R, E> {
        let mut function = self.function.lock();
        function()
    }
}

impl_function_debug_display!(ArcCallable<R, E>);
