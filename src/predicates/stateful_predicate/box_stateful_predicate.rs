// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
// qubit-style: allow explicit-imports
//! Defines the `BoxStatefulPredicate` public type.

#[cfg(feature = "combinators")]
use std::ops::Not;

use super::{
    ALWAYS_FALSE_NAME,
    ALWAYS_TRUE_NAME,
    StatefulPredicate,
    impl_predicate_common_methods,
    impl_predicate_debug_display,
};

/// A Box-based stateful predicate with single ownership.
///
/// This type stores a `Box<dyn FnMut(&T) -> bool>`, so each call may update
/// the predicate's internal state. Composition methods consume `self`,
/// matching the single-ownership model.
pub struct BoxStatefulPredicate<T> {
    pub(super) function: Box<dyn FnMut(&T) -> bool>,
    pub(super) name: Option<String>,
}

impl<T> BoxStatefulPredicate<T> {
    // Generates: new(), new_with_name(), name(), set_name(), always_true(),
    // always_false()
    impl_predicate_common_methods!(
        BoxStatefulPredicate<T>,
        (FnMut(&T) -> bool + 'static),
        |f| Box::new(f)
    );

    /// Returns a predicate representing logical AND with another predicate.
    ///
    /// This method consumes `self` and evaluates `other` only when this
    /// predicate returns `true`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other predicate to combine with.
    ///
    /// # Returns
    ///
    /// A new `BoxStatefulPredicate` representing logical AND.
    #[cfg(feature = "combinators")]
    #[inline]
    pub fn and<P>(mut self, mut other: P) -> BoxStatefulPredicate<T>
    where
        P: StatefulPredicate<T> + 'static,
        T: 'static,
    {
        BoxStatefulPredicate::new(move |value: &T| {
            self.test(value) && other.test(value)
        })
    }

    /// Returns a predicate representing logical OR with another predicate.
    ///
    /// This method consumes `self` and evaluates `other` only when this
    /// predicate returns `false`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other predicate to combine with.
    ///
    /// # Returns
    ///
    /// A new `BoxStatefulPredicate` representing logical OR.
    #[cfg(feature = "combinators")]
    #[inline]
    pub fn or<P>(mut self, mut other: P) -> BoxStatefulPredicate<T>
    where
        P: StatefulPredicate<T> + 'static,
        T: 'static,
    {
        BoxStatefulPredicate::new(move |value: &T| {
            self.test(value) || other.test(value)
        })
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
    /// A new `BoxStatefulPredicate` representing logical NAND.
    #[cfg(feature = "combinators")]
    #[inline]
    pub fn nand<P>(mut self, mut other: P) -> BoxStatefulPredicate<T>
    where
        P: StatefulPredicate<T> + 'static,
        T: 'static,
    {
        BoxStatefulPredicate::new(move |value: &T| {
            !(self.test(value) && other.test(value))
        })
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
    /// A new `BoxStatefulPredicate` representing logical XOR.
    #[cfg(feature = "combinators")]
    #[inline]
    pub fn xor<P>(mut self, mut other: P) -> BoxStatefulPredicate<T>
    where
        P: StatefulPredicate<T> + 'static,
        T: 'static,
    {
        BoxStatefulPredicate::new(move |value: &T| {
            self.test(value) ^ other.test(value)
        })
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
    /// A new `BoxStatefulPredicate` representing logical NOR.
    #[cfg(feature = "combinators")]
    #[inline]
    pub fn nor<P>(mut self, mut other: P) -> BoxStatefulPredicate<T>
    where
        P: StatefulPredicate<T> + 'static,
        T: 'static,
    {
        BoxStatefulPredicate::new(move |value: &T| {
            !(self.test(value) || other.test(value))
        })
    }
}

#[cfg(feature = "combinators")]
impl<T> Not for BoxStatefulPredicate<T>
where
    T: 'static,
{
    type Output = BoxStatefulPredicate<T>;

    fn not(mut self) -> Self::Output {
        BoxStatefulPredicate::new(move |value: &T| !self.test(value))
    }
}

// Generates: impl Debug for BoxStatefulPredicate<T> and impl Display for
// BoxStatefulPredicate<T>
impl_predicate_debug_display!(BoxStatefulPredicate<T>);

// Implements StatefulPredicate trait for BoxStatefulPredicate<T>
impl<T> StatefulPredicate<T> for BoxStatefulPredicate<T> {
    fn test(&mut self, value: &T) -> bool {
        (self.function)(value)
    }
}
