// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Supplier Types
//!
//! Provides stateless supplier implementations that generate and
//! return values without taking input.
//!
//! # Overview
//!
//! A **Supplier** is a functional abstraction equivalent to
//! `Fn() -> T`: it generates values without accepting input or
//! requiring mutable access to itself. The `get` method uses `&self`,
//! enabling use in read-only contexts and lock-free concurrent access
//! for the `Arc` implementation.
//!
//! For generators that need mutable internal state, such as counters
//! or sequences, use [`StatefulSupplier`](crate::StatefulSupplier).
//!
//! # Key Differences from StatefulSupplier
//!
//! | Aspect | `Supplier<T>` | `StatefulSupplier<T>` |
//! |--------|---------------|----------------------|
//! | self signature | `&self` | `&mut self` |
//! | Closure type | `Fn() -> T` | `FnMut() -> T` |
//! | Can modify internal state | No | Yes |
//! | Arc implementation | `Arc<dyn Fn() -> T + Send + Sync>` | `Arc<Mutex<dyn FnMut() -> T + Send>>` |
//! | Use cases | Factory, constant, high concurrency | Counter, sequence, generator |
//!
//! # Three Implementations
//!
//! - **`BoxSupplier<T>`**: Single ownership using `Box<dyn Fn() -> T>`. Zero
//!   overhead, cannot be cloned. Best for one-time use in read-only contexts.
//!
//! - **`ArcSupplier<T>`**: Thread-safe shared ownership using `Arc<dyn Fn() ->
//!   T + Send + Sync>`. **Lock-free** - no Mutex needed! Can be cloned and sent
//!   across threads with excellent performance.
//!
//! - **`RcSupplier<T>`**: Single-threaded shared ownership using `Rc<dyn Fn()
//!   -> T>`. Can be cloned but not sent across threads. Lightweight alternative
//!   to `ArcSupplier`.
//!
//! # Use Cases
//!
//! ## 1. Calling in `&self` Methods
//!
//! ```rust
//! use qubit_function::{ArcSupplier, Supplier};
//!
//! struct Executor<E> {
//!     error_supplier: ArcSupplier<E>,
//! }
//!
//! impl<E> Executor<E> {
//!     fn execute(&self) -> Result<(), E> {
//!         // Can call directly in &self method!
//!         Err(self.error_supplier.get())
//!     }
//! }
//! ```
//!
//! ## 2. High-Concurrency Lock-Free Access
//!
//! ```rust
//! use qubit_function::{ArcSupplier, Supplier};
//! use std::thread;
//!
//! let factory = ArcSupplier::new(|| {
//!     String::from("Hello, World!")
//! });
//!
//! let handles: Vec<_> = (0..10)
//!     .map(|_| {
//!         let f = factory.clone();
//!         thread::spawn(move || f.get()) // Lock-free!
//!     })
//!     .collect();
//!
//! for h in handles {
//!     assert_eq!(h.join().expect("thread should not panic"), "Hello, World!");
//! }
//! ```
//!
//! ## 3. Fixed Factories
//!
//! ```rust
//! use qubit_function::{BoxSupplier, Supplier};
//!
//! #[derive(Clone)]
//! struct Config {
//!     timeout: u64,
//! }
//!
//! let config_factory = BoxSupplier::new(|| Config {
//!     timeout: 30,
//! });
//!
//! assert_eq!(config_factory.get().timeout, 30);
//! assert_eq!(config_factory.get().timeout, 30);
//! ```
//!
//! # Performance Comparison
//!
//! For stateless scenarios in multi-threaded environments:
//!
//! - `ArcStatefulSupplier<T>`: Requires `Mutex`, lock contention on every
//!   `get()` call.
//! - `ArcSupplier<T>`: Lock-free, can call `get()` concurrently without
//!   contention.
//!
//! Benchmark results show `ArcSupplier` can be **10x
//! faster** than `ArcStatefulSupplier` in high-concurrency stateless
//! scenarios.

#[cfg(feature = "rc")]
use std::rc::Rc;
use std::sync::Arc;

use crate::predicates::predicate::Predicate;
use crate::suppliers::macros::{
    impl_box_supplier_methods, impl_shared_supplier_methods, impl_supplier_clone,
    impl_supplier_common_methods, impl_supplier_debug_display,
};
use crate::transformers::transformer::Transformer;

mod box_supplier;
pub use box_supplier::BoxSupplier;
mod arc_supplier;
pub use arc_supplier::ArcSupplier;
#[cfg(feature = "rc")]
mod rc_supplier;
#[cfg(feature = "rc")]
pub use rc_supplier::RcSupplier;

// ======================================================================
// Supplier Trait
// ======================================================================

/// Stateless supplier trait: generates values without modifying
/// state.
///
/// The core abstraction for stateless value generation. Unlike
/// `Supplier<T>`, it uses `&self` instead of `&mut self`, enabling
/// usage in read-only contexts and lock-free concurrent access.
///
/// # Key Characteristics
///
/// - **No input parameters**: Pure value generation
/// - **Read-only access**: Uses `&self`, doesn't modify state
/// - **Returns ownership**: Returns `T` (not `&T`) to avoid lifetime issues
/// - **Lock-free concurrency**: `Arc` implementation doesn't need `Mutex`
///
/// # Automatically Implemented for Closures
///
/// All `Fn() -> T` closures automatically implement this trait,
/// enabling seamless integration with both raw closures and
/// wrapped supplier types.
///
/// # Examples
///
/// ## Using with Generic Functions
///
/// ```rust
/// use qubit_function::{Supplier, BoxSupplier};
///
/// fn call_twice<S: Supplier<i32>>(supplier: &S)
///     -> (i32, i32)
/// {
///     (supplier.get(), supplier.get())
/// }
///
/// let s = BoxSupplier::new(|| 42);
/// assert_eq!(call_twice(&s), (42, 42));
///
/// let closure = || 100;
/// assert_eq!(call_twice(&closure), (100, 100));
/// ```
///
/// ## Stateless Factory
///
/// ```rust
/// use qubit_function::{Supplier, SupplierOnce};
///
/// struct User {
///     name: String,
/// }
///
/// impl User {
///     fn new() -> Self {
///         User {
///             name: String::from("Default"),
///         }
///     }
/// }
///
/// let factory = || User::new();
/// let user1 = factory.get();
/// let user2 = factory.get();
/// // Each call creates a new User instance
/// ```
pub trait Supplier<T> {
    /// Generates and returns a value.
    ///
    /// Executes the underlying function and returns the generated
    /// value. Uses `&self` because the supplier doesn't modify its
    /// own state.
    ///
    /// # Returns
    ///
    /// The generated value of type `T`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Supplier, BoxSupplier};
    ///
    /// let supplier = BoxSupplier::new(|| 42);
    /// assert_eq!(supplier.get(), 42);
    /// assert_eq!(supplier.get(), 42);
    /// ```
    fn get(&self) -> T;
}

impl<T, F> Supplier<T> for F
where
    F: Fn() -> T,
{
    fn get(&self) -> T {
        self()
    }
}
