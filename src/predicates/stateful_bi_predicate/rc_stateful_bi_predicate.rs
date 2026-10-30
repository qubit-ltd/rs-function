// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcStatefulBiPredicate` public type.

use std::cell::RefCell;
use std::ops::Not;
use std::rc::Rc;

use super::ALWAYS_FALSE_NAME;
use super::ALWAYS_TRUE_NAME;
use crate::StatefulBiPredicate;
use crate::predicates::macros::impl_predicate_clone;
use crate::predicates::macros::impl_predicate_common_methods;
use crate::predicates::macros::impl_predicate_debug_display;

/// Stores a mutable callback behind single-threaded shared ownership.
///
/// Cloned wrappers retain the same callback state through the `Rc` handle.
/// Each invocation requires a mutable `RefCell` borrow, so synchronous
/// re-entry through a shared wrapper panics while that borrow is active.
type RcStatefulBiPredicateFn<T, U> = Rc<RefCell<dyn FnMut(&T, &U) -> bool>>;

/// An Rc-based stateful bi-predicate with single-threaded shared ownership.
///
/// This type stores the predicate closure inside `Rc<RefCell<_>>`, allowing
/// cheap clones that share the same mutable predicate state on one thread.
///
/// # Borrowing and reentrancy
///
/// Each call holds a mutable `RefCell` borrow while the user callback runs.
/// Synchronous re-entry through the same shared wrapper panics with a borrow
/// error. Mutations completed before a panic are not rolled back.
///
/// # Type Parameters
///
/// * `T` - The type of the first value tested by the predicate.
/// * `U` - The type of the second value tested by the predicate.
///
/// # Examples
///
/// ```
/// use qubit_function::RcStatefulBiPredicate;
/// use qubit_function::StatefulBiPredicate;
///
/// use std::cell::Cell;
/// use std::rc::Rc;
///
/// let calls = Rc::new(Cell::new(0));
/// let callback_calls = Rc::clone(&calls);
/// let mut predicate = RcStatefulBiPredicate::new(move |first: &i32, second: &i32| {
///     callback_calls.set(callback_calls.get() + 1);
///     first + second == 10
/// });
///
/// assert!(predicate.test(&4, &6));
/// assert_eq!(calls.get(), 1);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct RcStatefulBiPredicate<T, U> {
    /// The wrapped callback implementation.
    pub(super) function: RcStatefulBiPredicateFn<T, U>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

impl<T, U> RcStatefulBiPredicate<T, U> {
    // Generates constructors, factories, and name methods.
    impl_predicate_common_methods!(
        RcStatefulBiPredicate<T, U>,
        (FnMut(&T, &U) -> bool + 'static),
        |f| Rc::new(RefCell::new(f)),
        defer_metadata
    );

    /// Returns a bi-predicate representing logical AND with another predicate.
    ///
    /// This method borrows `self`; the returned predicate shares this
    /// predicate's mutable state through the same `Rc<RefCell<_>>`.
    ///
    /// # Type Parameters
    ///
    /// * `P` - The type of the other stateful bi-predicate.
    ///
    /// # Parameters
    ///
    /// * `other` - The other bi-predicate to combine with.
    ///
    /// # Returns
    ///
    /// A new `RcStatefulBiPredicate` representing logical AND.
    ///
    /// # Panics
    ///
    /// Panics if either callback panics or synchronously re-enters this
    /// wrapper while its callback is already mutably borrowed.
    #[inline]
    pub fn and<P>(&self, mut other: P) -> RcStatefulBiPredicate<T, U>
    where
        P: StatefulBiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        let self_fn = self.function.clone();
        RcStatefulBiPredicate::new(move |first: &T, second: &U| {
            let matched = {
                let mut function = self_fn.borrow_mut();
                function(first, second)
            };
            matched && other.test(first, second)
        })
    }

