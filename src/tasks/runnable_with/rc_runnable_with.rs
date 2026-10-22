// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcRunnableWith` public type.

use std::cell::RefCell;
#[cfg(feature = "rc")]
use std::rc::Rc;

use crate::functions::macros::impl_function_debug_display;
use crate::macros::impl_common_name_methods;
use crate::macros::impl_common_new_methods;
use crate::tasks::runnable_with::RunnableWith;

/// The erased callback representation used by this implementation.
/// It is shared through `Rc` and dynamically mutably borrowed through
/// `RefCell` when invoked.
type RcRunnableWithFn<T, E> = Rc<RefCell<dyn FnMut(&mut T) -> Result<(), E>>>;

/// A single-threaded runnable that shares a mutable callback between clones.
///
/// `RcRunnableWith<T, E>` stores a
/// `Rc<RefCell<dyn FnMut(&mut T) -> Result<(), E>>>`.
/// Clones share the same callback state, and the wrapper is not thread-safe.
/// # Borrowing and reentrancy
///
/// Each call holds a mutable `RefCell` borrow while the user callback runs.
/// Synchronous re-entry through the same shared wrapper panics with a borrow
/// error. Mutations completed before a panic are not rolled back.
///
/// # Type Parameters
///
/// * `T` - The mutable input type accepted by the callback.
/// * `E` - The error type returned when the callback fails.
///
/// # Examples
///
/// ```
/// use qubit_function::tasks::runnable_with::{RcRunnableWith, RunnableWith};
///
/// let mut runnable = RcRunnableWith::new(|value: &mut i32| {
///     *value += 1;
///     Ok::<(), ()>(())
/// });
/// let mut value = 1;
/// runnable.run_with(&mut value).unwrap();
/// assert_eq!(value, 2);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct RcRunnableWith<T, E> {
    /// The stateful closure executed by this runnable.
    pub(super) function: RcRunnableWithFn<T, E>,
    /// The optional name of this runnable.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

impl<T, E> Clone for RcRunnableWith<T, E> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            function: Rc::clone(&self.function),
            metadata: self.metadata.clone(),
        }
    }
}

impl<T, E> RcRunnableWith<T, E> {
    impl_common_new_methods!(
        semantic_mut(RunnableWith<T, E> + 'static),
        |source| move |input: &mut T| source.run_with(input),
        |function| Rc::new(RefCell::new(function)),
        "runnable-with"
    );

    impl_common_name_methods!("runnable-with");
}

impl<T, E> RunnableWith<T, E> for RcRunnableWith<T, E> {
    /// Executes the shared runnable with mutable input.
    #[inline]
    fn run_with(&mut self, input: &mut T) -> Result<(), E> {
        let mut function = self.function.borrow_mut();
        function(input)
    }
}

impl_function_debug_display!(RcRunnableWith<T, E>);
