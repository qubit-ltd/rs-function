// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
// qubit-style: allow explicit-imports
//! Defines the `FnStatefulBiPredicateOps` public type.

use super::{BoxStatefulBiPredicate, StatefulBiPredicate};

/// Extension trait providing logical composition methods for stateful closures.
///
/// This trait is implemented for closures and function objects matching
/// `FnMut(&T, &U) -> bool`, allowing direct composition into
/// `BoxStatefulBiPredicate`.
pub trait FnStatefulBiPredicateOps<T, U>: FnMut(&T, &U) -> bool + Sized {
    /// Returns a bi-predicate representing logical AND with another predicate.
    ///
    /// This method consumes `self` and evaluates `other` only when this
    /// closure returns `true`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other bi-predicate to combine with.
    ///
    /// # Returns
    ///
    /// A `BoxStatefulBiPredicate` representing logical AND.
    #[inline]
    fn and<P>(mut self, mut other: P) -> BoxStatefulBiPredicate<T, U>
    where
        Self: 'static,
        P: StatefulBiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        BoxStatefulBiPredicate::new(move |first: &T, second: &U| {
            self(first, second) && other.test(first, second)
        })
    }

    /// Returns a bi-predicate representing logical OR with another predicate.
    ///
    /// This method consumes `self` and evaluates `other` only when this
    /// closure returns `false`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other bi-predicate to combine with.
    ///
    /// # Returns
    ///
    /// A `BoxStatefulBiPredicate` representing logical OR.
    #[inline]
    fn or<P>(mut self, mut other: P) -> BoxStatefulBiPredicate<T, U>
    where
        Self: 'static,
        P: StatefulBiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        BoxStatefulBiPredicate::new(move |first: &T, second: &U| {
            self(first, second) || other.test(first, second)
        })
    }

    /// Returns a bi-predicate representing logical negation.
    ///
    /// This method consumes `self` and returns a predicate that negates the
    /// closure result.
    ///
    /// # Returns
    ///
    /// A `BoxStatefulBiPredicate` representing logical negation.
    #[inline]
    fn not(mut self) -> BoxStatefulBiPredicate<T, U>
    where
        Self: 'static,
        T: 'static,
        U: 'static,
    {
        BoxStatefulBiPredicate::new(move |first: &T, second: &U| !self(first, second))
    }

    /// Returns a bi-predicate representing logical NAND with another predicate.
    ///
    /// NAND returns `true` unless both predicates return `true`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other bi-predicate to combine with.
    ///
    /// # Returns
    ///
    /// A `BoxStatefulBiPredicate` representing logical NAND.
    #[inline]
    fn nand<P>(mut self, mut other: P) -> BoxStatefulBiPredicate<T, U>
    where
        Self: 'static,
        P: StatefulBiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        BoxStatefulBiPredicate::new(move |first: &T, second: &U| {
            !(self(first, second) && other.test(first, second))
        })
    }

    /// Returns a bi-predicate representing logical XOR with another predicate.
    ///
    /// XOR evaluates both predicates and returns `true` when exactly one
    /// predicate returns `true`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other bi-predicate to combine with.
    ///
    /// # Returns
    ///
    /// A `BoxStatefulBiPredicate` representing logical XOR.
    #[inline]
    fn xor<P>(mut self, mut other: P) -> BoxStatefulBiPredicate<T, U>
    where
        Self: 'static,
        P: StatefulBiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        BoxStatefulBiPredicate::new(move |first: &T, second: &U| {
            self(first, second) ^ other.test(first, second)
        })
    }

    /// Returns a bi-predicate representing logical NOR with another predicate.
    ///
    /// NOR returns `true` only when both predicates return `false`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other bi-predicate to combine with.
    ///
    /// # Returns
    ///
    /// A `BoxStatefulBiPredicate` representing logical NOR.
    #[inline]
    fn nor<P>(mut self, mut other: P) -> BoxStatefulBiPredicate<T, U>
    where
        Self: 'static,
        P: StatefulBiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        BoxStatefulBiPredicate::new(move |first: &T, second: &U| {
            !(self(first, second) || other.test(first, second))
        })
    }
}

impl<T, U, F> FnStatefulBiPredicateOps<T, U> for F where F: FnMut(&T, &U) -> bool {}
