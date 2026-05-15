/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
// qubit-style: allow explicit-imports
//! Defines the `ArcRunnable` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// ArcRunnable
// ============================================================================

/// Thread-safe runnable.
///
/// `ArcRunnable<E>` stores an `Arc<Mutex<dyn FnMut() -> Result<(), E> + Send>>`
/// and can be called repeatedly across threads.
///
/// # Type Parameters
///
/// * `E` - The error value returned when the action fails.
///
pub struct ArcRunnable<E> {
    /// The stateful closure executed by this runnable.
    pub(super) function: Arc<Mutex<dyn FnMut() -> Result<(), E> + Send>>,
    /// The optional name of this runnable.
    pub(super) name: Option<String>,
}

impl<E> Clone for ArcRunnable<E> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            function: Arc::clone(&self.function),
            name: self.name.clone(),
        }
    }
}

impl<E> ArcRunnable<E> {
    impl_common_new_methods!(
        (FnMut() -> Result<(), E> + Send + 'static),
        |function| Arc::new(Mutex::new(function)),
        "runnable"
    );

    /// Creates a thread-safe runnable from a reusable supplier.
    ///
    /// # Parameters
    ///
    /// * `supplier` - The supplier that produces the runnable result.
    ///
    /// # Returns
    ///
    /// A new `ArcRunnable<E>`.
    #[inline]
    pub fn from_supplier<S>(supplier: S) -> Self
    where
        S: Supplier<Result<(), E>> + Send + 'static,
    {
        Self::new(move || supplier.get())
    }

    impl_common_name_methods!("runnable");
}

impl<E> Runnable<E> for ArcRunnable<E> {
    /// Executes the thread-safe runnable.
    #[inline]
    fn run(&mut self) -> Result<(), E> {
        (self.function.lock())()
    }

    impl_arc_conversions!(
        ArcRunnable<E>,
        BoxRunnable,
        RcRunnable,
        FnMut() -> Result<(), E>
    );
}

impl_supplier_debug_display!(ArcRunnable<E>);
