// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `StatefulTester` public trait.

pub mod arc_stateful_tester;
pub use arc_stateful_tester::ArcStatefulTester;
pub mod box_stateful_tester;
pub use box_stateful_tester::BoxStatefulTester;
#[cfg(feature = "rc")]
pub mod rc_stateful_tester;
#[cfg(feature = "rc")]
pub use rc_stateful_tester::RcStatefulTester;

/// Tests whether a zero-argument condition holds while allowing state mutation.
///
/// `StatefulTester` is the stateful counterpart of
/// [`Tester`](super::tester::Tester).
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
