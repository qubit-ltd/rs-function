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
//! Defines the `FnBiPredicateOps` public type.

#![allow(unused_imports)]

use super::*;

/// Extension trait providing logical composition methods for closures.
///
/// This trait is automatically implemented for all closures and
/// function pointers that match `Fn(&T, &U) -> bool`, enabling method
/// chaining starting from a closure.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BiPredicate, FnBiPredicateOps};
///
/// let is_sum_positive = |x: &i32, y: &i32| x + y > 0;
/// let first_larger = |x: &i32, y: &i32| x > y;
///
/// // Combine bi-predicates using extension methods
/// let pred = is_sum_positive.and(first_larger);
/// assert!(pred.test(&10, &5));
/// assert!(!pred.test(&3, &8));
/// ```
///
pub trait FnBiPredicateOps<T, U>: Fn(&T, &U) -> bool + Sized {
    /// Returns a bi-predicate that represents the logical AND of this
    /// bi-predicate and another.
    ///
    /// # Parameters
    ///
    /// * `other` - The other bi-predicate to combine with. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original bi-predicate, clone it first (if it implements
    ///   `Clone`). Can be:
    ///   - Another closure: `|x: &T, y: &U| -> bool`
    ///   - A function pointer: `fn(&T, &U) -> bool`
    ///   - A `BoxBiPredicate<T, U>`
    ///   - An `RcBiPredicate<T, U>`
    ///   - An `ArcBiPredicate<T, U>`
    ///   - Any type implementing `BiPredicate<T, U>`
    ///
    /// # Returns
    ///
    /// A `BoxBiPredicate` representing the logical AND.
    fn and<P>(self, other: P) -> BoxBiPredicate<T, U>
    where
        Self: 'static,
        P: BiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        BoxBiPredicate::new(move |first, second| self(first, second) && other.test(first, second))
    }

    /// Returns a bi-predicate that represents the logical OR of this
    /// bi-predicate and another.
    ///
    /// # Parameters
    ///
    /// * `other` - The other bi-predicate to combine with. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original bi-predicate, clone it first (if it implements
    ///   `Clone`). Can be:
    ///   - Another closure: `|x: &T, y: &U| -> bool`
    ///   - A function pointer: `fn(&T, &U) -> bool`
    ///   - A `BoxBiPredicate<T, U>`
    ///   - An `RcBiPredicate<T, U>`
    ///   - An `ArcBiPredicate<T, U>`
    ///   - Any type implementing `BiPredicate<T, U>`
    ///
    /// # Returns
    ///
    /// A `BoxBiPredicate` representing the logical OR.
    fn or<P>(self, other: P) -> BoxBiPredicate<T, U>
    where
        Self: 'static,
        P: BiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        BoxBiPredicate::new(move |first, second| self(first, second) || other.test(first, second))
    }

    /// Returns a bi-predicate that represents the logical negation of
    /// this bi-predicate.
    ///
    /// # Returns
    ///
    /// A `BoxBiPredicate` representing the logical negation.
    fn not(self) -> BoxBiPredicate<T, U>
    where
        Self: 'static,
        T: 'static,
        U: 'static,
    {
        BoxBiPredicate::new(move |first, second| !self(first, second))
    }

    /// Returns a bi-predicate that represents the logical NAND (NOT
    /// AND) of this bi-predicate and another.
    ///
    /// NAND returns `true` unless both bi-predicates are `true`.
    /// Equivalent to `!(self AND other)`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other bi-predicate to combine with. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original bi-predicate, clone it first (if it implements
    ///   `Clone`). Can be:
    ///   - Another closure: `|x: &T, y: &U| -> bool`
    ///   - A function pointer: `fn(&T, &U) -> bool`
    ///   - A `BoxBiPredicate<T, U>`
    ///   - An `RcBiPredicate<T, U>`
    ///   - An `ArcBiPredicate<T, U>`
    ///   - Any type implementing `BiPredicate<T, U>`
    ///
    /// # Returns
    ///
    /// A `BoxBiPredicate` representing the logical NAND.
    fn nand<P>(self, other: P) -> BoxBiPredicate<T, U>
    where
        Self: 'static,
        P: BiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        BoxBiPredicate::new(move |first, second| {
            !(self(first, second) && other.test(first, second))
        })
    }

    /// Returns a bi-predicate that represents the logical XOR
    /// (exclusive OR) of this bi-predicate and another.
    ///
    /// XOR returns `true` if exactly one of the bi-predicates is
    /// `true`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other bi-predicate to combine with. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original bi-predicate, clone it first (if it implements
    ///   `Clone`). Can be:
    ///   - Another closure: `|x: &T, y: &U| -> bool`
    ///   - A function pointer: `fn(&T, &U) -> bool`
    ///   - A `BoxBiPredicate<T, U>`
    ///   - An `RcBiPredicate<T, U>`
    ///   - An `ArcBiPredicate<T, U>`
    ///   - Any type implementing `BiPredicate<T, U>`
    ///
    /// # Returns
    ///
    /// A `BoxBiPredicate` representing the logical XOR.
    fn xor<P>(self, other: P) -> BoxBiPredicate<T, U>
    where
        Self: 'static,
        P: BiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        BoxBiPredicate::new(move |first, second| self(first, second) ^ other.test(first, second))
    }

    /// Returns a bi-predicate that represents the logical NOR (NOT
    /// OR) of this bi-predicate and another.
    ///
    /// NOR returns `true` only if both bi-predicates are `false`.
    /// Equivalent to `!(self OR other)`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other bi-predicate to combine with. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original bi-predicate, clone it first (if it implements
    ///   `Clone`). Can be:
    ///   - Another closure: `|x: &T, y: &U| -> bool`
    ///   - A function pointer: `fn(&T, &U) -> bool`
    ///   - A `BoxBiPredicate<T, U>`
    ///   - An `RcBiPredicate<T, U>`
    ///   - An `ArcBiPredicate<T, U>`
    ///   - Any type implementing `BiPredicate<T, U>`
    ///
    /// # Returns
    ///
    /// A `BoxBiPredicate` representing the logical NOR.
    fn nor<P>(self, other: P) -> BoxBiPredicate<T, U>
    where
        Self: 'static,
        P: BiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        BoxBiPredicate::new(move |first, second| {
            !(self(first, second) || other.test(first, second))
        })
    }
}

// Blanket implementation for all closures
impl<T, U, F> FnBiPredicateOps<T, U> for F where F: Fn(&T, &U) -> bool {}
