// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
// qubit-style: allow explicit-imports
//! Defines the `FnStatefulPredicateOps` public type.

use super::{BoxStatefulPredicate, StatefulPredicate};

/// Extension trait providing logical composition methods for stateful closures.
///
/// This trait is implemented for closures and function objects matching
/// `FnMut(&T) -> bool`, allowing direct composition into
/// `BoxStatefulPredicate`.
pub trait FnStatefulPredicateOps<T>: FnMut(&T) -> bool + Sized {
    /// Returns a predicate representing logical AND with another predicate.
    ///
    /// This method consumes `self` and evaluates `other` only when this
    /// closure returns `true`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other predicate to combine with.
    ///
    /// # Returns
    ///
    /// A `BoxStatefulPredicate` representing logical AND.
    #[inline]
    fn and<P>(mut self, mut other: P) -> BoxStatefulPredicate<T>
    where
        Self: 'static,
        P: StatefulPredicate<T> + 'static,
        T: 'static,
    {
        BoxStatefulPredicate::new(move |value: &T| self(value) && other.test(value))
    }

    /// Returns a predicate representing logical OR with another predicate.
    ///
    /// This method consumes `self` and evaluates `other` only when this
    /// closure returns `false`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other predicate to combine with.
    ///
    /// # Returns
    ///
    /// A `BoxStatefulPredicate` representing logical OR.
    #[inline]
    fn or<P>(mut self, mut other: P) -> BoxStatefulPredicate<T>
    where
        Self: 'static,
        P: StatefulPredicate<T> + 'static,
        T: 'static,
    {
        BoxStatefulPredicate::new(move |value: &T| self(value) || other.test(value))
    }

    /// Returns a predicate representing logical negation.
    ///
    /// This method consumes `self` and returns a predicate that negates the
    /// closure result.
    ///
    /// # Returns
    ///
    /// A `BoxStatefulPredicate` representing logical negation.
    #[inline]
    fn not(mut self) -> BoxStatefulPredicate<T>
    where
        Self: 'static,
        T: 'static,
    {
        BoxStatefulPredicate::new(move |value: &T| !self(value))
    }

    /// Returns a predicate representing logical NAND with another predicate.
    ///
    /// NAND returns `true` unless both predicates return `true`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other predicate to combine with.
    ///
    /// # Returns
    ///
    /// A `BoxStatefulPredicate` representing logical NAND.
    #[inline]
    fn nand<P>(mut self, mut other: P) -> BoxStatefulPredicate<T>
    where
        Self: 'static,
        P: StatefulPredicate<T> + 'static,
        T: 'static,
    {
        BoxStatefulPredicate::new(move |value: &T| !(self(value) && other.test(value)))
    }

    /// Returns a predicate representing logical XOR with another predicate.
    ///
    /// XOR evaluates both predicates and returns `true` when exactly one
    /// predicate returns `true`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other predicate to combine with.
    ///
    /// # Returns
    ///
    /// A `BoxStatefulPredicate` representing logical XOR.
    #[inline]
    fn xor<P>(mut self, mut other: P) -> BoxStatefulPredicate<T>
    where
        Self: 'static,
        P: StatefulPredicate<T> + 'static,
        T: 'static,
    {
        BoxStatefulPredicate::new(move |value: &T| self(value) ^ other.test(value))
    }

    /// Returns a predicate representing logical NOR with another predicate.
    ///
    /// NOR returns `true` only when both predicates return `false`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other predicate to combine with.
    ///
    /// # Returns
    ///
    /// A `BoxStatefulPredicate` representing logical NOR.
    #[inline]
    fn nor<P>(mut self, mut other: P) -> BoxStatefulPredicate<T>
    where
        Self: 'static,
        P: StatefulPredicate<T> + 'static,
        T: 'static,
    {
        BoxStatefulPredicate::new(move |value: &T| !(self(value) || other.test(value)))
    }
}

impl<T, F> FnStatefulPredicateOps<T> for F where F: FnMut(&T) -> bool {}
