// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Tester Types
//!
//! Provides zero-argument tester abstractions that return boolean values.
//!
//! # Overview
//!
//! **Tester** is the stateless zero-argument tester abstraction. It corresponds
//! to Rust's `Fn() -> bool`: callers may execute it repeatedly through `&self`,
//! and the implementation should not require mutable access to its own state.
//!
//! **StatefulTester** is the stateful zero-argument tester abstraction. It
//! corresponds to Rust's `FnMut() -> bool`: callers execute it through
//! `&mut self`, allowing the implementation to update captured or internal
//! state between test calls.
//!
//! # Core Design Principles
//!
//! 1. **Returns boolean**: `Tester` returns `bool` to indicate test results
//! 2. **Uses `&self`**: `Tester` represents `Fn() -> bool` and does not need
//!    mutable access to run
//! 3. **No TesterOnce**: Very limited use cases, lacks practical examples
//! 4. **Separate stateful API**: Use `StatefulTester` for `FnMut() -> bool`
//!    closures or custom testers that mutate internal state
//!
//! # Three Implementations
//!
//! - **`BoxTester`** / **`BoxStatefulTester`**: Single ownership using `Box<dyn
//!   Fn() -> bool>` or `Box<dyn FnMut() -> bool>`. Best for one-time use and
//!   builder patterns.
//!
//! - **`ArcTester`** / **`ArcStatefulTester`**: Thread-safe shared ownership.
//!   Stateless testers use `Arc<dyn Fn() -> bool + Send + Sync>`, while
//!   stateful testers use `Arc<Mutex<dyn FnMut() -> bool + Send>>`.
//!
//! - **`RcTester`** / **`RcStatefulTester`**: Single-threaded shared ownership.
//!   Stateless testers use `Rc<dyn Fn() -> bool>`, while stateful testers use
//!   `Rc<RefCell<dyn FnMut() -> bool>>`.
//!
//! # Comparison with Other Functional Abstractions
//!
//! | Type            | Input | Output | self        | Closure Shape     | Use Cases   |
//! |-----------------|-------|--------|-------------|-------------------|-------------|
//! | Tester          | None  | `bool` | `&self`     | `Fn() -> bool`    | State Check |
//! | StatefulTester  | None  | `bool` | `&mut self` | `FnMut() -> bool` | Stateful Check |
//! | Predicate       | `&T`  | `bool` | `&self`     | `Fn(&T) -> bool`  | Filter      |
//! | Supplier        | None  | `T`    | `&self`     | `Fn() -> T`       | Factory     |
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
//! assert!(handle.join().expect("thread should not panic"));
//! ```
#[cfg(feature = "rc")]
use std::rc::Rc;
use std::sync::Arc;

#[cfg(feature = "stateful")]
pub mod stateful_tester;
pub mod tester;

#[cfg(feature = "stateful")]
pub use stateful_tester::*;
pub use tester::*;
