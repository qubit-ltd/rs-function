/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Runnable Types
//!
//! Fallible, reusable, zero-argument actions. Design intent, equivalence to
//! [`FnMut`], and [`Send`] at executor boundaries are documented on
//! [`Runnable`].
//!
//! # Author
//!
//! Haixing Hu

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::{
    macros::{
        impl_arc_conversions,
        impl_box_conversions,
        impl_closure_trait,
        impl_common_name_methods,
        impl_common_new_methods,
        impl_rc_conversions,
    },
    suppliers::macros::impl_supplier_debug_display,
    suppliers::supplier::Supplier,
    suppliers::supplier_once::SupplierOnce,
    tasks::callable::BoxCallable,
};

// ============================================================================
// Runnable Trait
// ============================================================================

/// A fallible, reusable, zero-argument action.
///
/// Conceptually, `Runnable<E>` matches [`FnMut`] `() -> Result<(), E>`, but
/// uses task-oriented vocabulary. Prefer it when the operation’s side effect
/// matters and only success or failure need to be reported.
///
/// Each call borrows `self` mutably and returns [`Result::Ok`] with unit or
/// [`Result::Err`] with `E`. Semantically, this is a specialization of
/// [`SupplierOnce`]`<Result<(), E>>` for executable actions and deferred side
/// effects.
///
/// The trait does not require [`Send`]. Concurrent executors should require
/// `Runnable<E> + Send + 'static` (or similar) at their API boundary.
///
/// # Type Parameters
///
/// * `E` - The error value returned when the action fails.
///
/// # Examples
///
/// ```rust
/// use qubit_function::Runnable;
///
/// let mut task = || Ok::<(), String>(());
/// assert_eq!(task.run(), Ok(()));
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait Runnable<E> {
    /// Executes the action, borrowing `self` mutably.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` when the action succeeds, or `Err(E)` when it fails.
    /// The exact error meaning is defined by the concrete runnable.
    fn run(&mut self) -> Result<(), E>;

    /// Converts this runnable into a boxed runnable.
    ///
    /// # Returns
    ///
    /// A `BoxRunnable<E>` that executes this runnable when `run()` is invoked.
    fn into_box(mut self) -> BoxRunnable<E>
    where
        Self: Sized + 'static,
    {
        BoxRunnable::new(move || self.run())
    }

    /// Converts this runnable into a shared single-threaded runnable.
    ///
    /// # Returns
    ///
    /// An `RcRunnable<E>` that executes this runnable when `run()` is invoked.
    fn into_rc(mut self) -> RcRunnable<E>
    where
        Self: Sized + 'static,
    {
        RcRunnable::new(move || self.run())
    }

    /// Converts this runnable into a shared thread-safe runnable.
    ///
    /// # Returns
    ///
    /// An `ArcRunnable<E>` that executes this runnable when `run()` is invoked.
    fn into_arc(mut self) -> ArcRunnable<E>
    where
        Self: Sized + Send + 'static,
    {
        ArcRunnable::new(move || self.run())
    }

    /// Converts this runnable into a mutable closure.
    ///
    /// # Returns
    ///
    /// A closure implementing `FnMut() -> Result<(), E>`.
    fn into_fn(mut self) -> impl FnMut() -> Result<(), E>
    where
        Self: Sized + 'static,
    {
        move || self.run()
    }

    /// Converts this runnable into a boxed runnable without consuming `self`.
    ///
    /// The method clones `self` and boxes the clone. Use this for cloneable
    /// runnable values that need to be reused after boxing.
    ///
    /// # Returns
    ///
    /// A new `BoxRunnable<E>` built from a clone of this runnable.
    fn to_box(&self) -> BoxRunnable<E>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_box()
    }

    /// Converts this runnable into a mutable closure without consuming `self`.
    ///
    /// The method clones `self` and returns a mutable closure that executes
    /// the clone.
    ///
    /// # Returns
    ///
    /// A closure implementing `FnMut() -> Result<(), E>`.
    fn to_fn(&self) -> impl FnMut() -> Result<(), E>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_fn()
    }

    /// Converts this runnable into a shared single-threaded runnable without
    /// consuming `self`.
    ///
    /// The method clones `self` and wraps the clone.
    ///
    /// # Returns
    ///
    /// A new `RcRunnable<E>` built from a clone of this runnable.
    fn to_rc(&self) -> RcRunnable<E>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_rc()
    }

    /// Converts this runnable into a shared thread-safe runnable without
    /// consuming `self`.
    ///
    /// The method clones `self` and wraps the clone.
    ///
    /// # Returns
    ///
    /// A new `ArcRunnable<E>` built from a clone of this runnable.
    fn to_arc(&self) -> ArcRunnable<E>
    where
        Self: Clone + Send + Sized + 'static,
    {
        self.clone().into_arc()
    }

    /// Converts this runnable into a callable returning unit.
    ///
    /// # Returns
    ///
    /// A `BoxCallable<(), E>` that executes this runnable and returns
    /// `Ok(())` on success.
    fn into_callable(self) -> BoxCallable<(), E>
    where
        Self: Sized + 'static,
    {
        let mut runnable = self;
        BoxCallable::new(move || runnable.run())
    }
}

