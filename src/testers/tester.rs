/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Defines the `Tester` public trait.

use super::{
    Arc,
    Rc,
};

use self::{
    arc_tester::ArcTester,
    box_tester::BoxTester,
    rc_tester::RcTester,
};

pub mod arc_tester;
pub mod box_tester;
pub mod fn_tester_ops;
pub mod rc_tester;

/// Tests whether a zero-argument condition holds.
///
/// `Tester` is a functional abstraction equivalent to `Fn() -> bool`. It
/// accepts no parameters, uses `&self`, and returns a boolean value indicating
/// the test result of some state or condition.
///
/// # Core Characteristics
///
/// - **No input parameters**: Captures context through closures
/// - **Returns boolean**: Indicates test results
/// - **Uses `&self`**: Matches `Fn() -> bool` and does not require mutable
///   access to its own state
/// - **Repeatable calls**: The same Tester can call `test()` multiple times
/// - **Stateless closure shape**: For `FnMut() -> bool`, use
///   [`StatefulTester`](super::stateful_tester::StatefulTester)
///
/// # Use Cases
///
/// - **State checking**: Check system or service status
/// - **Condition waiting**: Repeatedly check until conditions are met
/// - **Health monitoring**: Check system health status
/// - **Precondition validation**: Verify conditions before operations
///
/// # Design Philosophy
///
/// Tester's responsibility is "test judgment", not "state management".
/// State management is the caller's responsibility. Tester only reads state
/// and returns judgment results.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BoxTester, Tester};
/// use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
///
/// // State managed externally
/// let ready = Arc::new(AtomicBool::new(false));
/// let ready_clone = Arc::clone(&ready);
///
/// // Tester only responsible for reading state
/// let tester = BoxTester::new(move || {
///     ready_clone.load(Ordering::Acquire)
/// });
///
/// // Can be called multiple times
/// assert!(!tester.test());
/// ready.store(true, Ordering::Release);
/// assert!(tester.test());
/// ```
pub trait Tester {
    /// Executes the test and returns the test result
    ///
    /// This method can be called multiple times without modifying the Tester's
    /// own state.
    ///
    /// # Return Value
    ///
    /// Returns `true` if the condition holds, otherwise returns `false`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BoxTester, Tester};
    ///
    /// let tester = BoxTester::new(|| true);
    /// assert!(tester.test());
    /// ```
    fn test(&self) -> bool;

    /// Converts this tester to `BoxTester`
    ///
    /// # Return Value
    ///
    /// A `BoxTester` that wraps this tester
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Tester, BoxTester};
    ///
    /// let closure = || true;
    /// let boxed: BoxTester = closure.into_box();
    /// ```
    #[inline]
    fn into_box(self) -> BoxTester
    where
        Self: Sized + 'static,
    {
        BoxTester {
            function: Box::new(move || self.test()),
        }
    }

    /// Converts this tester to `RcTester`
    ///
    /// # Return Value
    ///
    /// A `RcTester` that wraps this tester
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Tester, RcTester};
    ///
    /// let closure = || true;
    /// let rc: RcTester = closure.into_rc();
    /// ```
    #[inline]
    fn into_rc(self) -> RcTester
    where
        Self: Sized + 'static,
    {
        RcTester {
            function: Rc::new(move || self.test()),
        }
    }

    /// Converts this tester to `ArcTester`
    ///
    /// # Return Value
    ///
    /// An `ArcTester` that wraps this tester
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Tester, ArcTester};
    ///
    /// let closure = || true;
    /// let arc: ArcTester = closure.into_arc();
    /// ```
    #[inline]
    fn into_arc(self) -> ArcTester
    where
        Self: Sized + Send + Sync + 'static,
    {
        ArcTester {
            function: Arc::new(move || self.test()),
        }
    }

    /// Converts this tester to a boxed function pointer
    ///
    /// # Return Value
    ///
    /// A `Box<dyn Fn() -> bool>` that wraps this tester
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::Tester;
    ///
    /// let closure = || true;
    /// let func = closure.into_fn();
    /// assert!(func());
    /// ```
    #[inline]
    fn into_fn(self) -> impl Fn() -> bool
    where
        Self: Sized + 'static,
    {
        Box::new(move || self.test())
    }

    /// Clones and converts this tester to `BoxTester`
    ///
    /// # Return Value
    ///
    /// A `BoxTester` that wraps a clone of this tester
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Tester, BoxTester, ArcTester};
    ///
    /// let arc = ArcTester::new(|| true);
    /// let boxed: BoxTester = arc.to_box();
    /// // arc is still available
    /// ```
    #[inline]
    fn to_box(&self) -> BoxTester
    where
        Self: Clone + 'static,
    {
        self.clone().into_box()
    }

    /// Clones and converts this tester to `RcTester`
    ///
    /// # Return Value
    ///
    /// A `RcTester` that wraps a clone of this tester
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Tester, RcTester, ArcTester};
    ///
    /// let arc = ArcTester::new(|| true);
    /// let rc: RcTester = arc.to_rc();
    /// // arc is still available
    /// ```
    #[inline]
    fn to_rc(&self) -> RcTester
    where
        Self: Clone + 'static,
    {
        self.clone().into_rc()
    }

    /// Clones and converts this tester to `ArcTester`
    ///
    /// # Return Value
    ///
    /// An `ArcTester` that wraps a clone of this tester
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Tester, ArcTester, RcTester};
    ///
    /// let rc = RcTester::new(|| true);
    /// // Note: This will panic for RcTester as it's not Send + Sync
    /// // let arc: ArcTester = rc.to_arc();
    /// ```
    #[inline]
    fn to_arc(&self) -> ArcTester
    where
        Self: Clone + Send + Sync + 'static,
    {
        self.clone().into_arc()
    }

    /// Clones and converts this tester to a boxed function pointer
    ///
    /// # Return Value
    ///
    /// A `Box<dyn Fn() -> bool>` that wraps a clone of this tester
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Tester, ArcTester};
    ///
    /// let arc = ArcTester::new(|| true);
    /// let func = arc.to_fn();
    /// // arc is still available
    /// assert!(func());
    /// ```
    #[inline]
    fn to_fn(&self) -> impl Fn() -> bool
    where
        Self: Clone + 'static,
    {
        self.clone().into_fn()
    }
}
