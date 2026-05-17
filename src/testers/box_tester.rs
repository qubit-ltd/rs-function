/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
// qubit-style: allow explicit-imports
//! Defines the `BoxTester` public type.

use std::ops::Not;

use super::{
    Rc,
    RcTester,
    Tester,
};

// ============================================================================
// BoxTester: Single Ownership Implementation
// ============================================================================

/// Single ownership Tester implemented using `Box`
///
/// `BoxTester` wraps a closure in `Box<dyn Fn() -> bool>`, providing single
/// ownership semantics with no additional allocation overhead beyond the
/// initial boxing.
///
/// # Characteristics
///
/// - **Single ownership**: Cannot be cloned
/// - **Zero overhead**: Single heap allocation
/// - **Consuming combination**: `and()`/`or()` consume `self`
/// - **Type flexibility**: Accepts any `Tester` implementation
///
/// # Use Cases
///
/// - One-time testing scenarios
/// - Builder patterns requiring ownership transfer
/// - Simple state checking without sharing
/// - Chained calls with ownership transfer
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BoxTester, Tester};
/// use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
///
/// // State managed externally
/// let count = Arc::new(AtomicUsize::new(0));
/// let count_clone = Arc::clone(&count);
///
/// let tester = BoxTester::new(move || {
///     count_clone.load(Ordering::Relaxed) < 3
/// });
///
/// assert!(tester.test());
/// count.fetch_add(1, Ordering::Relaxed);
/// assert!(tester.test());
/// count.fetch_add(1, Ordering::Relaxed);
/// assert!(tester.test());
/// count.fetch_add(1, Ordering::Relaxed);
/// assert!(!tester.test());
///
/// // Logical combination
/// let combined = BoxTester::new(|| true)
///     .and(|| false)
///     .or(|| true);
/// assert!(combined.test());
/// ```
///
pub struct BoxTester {
    pub(super) function: Box<dyn Fn() -> bool>,
}

impl BoxTester {
    /// Creates a new `BoxTester` from a closure
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
    /// A new `BoxTester` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::BoxTester;
    ///
    /// let tester = BoxTester::new(|| true);
    /// ```
    #[inline]
    pub fn new<F>(f: F) -> Self
    where
        F: Fn() -> bool + 'static,
    {
        BoxTester {
            function: Box::new(f),
        }
    }

