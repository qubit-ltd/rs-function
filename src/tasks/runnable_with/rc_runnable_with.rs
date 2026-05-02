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
//! Defines the `RcRunnableWith` public type.

#![allow(unused_imports)]

use super::*;

/// Single-threaded shared runnable with mutable input.
///
/// `RcRunnableWith<T, E>` stores a
/// `Rc<RefCell<dyn FnMut(&mut T) -> Result<(), E>>>`.
///
pub struct RcRunnableWith<T, E> {
    /// The stateful closure executed by this runnable.
    pub(super) function: Rc<RefCell<dyn FnMut(&mut T) -> Result<(), E>>>,
    /// The optional name of this runnable.
    pub(super) name: Option<String>,
}

impl<T, E> Clone for RcRunnableWith<T, E> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            function: Rc::clone(&self.function),
            name: self.name.clone(),
        }
    }
}

impl<T, E> RcRunnableWith<T, E> {
    impl_common_new_methods!(
        (FnMut(&mut T) -> Result<(), E> + 'static),
        |function| Rc::new(RefCell::new(function)),
        "runnable-with"
    );

    impl_common_name_methods!("runnable-with");
}

impl<T, E> RunnableWith<T, E> for RcRunnableWith<T, E> {
    /// Executes the shared runnable with mutable input.
    #[inline]
    fn run_with(&mut self, input: &mut T) -> Result<(), E> {
        (self.function.borrow_mut())(input)
    }

    impl_rc_conversions!(
        RcRunnableWith<T, E>,
        BoxRunnableWith,
        FnMut(input: &mut T) -> Result<(), E>
    );
}
