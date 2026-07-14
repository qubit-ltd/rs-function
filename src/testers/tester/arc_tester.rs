// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `ArcTester` public type.

use std::{
    fmt,
    ops::Not,
};

use {
    crate::{
        Tester,
        callback_metadata::CallbackMetadata,
        macros::{
            impl_common_name_methods,
            impl_common_new_methods,
        },
    },
    std::sync::Arc,
};

// ============================================================================
// ArcTester: Thread-Safe Shared Ownership Implementation
// ============================================================================

/// Thread-safe shared ownership Tester implemented using `Arc`
///
/// `ArcTester` wraps a closure in `Arc<dyn Fn() -> bool + Send + Sync>`,
/// allowing the tester to be cloned and safely shared across threads.
///
/// # Characteristics
///
/// - **Shared ownership**: Can be cloned
/// - **Thread-safe**: Can be sent across threads
/// - **Lock-free overhead**: Uses `Fn` without needing `Mutex`
/// - **Borrowing combination**: `and()`/`or()` borrow `&self`
///
/// # Use Cases
///
/// - Multi-threaded testing scenarios
/// - Health checks shared across threads
/// - Test states requiring concurrent access
/// - Background monitoring tasks
///
/// # Examples
///
/// ```rust
/// use qubit_function::{ArcTester, Tester};
/// use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
/// use std::thread;
///
/// // Shared atomic counter
/// let counter = Arc::new(AtomicUsize::new(0));
/// let counter_clone = Arc::clone(&counter);
///
/// let shared = ArcTester::new(move || {
///     counter_clone.load(Ordering::Relaxed) <= 5
/// });
///
/// let clone = shared.clone();
/// let handle = thread::spawn(move || {
///     clone.test()
/// });
///
/// assert!(handle.join().expect("thread should not panic"));
/// counter.fetch_add(1, Ordering::Relaxed);
/// assert!(shared.test());
/// ```
pub struct ArcTester {
    pub(super) function: Arc<dyn Fn() -> bool + Send + Sync>,
    pub(super) metadata: CallbackMetadata,
}

