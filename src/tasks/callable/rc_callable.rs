// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
// qubit-style: allow explicit-imports
//! Defines the `RcCallable` public type.

use std::cell::RefCell;
#[cfg(feature = "rc")]
use std::rc::Rc;

use crate::{
    macros::{
        impl_common_name_methods,
        impl_common_new_methods,
    },
    suppliers::supplier::Supplier,
    tasks::callable::Callable,
};

// ============================================================================
// RcCallable
// ============================================================================

/// Single-threaded shared callable.
///
/// `RcCallable<R, E>` stores a `Rc<RefCell<dyn FnMut() -> Result<R, E>>>` and
/// can be called repeatedly through shared ownership.
///
/// # Type Parameters
///
/// * `R` - The success value returned by the computation.
/// * `E` - The error value returned when the computation fails.
/// # Borrowing and reentrancy
///
/// Each call holds a mutable `RefCell` borrow while the user callback runs.
/// Synchronous re-entry through the same shared wrapper panics with a borrow
/// error. Mutations completed before a panic are not rolled back.
pub struct RcCallable<R, E> {
    /// The stateful closure executed by this callable.
    pub(super) function: Rc<RefCell<dyn FnMut() -> Result<R, E>>>,
    /// The optional name of this callable.
    pub(super) name: Option<String>,
}

impl<R, E> Clone for RcCallable<R, E> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            function: Rc::clone(&self.function),
            name: self.name.clone(),
        }
    }
}

impl<R, E> RcCallable<R, E> {
    impl_common_new_methods!(
        semantic_mut(Callable<R, E> + 'static),
        |source| move || source.call(),
        |function| Rc::new(RefCell::new(function)),
        "callable"
    );

    /// Creates an `RcCallable` from a reusable supplier.
    ///
    /// # Parameters
    ///
    /// * `supplier` - The supplier that produces the callable result.
    ///
    /// # Returns
    ///
    /// A new `RcCallable<R, E>`.
    #[inline]
    pub fn from_supplier<S>(supplier: S) -> Self
    where
        S: Supplier<Result<R, E>> + 'static,
    {
        Self::new(move || supplier.get())
    }

    impl_common_name_methods!("callable");
}

impl<R, E> Callable<R, E> for RcCallable<R, E> {
    /// Executes the shared callable.
    #[inline]
    fn call(&mut self) -> Result<R, E> {
        (self.function.borrow_mut())()
    }
}
