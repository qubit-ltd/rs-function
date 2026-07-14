// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcRunnable` public type.

use std::cell::RefCell;
#[cfg(feature = "rc")]
use std::rc::Rc;

use crate::{
    macros::{
        impl_common_name_methods,
        impl_common_new_methods,
    },
    suppliers::{
        macros::impl_supplier_debug_display,
        supplier::Supplier,
    },
    tasks::runnable::Runnable,
};

// ============================================================================
// RcRunnable
// ============================================================================

/// Single-threaded shared runnable.
///
/// `RcRunnable<E>` stores a `Rc<RefCell<dyn FnMut() -> Result<(), E>>>` and can
/// be called repeatedly through shared ownership.
///
/// # Type Parameters
///
/// * `E` - The error value returned when the action fails.
/// # Borrowing and reentrancy
///
/// Each call holds a mutable `RefCell` borrow while the user callback runs.
/// Synchronous re-entry through the same shared wrapper panics with a borrow
/// error. Mutations completed before a panic are not rolled back.
pub struct RcRunnable<E> {
    /// The stateful closure executed by this runnable.
    pub(super) function: Rc<RefCell<dyn FnMut() -> Result<(), E>>>,
    /// The optional name of this runnable.
    pub(super) metadata: crate::callback_metadata::CallbackMetadata,
}

impl<E> Clone for RcRunnable<E> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            function: Rc::clone(&self.function),
            metadata: self.metadata.clone(),
        }
    }
}

impl<E> RcRunnable<E> {
    impl_common_new_methods!(
        semantic_mut(Runnable<E> + 'static),
        |source| move || source.run(),
        |function| Rc::new(RefCell::new(function)),
        "runnable"
    );

    /// Creates a shared runnable from a reusable supplier.
    ///
    /// # Parameters
    ///
    /// * `supplier` - The supplier that produces the runnable result.
    ///
    /// # Returns
    ///
    /// A new `RcRunnable<E>`.
    #[inline]
    pub fn from_supplier<S>(supplier: S) -> Self
    where
        S: Supplier<Result<(), E>> + 'static,
    {
        Self::new(move || supplier.get())
    }

    impl_common_name_methods!("runnable");
}

impl<E> Runnable<E> for RcRunnable<E> {
    /// Executes the shared runnable.
    #[inline]
    fn run(&mut self) -> Result<(), E> {
        let mut function = self.function.borrow_mut();
        function()
    }
}

impl_supplier_debug_display!(RcRunnable<E>);
