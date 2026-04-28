/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `RcTester` public type.

#![allow(unused_imports)]

use super::*;

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
/// - **Borrowing combination**: `and()`/`or()`/`not()` borrow `&self`
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
/// ```
///
/// # Author
///
/// Haixing Hu
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
    pub fn new<F>(f: F) -> Self
    where
        F: Fn() -> bool + 'static,
    {
        RcTester {
            function: Rc::new(f),
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
    pub fn or(&self, next: &RcTester) -> RcTester {
        let self_fn = Rc::clone(&self.function);
        let next_fn = Rc::clone(&next.function);
        RcTester {
            function: Rc::new(move || self_fn() || next_fn()),
        }
    }

    /// Negates the result of this tester
    ///
    /// Returns a new `RcTester` that returns the opposite value of the
    /// original test result. Borrows `&self`, so the original tester remains
    /// available.
    ///
    /// # Return Value
    ///
    /// A new `RcTester` representing logical NOT
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{RcTester, Tester};
    ///
    /// let original = RcTester::new(|| true);
    /// let negated = original.not();
    /// // original is still available
    /// ```
    #[allow(clippy::should_implement_trait)]
    #[inline]
    pub fn not(&self) -> RcTester {
        let self_fn = Rc::clone(&self.function);
        RcTester {
            function: Rc::new(move || !self_fn()),
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
    pub fn nor(&self, next: &RcTester) -> RcTester {
        let self_fn = Rc::clone(&self.function);
        let next_fn = Rc::clone(&next.function);
        RcTester {
            function: Rc::new(move || !(self_fn() || next_fn())),
        }
    }
}

impl Tester for RcTester {
    #[inline]
    fn test(&self) -> bool {
        (self.function)()
    }

    #[inline]
    fn into_box(self) -> BoxTester {
        BoxTester {
            function: Box::new(move || (self.function)()),
        }
    }

    #[inline]
    fn into_rc(self) -> RcTester {
        self
    }

    // Note: RcTester is not Send + Sync, so into_arc() cannot be
    // implemented. Calling into_arc() on RcTester will result in a
    // compile error due to the Send + Sync trait bounds not being
    // satisfied. The default Tester trait implementation will be used.

    #[inline]
    fn into_fn(self) -> impl Fn() -> bool {
        move || (self.function)()
    }

    #[inline]
    fn to_box(&self) -> BoxTester {
        let self_fn = self.function.clone();
        BoxTester {
            function: Box::new(move || self_fn()),
        }
    }

    #[inline]
    fn to_rc(&self) -> RcTester {
        self.clone()
    }

    // Note: RcTester is not Send + Sync, so to_arc() cannot be
    // implemented. Calling to_arc() on RcTester will result in a compile
    // error due to the Send + Sync trait bounds not being satisfied. The
    // default Tester trait implementation will be used.

    #[inline]
    fn to_fn(&self) -> impl Fn() -> bool {
        let self_fn = self.function.clone();
        move || self_fn()
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

// ============================================================================
// Tester Implementation for Closures
// ============================================================================

impl<F> Tester for F
where
    F: Fn() -> bool,
{
    #[inline]
    fn test(&self) -> bool {
        self()
    }

    #[inline]
    fn into_box(self) -> BoxTester
    where
        Self: Sized + 'static,
    {
        BoxTester::new(self)
    }

    #[inline]
    fn into_rc(self) -> RcTester
    where
        Self: Sized + 'static,
    {
        RcTester::new(self)
    }

    #[inline]
    fn into_arc(self) -> ArcTester
    where
        Self: Sized + Send + Sync + 'static,
    {
        ArcTester::new(self)
    }

    #[inline]
    fn into_fn(self) -> impl Fn() -> bool
    where
        Self: Sized + 'static,
    {
        self
    }

    #[inline]
    fn to_box(&self) -> BoxTester
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_box()
    }

    #[inline]
    fn to_rc(&self) -> RcTester
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_rc()
    }

    #[inline]
    fn to_arc(&self) -> ArcTester
    where
        Self: Clone + Sized + Send + Sync + 'static,
    {
        self.clone().into_arc()
    }

    #[inline]
    fn to_fn(&self) -> impl Fn() -> bool
    where
        Self: Clone + Sized,
    {
        self.clone()
    }
}
