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
//! Defines the `FnPredicateOps` public type.

use super::{
    BoxPredicate,
    Predicate,
};

/// Extension trait providing logical composition methods for closures.
///
/// This trait is automatically implemented for all closures and function
/// pointers that match `Fn(&T) -> bool`, enabling method chaining starting
/// from a closure.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{Predicate, FnPredicateOps};
///
/// let is_positive = |x: &i32| *x > 0;
/// let is_even = |x: &i32| x % 2 == 0;
///
/// // Combine predicates using extension methods
/// let pred = is_positive.and(is_even);
/// assert!(pred.test(&4));
/// assert!(!pred.test(&3));
/// ```
///
pub trait FnPredicateOps<T>: Fn(&T) -> bool + Sized {
    /// Returns a predicate that represents the logical AND of this predicate
    /// and another.
    ///
    /// # Parameters
    ///
    /// * `other` - The other predicate to combine with. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original predicate, clone it first (if it implements
    ///   `Clone`). Can be:
    ///   - Another closure
    ///   - A function pointer
    ///   - A `BoxPredicate<T>`, `RcPredicate<T>`, or `ArcPredicate<T>`
    ///
    /// # Returns
    ///
    /// A `BoxPredicate` representing the logical AND.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Predicate, FnPredicateOps};
    ///
    /// let is_positive = |x: &i32| *x > 0;
    /// let is_even = |x: &i32| x % 2 == 0;
    ///
    /// let combined = is_positive.and(is_even);
    /// assert!(combined.test(&4));
    /// assert!(!combined.test(&3));
    /// ```
    fn and<P>(self, other: P) -> BoxPredicate<T>
    where
        Self: 'static,
        P: Predicate<T> + 'static,
        T: 'static,
    {
        BoxPredicate::new(move |value: &T| self.test(value) && other.test(value))
    }

    /// Returns a predicate that represents the logical OR of this predicate
    /// and another.
    ///
    /// # Parameters
    ///
    /// * `other` - The other predicate to combine with. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original predicate, clone it first (if it implements
    ///   `Clone`). Can be:
    ///   - Another closure
    ///   - A function pointer
    ///   - A `BoxPredicate<T>`, `RcPredicate<T>`, or `ArcPredicate<T>`
    ///   - Any type implementing `Predicate<T>`
    ///
    /// # Returns
    ///
    /// A `BoxPredicate` representing the logical OR.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Predicate, FnPredicateOps};
    ///
    /// let is_negative = |x: &i32| *x < 0;
    /// let is_large = |x: &i32| *x > 100;
    ///
    /// let combined = is_negative.or(is_large);
    /// assert!(combined.test(&-5));
    /// assert!(combined.test(&150));
    /// assert!(!combined.test(&50));
    /// ```
    fn or<P>(self, other: P) -> BoxPredicate<T>
    where
        Self: 'static,
        P: Predicate<T> + 'static,
        T: 'static,
    {
        BoxPredicate::new(move |value: &T| self.test(value) || other.test(value))
    }

    /// Returns a predicate that represents the logical negation of this
    /// predicate.
    ///
    /// # Returns
    ///
    /// A `BoxPredicate` representing the logical negation.
    fn not(self) -> BoxPredicate<T>
    where
        Self: 'static,
        T: 'static,
    {
        BoxPredicate::new(move |value: &T| !self.test(value))
    }

    /// Returns a predicate that represents the logical NAND (NOT AND) of this
    /// predicate and another.
    ///
    /// NAND returns `true` unless both predicates are `true`.
    /// Equivalent to `!(self AND other)`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other predicate to combine with. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original predicate, clone it first (if it implements
    ///   `Clone`). Accepts closures, function pointers, or any
    ///   `Predicate<T>` implementation.
    ///
    /// # Returns
    ///
    /// A `BoxPredicate` representing the logical NAND.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Predicate, FnPredicateOps};
    ///
    /// let is_positive = |x: &i32| *x > 0;
    /// let is_even = |x: &i32| x % 2 == 0;
    ///
    /// let nand = is_positive.nand(is_even);
    /// assert!(nand.test(&3));   // !(true && false) = true
    /// assert!(!nand.test(&4));  // !(true && true) = false
    /// ```
    fn nand<P>(self, other: P) -> BoxPredicate<T>
    where
        Self: 'static,
        P: Predicate<T> + 'static,
        T: 'static,
    {
        BoxPredicate::new(move |value: &T| !(self.test(value) && other.test(value)))
    }

    /// Returns a predicate that represents the logical XOR (exclusive OR) of
    /// this predicate and another.
    ///
    /// XOR returns `true` if exactly one of the predicates is `true`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other predicate to combine with. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original predicate, clone it first (if it implements
    ///   `Clone`). Accepts closures, function pointers, or any
    ///   `Predicate<T>` implementation.
    ///
    /// # Returns
    ///
    /// A `BoxPredicate` representing the logical XOR.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Predicate, FnPredicateOps};
    ///
    /// let is_positive = |x: &i32| *x > 0;
    /// let is_even = |x: &i32| x % 2 == 0;
    ///
    /// let xor = is_positive.xor(is_even);
    /// assert!(xor.test(&3));    // true ^ false = true
    /// assert!(!xor.test(&4));   // true ^ true = false
    /// assert!(!xor.test(&-1));  // false ^ false = false
    /// ```
    fn xor<P>(self, other: P) -> BoxPredicate<T>
    where
        Self: 'static,
        P: Predicate<T> + 'static,
        T: 'static,
    {
        BoxPredicate::new(move |value: &T| self.test(value) ^ other.test(value))
    }

    /// Returns a predicate that represents the logical NOR (NOT OR) of this
    /// predicate and another.
    ///
    /// NOR returns `true` only when both predicates are `false`. Equivalent
    /// to `!(self OR other)`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other predicate to combine with. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original predicate, clone it first (if it implements
    ///   `Clone`). Accepts closures, function pointers, or any
    ///   `Predicate<T>` implementation.
    ///
    /// # Returns
    ///
    /// A `BoxPredicate` representing the logical NOR.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Predicate, FnPredicateOps};
    ///
    /// let is_positive = |x: &i32| *x > 0;
    /// let is_even = |x: &i32| x % 2 == 0;
    ///
    /// let nor = is_positive.nor(is_even);
    /// assert!(nor.test(&-3));   // !(false || false) = true
    /// assert!(!nor.test(&4));   // !(true || true) = false
    /// assert!(!nor.test(&3));   // !(true || false) = false
    /// ```
    fn nor<P>(self, other: P) -> BoxPredicate<T>
    where
        Self: 'static,
        P: Predicate<T> + 'static,
        T: 'static,
    {
        BoxPredicate::new(move |value: &T| !(self.test(value) || other.test(value)))
    }
}

// Blanket implementation for all closures
impl<T, F> FnPredicateOps<T> for F where F: Fn(&T) -> bool {}
