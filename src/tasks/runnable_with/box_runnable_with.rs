// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
// qubit-style: allow explicit-imports
//! Defines the `BoxRunnableWith` public type.

use crate::{
    functions::macros::impl_function_debug_display,
    macros::{
        impl_common_name_methods,
        impl_common_new_methods,
    },
    tasks::{
        callable_with::BoxCallableWith,
        runnable_with::RunnableWith,
    },
};

type BoxRunnableWithFn<T, E> = Box<dyn FnMut(&mut T) -> Result<(), E>>;

/// Box-based runnable with mutable input.
///
/// `BoxRunnableWith<T, E>` stores a
/// `Box<dyn FnMut(&mut T) -> Result<(), E>>` and can be called repeatedly.
pub struct BoxRunnableWith<T, E> {
    /// The stateful closure executed by this runnable.
    pub(super) function: BoxRunnableWithFn<T, E>,
    /// The optional name of this runnable.
    pub(super) name: Option<String>,
}

impl<T, E> BoxRunnableWith<T, E> {
    impl_common_new_methods!(
        semantic_mut(RunnableWith<T, E> + 'static),
        |source| move |input: &mut T| source.run_with(input),
        |function| Box::new(function),
        "runnable-with"
    );

    impl_common_name_methods!("runnable-with");

    /// Chains another runnable after this runnable succeeds.
    ///
    /// # Parameters
    ///
    /// * `next` - The runnable to execute after this runnable succeeds.
    ///
    /// # Returns
    ///
    /// A runnable executing both actions in sequence.
    #[cfg(feature = "combinators")]
    #[inline]
    pub fn and_then<N>(self, next: N) -> BoxRunnableWith<T, E>
    where
        N: RunnableWith<T, E> + 'static,
        T: 'static,
        E: 'static,
    {
        let name = self.name;
        let mut function = self.function;
        let mut next = next;
        BoxRunnableWith::new_with_optional_name(
            move |input: &mut T| {
                function(&mut *input)?;
                next.run_with(input)
            },
            name,
        )
    }

    /// Runs this runnable before a callable.
    ///
    /// The callable is not executed if this runnable returns `Err`.
    ///
    /// # Parameters
    ///
    /// * `callable` - The callable to execute after this runnable succeeds.
    ///
    /// # Returns
    ///
    /// A callable producing the second computation's result.
    #[inline]
    pub fn then_callable_with<R, C>(
        self,
        callable: C,
    ) -> BoxCallableWith<T, R, E>
    where
        C: crate::tasks::callable_with::CallableWith<T, R, E> + 'static,
        T: 'static,
        R: 'static,
        E: 'static,
    {
        let name = self.name;
        let mut function = self.function;
        let mut callable = callable;
        BoxCallableWith::new_with_optional_name(
            move |input: &mut T| {
                function(&mut *input)?;
                callable.call_with(input)
            },
            name,
        )
    }
}

impl<T, E> RunnableWith<T, E> for BoxRunnableWith<T, E> {
    /// Executes the boxed runnable with mutable input.
    #[inline]
    fn run_with(&mut self, input: &mut T) -> Result<(), E> {
        (self.function)(input)
    }
}

impl_function_debug_display!(BoxRunnableWith<T, E>);
