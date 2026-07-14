// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcStatefulBiPredicate` public type.

use std::ops::Not;

use {
    super::ALWAYS_FALSE_NAME,
    super::ALWAYS_TRUE_NAME,
    crate::StatefulBiPredicate,
    crate::predicates::macros::impl_predicate_clone,
    crate::predicates::macros::impl_predicate_common_methods,
    crate::predicates::macros::impl_predicate_debug_display,
    std::cell::RefCell,
    std::rc::Rc,
};

type RcStatefulBiPredicateFn<T, U> = Rc<RefCell<dyn FnMut(&T, &U) -> bool>>;

/// An Rc-based stateful bi-predicate with single-threaded shared ownership.
///
/// This type stores the predicate closure inside `Rc<RefCell<_>>`, allowing
/// cheap clones that share the same mutable predicate state on one thread.
/// # Borrowing and reentrancy
///
/// Each call holds a mutable `RefCell` borrow while the user callback runs.
/// Synchronous re-entry through the same shared wrapper panics with a borrow
/// error. Mutations completed before a panic are not rolled back.
pub struct RcStatefulBiPredicate<T, U> {
    pub(super) function: RcStatefulBiPredicateFn<T, U>,
    pub(super) metadata: crate::callback_metadata::CallbackMetadata,
}

impl<T, U> RcStatefulBiPredicate<T, U> {
    // Generates: new(), new_with_name(), name(), set_name(), always_true(),
    // always_false()
    impl_predicate_common_methods!(
        RcStatefulBiPredicate<T, U>,
        (FnMut(&T, &U) -> bool + 'static),
        |f| Rc::new(RefCell::new(f))
    );

    /// Returns a bi-predicate representing logical AND with another predicate.
    ///
    /// This method borrows `self`; the returned predicate shares this
    /// predicate's mutable state through the same `Rc<RefCell<_>>`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other bi-predicate to combine with.
    ///
    /// # Returns
    ///
    /// A new `RcStatefulBiPredicate` representing logical AND.
    #[inline]
    pub fn and<P>(&self, mut other: P) -> RcStatefulBiPredicate<T, U>
    where
        P: StatefulBiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        let self_fn = self.function.clone();
        RcStatefulBiPredicate::new(move |first: &T, second: &U| {
            let matched = (self_fn.borrow_mut())(first, second);
            matched && other.test(first, second)
        })
    }

    /// Returns a bi-predicate representing logical OR with another predicate.
    ///
    /// This method borrows `self`; the returned predicate shares this
    /// predicate's mutable state through the same `Rc<RefCell<_>>`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other bi-predicate to combine with.
    ///
    /// # Returns
    ///
    /// A new `RcStatefulBiPredicate` representing logical OR.
    #[inline]
    pub fn or<P>(&self, mut other: P) -> RcStatefulBiPredicate<T, U>
    where
        P: StatefulBiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        let self_fn = self.function.clone();
        RcStatefulBiPredicate::new(move |first: &T, second: &U| {
            let matched = (self_fn.borrow_mut())(first, second);
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
    /// A new `RcStatefulBiPredicate` representing logical NAND.
    #[inline]
    pub fn nand<P>(&self, mut other: P) -> RcStatefulBiPredicate<T, U>
    where
        P: StatefulBiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        let self_fn = self.function.clone();
        RcStatefulBiPredicate::new(move |first: &T, second: &U| {
            let matched = (self_fn.borrow_mut())(first, second);
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
    /// A new `RcStatefulBiPredicate` representing logical XOR.
    #[inline]
    pub fn xor<P>(&self, mut other: P) -> RcStatefulBiPredicate<T, U>
    where
        P: StatefulBiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        let self_fn = self.function.clone();
        RcStatefulBiPredicate::new(move |first: &T, second: &U| {
            let matched = (self_fn.borrow_mut())(first, second);
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
    /// A new `RcStatefulBiPredicate` representing logical NOR.
    #[inline]
    pub fn nor<P>(&self, mut other: P) -> RcStatefulBiPredicate<T, U>
    where
        P: StatefulBiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        let self_fn = self.function.clone();
        RcStatefulBiPredicate::new(move |first: &T, second: &U| {
            let matched = (self_fn.borrow_mut())(first, second);
            !(matched || other.test(first, second))
        })
    }
}

impl<T, U> Not for RcStatefulBiPredicate<T, U>
where
    T: 'static,
    U: 'static,
{
    type Output = RcStatefulBiPredicate<T, U>;

    fn not(self) -> Self::Output {
        let function = self.function;
        RcStatefulBiPredicate::new(move |first: &T, second: &U| {
            !((function.borrow_mut())(first, second))
        })
    }
}

impl<T, U> Not for &RcStatefulBiPredicate<T, U>
where
    T: 'static,
    U: 'static,
{
    type Output = RcStatefulBiPredicate<T, U>;

    fn not(self) -> Self::Output {
        let function = self.function.clone();
        RcStatefulBiPredicate::new(move |first: &T, second: &U| {
            !((function.borrow_mut())(first, second))
        })
    }
}

// Generates: impl Clone for RcStatefulBiPredicate<T, U>
impl_predicate_clone!(RcStatefulBiPredicate<T, U>);

// Generates: impl Debug for RcStatefulBiPredicate<T, U> and impl Display for
// RcStatefulBiPredicate<T, U>
impl_predicate_debug_display!(RcStatefulBiPredicate<T, U>);

impl<T, U> StatefulBiPredicate<T, U> for RcStatefulBiPredicate<T, U> {
    fn test(&mut self, first: &T, second: &U) -> bool {
        (self.function.borrow_mut())(first, second)
    }
}
