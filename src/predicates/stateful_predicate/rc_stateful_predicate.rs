// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
// qubit-style: allow explicit-imports
//! Defines the `RcStatefulPredicate` public type.

use std::ops::Not;

use super::{
    ALWAYS_FALSE_NAME,
    ALWAYS_TRUE_NAME,
    Rc,
    RefCell,
    StatefulPredicate,
    impl_predicate_clone,
    impl_predicate_common_methods,
    impl_predicate_debug_display,
};

type RcStatefulPredicateFn<T> = Rc<RefCell<dyn FnMut(&T) -> bool>>;

/// An Rc-based stateful predicate with single-threaded shared ownership.
///
/// This type stores the predicate closure inside `Rc<RefCell<_>>`, allowing
/// cheap clones that share the same mutable predicate state on one thread.
/// # Borrowing and reentrancy
///
/// Each call holds a mutable `RefCell` borrow while the user callback runs.
/// Synchronous re-entry through the same shared wrapper panics with a borrow
/// error. Mutations completed before a panic are not rolled back.
pub struct RcStatefulPredicate<T> {
    pub(super) function: RcStatefulPredicateFn<T>,
    pub(super) name: Option<String>,
}

impl<T> RcStatefulPredicate<T> {
    // Generates: new(), new_with_name(), name(), set_name(), always_true(),
    // always_false()
    impl_predicate_common_methods!(
        RcStatefulPredicate<T>,
        (FnMut(&T) -> bool + 'static),
        |f| { Rc::new(RefCell::new(f)) }
    );

    /// Returns a predicate representing logical AND with another predicate.
    ///
    /// This method borrows `self`; the returned predicate shares this
    /// predicate's mutable state through the same `Rc<RefCell<_>>`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other predicate to combine with.
    ///
    /// # Returns
    ///
    /// A new `RcStatefulPredicate` representing logical AND.
    #[inline]
    pub fn and<P>(&self, mut other: P) -> RcStatefulPredicate<T>
    where
        P: StatefulPredicate<T> + 'static,
        T: 'static,
    {
        let self_fn = self.function.clone();
        RcStatefulPredicate::new(move |value: &T| {
            let matched = (self_fn.borrow_mut())(value);
            matched && other.test(value)
        })
    }

    /// Returns a predicate representing logical OR with another predicate.
    ///
    /// This method borrows `self`; the returned predicate shares this
    /// predicate's mutable state through the same `Rc<RefCell<_>>`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other predicate to combine with.
    ///
    /// # Returns
    ///
    /// A new `RcStatefulPredicate` representing logical OR.
    #[inline]
    pub fn or<P>(&self, mut other: P) -> RcStatefulPredicate<T>
    where
        P: StatefulPredicate<T> + 'static,
        T: 'static,
    {
        let self_fn = self.function.clone();
        RcStatefulPredicate::new(move |value: &T| {
            let matched = (self_fn.borrow_mut())(value);
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
    /// A new `RcStatefulPredicate` representing logical NAND.
    #[inline]
    pub fn nand<P>(&self, mut other: P) -> RcStatefulPredicate<T>
    where
        P: StatefulPredicate<T> + 'static,
        T: 'static,
    {
        let self_fn = self.function.clone();
        RcStatefulPredicate::new(move |value: &T| {
            let matched = (self_fn.borrow_mut())(value);
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
    /// A new `RcStatefulPredicate` representing logical XOR.
    #[inline]
    pub fn xor<P>(&self, mut other: P) -> RcStatefulPredicate<T>
    where
        P: StatefulPredicate<T> + 'static,
        T: 'static,
    {
        let self_fn = self.function.clone();
        RcStatefulPredicate::new(move |value: &T| {
            let matched = (self_fn.borrow_mut())(value);
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
    /// A new `RcStatefulPredicate` representing logical NOR.
    #[inline]
    pub fn nor<P>(&self, mut other: P) -> RcStatefulPredicate<T>
    where
        P: StatefulPredicate<T> + 'static,
        T: 'static,
    {
        let self_fn = self.function.clone();
        RcStatefulPredicate::new(move |value: &T| {
            let matched = (self_fn.borrow_mut())(value);
            !(matched || other.test(value))
        })
    }
}

impl<T> Not for RcStatefulPredicate<T>
where
    T: 'static,
{
    type Output = RcStatefulPredicate<T>;

    fn not(self) -> Self::Output {
        let function = self.function;
        RcStatefulPredicate::new(move |value: &T| {
            !((function.borrow_mut())(value))
        })
    }
}

impl<T> Not for &RcStatefulPredicate<T>
where
    T: 'static,
{
    type Output = RcStatefulPredicate<T>;

    fn not(self) -> Self::Output {
        let function = self.function.clone();
        RcStatefulPredicate::new(move |value: &T| {
            !((function.borrow_mut())(value))
        })
    }
}

// Generates: impl Clone for RcStatefulPredicate<T>
impl_predicate_clone!(RcStatefulPredicate<T>);

// Generates: impl Debug for RcStatefulPredicate<T> and impl Display for
// RcStatefulPredicate<T>
impl_predicate_debug_display!(RcStatefulPredicate<T>);

// Implements StatefulPredicate trait for RcStatefulPredicate<T>
impl<T> StatefulPredicate<T> for RcStatefulPredicate<T> {
    fn test(&mut self, value: &T) -> bool {
        (self.function.borrow_mut())(value)
    }
}
