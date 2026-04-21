/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Callable Types
//!
//! Provides fallible, reusable, zero-argument computations.
//!
//! A `Callable<R, E>` is equivalent to `FnMut() -> Result<R, E>`, but uses
//! task-oriented vocabulary. Use it when the operation is a computation or task
//! whose success value matters. Use `Runnable<E>` when the operation only needs
//! to report success or failure.
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
    suppliers::supplier::Supplier,
    tasks::callable_once::BoxCallableOnce,
    tasks::runnable::BoxRunnable,
};

// ============================================================================
// Callable Trait
// ============================================================================

/// A fallible, reusable zero-argument computation.
///
/// Conceptually this is the same shape as `FnMut() -> Result<R, E>`: `call` takes
/// `&mut self` and returns `Result<R, E>`, but the API uses task-oriented naming
/// and helpers. In this crate it aligns with [`Supplier`] of `Result<R, E>`—a
/// fallible supplier—while emphasizing executable work rather than plain value
/// production.
///
/// Choose **`Callable`** when callers need the success value `R`. When only
/// success or failure matters, use [`Runnable`](crate::tasks::Runnable), whose
/// success type is `()`.
///
/// # Type Parameters
///
/// * `R` - The success value returned by the computation.
/// * `E` - The error value returned when the computation fails.
///
/// # Examples
///
/// ```rust
/// use qubit_function::Callable;
///
/// let mut task = || Ok::<i32, String>(21 * 2);
/// assert_eq!(task.call().expect("call should succeed"), 42);
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait Callable<R, E> {
    /// Executes the computation, borrowing `self` mutably.
    ///
    /// # Returns
    ///
    /// Returns `Ok(R)` when the computation succeeds, or `Err(E)` when it
    /// fails. The exact error meaning is defined by the concrete callable.
    fn call(&mut self) -> Result<R, E>;

    /// Converts this callable into a boxed callable.
    ///
    /// # Returns
    ///
    /// A `BoxCallable<R, E>` that executes this callable when `call()` is
    /// invoked.
    fn into_box(mut self) -> BoxCallable<R, E>
    where
        Self: Sized + 'static,
    {
        BoxCallable::new(move || self.call())
    }

    /// Converts this callable into an `Rc` callable.
    ///
    /// # Returns
    ///
    /// A `RcCallable<R, E>`.
    fn into_rc(mut self) -> RcCallable<R, E>
    where
        Self: Sized + 'static,
    {
        RcCallable::new(move || self.call())
    }

    /// Converts this callable into an `Arc` callable.
    ///
    /// # Returns
    ///
    /// An `ArcCallable<R, E>`.
    fn into_arc(mut self) -> ArcCallable<R, E>
    where
        Self: Sized + Send + 'static,
    {
        ArcCallable::new(move || self.call())
    }

    /// Converts this callable into a mutable closure.
    ///
    /// # Returns
    ///
    /// A closure implementing `FnMut() -> Result<R, E>`.
    fn into_fn(mut self) -> impl FnMut() -> Result<R, E>
    where
        Self: Sized + 'static,
    {
        move || self.call()
    }

    /// Converts this callable into a boxed callable without consuming `self`.
    ///
    /// The method clones `self` and boxes the clone. Use this for cloneable
    /// callable values that need to be reused after boxing.
    ///
    /// # Returns
    ///
    /// A new `BoxCallable<R, E>` built from a clone of this callable.
    fn to_box(&self) -> BoxCallable<R, E>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_box()
    }

    /// Converts this callable into an `Rc` callable without consuming `self`.
    ///
    /// The method clones `self` and wraps the clone.
    ///
    /// # Returns
    ///
    /// A `RcCallable<R, E>`.
    fn to_rc(&self) -> RcCallable<R, E>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_rc()
    }

    /// Converts this callable into an `Arc` callable without consuming `self`.
    ///
    /// The method clones `self` and wraps the clone.
    ///
    /// # Returns
    ///
    /// An `ArcCallable<R, E>`.
    fn to_arc(&self) -> ArcCallable<R, E>
    where
        Self: Clone + Send + Sized + 'static,
    {
        self.clone().into_arc()
    }

    /// Converts this callable into a mutable closure without consuming `self`.
    ///
    /// The method clones `self` and returns a closure that executes the clone
    /// on each call.
    ///
    /// # Returns
    ///
    /// A closure implementing `FnMut() -> Result<R, E>`.
    fn to_fn(&self) -> impl FnMut() -> Result<R, E>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_fn()
    }

    /// Converts this callable into a one-time callable.
    ///
    /// The returned callable consumes itself on each invocation.
    ///
    /// # Returns
    ///
    /// A `BoxCallableOnce<R, E>`.
    fn into_once(mut self) -> BoxCallableOnce<R, E>
    where
        Self: Sized + 'static,
    {
        BoxCallableOnce::new(move || self.call())
    }

    /// Converts this callable into a one-time callable without consuming
    /// `self`.
    ///
    /// The method clones `self` and returns a one-time callable.
    ///
    /// # Returns
    ///
    /// A `BoxCallableOnce<R, E>`.
    fn to_once(&self) -> BoxCallableOnce<R, E>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_once()
    }

    /// Converts this callable into a runnable by discarding the success value.
    ///
    /// The returned runnable preserves errors and maps any `Ok(R)` to
    /// `Ok(())`.
    ///
    /// # Returns
    ///
    /// A `BoxRunnable<E>` that executes this callable and discards its success
    /// value.
    fn into_runnable(mut self) -> BoxRunnable<E>
    where
        Self: Sized + 'static,
    {
        BoxRunnable::new(move || self.call().map(|_| ()))
    }
}

