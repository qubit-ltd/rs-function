/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # RunnableWith Types
//!
//! Provides fallible, reusable actions that operate on a mutable input.
//!
//! A `RunnableWith<T, E>` is equivalent to
//! `FnMut(&mut T) -> Result<(), E>`, but uses task-oriented vocabulary. Use it
//! when the operation needs access to protected or caller-provided state and
//! only success or failure should be reported.
//!
//! The trait itself does not require `Send`; concurrent executors should add
//! `+ Send + 'static` at their API boundary.
//!
//! # Author
//!
//! Haixing Hu

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::{
    functions::macros::impl_function_debug_display,
    macros::{
        impl_arc_conversions,
        impl_box_conversions,
        impl_closure_trait,
        impl_common_name_methods,
        impl_common_new_methods,
        impl_rc_conversions,
    },
    tasks::callable_with::BoxCallableWith,
};

/// A fallible, reusable action that receives mutable input.
///
/// Conceptually this is `FnMut(&mut T) -> Result<(), E>` with task-oriented
/// naming. It is useful for executor-style APIs that run an action with access
/// to protected state, such as a value held under a lock.
///
/// # Type Parameters
///
/// * `T` - The mutable input type.
/// * `E` - The error value returned when the action fails.
///
/// # Author
///
/// Haixing Hu
pub trait RunnableWith<T, E> {
    /// Executes the action with mutable input.
    ///
    /// # Parameters
    ///
    /// * `input` - The mutable input passed to this task.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` when the action succeeds, or `Err(E)` when it fails.
    /// The exact error meaning is defined by the concrete runnable.
    fn run_with(&mut self, input: &mut T) -> Result<(), E>;

    /// Converts this runnable into a boxed runnable.
    ///
    /// # Returns
    ///
    /// A `BoxRunnableWith<T, E>`.
    fn into_box(mut self) -> BoxRunnableWith<T, E>
    where
        Self: Sized + 'static,
    {
        BoxRunnableWith::new(move |input| self.run_with(input))
    }

    /// Converts this runnable into an `Rc` runnable.
    ///
    /// # Returns
    ///
    /// A `RcRunnableWith<T, E>`.
    fn into_rc(mut self) -> RcRunnableWith<T, E>
    where
        Self: Sized + 'static,
    {
        RcRunnableWith::new(move |input| self.run_with(input))
    }

    /// Converts this runnable into an `Arc` runnable.
    ///
    /// # Returns
    ///
    /// An `ArcRunnableWith<T, E>`.
    fn into_arc(mut self) -> ArcRunnableWith<T, E>
    where
        Self: Sized + Send + 'static,
    {
        ArcRunnableWith::new(move |input| self.run_with(input))
    }

    /// Converts this runnable into a mutable closure.
    ///
    /// # Returns
    ///
    /// A closure implementing `FnMut(&mut T) -> Result<(), E>`.
    fn into_fn(mut self) -> impl FnMut(&mut T) -> Result<(), E>
    where
        Self: Sized + 'static,
    {
        move |input| self.run_with(input)
    }

    /// Converts this runnable into a boxed runnable without consuming `self`.
    ///
    /// # Returns
    ///
    /// A `BoxRunnableWith<T, E>` built from a clone of this runnable.
    fn to_box(&self) -> BoxRunnableWith<T, E>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_box()
    }

    /// Converts this runnable into an `Rc` runnable without consuming `self`.
    ///
    /// # Returns
    ///
    /// A `RcRunnableWith<T, E>` built from a clone of this runnable.
    fn to_rc(&self) -> RcRunnableWith<T, E>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_rc()
    }

    /// Converts this runnable into an `Arc` runnable without consuming `self`.
    ///
    /// # Returns
    ///
    /// An `ArcRunnableWith<T, E>` built from a clone of this runnable.
    fn to_arc(&self) -> ArcRunnableWith<T, E>
    where
        Self: Clone + Send + Sized + 'static,
    {
        self.clone().into_arc()
    }

    /// Converts this runnable into a mutable closure without consuming `self`.
    ///
    /// # Returns
    ///
    /// A closure implementing `FnMut(&mut T) -> Result<(), E>`.
    fn to_fn(&self) -> impl FnMut(&mut T) -> Result<(), E>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_fn()
    }

    /// Converts this runnable into a callable returning unit.
    ///
    /// # Returns
    ///
    /// A `BoxCallableWith<T, (), E>` that runs this task and returns unit on
    /// success.
    fn into_callable_with(mut self) -> BoxCallableWith<T, (), E>
    where
        Self: Sized + 'static,
    {
        BoxCallableWith::new(move |input| self.run_with(input))
    }
}

/// Box-based runnable with mutable input.
///
/// `BoxRunnableWith<T, E>` stores a
/// `Box<dyn FnMut(&mut T) -> Result<(), E>>` and can be called repeatedly.
///
/// # Author
///
/// Haixing Hu
pub struct BoxRunnableWith<T, E> {
    /// The stateful closure executed by this runnable.
    function: Box<dyn FnMut(&mut T) -> Result<(), E>>,
    /// The optional name of this runnable.
    name: Option<String>,
}

impl<T, E> BoxRunnableWith<T, E> {
    impl_common_new_methods!(
        (FnMut(&mut T) -> Result<(), E> + 'static),
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
        N: RunnableWith<T, E> + 'static,
        T: 'static,
        E: 'static,
    {
        let name = self.name;
        let mut function = self.function;
        let mut next = next;
        BoxRunnableWith::new_with_optional_name(
            move |input| {
                function(input)?;
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
    pub fn then_callable_with<R, C>(self, callable: C) -> BoxCallableWith<T, R, E>
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
            move |input| {
                function(input)?;
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

    impl_box_conversions!(
        BoxRunnableWith<T, E>,
        RcRunnableWith,
        FnMut(&mut T) -> Result<(), E>
    );

    /// Converts this boxed runnable into a boxed callable while preserving its
    /// name.
    #[inline]
    fn into_callable_with(self) -> BoxCallableWith<T, (), E>
    where
        Self: Sized + 'static,
    {
        let name = self.name;
        let mut function = self.function;
        BoxCallableWith::new_with_optional_name(
            move |input| {
                function(input)?;
                Ok(())
            },
            name,
        )
    }
}

/// Single-threaded shared runnable with mutable input.
///
/// `RcRunnableWith<T, E>` stores a
/// `Rc<RefCell<dyn FnMut(&mut T) -> Result<(), E>>>`.
///
/// # Author
///
/// Haixing Hu
pub struct RcRunnableWith<T, E> {
    /// The stateful closure executed by this runnable.
    function: Rc<RefCell<dyn FnMut(&mut T) -> Result<(), E>>>,
    /// The optional name of this runnable.
    name: Option<String>,
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

/// Thread-safe shared runnable with mutable input.
///
/// `ArcRunnableWith<T, E>` stores an
/// `Arc<Mutex<dyn FnMut(&mut T) -> Result<(), E> + Send>>`.
///
/// # Author
///
/// Haixing Hu
pub struct ArcRunnableWith<T, E> {
    /// The stateful closure executed by this runnable.
    function: Arc<Mutex<dyn FnMut(&mut T) -> Result<(), E> + Send>>,
    /// The optional name of this runnable.
    name: Option<String>,
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
