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
//! Defines the `ArcRunnableWith` public type.

#![allow(unused_imports)]

use super::*;

/// Thread-safe shared runnable with mutable input.
///
/// `ArcRunnableWith<T, E>` stores an
/// `Arc<Mutex<dyn FnMut(&mut T) -> Result<(), E> + Send>>`.
///
pub struct ArcRunnableWith<T, E> {
    /// The stateful closure executed by this runnable.
    pub(super) function: Arc<Mutex<dyn FnMut(&mut T) -> Result<(), E> + Send>>,
    /// The optional name of this runnable.
    pub(super) name: Option<String>,
}

impl<T, E> Clone for ArcRunnableWith<T, E> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            function: Arc::clone(&self.function),
            name: self.name.clone(),
        }
    }
}

impl<T, E> ArcRunnableWith<T, E> {
    impl_common_new_methods!(
        (FnMut(&mut T) -> Result<(), E> + Send + 'static),
        |function| Arc::new(Mutex::new(function)),
        "runnable-with"
    );

    impl_common_name_methods!("runnable-with");
}

impl<T, E> RunnableWith<T, E> for ArcRunnableWith<T, E> {
    /// Executes the thread-safe runnable with mutable input.
    #[inline]
    fn run_with(&mut self, input: &mut T) -> Result<(), E> {
        (self.function.lock())(input)
    }

    impl_arc_conversions!(
        ArcRunnableWith<T, E>,
        BoxRunnableWith,
        RcRunnableWith,
        FnMut(input: &mut T) -> Result<(), E>
    );
}

impl_closure_trait!(
    RunnableWith<T, E>,
    run_with,
    FnMut(input: &mut T) -> Result<(), E>
);

impl_function_debug_display!(BoxRunnableWith<T, E>);
impl_function_debug_display!(RcRunnableWith<T, E>);
impl_function_debug_display!(ArcRunnableWith<T, E>);
