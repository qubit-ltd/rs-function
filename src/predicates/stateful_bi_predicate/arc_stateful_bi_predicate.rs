// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
// qubit-style: allow explicit-imports
//! Defines the `ArcStatefulBiPredicate` public type.

#[cfg(feature = "combinators")]
use std::ops::Not;

use super::{
    ALWAYS_FALSE_NAME,
    ALWAYS_TRUE_NAME,
    Arc,
    Mutex,
    StatefulBiPredicate,
    impl_closure_trait,
    impl_predicate_clone,
    impl_predicate_common_methods,
    impl_predicate_debug_display,
};

type ArcStatefulBiPredicateFn<T, U> =
    Arc<Mutex<dyn FnMut(&T, &U) -> bool + Send + 'static>>;

/// An Arc-based stateful bi-predicate with thread-safe shared ownership.
///
/// This type stores the predicate closure inside `Arc<Mutex<_>>`, allowing
/// cheap clones that share mutable predicate state across threads.
///
/// # Locking and reentrancy
///
/// Each call acquires a `parking_lot::Mutex` and holds it while the user
/// callback runs. Synchronous re-entry through the same shared state
/// deadlocks. The mutex is not poisoned after a panic, and mutations completed
/// before a panic are not rolled back.
pub struct ArcStatefulBiPredicate<T, U> {
    pub(super) function: ArcStatefulBiPredicateFn<T, U>,
    pub(super) name: Option<String>,
}

impl<T, U> ArcStatefulBiPredicate<T, U> {
    // Generates: new(), new_with_name(), name(), set_name(), always_true(),
    // always_false()
    impl_predicate_common_methods!(
        ArcStatefulBiPredicate<T, U>,
        (FnMut(&T, &U) -> bool + Send + 'static),
        |f| Arc::new(Mutex::new(f))
    );

    /// Returns a bi-predicate representing logical AND with another predicate.
    ///
    /// This method borrows `self`; the returned predicate shares this
    /// predicate's mutable state through the same `Arc<Mutex<_>>`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other bi-predicate to combine with.
    ///
    /// # Returns
    ///
    /// A new `ArcStatefulBiPredicate` representing logical AND.
    #[cfg(feature = "combinators")]
    #[inline]
    pub fn and<P>(&self, mut other: P) -> ArcStatefulBiPredicate<T, U>
    where
        P: StatefulBiPredicate<T, U> + Send + 'static,
        T: 'static,
        U: 'static,
    {
        let self_fn = self.function.clone();
        ArcStatefulBiPredicate::new(move |first: &T, second: &U| {
            let matched = (self_fn.lock())(first, second);
            matched && other.test(first, second)
        })
    }

    /// Returns a bi-predicate representing logical OR with another predicate.
    ///
    /// This method borrows `self`; the returned predicate shares this
    /// predicate's mutable state through the same `Arc<Mutex<_>>`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other bi-predicate to combine with.
    ///
    /// # Returns
    ///
    /// A new `ArcStatefulBiPredicate` representing logical OR.
    #[cfg(feature = "combinators")]
    #[inline]
    pub fn or<P>(&self, mut other: P) -> ArcStatefulBiPredicate<T, U>
    where
        P: StatefulBiPredicate<T, U> + Send + 'static,
        T: 'static,
        U: 'static,
    {
        let self_fn = self.function.clone();
        ArcStatefulBiPredicate::new(move |first: &T, second: &U| {
            let matched = (self_fn.lock())(first, second);
            matched || other.test(first, second)
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
    /// A new `ArcStatefulBiPredicate` representing logical NAND.
    #[cfg(feature = "combinators")]
    #[inline]
    pub fn nand<P>(&self, mut other: P) -> ArcStatefulBiPredicate<T, U>
    where
        P: StatefulBiPredicate<T, U> + Send + 'static,
        T: 'static,
        U: 'static,
    {
        let self_fn = self.function.clone();
        ArcStatefulBiPredicate::new(move |first: &T, second: &U| {
            let matched = (self_fn.lock())(first, second);
            !(matched && other.test(first, second))
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
    /// A new `ArcStatefulBiPredicate` representing logical XOR.
    #[cfg(feature = "combinators")]
    #[inline]
    pub fn xor<P>(&self, mut other: P) -> ArcStatefulBiPredicate<T, U>
    where
        P: StatefulBiPredicate<T, U> + Send + 'static,
        T: 'static,
        U: 'static,
    {
        let self_fn = self.function.clone();
        ArcStatefulBiPredicate::new(move |first: &T, second: &U| {
            let matched = (self_fn.lock())(first, second);
            matched ^ other.test(first, second)
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
    /// A new `ArcStatefulBiPredicate` representing logical NOR.
    #[cfg(feature = "combinators")]
    #[inline]
    pub fn nor<P>(&self, mut other: P) -> ArcStatefulBiPredicate<T, U>
    where
        P: StatefulBiPredicate<T, U> + Send + 'static,
        T: 'static,
        U: 'static,
    {
        let self_fn = self.function.clone();
        ArcStatefulBiPredicate::new(move |first: &T, second: &U| {
            let matched = (self_fn.lock())(first, second);
            !(matched || other.test(first, second))
        })
    }
}

#[cfg(feature = "combinators")]
impl<T, U> Not for ArcStatefulBiPredicate<T, U>
where
    T: 'static,
    U: 'static,
{
    type Output = ArcStatefulBiPredicate<T, U>;

    fn not(self) -> Self::Output {
        let function = self.function;
        ArcStatefulBiPredicate::new(move |first: &T, second: &U| {
            !((function.lock())(first, second))
        })
    }
}

#[cfg(feature = "combinators")]
impl<T, U> Not for &ArcStatefulBiPredicate<T, U>
where
    T: 'static,
    U: 'static,
{
    type Output = ArcStatefulBiPredicate<T, U>;

    fn not(self) -> Self::Output {
        let function = self.function.clone();
        ArcStatefulBiPredicate::new(move |first: &T, second: &U| {
            !((function.lock())(first, second))
        })
    }
}

// Generates: impl Clone for ArcStatefulBiPredicate<T, U>
impl_predicate_clone!(ArcStatefulBiPredicate<T, U>);

// Generates: impl Debug for ArcStatefulBiPredicate<T, U> and impl Display for
// ArcStatefulBiPredicate<T, U>
impl_predicate_debug_display!(ArcStatefulBiPredicate<T, U>);

impl<T, U> StatefulBiPredicate<T, U> for ArcStatefulBiPredicate<T, U> {
    fn test(&mut self, first: &T, second: &U) -> bool {
        (self.function.lock())(first, second)
    }
}

// Blanket implementation for mutable closures matching FnMut(&T, &U) -> bool.
impl_closure_trait!(
    StatefulBiPredicate<T, U>,
    test,
    FnMut(first: &T, second: &U) -> bool
);
