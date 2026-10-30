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
//! or sequences, use `StatefulSupplier`.
//!
//! # Key Differences from StatefulSupplier
//!
//! | Aspect | `Supplier<T>` | `StatefulSupplier<T>` |
//! |--------|---------------|----------------------|
//! | self signature | `&self` | `&mut self` |
//! | Closure type | `Fn() -> T` | `FnMut() -> T` |
//! | Receiver required to call | Shared (`&self`) | Mutable (`&mut self`) |
//! | Arc implementation | `Arc<dyn Fn() -> T + Send + Sync>` | `Arc<Mutex<dyn FnMut() -> T + Send>>` |
//! | Use cases | Factory, constant, high concurrency | Counter, sequence, generator |
//!
//! # Three Implementations
//!
//! - **`BoxSupplier<T>`**: Single ownership using `Box<dyn Fn() -> T>`. It uses
//!   one heap allocation and dynamic dispatch and cannot be cloned.
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
//! # Concurrency Characteristics
//!
//! For stateless scenarios in multi-threaded environments:
//!
//! - `ArcStatefulSupplier<T>`: Requires `Mutex`, lock contention on every
//!   `get()` call.
//! - `ArcSupplier<T>`: Lock-free, can call `get()` concurrently without
//!   contention.
//!
//! Actual performance depends on the callback and contention pattern; measure
//! the workload when the distinction matters.

mod box_supplier;
pub use box_supplier::BoxSupplier;
mod arc_supplier;
pub use arc_supplier::ArcSupplier;
#[path = "supplier/supplier.rs"]
mod supplier_trait;
pub use supplier_trait::Supplier;
#[cfg(feature = "rc")]
mod rc_supplier;
#[cfg(feature = "rc")]
pub use rc_supplier::RcSupplier;