// ============================================================================
// BoxRunnable
// ============================================================================

/// Box-based reusable runnable.
///
/// `BoxRunnable<E>` stores a `Box<dyn FnMut() -> Result<(), E>>` and can be
/// executed repeatedly. It is the boxed concrete implementation of
/// [`Runnable`].
///
/// # Type Parameters
///
/// * `E` - The error value returned when the action fails.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BoxRunnable, Runnable};
///
/// let mut task = BoxRunnable::new(|| Ok::<(), String>(()));
/// assert_eq!(task.run(), Ok(()));
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxRunnable<E> {
    /// The stateful closure executed by this runnable.
    function: Box<dyn FnMut() -> Result<(), E>>,
    /// The optional name of this runnable.
    name: Option<String>,
}

impl<E> BoxRunnable<E> {
    impl_common_new_methods!(
        (FnMut() -> Result<(), E> + 'static),
        |function| Box::new(function),
        "runnable"
    );

    /// Creates a boxed runnable from a reusable supplier.
    ///
    /// This is an explicit bridge from `Supplier<Result<(), E>>` to
    /// `Runnable<E>`.
    ///
    /// # Parameters
    ///
    /// * `supplier` - The supplier that produces the runnable result.
    ///
    /// # Returns
    ///
    /// A new `BoxRunnable<E>`.
    #[inline]
    pub fn from_supplier<S>(supplier: S) -> Self
    where
        S: Supplier<Result<(), E>> + 'static,
    {
        Self::new(move || supplier.get())
    }

    impl_common_name_methods!("runnable");

    /// Chains another runnable after this runnable succeeds.
    ///
    /// The second runnable is not executed if this runnable returns `Err`.
    ///
    /// # Parameters
    ///
    /// * `next` - The runnable to execute after this runnable succeeds.
    ///
    /// # Returns
    ///
    /// A runnable executing both actions in sequence.
    #[inline]
    pub fn and_then<N>(self, next: N) -> BoxRunnable<E>
    where
        N: Runnable<E> + 'static,
        E: 'static,
    {
        let name = self.name;
        let mut function = self.function;
        let mut next = next;
        BoxRunnable::new_with_optional_name(
            move || {
                function()?;
                next.run()
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
    pub fn then_callable<R, C>(self, callable: C) -> BoxCallable<R, E>
    where
        C: crate::tasks::callable::Callable<R, E> + 'static,
        R: 'static,
        E: 'static,
    {
        let name = self.name;
        let mut function = self.function;
        let mut callable = callable;
        BoxCallable::new_with_optional_name(
            move || {
                function()?;
                callable.call()
            },
            name,
        )
    }
}

impl<E> Runnable<E> for BoxRunnable<E> {
    /// Executes the boxed runnable.
    #[inline]
    fn run(&mut self) -> Result<(), E> {
        (self.function)()
    }

    impl_box_conversions!(
        BoxRunnable<E>,
        RcRunnable,
        FnMut() -> Result<(), E>
    );

    /// Converts this boxed runnable into a boxed callable while preserving its
    /// name.
    #[inline]
    fn into_callable(self) -> BoxCallable<(), E>
    where
        Self: Sized + 'static,
    {
        let name = self.name;
        let mut function = self.function;
        BoxCallable::new_with_optional_name(
            move || {
                function()?;
                Ok(())
            },
            name,
        )
    }
}

// ============================================================================
// RcRunnable
// ============================================================================

/// Single-threaded shared runnable.
///
/// `RcRunnable<E>` stores a `Rc<RefCell<dyn FnMut() -> Result<(), E>>>` and can
/// be called repeatedly through shared ownership.
///
/// # Type Parameters
///
/// * `E` - The error value returned when the action fails.
///
/// # Author
///
/// Haixing Hu
pub struct RcRunnable<E> {
    /// The stateful closure executed by this runnable.
    function: Rc<RefCell<dyn FnMut() -> Result<(), E>>>,
    /// The optional name of this runnable.
    name: Option<String>,
}

impl<E> Clone for RcRunnable<E> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            function: Rc::clone(&self.function),
            name: self.name.clone(),
        }
    }
}

impl<E> RcRunnable<E> {
    impl_common_new_methods!(
        (FnMut() -> Result<(), E> + 'static),
        |function| Rc::new(RefCell::new(function)),
        "runnable"
    );

    /// Creates a shared runnable from a reusable supplier.
    ///
    /// # Parameters
    ///
    /// * `supplier` - The supplier that produces the runnable result.
    ///
    /// # Returns
    ///
    /// A new `RcRunnable<E>`.
    #[inline]
    pub fn from_supplier<S>(supplier: S) -> Self
    where
        S: Supplier<Result<(), E>> + 'static,
    {
        Self::new(move || supplier.get())
    }

    impl_common_name_methods!("runnable");
}

impl<E> Runnable<E> for RcRunnable<E> {
    /// Executes the shared runnable.
    #[inline]
    fn run(&mut self) -> Result<(), E> {
        (self.function.borrow_mut())()
    }

