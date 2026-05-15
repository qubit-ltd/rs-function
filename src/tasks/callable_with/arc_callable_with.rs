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
//! Defines the `ArcCallableWith` public type.

use std::sync::Arc;

use parking_lot::Mutex;

use crate::{
    functions::macros::impl_function_debug_display,
    macros::{
        impl_arc_conversions,
        impl_closure_trait,
        impl_common_name_methods,
        impl_common_new_methods,
    },
    tasks::{
        callable_with::{
            BoxCallableWith,
            CallableWith,
            RcCallableWith,
        },
        runnable_with::BoxRunnableWith,
    },
};

/// Thread-safe shared callable with mutable input.
///
/// `ArcCallableWith<T, R, E>` stores an
/// `Arc<Mutex<dyn FnMut(&mut T) -> Result<R, E> + Send>>`.
///
pub struct ArcCallableWith<T, R, E> {
    /// The stateful closure executed by this callable.
    pub(super) function: Arc<Mutex<dyn FnMut(&mut T) -> Result<R, E> + Send>>,
    /// The optional name of this callable.
    pub(super) name: Option<String>,
}

impl<T, R, E> Clone for ArcCallableWith<T, R, E> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            function: Arc::clone(&self.function),
            name: self.name.clone(),
        }
    }
}

impl<T, R, E> ArcCallableWith<T, R, E> {
    impl_common_new_methods!(
        (FnMut(&mut T) -> Result<R, E> + Send + 'static),
        |function| Arc::new(Mutex::new(function)),
        "callable-with"
    );

    impl_common_name_methods!("callable-with");
}

impl<T, R, E> CallableWith<T, R, E> for ArcCallableWith<T, R, E> {
    /// Executes the thread-safe callable with mutable input.
    #[inline]
    fn call_with(&mut self, input: &mut T) -> Result<R, E> {
        (self.function.lock())(input)
    }

    impl_arc_conversions!(
        ArcCallableWith<T, R, E>,
        BoxCallableWith,
        RcCallableWith,
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
            move |input| (function.lock())(input).map(|_| ()),
            name,
        )
    }
}

impl_closure_trait!(
    CallableWith<T, R, E>,
    call_with,
    FnMut(input: &mut T) -> Result<R, E>
);

impl_function_debug_display!(BoxCallableWith<T, R, E>);
impl_function_debug_display!(RcCallableWith<T, R, E>);
impl_function_debug_display!(ArcCallableWith<T, R, E>);
