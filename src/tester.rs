/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Tester Type
//!
//! Provides tester implementations that test conditions or states and return
//! boolean values, without accepting input parameters.
//!
//! # Overview
//!
//! **Tester** is a functional abstraction for testing conditions or states
//! without accepting input. It can check system status, wait for conditions,
//! or perform health checks.
//!
//! This module implements **Option 3** from the design document: a unified
//! `Tester` trait with multiple concrete implementations optimized for
//! different ownership and concurrency scenarios.
//!
//! # Core Design Principles
//!
//! 1. **Returns boolean**: `Tester` returns `bool` to indicate test results
//! 2. **Uses `&self`**: Tester is only responsible for "judgment", not
//!    "state management"
//! 3. **No TesterOnce**: Very limited use cases, lacks practical examples
//! 4. **State management is caller's responsibility**: Tester only reads
//!    state, does not modify state
//!
//! # Three Implementations
//!
//! - **`BoxTester`**: Single ownership using `Box<dyn Fn() -> bool>`.
//!   Zero overhead, cannot be cloned. Best for one-time use and builder
//!   patterns.
//!
//! - **`ArcTester`**: Thread-safe shared ownership using
//!   `Arc<dyn Fn() -> bool + Send + Sync>`. Can be cloned and sent across
//!   threads. Lock-free overhead.
//!
//! - **`RcTester`**: Single-threaded shared ownership using
//!   `Rc<dyn Fn() -> bool>`. Can be cloned but cannot be sent across
//!   threads. Lower overhead than `ArcTester`.
//!
//! # Comparison with Other Functional Abstractions
//!
//! | Type      | Input | Output | self      | Modify? | Use Cases   |
//! |-----------|-------|--------|-----------|---------|-------------|
//! | Tester    | None  | `bool` | `&self`   | No      | State Check |
//! | Predicate | `&T`  | `bool` | `&self`   | No      | Filter      |
//! | Supplier  | None  | `T`    | `&mut`    | Yes     | Factory     |
//!
//! # Examples
//!
//! ## Basic State Checking
//!
//! ```rust
//! use qubit_function::{BoxTester, Tester};
//! use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
//!
//! // State managed externally
//! let count = Arc::new(AtomicUsize::new(0));
//! let count_clone = Arc::clone(&count);
//!
//! let tester = BoxTester::new(move || {
//!     count_clone.load(Ordering::Relaxed) <= 3
//! });
//!
//! assert!(tester.test());  // true (0)
//! count.fetch_add(1, Ordering::Relaxed);
//! assert!(tester.test());  // true (1)
//! count.fetch_add(1, Ordering::Relaxed);
//! assert!(tester.test());  // true (2)
//! count.fetch_add(1, Ordering::Relaxed);
//! assert!(tester.test());  // true (3)
//! count.fetch_add(1, Ordering::Relaxed);
//! assert!(!tester.test()); // false (4)
//! ```
//!
//! ## Logical Combination
//!
//! ```rust
//! use qubit_function::{BoxTester, Tester};
//! use std::sync::{Arc, atomic::{AtomicUsize, AtomicBool, Ordering}};
//!
//! // Simulate microservice health check scenario
//! let cpu_usage = Arc::new(AtomicUsize::new(0));
//! let memory_usage = Arc::new(AtomicUsize::new(0));
//! let is_healthy = Arc::new(AtomicBool::new(true));
//! let is_ready = Arc::new(AtomicBool::new(false));
//! let max_cpu = 80;
//! let max_memory = 90;
//!
//! let cpu_clone = Arc::clone(&cpu_usage);
//! let memory_clone = Arc::clone(&memory_usage);
//! let health_clone = Arc::clone(&is_healthy);
//! let ready_clone = Arc::clone(&is_ready);
//!
//! // System resource check: CPU and memory within safe limits
//! let resources_ok = BoxTester::new(move || {
//!     cpu_clone.load(Ordering::Relaxed) < max_cpu
//! })
//! .and(move || {
//!     memory_clone.load(Ordering::Relaxed) < max_memory
//! });
//!
//! // Service status check: healthy or ready
//! let service_ok = BoxTester::new(move || {
//!     health_clone.load(Ordering::Relaxed)
//! })
//! .or(move || {
//!     ready_clone.load(Ordering::Relaxed)
//! });
//!
//! // Combined condition: resources normal and service available
//! let can_accept_traffic = resources_ok.and(service_ok);
//!
//! // Test different state combinations
//! // Initial state: resources normal and service healthy
//! cpu_usage.store(50, Ordering::Relaxed);
//! memory_usage.store(60, Ordering::Relaxed);
//! assert!(can_accept_traffic.test()); // resources normal and service healthy
//!
//! // Service unhealthy but ready
//! is_healthy.store(false, Ordering::Relaxed);
//! is_ready.store(true, Ordering::Relaxed);
//! assert!(can_accept_traffic.test()); // resources normal and service ready
//!
//! // CPU usage too high
//! cpu_usage.store(95, Ordering::Relaxed);
//! assert!(!can_accept_traffic.test()); // resources exceeded
//!
//! // Service unhealthy but ready
//! is_healthy.store(false, Ordering::Relaxed);
//! cpu_usage.store(50, Ordering::Relaxed);
//! assert!(can_accept_traffic.test()); // still ready
//! ```
//!
//! ## Thread-Safe Sharing
//!
//! ```rust
//! use qubit_function::{ArcTester, Tester};
//! use std::thread;
//!
//! let shared = ArcTester::new(|| true);
//! let clone = shared.clone();
//!
//! let handle = thread::spawn(move || {
//!     clone.test()
//! });
//!
//! assert!(handle.join().unwrap());
//! ```
//!
//! # Author
//!
//! Haixing Hu
use std::rc::Rc;
use std::sync::Arc;

