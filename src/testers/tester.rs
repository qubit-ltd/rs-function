// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `Tester` public trait.

mod arc_tester;
pub use arc_tester::ArcTester;
mod box_tester;
pub use box_tester::BoxTester;
#[cfg(feature = "rc")]
mod rc_tester;
#[cfg(feature = "rc")]
pub use rc_tester::RcTester;

/// Tests whether a zero-argument condition holds.
///
/// `Tester` is a functional abstraction equivalent to `Fn() -> bool`. It
/// accepts no parameters, uses `&self`, and returns a boolean value indicating
/// the test result of some state or condition.
///
/// # Core Characteristics
///
/// - **No input parameters**: Captures context through closures
/// - **Returns boolean**: Indicates test results
/// - **Uses `&self`**: Matches `Fn() -> bool` and does not require `&mut self`;
///   interior mutability and external side effects remain possible
/// - **Repeatable calls**: The same Tester can call `test()` multiple times
/// - **Stateless closure shape**: For `FnMut() -> bool`, use `StatefulTester`
///
/// # Use Cases
///
/// - **State checking**: Check system or service status
/// - **Condition waiting**: Repeatedly check until conditions are met
/// - **Health monitoring**: Check system health status
/// - **Precondition validation**: Verify conditions before operations
///
/// # Design Philosophy
///
/// Tester's responsibility is "test judgment". The `Fn` call shape permits
/// repeated invocation through `&self`; it does not guarantee purity or forbid
/// interior mutability and external state changes.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BoxTester, Tester};
/// use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
///
/// // State managed externally
/// let ready = Arc::new(AtomicBool::new(false));
/// let ready_clone = Arc::clone(&ready);
///
/// // Tester only responsible for reading state
/// let tester = BoxTester::new(move || {
///     ready_clone.load(Ordering::Acquire)
/// });
///
/// // Can be called multiple times
/// assert!(!tester.test());
/// ready.store(true, Ordering::Release);
/// assert!(tester.test());
/// ```
pub trait Tester {
    /// Executes the test and returns the test result
    ///
    /// This method can be called multiple times through `&self`.
    /// Implementations may still use interior mutability or external side
    /// effects.
    ///
    /// # Return Value
    ///
    /// Returns `true` if the condition holds, otherwise returns `false`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BoxTester, Tester};
    ///
    /// let tester = BoxTester::new(|| true);
    /// assert!(tester.test());
    /// ```
    fn test(&self) -> bool;
}

impl<F> Tester for F
where
    F: Fn() -> bool,
{
    fn test(&self) -> bool {
        self()
    }
}
