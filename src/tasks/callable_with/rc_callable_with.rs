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
//! Defines the `RcCallableWith` public type.

use std::cell::RefCell;
use std::rc::Rc;

use crate::{
    macros::{
        impl_common_name_methods,
        impl_common_new_methods,
        impl_rc_conversions,
    },
    tasks::{
        callable_with::{
            BoxCallableWith,
            CallableWith,
        },
        runnable_with::BoxRunnableWith,
    },
};

/// Single-threaded shared callable with mutable input.
///
/// `RcCallableWith<T, R, E>` stores a
/// `Rc<RefCell<dyn FnMut(&mut T) -> Result<R, E>>>`.
///
pub struct RcCallableWith<T, R, E> {
    /// The stateful closure executed by this callable.
    pub(super) function: Rc<RefCell<dyn FnMut(&mut T) -> Result<R, E>>>,
    /// The optional name of this callable.
    pub(super) name: Option<String>,
}

impl<T, R, E> Clone for RcCallableWith<T, R, E> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            function: Rc::clone(&self.function),
            name: self.name.clone(),
        }
    }
}

impl<T, R, E> RcCallableWith<T, R, E> {
    impl_common_new_methods!(
        (FnMut(&mut T) -> Result<R, E> + 'static),
        |function| Rc::new(RefCell::new(function)),
        "callable-with"
    );

    impl_common_name_methods!("callable-with");
}

impl<T, R, E> CallableWith<T, R, E> for RcCallableWith<T, R, E> {
    /// Executes the shared callable with mutable input.
    #[inline]
    fn call_with(&mut self, input: &mut T) -> Result<R, E> {
        (self.function.borrow_mut())(input)
    }

    impl_rc_conversions!(
        RcCallableWith<T, R, E>,
        BoxCallableWith,
        FnMut(input: &mut T) -> Result<R, E>
    );

    /// Converts this shared callable into a boxed runnable while preserving its
    /// name.
    #[inline]
    fn into_runnable_with(self) -> BoxRunnableWith<T, E>
    where
        Self: Sized + 'static,
    {
        let name = self.name;
        let function = self.function;
        BoxRunnableWith::new_with_optional_name(
            move |input| (function.borrow_mut())(input).map(|_| ()),
            name,
        )
    }
}
