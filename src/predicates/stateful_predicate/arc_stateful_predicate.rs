// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
// qubit-style: allow explicit-imports
//! Defines the `ArcStatefulPredicate` public type.

#[cfg(feature = "combinators")]
use std::ops::Not;

use super::{
    ALWAYS_FALSE_NAME,
    ALWAYS_TRUE_NAME,
    Arc,
    Mutex,
    StatefulPredicate,
    impl_predicate_clone,
    impl_predicate_common_methods,
    impl_predicate_debug_display,
};

type ArcStatefulPredicateFn<T> =
    Arc<Mutex<dyn FnMut(&T) -> bool + Send + 'static>>;

/// An Arc-based stateful predicate with thread-safe shared ownership.
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
pub struct ArcStatefulPredicate<T> {
    pub(super) function: ArcStatefulPredicateFn<T>,
    pub(super) name: Option<String>,
}

impl<T> ArcStatefulPredicate<T> {
    // Generates: new(), new_with_name(), name(), set_name(), always_true(),
    // always_false()
    impl_predicate_common_methods!(
        ArcStatefulPredicate<T>,
        (FnMut(&T) -> bool + Send + 'static),
        |f| Arc::new(Mutex::new(f))
    );

    /// Returns a predicate representing logical AND with another predicate.
    ///
    /// This method borrows `self`; the returned predicate shares this
    /// predicate's mutable state through the same `Arc<Mutex<_>>`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other predicate to combine with.
    ///
    /// # Returns
    ///
    /// A new `ArcStatefulPredicate` representing logical AND.
    #[cfg(feature = "combinators")]
    #[inline]
    pub fn and<P>(&self, mut other: P) -> ArcStatefulPredicate<T>
    where
        P: StatefulPredicate<T> + Send + 'static,
        T: 'static,
    {
        let self_fn = self.function.clone();
        ArcStatefulPredicate::new(move |value: &T| {
            let matched = (self_fn.lock())(value);
            matched && other.test(value)
        })
    }

    /// Returns a predicate representing logical OR with another predicate.
    ///
    /// This method borrows `self`; the returned predicate shares this
    /// predicate's mutable state through the same `Arc<Mutex<_>>`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other predicate to combine with.
    ///
    /// # Returns
    ///
    /// A new `ArcStatefulPredicate` representing logical OR.
    #[cfg(feature = "combinators")]
    #[inline]
    pub fn or<P>(&self, mut other: P) -> ArcStatefulPredicate<T>
    where
        P: StatefulPredicate<T> + Send + 'static,
        T: 'static,
    {
        let self_fn = self.function.clone();
        ArcStatefulPredicate::new(move |value: &T| {
            let matched = (self_fn.lock())(value);
            matched || other.test(value)
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
    /// A new `ArcStatefulPredicate` representing logical NAND.
    #[cfg(feature = "combinators")]
    #[inline]
    pub fn nand<P>(&self, mut other: P) -> ArcStatefulPredicate<T>
    where
        P: StatefulPredicate<T> + Send + 'static,
        T: 'static,
    {
        let self_fn = self.function.clone();
        ArcStatefulPredicate::new(move |value: &T| {
            let matched = (self_fn.lock())(value);
            !(matched && other.test(value))
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
    /// A new `ArcStatefulPredicate` representing logical XOR.
    #[cfg(feature = "combinators")]
    #[inline]
    pub fn xor<P>(&self, mut other: P) -> ArcStatefulPredicate<T>
    where
        P: StatefulPredicate<T> + Send + 'static,
        T: 'static,
    {
        let self_fn = self.function.clone();
        ArcStatefulPredicate::new(move |value: &T| {
            let matched = (self_fn.lock())(value);
            matched ^ other.test(value)
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
    /// A new `ArcStatefulPredicate` representing logical NOR.
    #[cfg(feature = "combinators")]
    #[inline]
    pub fn nor<P>(&self, mut other: P) -> ArcStatefulPredicate<T>
    where
        P: StatefulPredicate<T> + Send + 'static,
        T: 'static,
    {
        let self_fn = self.function.clone();
        ArcStatefulPredicate::new(move |value: &T| {
            let matched = (self_fn.lock())(value);
            !(matched || other.test(value))
        })
    }
}

#[cfg(feature = "combinators")]
impl<T> Not for ArcStatefulPredicate<T>
where
    T: 'static,
{
    type Output = ArcStatefulPredicate<T>;

    fn not(self) -> Self::Output {
        let function = self.function;
        ArcStatefulPredicate::new(move |value: &T| !((function.lock())(value)))
    }
}

#[cfg(feature = "combinators")]
impl<T> Not for &ArcStatefulPredicate<T>
where
    T: 'static,
{
    type Output = ArcStatefulPredicate<T>;

    fn not(self) -> Self::Output {
        let function = self.function.clone();
        ArcStatefulPredicate::new(move |value: &T| !((function.lock())(value)))
    }
}

// Generates: impl Clone for ArcStatefulPredicate<T>
impl_predicate_clone!(ArcStatefulPredicate<T>);

// Generates: impl Debug for ArcStatefulPredicate<T> and impl Display for
// ArcStatefulPredicate<T>
impl_predicate_debug_display!(ArcStatefulPredicate<T>);

// Implements StatefulPredicate trait for ArcStatefulPredicate<T>
impl<T> StatefulPredicate<T> for ArcStatefulPredicate<T> {
    fn test(&mut self, value: &T) -> bool {
        (self.function.lock())(value)
    }
}

/// Implements `StatefulPredicate<T>` for `FnMut(&T) -> bool` closures.
///
/// This blanket implementation lets mutable closures be used directly
/// wherever a `StatefulPredicate` is expected.
impl<F, T> StatefulPredicate<T> for F
where
    F: FnMut(&T) -> bool,
{
    fn test(&mut self, value: &T) -> bool {
        self(value)
    }
}