    impl_rc_conversions!(
        RcRunnable<E>,
        BoxRunnable,
        FnMut() -> Result<(), E>
    );
}

// ============================================================================
// ArcRunnable
// ============================================================================

/// Thread-safe runnable.
///
/// `ArcRunnable<E>` stores an `Arc<Mutex<dyn FnMut() -> Result<(), E> + Send>>`
/// and can be called repeatedly across threads.
///
/// # Type Parameters
///
/// * `E` - The error value returned when the action fails.
///
/// # Author
///
/// Haixing Hu
pub struct ArcRunnable<E> {
    /// The stateful closure executed by this runnable.
    function: Arc<Mutex<dyn FnMut() -> Result<(), E> + Send>>,
    /// The optional name of this runnable.
    name: Option<String>,
}

impl<E> Clone for ArcRunnable<E> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            function: Arc::clone(&self.function),
            name: self.name.clone(),
        }
    }
}

impl<E> ArcRunnable<E> {
    impl_common_new_methods!(
        (FnMut() -> Result<(), E> + Send + 'static),
        |function| Arc::new(Mutex::new(function)),
        "runnable"
    );

    /// Creates a thread-safe runnable from a reusable supplier.
    ///
    /// # Parameters
    ///
    /// * `supplier` - The supplier that produces the runnable result.
    ///
    /// # Returns
    ///
    /// A new `ArcRunnable<E>`.
    #[inline]
    pub fn from_supplier<S>(supplier: S) -> Self
    where
        S: Supplier<Result<(), E>> + Send + 'static,
    {
        Self::new(move || supplier.get())
    }

    impl_common_name_methods!("runnable");
}

impl<E> Runnable<E> for ArcRunnable<E> {
    /// Executes the thread-safe runnable.
    #[inline]
    fn run(&mut self) -> Result<(), E> {
        (self.function.lock())()
    }

    impl_arc_conversions!(
        ArcRunnable<E>,
        BoxRunnable,
        RcRunnable,
        FnMut() -> Result<(), E>
    );
}

impl<E> SupplierOnce<Result<(), E>> for BoxRunnable<E> {
    /// Executes the boxed runnable as a one-time supplier of `Result<(), E>`.
    #[inline]
    fn get(mut self) -> Result<(), E> {
        self.run()
    }
}

impl_closure_trait!(
    Runnable<E>,
    run,
    FnMut() -> Result<(), E>
);

impl_supplier_debug_display!(BoxRunnable<E>);