impl ArcTester {
    impl_common_new_methods!(
        semantic(Tester + Send + Sync + 'static),
        |source| move || source.test(),
        |function| Arc::new(function),
        "tester"
    );

    impl_common_name_methods!("tester");

    /// Combines this tester with another tester using logical AND
    ///
    /// Returns a new `ArcTester` that returns `true` only when both tests
    /// pass. Borrows `&self`, so the original tester remains available, and
    /// moves `next` into the result.
    ///
    /// # Parameters
    ///
    /// * `next` - The tester to combine with
    ///
    /// # Return Value
    ///
    /// A new `ArcTester` representing logical AND
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{ArcTester, Tester};
    /// use std::sync::{Arc, atomic::{AtomicUsize, AtomicBool, Ordering}};
    /// use std::thread;
    ///
    /// // Simulate database connection pool status
    /// let active_connections = Arc::new(AtomicUsize::new(0));
    /// let is_pool_healthy = Arc::new(AtomicBool::new(true));
    /// let max_connections = 50;
    ///
    /// let conn_clone = Arc::clone(&active_connections);
    /// let health_clone = Arc::clone(&is_pool_healthy);
    ///
    /// // Connection pool health check
    /// let pool_healthy = ArcTester::new(move || {
    ///     health_clone.load(Ordering::Relaxed)
    /// });
    ///
    /// // Connection count check
    /// let conn_ok = ArcTester::new(move || {
    ///     conn_clone.load(Ordering::Relaxed) < max_connections
    /// });
    ///
    /// // Combined check: pool healthy and connection count not exceeded
    /// let pool_ready = pool_healthy.and(conn_ok.clone());
    ///
    /// // Multi-threaded test
    /// let pool_ready_clone = pool_ready.clone();
    /// let handle = thread::spawn(move || {
    ///     pool_ready_clone.test()
    /// });
    ///
    /// // Initial state should pass
    /// assert!(handle.join().expect("thread should not panic"));
    /// assert!(pool_ready.test());
    ///
    /// // Connection count exceeded
    /// active_connections.store(60, Ordering::Relaxed);
    /// assert!(!pool_ready.test());
    ///
    /// // Connection pool unhealthy
    /// is_pool_healthy.store(false, Ordering::Relaxed);
    /// assert!(!pool_ready.test());
    /// ```
    #[inline]
    pub fn and<T>(&self, next: T) -> ArcTester
    where
        T: Tester + Send + Sync + 'static,
    {
        let self_fn = Arc::clone(&self.function);
        ArcTester::new(move || self_fn() && next.test())
    }

    /// Combines this tester with another tester using logical OR
    ///
    /// Returns a new `ArcTester` that returns `true` if either test passes.
    /// Borrows `&self`, so the original tester remains available, and moves
    /// `next` into the result.
    ///
    /// # Parameters
    ///
    /// * `next` - The tester to combine with
    ///
    /// # Return Value
    ///
    /// A new `ArcTester` representing logical OR
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{ArcTester, Tester};
    /// use std::sync::{Arc, atomic::{AtomicUsize, AtomicBool, Ordering}};
    /// use std::thread;
    ///
    /// // Simulate load balancer status
    /// let server_load = Arc::new(AtomicUsize::new(0));
    /// let is_server_healthy = Arc::new(AtomicBool::new(true));
    /// let max_load = 80;
    /// let emergency_mode = Arc::new(AtomicBool::new(false));
    ///
    /// let load_clone = Arc::clone(&server_load);
    /// let health_clone = Arc::clone(&is_server_healthy);
    /// let emergency_clone = Arc::clone(&emergency_mode);
    ///
    /// // Server low load
    /// let low_load = ArcTester::new(move || {
    ///     load_clone.load(Ordering::Relaxed) < max_load
    /// });
    ///
    /// // Emergency mode check
    /// let emergency_check = ArcTester::new(move || {
    ///     emergency_clone.load(Ordering::Relaxed)
    /// });
    ///
    /// // Server health check
    /// let server_healthy = ArcTester::new(move || {
    ///     health_clone.load(Ordering::Relaxed)
    /// });
    ///
    /// // Emergency mode or server healthy
    /// let can_handle_requests = emergency_check.or(server_healthy.clone());
    ///
    /// // Combined condition: low load or can handle requests
    /// let should_route_here = low_load.or(can_handle_requests.clone());
    ///
    /// // Multi-threaded test
    /// let router_clone = should_route_here.clone();
    /// let handle = thread::spawn(move || {
    ///     router_clone.test()
    /// });
    ///
    /// // Initial state: low load and healthy
    /// assert!(handle.join().expect("thread should not panic"));
    /// assert!(should_route_here.test());
    ///
    /// // High load but server healthy
    /// server_load.store(90, Ordering::Relaxed);
    /// assert!(should_route_here.test()); // still healthy
    ///
    /// // Server unhealthy but emergency mode
    /// is_server_healthy.store(false, Ordering::Relaxed);
    /// emergency_mode.store(true, Ordering::Relaxed);
    /// assert!(should_route_here.test()); // emergency mode
    ///
    /// // Unhealthy and not emergency mode
    /// emergency_mode.store(false, Ordering::Relaxed);
    /// assert!(!should_route_here.test());
    /// ```
    #[inline]
    pub fn or<T>(&self, next: T) -> ArcTester
    where
        T: Tester + Send + Sync + 'static,
    {
        let self_fn = Arc::clone(&self.function);
        ArcTester::new(move || self_fn() || next.test())
    }

    /// Combines this tester with another tester using logical NAND
    ///
    /// Returns a new `ArcTester` that returns `true` unless both tests pass.
    /// Borrows `&self`, so the original tester remains available, and moves
    /// `next` into the result.
    ///
    /// # Parameters
    ///
    /// * `next` - The tester to combine with
    ///
    /// # Return Value
    ///
    /// A new `ArcTester` representing logical NAND
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{ArcTester, Tester};
    /// use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
    /// use std::thread;
    ///
    /// let flag1 = Arc::new(AtomicBool::new(true));
    /// let flag2 = Arc::new(AtomicBool::new(true));
    ///
    /// let flag1_clone = Arc::clone(&flag1);
    /// let flag2_clone = Arc::clone(&flag2);
    ///
    /// let tester1 = ArcTester::new(move || {
    ///     flag1_clone.load(Ordering::Relaxed)
    /// });
    ///
    /// let tester2 = ArcTester::new(move || {
    ///     flag2_clone.load(Ordering::Relaxed)
    /// });
    ///
    /// let nand = tester1.nand(tester2.clone());
    ///
    /// // Both true returns false
    /// assert!(!nand.test());
    ///
    /// // At least one false returns true
    /// flag1.store(false, Ordering::Relaxed);
    /// assert!(nand.test());
    ///
    /// // Original tester still available
    /// assert!(!tester1.test());
    /// assert!(tester2.test());
    /// ```
    #[inline]
    pub fn nand<T>(&self, next: T) -> ArcTester
    where
        T: Tester + Send + Sync + 'static,
    {
        let self_fn = Arc::clone(&self.function);
        ArcTester::new(move || !(self_fn() && next.test()))
    }

    /// Combines this tester with another tester using logical XOR
    ///
    /// Returns a new `ArcTester` that returns `true` if exactly one test
    /// passes. Borrows `&self`, so the original tester remains available, and
    /// moves `next` into the result.
    ///
    /// # Parameters
    ///
    /// * `next` - The tester to combine with
    ///
    /// # Return Value
    ///
    /// A new `ArcTester` representing logical XOR
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{ArcTester, Tester};
    /// use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
    /// use std::thread;
    ///
    /// let flag1 = Arc::new(AtomicBool::new(true));
    /// let flag2 = Arc::new(AtomicBool::new(false));
    ///
    /// let flag1_clone = Arc::clone(&flag1);
    /// let flag2_clone = Arc::clone(&flag2);
    ///
    /// let tester1 = ArcTester::new(move || {
    ///     flag1_clone.load(Ordering::Relaxed)
    /// });
    ///
    /// let tester2 = ArcTester::new(move || {
    ///     flag2_clone.load(Ordering::Relaxed)
    /// });
    ///
    /// let xor = tester1.xor(tester2.clone());
    ///
    /// // One true one false returns true
    /// assert!(xor.test());
    ///
    /// // Both true returns false
    /// flag2.store(true, Ordering::Relaxed);
    /// assert!(!xor.test());
    ///
    /// // Both false returns false
    /// flag1.store(false, Ordering::Relaxed);
    /// flag2.store(false, Ordering::Relaxed);
    /// assert!(!xor.test());
    ///
    /// // Original tester still available
    /// assert!(!tester1.test());
    /// assert!(!tester2.test());
    /// ```
    #[inline]
    pub fn xor<T>(&self, next: T) -> ArcTester
    where
        T: Tester + Send + Sync + 'static,
    {
        let self_fn = Arc::clone(&self.function);
        ArcTester::new(move || self_fn() ^ next.test())
    }

    /// Combines this tester with another tester using logical NOR
    ///
    /// Returns a new `ArcTester` that returns `true` only when both tests
    /// fail. Borrows `&self`, so the original tester remains available, and
    /// moves `next` into the result.
    ///
    /// # Parameters
    ///
    /// * `next` - The tester to combine with
    ///
    /// # Return Value
    ///
    /// A new `ArcTester` representing logical NOR
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{ArcTester, Tester};
    /// use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
    /// use std::thread;
    ///
    /// let flag1 = Arc::new(AtomicBool::new(false));
    /// let flag2 = Arc::new(AtomicBool::new(false));
    ///
    /// let flag1_clone = Arc::clone(&flag1);
    /// let flag2_clone = Arc::clone(&flag2);
    ///
    /// let tester1 = ArcTester::new(move || {
    ///     flag1_clone.load(Ordering::Relaxed)
    /// });
    ///
    /// let tester2 = ArcTester::new(move || {
    ///     flag2_clone.load(Ordering::Relaxed)
    /// });
    ///
    /// let nor = tester1.nor(tester2.clone());
    ///
    /// // Both false returns true
    /// assert!(nor.test());
    ///
    /// // At least one true returns false
    /// flag1.store(true, Ordering::Relaxed);
    /// assert!(!nor.test());
    ///
    /// // Original tester still available
    /// assert!(tester1.test());
    /// assert!(!tester2.test());
    /// ```
    #[inline]
    pub fn nor<T>(&self, next: T) -> ArcTester
    where
        T: Tester + Send + Sync + 'static,
    {
        let self_fn = Arc::clone(&self.function);
        ArcTester::new(move || !(self_fn() || next.test()))
    }
}

impl Not for ArcTester {
    type Output = ArcTester;

    #[inline]
    fn not(self) -> Self::Output {
        let metadata = self.metadata;
        let func = self.function;
        ArcTester::new_with_metadata(move || !func(), metadata)
    }
}

impl Not for &ArcTester {
    type Output = ArcTester;

    #[inline]
    fn not(self) -> Self::Output {
        let func = Arc::clone(&self.function);
        ArcTester::new_with_metadata(move || !func(), self.metadata.clone())
    }
}

impl Tester for ArcTester {
    #[inline]
    fn test(&self) -> bool {
        (self.function)()
    }
}

impl Clone for ArcTester {
    /// Creates a clone of this `ArcTester`.
    ///
    /// The cloned instance shares the same underlying function with
    /// the original, allowing multiple references to the same test
    /// logic.
    #[inline]
    fn clone(&self) -> Self {
        Self {
            function: Arc::clone(&self.function),
            metadata: self.metadata.clone(),
        }
    }
}

impl fmt::Debug for ArcTester {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ArcTester")
            .field("name", &self.metadata.name())
            .finish()
    }
}

impl fmt::Display for ArcTester {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.metadata.name() {
            Some(name) => write!(formatter, "ArcTester({name})"),
            None => formatter.write_str("ArcTester"),
        }
    }
}