// ============================================================================
// BoxCallable
// ============================================================================

/// Box-based callable.
///
/// `BoxCallable<R, E>` stores a `Box<dyn FnMut() -> Result<R, E>>` and can be
/// called repeatedly. It is the boxed concrete implementation of
/// [`Callable`].
///
/// # Type Parameters
///
/// * `R` - The success value returned by the computation.
/// * `E` - The error value returned when the computation fails.
///
/// # Example
///
/// ```rust
/// use qubit_function::{BoxCallable, Callable};
///
/// let mut task = BoxCallable::new(|| Ok::<i32, String>(42));
/// assert_eq!(task.call().expect("call should succeed"), 42);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxCallable<R, E> {
    /// The stateful closure executed by this callable.
    function: Box<dyn FnMut() -> Result<R, E>>,
    /// The optional name of this callable.
    name: Option<String>,
}

impl<R, E> BoxCallable<R, E> {
    impl_common_new_methods!(
        (FnMut() -> Result<R, E> + 'static),
        |function| Box::new(function),
        "callable"
    );

    /// Creates a boxed callable from a reusable supplier.
    ///
    /// This is an explicit bridge from `Supplier<Result<R, E>>` to
    /// `Callable<R, E>`.
    ///
    /// # Parameters
    ///
    /// * `supplier` - The supplier that produces the callable result.
    ///
    /// # Returns
    ///
    /// A new `BoxCallable<R, E>`.
    #[inline]
    pub fn from_supplier<S>(supplier: S) -> Self
    where
        S: Supplier<Result<R, E>> + 'static,
    {
        Self::new(move || supplier.get())
    }

    impl_common_name_methods!("callable");

    /// Maps the success value of this callable.
    ///
    /// # Parameters
    ///
    /// * `mapper` - Function that transforms the success value.
    ///
    /// # Returns
    ///
    /// A new callable that applies `mapper` when this callable succeeds.
    #[inline]
    pub fn map<U, M>(self, mut mapper: M) -> BoxCallable<U, E>
    where
        M: FnMut(R) -> U + 'static,
        R: 'static,
        E: 'static,
    {
        let name = self.name;
        let mut function = self.function;
        BoxCallable::new_with_optional_name(move || function().map(&mut mapper), name)
    }

    /// Maps the error value of this callable.
    ///
    /// # Parameters
    ///
    /// * `mapper` - Function that transforms the error value.
    ///
    /// # Returns
    ///
    /// A new callable that applies `mapper` when this callable fails.
    #[inline]
    pub fn map_err<E2, M>(self, mut mapper: M) -> BoxCallable<R, E2>
    where
        M: FnMut(E) -> E2 + 'static,
        R: 'static,
        E: 'static,
    {
        let name = self.name;
        let mut function = self.function;
        BoxCallable::new_with_optional_name(move || function().map_err(&mut mapper), name)
    }

    /// Chains another fallible computation after this callable succeeds.
    ///
    /// # Parameters
    ///
    /// * `next` - Function that receives the success value and returns the next
    ///   result.
    ///
    /// # Returns
    ///
    /// A new callable that runs `next` only when this callable succeeds.
    #[inline]
    pub fn and_then<U, N>(self, next: N) -> BoxCallable<U, E>
    where
        N: FnMut(R) -> Result<U, E> + 'static,
        R: 'static,
        E: 'static,
    {
        let name = self.name;
        let mut function = self.function;
        let mut next = next;
        BoxCallable::new_with_optional_name(
            move || {
                let value = function()?;
                next(value)
            },
            name,
        )
    }
}

impl<R, E> Callable<R, E> for BoxCallable<R, E> {
    /// Executes the boxed callable.
    #[inline]
    fn call(&mut self) -> Result<R, E> {
        (self.function)()
    }

    impl_box_conversions!(
        BoxCallable<R, E>,
        RcCallable,
        FnMut() -> Result<R, E>,
        BoxCallableOnce
    );

