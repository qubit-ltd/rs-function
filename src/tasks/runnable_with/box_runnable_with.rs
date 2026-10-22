// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxRunnableWith` public type.

use crate::functions::macros::impl_function_debug_display;
use crate::macros::impl_common_name_methods;
use crate::macros::impl_common_new_methods;
use crate::tasks::callable_with::BoxCallableWith;
use crate::tasks::runnable_with::RunnableWith;

/// The erased callback representation used by this implementation.
type BoxRunnableWithFn<T, E> = Box<dyn FnMut(&mut T) -> Result<(), E> + Send>;

/// A boxed, fallible action that can mutate its input and move across threads.
///
/// The callback is stored as a `Send` trait object, so the wrapper can be
/// transferred between threads when its input and error types satisfy the
/// requirements of the surrounding operation. Calls may mutate the input and
/// retain callback state between invocations.
///
/// # Type Parameters
///
/// * `T` - The mutable input type accepted by the callback.
/// * `E` - The error type returned when the callback fails.
///
/// # Examples
///
/// ```
/// use qubit_function::tasks::runnable_with::BoxRunnableWith;
/// use qubit_function::tasks::runnable_with::RunnableWith;
///
/// let mut increment = BoxRunnableWith::new(|value: &mut i32| {
///     *value += 1;
///     Ok::<(), ()>(())
/// });
/// let mut value = 1;
/// increment.run_with(&mut value).unwrap();
/// assert_eq!(value, 2);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxRunnableWith<T, E> {
    /// The stateful closure executed by this runnable.
    pub(super) function: BoxRunnableWithFn<T, E>,
    /// The optional name of this runnable.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

impl<T, E> BoxRunnableWith<T, E> {
    impl_common_new_methods!(
        semantic_mut(RunnableWith<T, E> + Send + 'static),
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
    #[inline]
    pub fn and_then<N>(self, next: N) -> BoxRunnableWith<T, E>
    where
        N: RunnableWith<T, E> + Send + 'static,
        T: 'static,
        E: 'static,
    {
        let mut function = self.function;
        let mut next = next;
        BoxRunnableWith::new(move |input: &mut T| {
            function(&mut *input)?;
            next.run_with(input)
        })
    }

    /// Runs this runnable before a callable.
    ///
    /// The callable is not executed if this runnable returns `Err`.
    /// Because this operation sequences two independent callbacks, the
    /// returned callable is unnamed.
    ///
    /// # Parameters
    ///
    /// * `callable` - The callable to execute after this runnable succeeds.
    ///
    /// # Returns
    ///
    /// A callable producing the second computation's result.
    #[inline]
    pub fn then_callable_with<R, C>(self, callable: C) -> BoxCallableWith<T, R, E>
    where
        C: crate::tasks::callable_with::CallableWith<T, R, E> + Send + 'static,
        T: 'static,
        R: 'static,
        E: 'static,
    {
        let mut function = self.function;
        let mut callable = callable;
        BoxCallableWith::new(move |input: &mut T| {
            function(&mut *input)?;
            callable.call_with(input)
        })
    }
}

impl<T, E> RunnableWith<T, E> for BoxRunnableWith<T, E> {
    /// Executes the boxed runnable with mutable input.
    #[inline(always)]
    fn run_with(&mut self, input: &mut T) -> Result<(), E> {
        (self.function)(input)
    }
}

impl_function_debug_display!(BoxRunnableWith<T, E>);
