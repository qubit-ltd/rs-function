// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcTester` public type.

use std::fmt;
use std::ops::Not;
use std::rc::Rc;

use crate::Tester;
use crate::internal::CallbackMetadata;
use crate::macros::impl_common_name_methods;
use crate::macros::impl_common_new_methods;

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
/// - **Borrowing combination**: `and()`/`or()` borrow `&self`
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
/// let combined = shared.and(clone1.clone());
/// # }
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct RcTester {
    /// The wrapped callback implementation.
    pub(super) function: Rc<dyn Fn() -> bool>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: CallbackMetadata,
}

impl RcTester {
    impl_common_new_methods!(
        semantic(Tester + 'static),
        |source| move || source.test(),
        |function| Rc::new(function),
        "tester"
    );

    impl_common_name_methods!("tester");

    /// Combines this tester with another tester using logical AND
    ///
    /// Returns a new `RcTester` that returns `true` only when both tests
    /// pass. Borrows `&self`, so the original tester remains available, and
    /// moves `next` into the result.
    ///
    /// # Parameters
    ///
    /// * `next` - The tester to combine with
    ///
    /// # Returns
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
    /// let combined = first.and(second.clone());
    /// // first and second are still available
    /// ```
    #[inline]
    pub fn and<T>(&self, next: T) -> RcTester
    where
        T: Tester + 'static,
    {
        let self_fn = Rc::clone(&self.function);
        RcTester::new(move || self_fn() && next.test())
    }

    /// Combines this tester with another tester using logical OR
    ///
    /// Returns a new `RcTester` that returns `true` if either test passes.
    /// Borrows `&self`, so the original tester remains available, and moves
    /// `next` into the result.
    ///
    /// # Parameters
    ///
    /// * `next` - The tester to combine with
    ///
    /// # Returns
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
    /// let combined = first.or(second.clone());
    /// // first and second are still available
    /// ```
    #[inline]
    pub fn or<T>(&self, next: T) -> RcTester
    where
        T: Tester + 'static,
    {
        let self_fn = Rc::clone(&self.function);
        RcTester::new(move || self_fn() || next.test())
    }

    /// Combines this tester with another tester using logical NAND
    ///
    /// Returns a new `RcTester` that returns `true` unless both tests pass.
    /// Borrows `&self`, so the original tester remains available, and moves
    /// `next` into the result.
    ///
    /// # Parameters
    ///
    /// * `next` - The tester to combine with
    ///
    /// # Returns
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
    /// let nand = first.nand(second.clone());
    ///
    /// // Both true returns false
    /// assert!(!nand.test());
    ///
    /// // first and second still available
    /// assert!(first.test());
    /// assert!(second.test());
    /// ```
    #[inline]
    pub fn nand<T>(&self, next: T) -> RcTester
    where
        T: Tester + 'static,
    {
        let self_fn = Rc::clone(&self.function);
        RcTester::new(move || !(self_fn() && next.test()))
    }

    /// Combines this tester with another tester using logical XOR
    ///
    /// Returns a new `RcTester` that returns `true` if exactly one test
    /// passes. Borrows `&self`, so the original tester remains available, and
    /// moves `next` into the result.
    ///
    /// # Parameters
    ///
    /// * `next` - The tester to combine with
    ///
    /// # Returns
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
    /// let xor = first.xor(second.clone());
    ///
    /// // One true one false returns true
    /// assert!(xor.test());
    ///
    /// // first and second still available
    /// assert!(first.test());
    /// assert!(!second.test());
    /// ```
    #[inline]
    pub fn xor<T>(&self, next: T) -> RcTester
    where
        T: Tester + 'static,
    {
        let self_fn = Rc::clone(&self.function);
        RcTester::new(move || self_fn() ^ next.test())
    }

    /// Combines this tester with another tester using logical NOR
    ///
    /// Returns a new `RcTester` that returns `true` only when both tests
    /// fail. Borrows `&self`, so the original tester remains available, and
    /// moves `next` into the result.
    ///
    /// # Parameters
    ///
    /// * `next` - The tester to combine with
    ///
    /// # Returns
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
    /// let nor = first.nor(second.clone());
    ///
    /// // Both false returns true
    /// assert!(nor.test());
    ///
    /// // first and second still available
    /// assert!(!first.test());
    /// assert!(!second.test());
    /// ```
    #[inline]
    pub fn nor<T>(&self, next: T) -> RcTester
    where
        T: Tester + 'static,
    {
        let self_fn = Rc::clone(&self.function);
        RcTester::new(move || !(self_fn() || next.test()))
    }
}

impl Not for RcTester {
    type Output = RcTester;

    #[inline]
    fn not(self) -> Self::Output {
        let metadata = self.metadata;
        let func = self.function;
        RcTester::new_with_metadata(move || !func(), metadata)
    }
}

impl Not for &RcTester {
    type Output = RcTester;

    #[inline]
    fn not(self) -> Self::Output {
        let func = Rc::clone(&self.function);
        RcTester::new_with_metadata(move || !func(), self.metadata.clone())
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
            metadata: self.metadata.clone(),
        }
    }
}

impl fmt::Debug for RcTester {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RcTester")
            .field("name", &self.metadata.name())
            .finish()
    }
}

impl fmt::Display for RcTester {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.metadata.name() {
            Some(name) => write!(formatter, "RcTester({name})"),
            None => formatter.write_str("RcTester"),
        }
    }
}