    /// Converts this boxed callable into a boxed runnable while preserving its
    /// name.
    #[inline]
    fn into_runnable(self) -> BoxRunnable<E>
    where
        Self: Sized + 'static,
    {
        let name = self.name;
        let mut function = self.function;
        BoxRunnable::new_with_optional_name(move || function().map(|_| ()), name)
    }
}

// ============================================================================
// RcCallable
// ============================================================================

/// Single-threaded shared callable.
///
/// `RcCallable<R, E>` stores a `Rc<RefCell<dyn FnMut() -> Result<R, E>>>` and
/// can be called repeatedly through shared ownership.
///
/// # Type Parameters
///
/// * `R` - The success value returned by the computation.
/// * `E` - The error value returned when the computation fails.
///
/// # Author
///
/// Haixing Hu
pub struct RcCallable<R, E> {
    /// The stateful closure executed by this callable.
    function: Rc<RefCell<dyn FnMut() -> Result<R, E>>>,
    /// The optional name of this callable.
    name: Option<String>,
}

impl<R, E> Clone for RcCallable<R, E> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            function: Rc::clone(&self.function),
            name: self.name.clone(),
        }
    }
}

impl<R, E> RcCallable<R, E> {
    impl_common_new_methods!(
        (FnMut() -> Result<R, E> + 'static),
        |function| Rc::new(RefCell::new(function)),
        "callable"
    );

    /// Creates an `RcCallable` from a reusable supplier.
    ///
    /// # Parameters
    ///
    /// * `supplier` - The supplier that produces the callable result.
    ///
    /// # Returns
    ///
    /// A new `RcCallable<R, E>`.
    #[inline]
    pub fn from_supplier<S>(supplier: S) -> Self
    where
        S: Supplier<Result<R, E>> + 'static,
    {
        Self::new(move || supplier.get())
    }

    impl_common_name_methods!("callable");
}

impl<R, E> Callable<R, E> for RcCallable<R, E> {
    /// Executes the shared callable.
    #[inline]
    fn call(&mut self) -> Result<R, E> {
        (self.function.borrow_mut())()
    }

    impl_rc_conversions!(
        RcCallable<R, E>,
        BoxCallable,
        BoxCallableOnce,
        FnMut() -> Result<R, E>
    );

    /// Converts this shared callable into a boxed runnable while preserving its
    /// name.
    #[inline]
    fn into_runnable(self) -> BoxRunnable<E>
    where
        Self: Sized + 'static,
    {
        let name = self.name;
        let function = self.function;
        BoxRunnable::new_with_optional_name(move || (function.borrow_mut())().map(|_| ()), name)
    }
}

// ============================================================================
// ArcCallable
// ============================================================================

/// Thread-safe callable.
///
/// `ArcCallable<R, E>` stores a `Arc<Mutex<dyn FnMut() -> Result<R, E> + Send>>`
/// and can be called repeatedly across threads.
///
/// # Type Parameters
///
/// * `R` - The success value returned by the computation.
/// * `E` - The error value returned when the computation fails.
///
/// # Author
///
/// Haixing Hu
pub struct ArcCallable<R, E> {
    /// The stateful closure executed by this callable.
    function: Arc<Mutex<dyn FnMut() -> Result<R, E> + Send>>,
    /// The optional name of this callable.
    name: Option<String>,
}

impl<R, E> Clone for ArcCallable<R, E> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            function: Arc::clone(&self.function),
            name: self.name.clone(),
        }
    }
}

impl<R, E> ArcCallable<R, E> {
    impl_common_new_methods!(
        (FnMut() -> Result<R, E> + Send + 'static),
        |function| Arc::new(Mutex::new(function)),
        "callable"
    );

    /// Creates an `ArcCallable` from a reusable supplier.
    ///
    /// # Parameters
    ///
    /// * `supplier` - The supplier that produces the callable result.
    ///
    /// # Returns
    ///
    /// A new `ArcCallable<R, E>`.
    #[inline]
    pub fn from_supplier<S>(supplier: S) -> Self
    where
        S: Supplier<Result<R, E>> + Send + 'static,
    {
        Self::new(move || supplier.get())
    }

    impl_common_name_methods!("callable");
}

impl<R, E> Callable<R, E> for ArcCallable<R, E> {
    /// Executes the thread-safe callable.
    #[inline]
    fn call(&mut self) -> Result<R, E> {
        (self.function.lock())()
    }

    impl_arc_conversions!(
        ArcCallable<R, E>,
        BoxCallable,
        RcCallable,
        BoxCallableOnce,
        FnMut() -> Result<R, E>
    );

    /// Converts this shared callable into a boxed runnable while preserving its
    /// name.
    #[inline]
    fn into_runnable(self) -> BoxRunnable<E>
    where
        Self: Sized + 'static,
    {
        let name = self.name;
        let function = self.function;
        BoxRunnable::new_with_optional_name(move || (function.lock())().map(|_| ()), name)
    }
}

impl_closure_trait!(
    Callable<R, E>,
    call,
    BoxCallableOnce,
    FnMut() -> Result<R, E>
);

impl_function_debug_display!(BoxCallable<R, E>);
impl_function_debug_display!(RcCallable<R, E>);
impl_function_debug_display!(ArcCallable<R, E>);