    /// Returns a bi-predicate representing logical OR with another predicate.
    ///
    /// This method borrows `self`; the returned predicate shares this
    /// predicate's mutable state through the same `Rc<RefCell<_>>`.
    ///
    /// # Type Parameters
    ///
    /// * `P` - The type of the other stateful bi-predicate.
    ///
    /// # Parameters
    ///
    /// * `other` - The other bi-predicate to combine with.
    ///
    /// # Returns
    ///
    /// A new `RcStatefulBiPredicate` representing logical OR.
    ///
    /// # Panics
    ///
    /// Panics if either callback panics or synchronously re-enters this
    /// wrapper while its callback is already mutably borrowed.
    #[inline]
    pub fn or<P>(&self, mut other: P) -> RcStatefulBiPredicate<T, U>
    where
        P: StatefulBiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        let self_fn = self.function.clone();
        RcStatefulBiPredicate::new(move |first: &T, second: &U| {
            let matched = {
                let mut function = self_fn.borrow_mut();
                function(first, second)
            };
            matched || other.test(first, second)
        })
    }

    /// Returns a bi-predicate representing logical NAND with another predicate.
    ///
    /// NAND returns `true` unless both predicates return `true`.
    ///
    /// # Type Parameters
    ///
    /// * `P` - The type of the other stateful bi-predicate.
    ///
    /// # Parameters
    ///
    /// * `other` - The other bi-predicate to combine with.
    ///
    /// # Returns
    ///
    /// A new `RcStatefulBiPredicate` representing logical NAND.
    ///
    /// # Panics
    ///
    /// Panics if either callback panics or synchronously re-enters this
    /// wrapper while its callback is already mutably borrowed.
    #[inline]
    pub fn nand<P>(&self, mut other: P) -> RcStatefulBiPredicate<T, U>
    where
        P: StatefulBiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        let self_fn = self.function.clone();
        RcStatefulBiPredicate::new(move |first: &T, second: &U| {
            let matched = {
                let mut function = self_fn.borrow_mut();
                function(first, second)
            };
            !(matched && other.test(first, second))
        })
    }

    /// Returns a bi-predicate representing logical XOR with another predicate.
    ///
    /// XOR evaluates both predicates and returns `true` when exactly one
    /// predicate returns `true`.
    ///
    /// # Type Parameters
    ///
    /// * `P` - The type of the other stateful bi-predicate.
    ///
    /// # Parameters
    ///
    /// * `other` - The other bi-predicate to combine with.
    ///
    /// # Returns
    ///
    /// A new `RcStatefulBiPredicate` representing logical XOR.
    ///
    /// # Panics
    ///
    /// Panics if either callback panics or synchronously re-enters this
    /// wrapper while its callback is already mutably borrowed.
    #[inline]
    pub fn xor<P>(&self, mut other: P) -> RcStatefulBiPredicate<T, U>
    where
        P: StatefulBiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        let self_fn = self.function.clone();
        RcStatefulBiPredicate::new(move |first: &T, second: &U| {
            let matched = {
                let mut function = self_fn.borrow_mut();
                function(first, second)
            };
            matched ^ other.test(first, second)
        })
    }

    /// Returns a bi-predicate representing logical NOR with another predicate.
    ///
    /// NOR returns `true` only when both predicates return `false`.
    ///
    /// # Type Parameters
    ///
    /// * `P` - The type of the other stateful bi-predicate.
    ///
    /// # Parameters
    ///
    /// * `other` - The other bi-predicate to combine with.
    ///
    /// # Returns
    ///
    /// A new `RcStatefulBiPredicate` representing logical NOR.
    ///
    /// # Panics
    ///
    /// Panics if either callback panics or synchronously re-enters this
    /// wrapper while its callback is already mutably borrowed.
    #[inline]
    pub fn nor<P>(&self, mut other: P) -> RcStatefulBiPredicate<T, U>
    where
        P: StatefulBiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        let self_fn = self.function.clone();
        RcStatefulBiPredicate::new(move |first: &T, second: &U| {
            let matched = {
                let mut function = self_fn.borrow_mut();
                function(first, second)
            };
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

    /// Negates this predicate while transferring its shared callback state.
    ///
    /// The returned predicate keeps the same callback allocation, so its
    /// mutable state remains shared with any existing clones.
    ///
    /// # Returns
    ///
    /// A predicate that returns the opposite result while sharing the
    /// original callback state.
    ///
    /// # Panics
    ///
    /// The predicate panics if its callback panics or synchronously re-enters
    /// a wrapper that already holds a mutable borrow of the callback.
    fn not(self) -> Self::Output {
        let metadata = self.metadata;
        let function = self.function;
        RcStatefulBiPredicate::new_with_metadata(
            move |first: &T, second: &U| {
                let mut function = function.borrow_mut();
                !function(first, second)
            },
            metadata,
        )
    }
}

impl<T, U> Not for &RcStatefulBiPredicate<T, U>
where
    T: 'static,
    U: 'static,
{
    type Output = RcStatefulBiPredicate<T, U>;

    /// Creates a negated predicate that shares this predicate's callback.
    ///
    /// Cloning the callback handle leaves `self` available and makes calls to
    /// either wrapper update the same mutable callback state.
    ///
    /// # Returns
    ///
    /// A predicate that returns the opposite result while sharing this
    /// predicate's callback state.
    ///
    /// # Panics
    ///
    /// The predicate panics if its callback panics or synchronously re-enters
    /// a wrapper that already holds a mutable borrow of the callback.
    fn not(self) -> Self::Output {
        let function = self.function.clone();
        RcStatefulBiPredicate::new_with_metadata(
            move |first: &T, second: &U| {
                let mut function = function.borrow_mut();
                !function(first, second)
            },
            self.metadata.clone(),
        )
    }
}

// Generates: impl Clone for RcStatefulBiPredicate<T, U>
impl_predicate_clone!(RcStatefulBiPredicate<T, U>);

// Generates: impl Debug for RcStatefulBiPredicate<T, U> and impl Display for
// RcStatefulBiPredicate<T, U>
impl_predicate_debug_display!(RcStatefulBiPredicate<T, U>);

impl<T, U> StatefulBiPredicate<T, U> for RcStatefulBiPredicate<T, U> {
    /// Evaluates the callback with mutable access to its shared state.
    ///
    /// Clones of this predicate use the same callback state. The two tested
    /// values remain immutably borrowed for the duration of the callback.
    ///
    /// # Parameters
    ///
    /// * `first` - The first value supplied to the callback.
    /// * `second` - The second value supplied to the callback.
    ///
    /// # Returns
    ///
    /// The boolean result returned by the callback.
    ///
    /// # Panics
    ///
    /// Panics if the callback panics or synchronously re-enters this wrapper
    /// while the callback is already mutably borrowed.
    #[inline]
    fn test(&mut self, first: &T, second: &U) -> bool {
        let mut function = self.function.borrow_mut();
        function(first, second)
    }
}
