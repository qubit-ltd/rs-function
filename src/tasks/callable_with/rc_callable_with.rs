// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcCallableWith` public type.

use std::cell::RefCell;
#[cfg(feature = "rc")]
use std::rc::Rc;

use crate::functions::macros::impl_function_debug_display;
use crate::macros::impl_common_name_methods;
use crate::macros::impl_common_new_methods;
use crate::tasks::callable_with::CallableWith;

/// The erased callback representation used by this implementation.
type RcCallableWithFn<T, R, E> = Rc<RefCell<dyn FnMut(&mut T) -> Result<R, E>>>;

/// Single-threaded shared callable with mutable input.
///
/// `RcCallableWith<T, R, E>` stores a
/// `Rc<RefCell<dyn FnMut(&mut T) -> Result<R, E>>>`.
/// # Borrowing and reentrancy
///
/// Each call holds a mutable `RefCell` borrow while the user callback runs.
/// Synchronous re-entry through the same shared wrapper panics with a borrow
/// error. Mutations completed before a panic are not rolled back.
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct RcCallableWith<T, R, E> {
    /// The stateful closure executed by this callable.
    pub(super) function: RcCallableWithFn<T, R, E>,
    /// The optional name of this callable.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

impl<T, R, E> Clone for RcCallableWith<T, R, E> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            function: Rc::clone(&self.function),
            metadata: self.metadata.clone(),
        }
    }
}

impl<T, R, E> RcCallableWith<T, R, E> {
    impl_common_new_methods!(
        semantic_mut(CallableWith<T, R, E> + 'static),
        |source| move |input: &mut T| source.call_with(input),
        |function| Rc::new(RefCell::new(function)),
        "callable-with"
    );

    impl_common_name_methods!("callable-with");
}

impl<T, R, E> CallableWith<T, R, E> for RcCallableWith<T, R, E> {
    /// Executes the shared callable with mutable input.
    #[inline]
    fn call_with(&mut self, input: &mut T) -> Result<R, E> {
        let mut function = self.function.borrow_mut();
        function(input)
    }
}

impl_function_debug_display!(RcCallableWith<T, R, E>);
