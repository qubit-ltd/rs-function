// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `LocalBoxRunnableWith` public type.

use crate::functions::macros::impl_function_debug_display;
use crate::macros::impl_common_name_methods;
use crate::macros::impl_common_new_methods;
use crate::tasks::callable_with::CallableWith;
use crate::tasks::callable_with::LocalBoxCallableWith;
use crate::tasks::runnable_with::RunnableWith;

/// The erased callback representation used by this implementation.
type LocalBoxRunnableWithFn<T, E> = Box<dyn FnMut(&mut T) -> Result<(), E>>;

/// Local box-based runnable with mutable input.
///
/// `LocalBoxRunnableWith<T, E>` can be called repeatedly on the local thread
/// and permits non-`Send` captures. Use
/// [`BoxRunnableWith`](crate::tasks::runnable_with::BoxRunnableWith) when the
/// runnable must be movable across threads.
///
/// # Type Parameters
///
/// * `T` - The mutable input type accepted by the callback.
/// * `E` - The error type returned when the callback fails.
///
/// # Examples
///
/// ```
/// use qubit_function::tasks::runnable_with::LocalBoxRunnableWith;
/// use qubit_function::tasks::runnable_with::RunnableWith;
///
/// let mut increment = LocalBoxRunnableWith::new(|value: &mut i32| {
///     *value += 1;
///     Ok::<(), ()>(())
/// });
/// let mut value = 1;
/// increment.run_with(&mut value).unwrap();
/// assert_eq!(value, 2);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct LocalBoxRunnableWith<T, E> {
    /// The stateful closure executed by this runnable.
    pub(super) function: LocalBoxRunnableWithFn<T, E>,
    /// The optional name of this runnable.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

impl<T, E> LocalBoxRunnableWith<T, E> {
    impl_common_new_methods!(
        semantic_mut(RunnableWith<T, E> + 'static),
        |source| move |input: &mut T| source.run_with(input),
        |function| Box::new(function),
        "local runnable-with"
    );

    impl_common_name_methods!("local runnable-with");

    /// Chains another runnable after this runnable succeeds.
    ///
    /// # Parameters
    ///
    /// * `next` - The runnable to execute after this runnable succeeds.
    ///
    /// # Returns
    ///
    /// A local runnable executing both actions in sequence.
    ///
    /// # Errors
    ///
    /// When invoked, the returned runnable propagates an error from this
    /// runnable and skips `next` in that case.
    #[inline]
    pub fn and_then<N>(self, mut next: N) -> LocalBoxRunnableWith<T, E>
    where
        N: RunnableWith<T, E> + 'static,
        T: 'static,
        E: 'static,
    {
        let mut function = self.function;
        LocalBoxRunnableWith::new(move |input: &mut T| {
            function(&mut *input)?;
            next.run_with(input)
        })
    }

    /// Runs this runnable before a local callable.
    ///
    /// # Parameters
    ///
    /// * `callable` - The callable to execute after this runnable succeeds.
    ///
    /// # Returns
    ///
    /// A local callable producing the second computation's result.
    ///
    /// # Errors
    ///
    /// When invoked, the returned callable propagates an error from this
    /// runnable and skips `callable` in that case; otherwise it propagates the
    /// callable's result.
    #[inline]
    pub fn then_callable_with<R, C>(self, mut callable: C) -> LocalBoxCallableWith<T, R, E>
    where
        C: CallableWith<T, R, E> + 'static,
        T: 'static,
        R: 'static,
        E: 'static,
    {
        let mut function = self.function;
        LocalBoxCallableWith::new(move |input: &mut T| {
            function(&mut *input)?;
            callable.call_with(input)
        })
    }
}

impl<T, E> RunnableWith<T, E> for LocalBoxRunnableWith<T, E> {
    /// Executes the local boxed runnable with mutable input.
    #[inline(always)]
    fn run_with(&mut self, input: &mut T) -> Result<(), E> {
        (self.function)(input)
    }
}

impl_function_debug_display!(LocalBoxRunnableWith<T, E>);
