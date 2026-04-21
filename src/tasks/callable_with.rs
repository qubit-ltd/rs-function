/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # CallableWith Types
//!
//! Provides fallible, reusable computations that operate on a mutable input.
//!
//! A `CallableWith<T, R, E>` is equivalent to
//! `FnMut(&mut T) -> Result<R, E>`, but uses task-oriented vocabulary. Use it
//! when the operation needs access to protected or caller-provided state and
//! returns a success value.
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
    tasks::runnable_with::BoxRunnableWith,
};

/// A fallible, reusable computation that receives mutable input.
///
/// Conceptually this is `FnMut(&mut T) -> Result<R, E>` with task-oriented
/// naming. It is useful for executor-style APIs that run a task with access to
/// protected state, such as a value held under a lock.
///
/// # Type Parameters
///
/// * `T` - The mutable input type.
/// * `R` - The success value returned by the computation.
/// * `E` - The error value returned when the computation fails.
///
/// # Author
///
/// Haixing Hu
pub trait CallableWith<T, R, E> {
    /// Executes the computation with mutable input.
    ///
    /// # Parameters
    ///
    /// * `input` - The mutable input passed to this task.
    ///
    /// # Returns
    ///
    /// Returns `Ok(R)` when the computation succeeds, or `Err(E)` when it
    /// fails. The exact error meaning is defined by the concrete callable.
    fn call_with(&mut self, input: &mut T) -> Result<R, E>;

    /// Converts this callable into a boxed callable.
    ///
    /// # Returns
    ///
    /// A `BoxCallableWith<T, R, E>`.
    fn into_box(mut self) -> BoxCallableWith<T, R, E>
    where
        Self: Sized + 'static,
    {
        BoxCallableWith::new(move |input| self.call_with(input))
    }

    /// Converts this callable into an `Rc` callable.
    ///
    /// # Returns
    ///
    /// A `RcCallableWith<T, R, E>`.
    fn into_rc(mut self) -> RcCallableWith<T, R, E>
    where
        Self: Sized + 'static,
    {
        RcCallableWith::new(move |input| self.call_with(input))
    }

    /// Converts this callable into an `Arc` callable.
    ///
    /// # Returns
    ///
    /// An `ArcCallableWith<T, R, E>`.
    fn into_arc(mut self) -> ArcCallableWith<T, R, E>
    where
        Self: Sized + Send + 'static,
    {
        ArcCallableWith::new(move |input| self.call_with(input))
    }

    /// Converts this callable into a mutable closure.
    ///
    /// # Returns
    ///
    /// A closure implementing `FnMut(&mut T) -> Result<R, E>`.
    fn into_fn(mut self) -> impl FnMut(&mut T) -> Result<R, E>
    where
        Self: Sized + 'static,
    {
        move |input| self.call_with(input)
    }

    /// Converts this callable into a boxed callable without consuming `self`.
    ///
    /// # Returns
    ///
    /// A `BoxCallableWith<T, R, E>` built from a clone of this callable.
    fn to_box(&self) -> BoxCallableWith<T, R, E>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_box()
    }

    /// Converts this callable into an `Rc` callable without consuming `self`.
    ///
    /// # Returns
    ///
    /// A `RcCallableWith<T, R, E>` built from a clone of this callable.
    fn to_rc(&self) -> RcCallableWith<T, R, E>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_rc()
    }

    /// Converts this callable into an `Arc` callable without consuming `self`.
    ///
    /// # Returns
    ///
    /// An `ArcCallableWith<T, R, E>` built from a clone of this callable.
    fn to_arc(&self) -> ArcCallableWith<T, R, E>
    where
        Self: Clone + Send + Sized + 'static,
    {
        self.clone().into_arc()
    }

    /// Converts this callable into a mutable closure without consuming `self`.
    ///
    /// # Returns
    ///
    /// A closure implementing `FnMut(&mut T) -> Result<R, E>`.
    fn to_fn(&self) -> impl FnMut(&mut T) -> Result<R, E>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_fn()
    }

    /// Converts this callable into a runnable by discarding the success value.
    ///
    /// # Returns
    ///
    /// A `BoxRunnableWith<T, E>` preserving errors and mapping success to unit.
    fn into_runnable_with(mut self) -> BoxRunnableWith<T, E>
    where
        Self: Sized + 'static,
    {
        BoxRunnableWith::new(move |input| self.call_with(input).map(|_| ()))
    }
}

/// Box-based callable with mutable input.
///
/// `BoxCallableWith<T, R, E>` stores a
/// `Box<dyn FnMut(&mut T) -> Result<R, E>>` and can be called repeatedly.
///
/// # Author
///
/// Haixing Hu
pub struct BoxCallableWith<T, R, E> {
    /// The stateful closure executed by this callable.
    function: Box<dyn FnMut(&mut T) -> Result<R, E>>,
    /// The optional name of this callable.
    name: Option<String>,
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

/// Single-threaded shared callable with mutable input.
///
/// `RcCallableWith<T, R, E>` stores a
/// `Rc<RefCell<dyn FnMut(&mut T) -> Result<R, E>>>`.
///
/// # Author
///
/// Haixing Hu
pub struct RcCallableWith<T, R, E> {
    /// The stateful closure executed by this callable.
    function: Rc<RefCell<dyn FnMut(&mut T) -> Result<R, E>>>,
    /// The optional name of this callable.
    name: Option<String>,
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

/// Thread-safe shared callable with mutable input.
///
/// `ArcCallableWith<T, R, E>` stores an
/// `Arc<Mutex<dyn FnMut(&mut T) -> Result<R, E> + Send>>`.
///
/// # Author
///
/// Haixing Hu
pub struct ArcCallableWith<T, R, E> {
    /// The stateful closure executed by this callable.
    function: Arc<Mutex<dyn FnMut(&mut T) -> Result<R, E> + Send>>,
    /// The optional name of this callable.
    name: Option<String>,
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
