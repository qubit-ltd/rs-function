/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Defines the `StatefulTester` public trait.

/// Tests whether a zero-argument condition holds while allowing state mutation.
///
/// `StatefulTester` is the stateful counterpart of [`Tester`](super::Tester).
/// It is equivalent to `FnMut() -> bool`: callers execute it through
/// `&mut self`, so the implementation may update captured or internal state
/// between calls.
///
/// # Examples
///
/// ```rust
/// use qubit_function::StatefulTester;
///
/// let mut count = 0;
/// let mut tester = move || {
///     count += 1;
///     count >= 2
/// };
///
/// assert!(!tester.test());
/// assert!(tester.test());
/// ```
pub trait StatefulTester {
    /// Executes the stateful test and returns the test result.
    ///
    /// This method uses `&mut self` to match `FnMut() -> bool`, allowing each
    /// call to update internal state before producing the boolean result.
    ///
    /// # Return Value
    ///
    /// Returns `true` if the condition holds, otherwise returns `false`.
    fn test(&mut self) -> bool;

    /// Converts this tester to a mutable closure.
    ///
    /// # Return Value
    ///
    /// A closure implementing `FnMut() -> bool` that owns this tester and
    /// delegates each call to [`StatefulTester::test`].
    fn into_fn(mut self) -> impl FnMut() -> bool
    where
        Self: Sized + 'static,
    {
        move || self.test()
    }

    /// Converts this tester to a mutable closure with an explicit method name.
    ///
    /// This is a naming alias of [`StatefulTester::into_fn`] for call sites
    /// that want the returned `FnMut` shape to be obvious.
    fn into_mut_fn(self) -> impl FnMut() -> bool
    where
        Self: Sized + 'static,
    {
        self.into_fn()
    }

    /// Creates a mutable closure from a cloned tester.
    ///
    /// The original tester remains available. The returned closure owns a clone
    /// and mutates that clone on each call.
    fn to_fn(&self) -> impl FnMut() -> bool
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_fn()
    }

    /// Creates a mutable closure from a cloned tester with an explicit method
    /// name.
    ///
    /// This is a naming alias of [`StatefulTester::to_fn`] and preserves the
    /// same clone-based behavior.
    fn to_mut_fn(&self) -> impl FnMut() -> bool
    where
        Self: Clone + Sized + 'static,
    {
        self.to_fn()
    }
}

impl<F> StatefulTester for F
where
    F: FnMut() -> bool,
{
    #[inline]
    fn test(&mut self) -> bool {
        self()
    }
}
