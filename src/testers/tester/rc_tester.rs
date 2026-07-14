// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
// qubit-style: allow explicit-imports
//! Defines the `RcTester` public type.

#[cfg(feature = "combinators")]
use std::ops::Not;

use super::{
    Rc,
    Tester,
};

// ============================================================================
// RcTester: Single-Threaded Shared Ownership Implementation
// ============================================================================

/// Single-threaded shared ownership Tester implemented using `Rc`
///
/// `RcTester` wraps a closure in `Rc<dyn Fn() -> bool>`, allowing the tester
/// to be cloned and shared within a single thread. Since it doesn't use atomic
/// operations, it has lower overhead than `ArcTester`.
///
/// # Characteristics
///
/// - **Shared ownership**: Can be cloned
/// - **Single-threaded**: Cannot be sent across threads
/// - **Low overhead**: Uses `Fn` without needing `RefCell`
/// - **Borrowing combination**: With `combinators`, `and()`/`or()` borrow
///   `&self`
///
/// # Use Cases
///
/// - Single-threaded testing scenarios requiring sharing
/// - Event-driven systems (single-threaded)
/// - Callback-intensive code requiring cloneable tests
/// - Performance-sensitive single-threaded code
///
/// # Examples
///
/// ```rust
/// # #[cfg(feature = "combinators")]
/// # {
/// use qubit_function::{RcTester, Tester};
///
/// let shared = RcTester::new(|| true);
///
/// // Clone for multiple uses
/// let clone1 = shared.clone();
/// let clone2 = shared.clone();
///
/// // Non-consuming combination
/// let combined = shared.and(&clone1);
/// # }
/// ```
pub struct RcTester {
    pub(super) function: Rc<dyn Fn() -> bool>,
}

impl RcTester {
    /// Creates a new `RcTester` from a closure
    ///
    /// # Type Parameters
    ///
    /// * `F` - Closure type implementing `Fn() -> bool`
    ///
    /// # Parameters
    ///
    /// * `f` - The closure to wrap
    ///
    /// # Return Value
    ///
    /// A new `RcTester` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::RcTester;
    ///
    /// let tester = RcTester::new(|| true);
    /// ```
    #[inline]
    pub fn new<F>(source: F) -> Self
    where
        F: Tester + 'static,
    {
        RcTester {
            function: Rc::new(move || source.test()),
        }
    }

    /// Combines this tester with another tester using logical AND
    ///
    /// Returns a new `RcTester` that returns `true` only when both tests
    /// pass. Borrows `&self`, so the original tester remains available.
    ///
    /// # Parameters
    ///
    /// * `next` - The tester to combine with
    ///
    /// # Return Value
    ///
    /// A new `RcTester` representing logical AND
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{RcTester, Tester};
    ///
    /// let first = RcTester::new(|| true);
    /// let second = RcTester::new(|| true);
    /// let combined = first.and(&second);
    /// // first and second are still available
    /// ```
    #[inline]
    #[cfg(feature = "combinators")]
    pub fn and(&self, next: &RcTester) -> RcTester {
        let self_fn = Rc::clone(&self.function);
        let next_fn = Rc::clone(&next.function);
        RcTester {
            function: Rc::new(move || self_fn() && next_fn()),
        }
    }

    /// Combines this tester with another tester using logical OR
    ///
    /// Returns a new `RcTester` that returns `true` if either test passes.
    /// Borrows `&self`, so the original tester remains available.
    ///
    /// # Parameters
    ///
    /// * `next` - The tester to combine with
    ///
    /// # Return Value
    ///
    /// A new `RcTester` representing logical OR
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{RcTester, Tester};
    ///
    /// let first = RcTester::new(|| false);
    /// let second = RcTester::new(|| true);
    /// let combined = first.or(&second);
    /// // first and second are still available
    /// ```
    #[inline]
    #[cfg(feature = "combinators")]
    pub fn or(&self, next: &RcTester) -> RcTester {
        let self_fn = Rc::clone(&self.function);
        let next_fn = Rc::clone(&next.function);
        RcTester {
            function: Rc::new(move || self_fn() || next_fn()),
        }
    }

