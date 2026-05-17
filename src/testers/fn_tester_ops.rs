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
//! Defines the `FnTesterOps` public type.

use super::{
    BoxTester,
    Tester,
};

// ============================================================================
// Extension Trait for Convenient Closure Conversion
// ============================================================================

/// Extension trait providing logical composition methods for closures
///
/// This trait is automatically implemented for all closures and function
/// pointers that match `Fn() -> bool`, enabling method chaining starting
/// from a closure.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{FnTesterOps, Tester};
///
/// let is_ready = || true;
/// let is_available = || true;
///
/// // Combine testers using extension methods
/// let combined = is_ready.and(is_available);
/// assert!(combined.test());
/// ```
///
pub trait FnTesterOps: Sized + Fn() -> bool {
    /// Returns a tester that represents the logical AND of this tester
    /// and another
    ///
    /// # Parameters
    ///
    /// * `other` - The other tester to combine with. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original tester, clone it first (if it implements
    ///   `Clone`). Can be:
    ///   - Another closure
    ///   - A function pointer
    ///   - A `BoxTester`, `RcTester`, or `ArcTester`
    ///
    /// # Return Value
    ///
    /// A `BoxTester` representing the logical AND
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{FnTesterOps, Tester};
    ///
    /// let is_ready = || true;
    /// let is_available = || true;
    ///
    /// let combined = is_ready.and(is_available);
    /// assert!(combined.test());
    /// ```
    #[inline]
    fn and<T>(self, other: T) -> BoxTester
    where
        Self: 'static,
        T: Tester + 'static,
    {
        BoxTester::new(move || self.test() && other.test())
    }

    /// Returns a tester that represents the logical OR of this tester
    /// and another
    ///
    /// # Parameters
    ///
    /// * `other` - The other tester to combine with. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original tester, clone it first (if it implements
    ///   `Clone`). Can be:
    ///   - Another closure
    ///   - A function pointer
    ///   - A `BoxTester`, `RcTester`, or `ArcTester`
    ///   - Any type implementing `Tester`
    ///
    /// # Return Value
    ///
    /// A `BoxTester` representing the logical OR
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{FnTesterOps, Tester};
    ///
    /// let is_ready = || false;
    /// let is_fallback = || true;
    ///
    /// let combined = is_ready.or(is_fallback);
    /// assert!(combined.test());
    /// ```
    #[inline]
    fn or<T>(self, other: T) -> BoxTester
    where
        Self: 'static,
        T: Tester + 'static,
    {
        BoxTester::new(move || self.test() || other.test())
    }

    /// Returns a tester that represents the logical negation of this tester
    ///
    /// # Return Value
    ///
    /// A `BoxTester` representing the logical negation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{FnTesterOps, Tester};
    ///
    /// let is_ready = || false;
    /// let not_ready = is_ready.not();
    /// assert!(not_ready.test());
    /// ```
    #[inline]
    fn not(self) -> BoxTester
    where
        Self: 'static,
    {
        BoxTester::new(move || !self.test())
    }

    /// Returns a tester that represents the logical NAND (NOT AND) of this
    /// tester and another
    ///
    /// NAND returns `true` unless both testers are `true`.
    /// Equivalent to `!(self AND other)`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other tester to combine with. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original tester, clone it first (if it implements
    ///   `Clone`). Accepts closures, function pointers, or any
    ///   `Tester` implementation.
    ///
    /// # Return Value
    ///
    /// A `BoxTester` representing the logical NAND
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{FnTesterOps, Tester};
    ///
    /// let is_ready = || true;
    /// let is_available = || true;
    ///
    /// let nand = is_ready.nand(is_available);
    /// assert!(!nand.test());  // !(true && true) = false
    /// ```
    #[inline]
    fn nand<T>(self, other: T) -> BoxTester
    where
        Self: 'static,
        T: Tester + 'static,
    {
        BoxTester::new(move || !(self.test() && other.test()))
    }

    /// Returns a tester that represents the logical XOR (exclusive OR) of
    /// this tester and another
    ///
    /// XOR returns `true` if exactly one of the testers is `true`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other tester to combine with. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original tester, clone it first (if it implements
    ///   `Clone`). Accepts closures, function pointers, or any
    ///   `Tester` implementation.
    ///
    /// # Return Value
    ///
    /// A `BoxTester` representing the logical XOR
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{FnTesterOps, Tester};
    ///
    /// let is_ready = || true;
    /// let is_available = || false;
    ///
    /// let xor = is_ready.xor(is_available);
    /// assert!(xor.test());  // true ^ false = true
    /// ```
    #[inline]
    fn xor<T>(self, other: T) -> BoxTester
    where
        Self: 'static,
        T: Tester + 'static,
    {
        BoxTester::new(move || self.test() ^ other.test())
    }

    /// Returns a tester that represents the logical NOR (NOT OR) of this
    /// tester and another
    ///
    /// NOR returns `true` only when both testers are `false`. Equivalent
    /// to `!(self OR other)`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other tester to combine with. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original tester, clone it first (if it implements
    ///   `Clone`). Accepts closures, function pointers, or any
    ///   `Tester` implementation.
    ///
    /// # Return Value
    ///
    /// A `BoxTester` representing the logical NOR
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{FnTesterOps, Tester};
    ///
    /// let is_ready = || false;
    /// let is_available = || false;
    ///
    /// let nor = is_ready.nor(is_available);
    /// assert!(nor.test());  // !(false || false) = true
    /// ```
    #[inline]
    fn nor<T>(self, other: T) -> BoxTester
    where
        Self: 'static,
        T: Tester + 'static,
    {
        BoxTester::new(move || !(self.test() || other.test()))
    }
}

// Blanket implementation for all closures
impl<F> FnTesterOps for F where F: Fn() -> bool {}
