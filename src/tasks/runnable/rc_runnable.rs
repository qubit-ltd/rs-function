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
//! Defines the `RcRunnable` public type.

#![allow(unused_imports)]

use super::*;

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
///
pub struct RcRunnable<E> {
    /// The stateful closure executed by this runnable.
    pub(super) function: Rc<RefCell<dyn FnMut() -> Result<(), E>>>,
    /// The optional name of this runnable.
    pub(super) name: Option<String>,
}

impl<E> Clone for RcRunnable<E> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            function: Rc::clone(&self.function),
            name: self.name.clone(),
        }
    }
}

impl<E> RcRunnable<E> {
    impl_common_new_methods!(
        (FnMut() -> Result<(), E> + 'static),
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
        (self.function.borrow_mut())()
    }

    impl_rc_conversions!(
        RcRunnable<E>,
        BoxRunnable,
        FnMut() -> Result<(), E>
    );
}

impl_supplier_debug_display!(RcRunnable<E>);