mod box_tester;
pub use box_tester::BoxTester;
mod arc_tester;
pub use arc_tester::ArcTester;
mod rc_tester;
pub use rc_tester::RcTester;
mod fn_tester_ops;
pub use fn_tester_ops::FnTesterOps;

// ============================================================================
// Core Tester Trait
// ============================================================================

/// Tests whether a state or condition holds
///
/// Tester is a functional abstraction for testing states or conditions. It
/// accepts no parameters and returns a boolean value indicating the test
/// result of some state or condition.
///
/// # Core Characteristics
///
/// - **No input parameters**: Captures context through closures
/// - **Returns boolean**: Indicates test results
/// - **Uses `&self`**: Does not modify its own state, only reads external
///   state
/// - **Repeatable calls**: The same Tester can call `test()` multiple times
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
/// Tester's responsibility is "test judgment", not "state management".
/// State management is the caller's responsibility. Tester only reads state
/// and returns judgment results.
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
///
/// # Author
///
/// Haixing Hu
pub trait Tester {
    /// Executes the test and returns the test result
    ///
    /// This method can be called multiple times without modifying the Tester's
    /// own state.
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

    /// Converts this tester to `BoxTester`
    ///
    /// # Return Value
    ///
    /// A `BoxTester` that wraps this tester
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Tester, BoxTester};
    ///
    /// let closure = || true;
    /// let boxed: BoxTester = closure.into_box();
    /// ```
    #[inline]
    fn into_box(self) -> BoxTester
    where
        Self: Sized + 'static,
    {
        BoxTester {
            function: Box::new(move || self.test()),
        }
    }

    /// Converts this tester to `RcTester`
    ///
    /// # Return Value
    ///
    /// A `RcTester` that wraps this tester
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Tester, RcTester};
    ///
    /// let closure = || true;
    /// let rc: RcTester = closure.into_rc();
    /// ```
    #[inline]
    fn into_rc(self) -> RcTester
    where
        Self: Sized + 'static,
    {
        RcTester {
            function: Rc::new(move || self.test()),
        }
    }

    /// Converts this tester to `ArcTester`
    ///
    /// # Return Value
    ///
    /// An `ArcTester` that wraps this tester
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Tester, ArcTester};
    ///
    /// let closure = || true;
    /// let arc: ArcTester = closure.into_arc();
    /// ```
    #[inline]
    fn into_arc(self) -> ArcTester
    where
        Self: Sized + Send + Sync + 'static,
    {
        ArcTester {
            function: Arc::new(move || self.test()),
        }
    }

    /// Converts this tester to a boxed function pointer
    ///
    /// # Return Value
    ///
    /// A `Box<dyn Fn() -> bool>` that wraps this tester
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::Tester;
    ///
    /// let closure = || true;
    /// let func = closure.into_fn();
    /// assert!(func());
    /// ```
    #[inline]
    fn into_fn(self) -> impl Fn() -> bool
    where
        Self: Sized + 'static,
    {
        Box::new(move || self.test())
    }

    /// Clones and converts this tester to `BoxTester`
    ///
    /// # Return Value
    ///
    /// A `BoxTester` that wraps a clone of this tester
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Tester, BoxTester, ArcTester};
    ///
    /// let arc = ArcTester::new(|| true);
    /// let boxed: BoxTester = arc.to_box();
    /// // arc is still available
    /// ```
    #[inline]
    fn to_box(&self) -> BoxTester
    where
        Self: Clone + 'static,
    {
        self.clone().into_box()
    }

    /// Clones and converts this tester to `RcTester`
    ///
    /// # Return Value
    ///
    /// A `RcTester` that wraps a clone of this tester
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Tester, RcTester, ArcTester};
    ///
    /// let arc = ArcTester::new(|| true);
    /// let rc: RcTester = arc.to_rc();
    /// // arc is still available
    /// ```
    #[inline]
    fn to_rc(&self) -> RcTester
    where
        Self: Clone + 'static,
    {
        self.clone().into_rc()
    }

    /// Clones and converts this tester to `ArcTester`
    ///
    /// # Return Value
    ///
    /// An `ArcTester` that wraps a clone of this tester
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Tester, ArcTester, RcTester};
    ///
    /// let rc = RcTester::new(|| true);
    /// // Note: This will panic for RcTester as it's not Send + Sync
    /// // let arc: ArcTester = rc.to_arc();
    /// ```
    #[inline]
    fn to_arc(&self) -> ArcTester
    where
        Self: Clone + Send + Sync + 'static,
    {
        self.clone().into_arc()
    }

    /// Clones and converts this tester to a boxed function pointer
    ///
    /// # Return Value
    ///
    /// A `Box<dyn Fn() -> bool>` that wraps a clone of this tester
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Tester, ArcTester};
    ///
    /// let arc = ArcTester::new(|| true);
    /// let func = arc.to_fn();
    /// // arc is still available
    /// assert!(func());
    /// ```
    #[inline]
    fn to_fn(&self) -> impl Fn() -> bool
    where
        Self: Clone + 'static,
    {
        self.clone().into_fn()
    }
}