    /// Combines this tester with another tester using logical AND
    ///
    /// Returns a new `BoxTester` that returns `true` only when both tests
    /// pass. Short-circuit evaluation: if the first test fails, the second
    /// will not be executed.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Type implementing `Tester`
    ///
    /// # Parameters
    ///
    /// * `next` - The tester to combine with
    ///
    /// # Return Value
    ///
    /// A new `BoxTester` representing logical AND
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BoxTester, Tester};
    /// use std::sync::{Arc, atomic::{AtomicUsize, AtomicBool, Ordering}};
    ///
    /// // Simulate service status
    /// let request_count = Arc::new(AtomicUsize::new(0));
    /// let is_available = Arc::new(AtomicBool::new(true));
    /// let max_requests = 1000;
    ///
    /// let count_clone = Arc::clone(&request_count);
    /// let available_clone = Arc::clone(&is_available);
    ///
    /// // Service available and request count not exceeded
    /// let service_ok = BoxTester::new(move || {
    ///     available_clone.load(Ordering::Relaxed)
    /// })
    /// .and(move || {
    ///     count_clone.load(Ordering::Relaxed) < max_requests
    /// });
    ///
    /// // Initial state: available and request count 0
    /// assert!(service_ok.test());
    ///
    /// // Simulate request increase
    /// request_count.store(500, Ordering::Relaxed);
    /// assert!(service_ok.test());
    ///
    /// // Request count exceeded
    /// request_count.store(1500, Ordering::Relaxed);
    /// assert!(!service_ok.test());
    ///
    /// // Service unavailable
    /// is_available.store(false, Ordering::Relaxed);
    /// assert!(!service_ok.test());
    /// ```
    #[inline]
    pub fn and<T>(self, next: T) -> BoxTester
    where
        T: Tester + 'static,
    {
        let self_fn = self.function;
        let next_tester = next;
        BoxTester::new(move || self_fn() && next_tester.test())
    }

    /// Combines this tester with another tester using logical OR
    ///
    /// Returns a new `BoxTester` that returns `true` if either test passes.
    /// Short-circuit evaluation: if the first test passes, the second will
    /// not be executed.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Type implementing `Tester`
    ///
    /// # Parameters
    ///
    /// * `next` - The tester to combine with
    ///
    /// # Return Value
    ///
    /// A new `BoxTester` representing logical OR
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BoxTester, Tester};
    /// use std::sync::{Arc, atomic::{AtomicUsize, AtomicBool, Ordering}};
    ///
    /// // Simulate service status
    /// let request_count = Arc::new(AtomicUsize::new(0));
    /// let is_healthy = Arc::new(AtomicBool::new(true));
    /// let max_requests = 100;
    ///
    /// let count_clone = Arc::clone(&request_count);
    /// let health_clone = Arc::clone(&is_healthy);
    ///
    /// // Service healthy or low request count
    /// let can_serve = BoxTester::new(move || {
    ///     health_clone.load(Ordering::Relaxed)
    /// })
    /// .or(move || {
    ///     count_clone.load(Ordering::Relaxed) < max_requests
    /// });
    ///
    /// // Initial state: healthy and request count 0
    /// assert!(can_serve.test());
    ///
    /// // Request count increased but within limit
    /// request_count.store(50, Ordering::Relaxed);
    /// assert!(can_serve.test());
    ///
    /// // Request count exceeded but service healthy
    /// request_count.store(150, Ordering::Relaxed);
    /// assert!(can_serve.test()); // still healthy
    ///
    /// // Service unhealthy but low request count
    /// is_healthy.store(false, Ordering::Relaxed);
    /// request_count.store(50, Ordering::Relaxed);
    /// assert!(can_serve.test()); // low request count
    ///
    /// // Unhealthy and high request count
    /// request_count.store(150, Ordering::Relaxed);
    /// assert!(!can_serve.test());
    /// ```
    #[inline]
    pub fn or<T>(self, next: T) -> BoxTester
    where
        T: Tester + 'static,
    {
        let self_fn = self.function;
        let next_tester = next;
        BoxTester::new(move || self_fn() || next_tester.test())
    }

    /// Combines this tester with another tester using logical NAND
    ///
    /// Returns a new `BoxTester` that returns `true` unless both tests pass.
    /// Equivalent to `!(self AND other)`.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Type implementing `Tester`
    ///
    /// # Parameters
    ///
    /// * `next` - The tester to combine with
    ///
    /// # Return Value
    ///
    /// A new `BoxTester` representing logical NAND
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BoxTester, Tester};
    /// use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
    ///
    /// let flag1 = Arc::new(AtomicBool::new(true));
    /// let flag2 = Arc::new(AtomicBool::new(true));
    ///
    /// let flag1_clone = Arc::clone(&flag1);
    /// let flag2_clone = Arc::clone(&flag2);
    ///
    /// let nand = BoxTester::new(move || {
    ///     flag1_clone.load(Ordering::Relaxed)
    /// })
    /// .nand(move || {
    ///     flag2_clone.load(Ordering::Relaxed)
    /// });
    ///
    /// // Both true returns false
    /// assert!(!nand.test());
    ///
    /// // At least one false returns true
    /// flag1.store(false, Ordering::Relaxed);
    /// assert!(nand.test());
    /// ```
    #[inline]
    pub fn nand<T>(self, next: T) -> BoxTester
    where
        T: Tester + 'static,
    {
        let self_fn = self.function;
        let next_tester = next;
        BoxTester::new(move || !(self_fn() && next_tester.test()))
    }

    /// Combines this tester with another tester using logical XOR
    ///
    /// Returns a new `BoxTester` that returns `true` if exactly one test
    /// passes.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Type implementing `Tester`
    ///
    /// # Parameters
    ///
    /// * `next` - The tester to combine with
    ///
    /// # Return Value
    ///
    /// A new `BoxTester` representing logical XOR
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BoxTester, Tester};
    /// use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
    ///
    /// let flag1 = Arc::new(AtomicBool::new(true));
    /// let flag2 = Arc::new(AtomicBool::new(false));
    ///
    /// let flag1_clone1 = Arc::clone(&flag1);
    /// let flag2_clone1 = Arc::clone(&flag2);
    ///
    /// let xor = BoxTester::new(move || {
    ///     flag1_clone1.load(Ordering::Relaxed)
    /// })
    /// .xor(move || {
    ///     flag2_clone1.load(Ordering::Relaxed)
    /// });
    ///
    /// // One true one false returns true
    /// assert!(xor.test());
    ///
    /// // Both true returns false
    /// flag2.store(true, Ordering::Relaxed);
    /// assert!(!xor.test());
    ///
    /// // Both false returns false
    /// flag1.store(false, Ordering::Relaxed);
    /// flag2.store(false, Ordering::Relaxed);
    /// assert!(!xor.test());
    /// ```
    #[inline]
    pub fn xor<T>(self, next: T) -> BoxTester
    where
        T: Tester + 'static,
    {
        let self_fn = self.function;
        let next_tester = next;
        BoxTester::new(move || self_fn() ^ next_tester.test())
    }

    /// Combines this tester with another tester using logical NOR
    ///
    /// Returns a new `BoxTester` that returns `true` only when both tests
    /// fail. Equivalent to `!(self OR other)`.
    ///
    /// # Type Parameters
    ///
    /// * `T` - Type implementing `Tester`
    ///
    /// # Parameters
    ///
    /// * `next` - The tester to combine with
    ///
    /// # Return Value
    ///
    /// A new `BoxTester` representing logical NOR
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BoxTester, Tester};
    /// use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
    ///
    /// let flag1 = Arc::new(AtomicBool::new(false));
    /// let flag2 = Arc::new(AtomicBool::new(false));
    ///
    /// let flag1_clone = Arc::clone(&flag1);
    /// let flag2_clone = Arc::clone(&flag2);
    ///
    /// let nor = BoxTester::new(move || {
    ///     flag1_clone.load(Ordering::Relaxed)
    /// })
    /// .nor(move || {
    ///     flag2_clone.load(Ordering::Relaxed)
    /// });
    ///
    /// // Both false returns true
    /// assert!(nor.test());
    ///
    /// // At least one true returns false
    /// flag1.store(true, Ordering::Relaxed);
    /// assert!(!nor.test());
    /// ```
    #[inline]
    pub fn nor<T>(self, next: T) -> BoxTester
    where
        T: Tester + 'static,
    {
        let self_fn = self.function;
        let next_tester = next;
        BoxTester::new(move || !(self_fn() || next_tester.test()))
    }
}

impl Not for BoxTester {
    type Output = BoxTester;

    #[inline]
    fn not(self) -> Self::Output {
        let self_fn = self.function;
        BoxTester::new(move || !self_fn())
    }
}

impl Tester for BoxTester {
    #[inline]
    fn test(&self) -> bool {
        (self.function)()
    }

    #[inline]
    fn into_box(self) -> BoxTester {
        self
    }

    #[inline]
    fn into_rc(self) -> RcTester {
        let func = self.function;
        RcTester {
            function: Rc::new(func),
        }
    }

    // Note: BoxTester does not implement Send + Sync, so into_arc()
    // cannot be implemented. Calling into_arc() on BoxTester will result
    // in a compile error due to the Send + Sync trait bounds not being
    // satisfied. The default Tester trait implementation will be used.

    #[inline]
    fn into_fn(self) -> impl Fn() -> bool {
        self.function
    }

    // Note: BoxTester does not implement Clone, so to_box(), to_rc(),
    // to_arc(), and to_fn() cannot be implemented. Calling these methods
    // on BoxTester will result in a compile error due to the Clone trait
    // bound not being satisfied. The default Tester trait implementations
    // will be used.
}
