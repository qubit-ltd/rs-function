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
//! Defines the `BoxCallableWith` public type.

use crate::{
    macros::{
        impl_box_conversions,
        impl_common_name_methods,
        impl_common_new_methods,
    },
    tasks::{
        callable_with::{
            CallableWith,
            RcCallableWith,
        },
        runnable_with::BoxRunnableWith,
    },
};

type BoxCallableWithFn<T, R, E> = Box<dyn FnMut(&mut T) -> Result<R, E>>;

/// Box-based callable with mutable input.
///
/// `BoxCallableWith<T, R, E>` stores a
/// `Box<dyn FnMut(&mut T) -> Result<R, E>>` and can be called repeatedly.
///
pub struct BoxCallableWith<T, R, E> {
    /// The stateful closure executed by this callable.
    pub(super) function: BoxCallableWithFn<T, R, E>,
    /// The optional name of this callable.
    pub(super) name: Option<String>,
}

impl<T, R, E> BoxCallableWith<T, R, E> {
    impl_common_new_methods!(
        (FnMut(&mut T) -> Result<R, E> + 'static),
        |function| Box::new(function),
        "callable-with"
    );

    impl_common_name_methods!("callable-with");

    /// Maps the success value of this callable.
    ///
    /// # Parameters
    ///
    /// * `mapper` - Function that transforms the success value.
    ///
    /// # Returns
    ///
    /// A new callable with mutable input that applies `mapper` on success.
    #[inline]
    pub fn map<U, M>(self, mut mapper: M) -> BoxCallableWith<T, U, E>
    where
        M: FnMut(R) -> U + 'static,
        T: 'static,
        R: 'static,
        E: 'static,
    {
        let name = self.name;
        let mut function = self.function;
        BoxCallableWith::new_with_optional_name(move |input| function(input).map(&mut mapper), name)
    }

    /// Maps the error value of this callable.
    ///
    /// # Parameters
    ///
    /// * `mapper` - Function that transforms the error value.
    ///
    /// # Returns
    ///
    /// A new callable with mutable input that applies `mapper` on failure.
    #[inline]
    pub fn map_err<E2, M>(self, mut mapper: M) -> BoxCallableWith<T, R, E2>
    where
        M: FnMut(E) -> E2 + 'static,
        T: 'static,
        R: 'static,
        E: 'static,
    {
        let name = self.name;
        let mut function = self.function;
        BoxCallableWith::new_with_optional_name(
            move |input| function(input).map_err(&mut mapper),
            name,
        )
    }

    /// Chains another fallible computation after this callable succeeds.
    ///
    /// # Parameters
    ///
    /// * `next` - Function receiving the success value and mutable input.
    ///
    /// # Returns
    ///
    /// A new callable that runs `next` only when this callable succeeds.
    #[inline]
    pub fn and_then<U, N>(self, next: N) -> BoxCallableWith<T, U, E>
    where
        N: FnMut(R, &mut T) -> Result<U, E> + 'static,
        T: 'static,
        R: 'static,
        E: 'static,
    {
        let name = self.name;
        let mut function = self.function;
        let mut next = next;
        BoxCallableWith::new_with_optional_name(
            move |input| {
                let value = function(input)?;
                next(value, input)
            },
            name,
        )
    }
}

impl<T, R, E> CallableWith<T, R, E> for BoxCallableWith<T, R, E> {
    /// Executes the boxed callable with mutable input.
    #[inline]
    fn call_with(&mut self, input: &mut T) -> Result<R, E> {
        (self.function)(input)
    }

    impl_box_conversions!(
        BoxCallableWith<T, R, E>,
        RcCallableWith,
        FnMut(&mut T) -> Result<R, E>
    );

    /// Converts this boxed callable into a boxed runnable while preserving its
    /// name.
    #[inline]
    fn into_runnable_with(self) -> BoxRunnableWith<T, E>
    where
        Self: Sized + 'static,
    {
        let name = self.name;
        let mut function = self.function;
        BoxRunnableWith::new_with_optional_name(move |input| function(input).map(|_| ()), name)
    }
}