    /// Combines this tester with another tester using logical NAND
    ///
    /// Returns a new `RcTester` that returns `true` unless both tests pass.
    /// Borrows `&self`, so the original tester remains available.
    ///
    /// # Parameters
    ///
    /// * `next` - The tester to combine with
    ///
    /// # Return Value
    ///
    /// A new `RcTester` representing logical NAND
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{RcTester, Tester};
    ///
    /// let first = RcTester::new(|| true);
    /// let second = RcTester::new(|| true);
    /// let nand = first.nand(&second);
    ///
    /// // Both true returns false
    /// assert!(!nand.test());
    ///
    /// // first and second still available
    /// assert!(first.test());
    /// assert!(second.test());
    /// ```
    #[inline]
    #[cfg(feature = "combinators")]
    pub fn nand(&self, next: &RcTester) -> RcTester {
        let self_fn = Rc::clone(&self.function);
        let next_fn = Rc::clone(&next.function);
        RcTester {
            function: Rc::new(move || !(self_fn() && next_fn())),
        }
    }

    /// Combines this tester with another tester using logical XOR
    ///
    /// Returns a new `RcTester` that returns `true` if exactly one test
    /// passes. Borrows `&self`, so the original tester remains available.
    ///
    /// # Parameters
    ///
    /// * `next` - The tester to combine with
    ///
    /// # Return Value
    ///
    /// A new `RcTester` representing logical XOR
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{RcTester, Tester};
    ///
    /// let first = RcTester::new(|| true);
    /// let second = RcTester::new(|| false);
    /// let xor = first.xor(&second);
    ///
    /// // One true one false returns true
    /// assert!(xor.test());
    ///
    /// // first and second still available
    /// assert!(first.test());
    /// assert!(!second.test());
    /// ```
    #[inline]
    #[cfg(feature = "combinators")]
    pub fn xor(&self, next: &RcTester) -> RcTester {
        let self_fn = Rc::clone(&self.function);
        let next_fn = Rc::clone(&next.function);
        RcTester {
            function: Rc::new(move || self_fn() ^ next_fn()),
        }
    }

    /// Combines this tester with another tester using logical NOR
    ///
    /// Returns a new `RcTester` that returns `true` only when both tests
    /// fail. Borrows `&self`, so the original tester remains available.
    ///
    /// # Parameters
    ///
    /// * `next` - The tester to combine with
    ///
    /// # Return Value
    ///
    /// A new `RcTester` representing logical NOR
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{RcTester, Tester};
    ///
    /// let first = RcTester::new(|| false);
    /// let second = RcTester::new(|| false);
    /// let nor = first.nor(&second);
    ///
    /// // Both false returns true
    /// assert!(nor.test());
    ///
    /// // first and second still available
    /// assert!(!first.test());
    /// assert!(!second.test());
    /// ```
    #[inline]
    #[cfg(feature = "combinators")]
    pub fn nor(&self, next: &RcTester) -> RcTester {
        let self_fn = Rc::clone(&self.function);
        let next_fn = Rc::clone(&next.function);
        RcTester {
            function: Rc::new(move || !(self_fn() || next_fn())),
        }
    }
}

#[cfg(feature = "combinators")]
impl Not for RcTester {
    type Output = RcTester;

    #[inline]
    fn not(self) -> Self::Output {
        let func = self.function;
        RcTester {
            function: Rc::new(move || !func()),
        }
    }
}

#[cfg(feature = "combinators")]
impl Not for &RcTester {
    type Output = RcTester;

    #[inline]
    fn not(self) -> Self::Output {
        let func = Rc::clone(&self.function);
        RcTester {
            function: Rc::new(move || !func()),
        }
    }
}

impl Tester for RcTester {
    #[inline]
    fn test(&self) -> bool {
        (self.function)()
    }
}

impl Clone for RcTester {
    /// Creates a clone of this `RcTester`.
    ///
    /// The cloned instance shares the same underlying function with
    /// the original, allowing multiple references to the same test
    /// logic.
    #[inline]
    fn clone(&self) -> Self {
        Self {
            function: Rc::clone(&self.function),
        }
    }
}
