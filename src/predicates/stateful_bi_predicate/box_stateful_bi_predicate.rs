// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
// qubit-style: allow explicit-imports
//! Defines the `BoxStatefulBiPredicate` public type.

use std::ops::Not;

use super::{
    ALWAYS_FALSE_NAME, ALWAYS_TRUE_NAME, StatefulBiPredicate, impl_predicate_common_methods,
    impl_predicate_debug_display,
};

type BoxStatefulBiPredicateFn<T, U> = dyn FnMut(&T, &U) -> bool;

/// A Box-based stateful bi-predicate with single ownership.
///
/// This type stores a `Box<dyn FnMut(&T, &U) -> bool>`, so each call may
/// update the predicate's internal state. Composition methods consume `self`,
/// matching the single-ownership model.
pub struct BoxStatefulBiPredicate<T, U> {
    pub(super) function: Box<BoxStatefulBiPredicateFn<T, U>>,
    pub(super) name: Option<String>,
}

impl<T, U> BoxStatefulBiPredicate<T, U> {
    // Generates: new(), new_with_name(), name(), set_name(), always_true(),
    // always_false()
    impl_predicate_common_methods!(
        BoxStatefulBiPredicate<T, U>,
        (FnMut(&T, &U) -> bool + 'static),
        |f| Box::new(f)
    );

    /// Returns a bi-predicate representing logical AND with another predicate.
    ///
    /// This method consumes `self` and evaluates `other` only when this
    /// predicate returns `true`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other bi-predicate to combine with.
    ///
    /// # Returns
    ///
    /// A new `BoxStatefulBiPredicate` representing logical AND.
    #[inline]
    pub fn and<P>(mut self, mut other: P) -> BoxStatefulBiPredicate<T, U>
    where
        P: StatefulBiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        BoxStatefulBiPredicate::new(move |first: &T, second: &U| {
            self.test(first, second) && other.test(first, second)
        })
    }

    /// Returns a bi-predicate representing logical OR with another predicate.
    ///
    /// This method consumes `self` and evaluates `other` only when this
    /// predicate returns `false`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other bi-predicate to combine with.
    ///
    /// # Returns
    ///
    /// A new `BoxStatefulBiPredicate` representing logical OR.
    #[inline]
    pub fn or<P>(mut self, mut other: P) -> BoxStatefulBiPredicate<T, U>
    where
        P: StatefulBiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        BoxStatefulBiPredicate::new(move |first: &T, second: &U| {
            self.test(first, second) || other.test(first, second)
        })
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
    /// A new `BoxStatefulBiPredicate` representing logical NAND.
    #[inline]
    pub fn nand<P>(mut self, mut other: P) -> BoxStatefulBiPredicate<T, U>
    where
        P: StatefulBiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        BoxStatefulBiPredicate::new(move |first: &T, second: &U| {
            !(self.test(first, second) && other.test(first, second))
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
    /// A new `BoxStatefulBiPredicate` representing logical XOR.
    #[inline]
    pub fn xor<P>(mut self, mut other: P) -> BoxStatefulBiPredicate<T, U>
    where
        P: StatefulBiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        BoxStatefulBiPredicate::new(move |first: &T, second: &U| {
            self.test(first, second) ^ other.test(first, second)
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
    /// A new `BoxStatefulBiPredicate` representing logical NOR.
    #[inline]
    pub fn nor<P>(mut self, mut other: P) -> BoxStatefulBiPredicate<T, U>
    where
        P: StatefulBiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        BoxStatefulBiPredicate::new(move |first: &T, second: &U| {
            !(self.test(first, second) || other.test(first, second))
        })
    }
}

impl<T, U> Not for BoxStatefulBiPredicate<T, U>
where
    T: 'static,
    U: 'static,
{
    type Output = BoxStatefulBiPredicate<T, U>;

    fn not(mut self) -> Self::Output {
        BoxStatefulBiPredicate::new(move |first: &T, second: &U| !self.test(first, second))
    }
}

// Generates: impl Debug for BoxStatefulBiPredicate<T, U> and impl Display for
// BoxStatefulBiPredicate<T, U>
impl_predicate_debug_display!(BoxStatefulBiPredicate<T, U>);

impl<T, U> StatefulBiPredicate<T, U> for BoxStatefulBiPredicate<T, U> {
    fn test(&mut self, first: &T, second: &U) -> bool {
        (self.function)(first, second)
    }
}
