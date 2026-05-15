/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
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
//! - **`BoxSupplier<T>`**: Single ownership using `Box<dyn
//!   Fn() -> T>`. Zero overhead, cannot be cloned. Best for
//!   one-time use in read-only contexts.
//!
//! - **`ArcSupplier<T>`**: Thread-safe shared ownership
//!   using `Arc<dyn Fn() -> T + Send + Sync>`. **Lock-free** - no
//!   Mutex needed! Can be cloned and sent across threads with
//!   excellent performance.
//!
//! - **`RcSupplier<T>`**: Single-threaded shared ownership
//!   using `Rc<dyn Fn() -> T>`. Can be cloned but not sent across
//!   threads. Lightweight alternative to `ArcSupplier`.
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
//! - `ArcStatefulSupplier<T>`: Requires `Mutex`, lock contention on
//!   every `get()` call.
//! - `ArcSupplier<T>`: Lock-free, can call `get()`
//!   concurrently without contention.
//!
//! Benchmark results show `ArcSupplier` can be **10x
//! faster** than `ArcStatefulSupplier` in high-concurrency stateless
//! scenarios.
//!

use std::rc::Rc;
use std::sync::Arc;

use crate::BoxSupplierOnce;
use crate::macros::{
    impl_arc_conversions,
    impl_box_conversions,
    impl_closure_trait,
    impl_rc_conversions,
};
use crate::predicates::predicate::Predicate;
use crate::suppliers::macros::{
    impl_box_supplier_methods,
    impl_shared_supplier_methods,
    impl_supplier_clone,
    impl_supplier_common_methods,
    impl_supplier_debug_display,
};
use crate::transformers::transformer::Transformer;

mod box_supplier;
pub use box_supplier::BoxSupplier;
mod arc_supplier;
pub use arc_supplier::ArcSupplier;
mod rc_supplier;
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
/// - **Returns ownership**: Returns `T` (not `&T`) to avoid
///   lifetime issues
/// - **Lock-free concurrency**: `Arc` implementation doesn't need
///   `Mutex`
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
///
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

    /// Converts to `BoxSupplier`.
    ///
    /// This method has a default implementation that wraps the
    /// supplier in a `BoxSupplier`. Custom implementations
    /// can override this method for optimization purposes.
    ///
    /// # Returns
    ///
    /// A new `BoxSupplier<T>` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Supplier, SupplierOnce};
    ///
    /// let closure = || 42;
    /// let boxed = Supplier::into_box(closure);
    /// assert_eq!(boxed.get(), 42);
    /// ```
    fn into_box(self) -> BoxSupplier<T>
    where
        Self: Sized + 'static,
    {
        BoxSupplier::new(move || self.get())
    }

    /// Converts to `RcSupplier`.
    ///
    /// This method has a default implementation that wraps the
    /// supplier in an `RcSupplier`. Custom implementations
    /// can override this method for optimization purposes.
    ///
    /// # Returns
    ///
    /// A new `RcSupplier<T>` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Supplier, SupplierOnce};
    ///
    /// let closure = || 42;
    /// let rc = closure.into_rc();
    /// assert_eq!(rc.get(), 42);
    /// ```
    fn into_rc(self) -> RcSupplier<T>
    where
        Self: Sized + 'static,
    {
        RcSupplier::new(move || self.get())
    }

    /// Converts to `ArcSupplier`.
    ///
    /// This method has a default implementation that wraps the
    /// supplier in an `ArcSupplier`. Custom implementations
    /// can override this method for optimization purposes.
    ///
    /// # Returns
    ///
    /// A new `ArcSupplier<T>` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Supplier, SupplierOnce};
    ///
    /// let closure = || 42;
    /// let arc = closure.into_arc();
    /// assert_eq!(arc.get(), 42);
    /// ```
    fn into_arc(self) -> ArcSupplier<T>
    where
        Self: Sized + Send + Sync + 'static,
    {
        ArcSupplier::new(move || self.get())
    }

    /// Converts to a closure implementing `Fn() -> T`.
    ///
    /// This method has a default implementation that wraps the
    /// supplier in a closure. Custom implementations can override
    /// this method for optimization purposes.
    ///
    /// # Returns
    ///
    /// A closure implementing `Fn() -> T`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Supplier, SupplierOnce};
    ///
    /// let closure = || 42;
    /// let fn_closure = Supplier::into_fn(closure);
    /// assert_eq!(fn_closure(), 42);
    /// assert_eq!(fn_closure(), 42);
    /// ```
    fn into_fn(self) -> impl Fn() -> T
    where
        Self: Sized + 'static,
    {
        move || self.get()
    }

    /// Converts to `BoxSupplierOnce`.
    ///
    /// This method has a default implementation that wraps the
    /// supplier in a `BoxSupplierOnce`. Custom implementations
    /// can override this method for optimization purposes.
    ///
    /// # Returns
    ///
    /// A new `BoxSupplierOnce<T>` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{Supplier, SupplierOnce};
    ///
    /// let closure = || 42;
    /// let once = closure.into_once();
    /// assert_eq!(once.get(), 42);
    /// ```
    fn into_once(self) -> BoxSupplierOnce<T>
    where
        Self: Sized + 'static,
    {
        BoxSupplierOnce::new(move || self.get())
    }

    /// Converts to `BoxSupplier` by cloning.
    ///
    /// This method clones the supplier and wraps it in a
    /// `BoxSupplier`. Requires `Self: Clone`. Custom
    /// implementations can override this method for optimization.
    ///
    /// # Returns
    ///
    /// A new `BoxSupplier<T>` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::Supplier;
    ///
    /// let closure = || 42;
    /// let boxed = closure.to_box();
    /// assert_eq!(boxed.get(), 42);
    /// ```
    fn to_box(&self) -> BoxSupplier<T>
    where
        Self: Clone + 'static,
    {
        self.clone().into_box()
    }

    /// Converts to `RcSupplier` by cloning.
    ///
    /// This method clones the supplier and wraps it in an
    /// `RcSupplier`. Requires `Self: Clone`. Custom
    /// implementations can override this method for optimization.
    ///
    /// # Returns
    ///
    /// A new `RcSupplier<T>` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::Supplier;
    ///
    /// let closure = || 42;
    /// let rc = closure.to_rc();
    /// assert_eq!(rc.get(), 42);
    /// ```
    fn to_rc(&self) -> RcSupplier<T>
    where
        Self: Clone + 'static,
    {
        self.clone().into_rc()
    }

    /// Converts to `ArcSupplier` by cloning.
    ///
    /// This method clones the supplier and wraps it in an
    /// `ArcSupplier`. Requires `Self: Clone + Send + Sync`.
    /// Custom implementations can override this method for
    /// optimization.
    ///
    /// # Returns
    ///
    /// A new `ArcSupplier<T>` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::Supplier;
    ///
    /// let closure = || 42;
    /// let arc = closure.to_arc();
    /// assert_eq!(arc.get(), 42);
    /// ```
    fn to_arc(&self) -> ArcSupplier<T>
    where
        Self: Clone + Send + Sync + 'static,
    {
        self.clone().into_arc()
    }

    /// Converts to a closure by cloning.
    ///
    /// This method clones the supplier and wraps it in a closure
    /// implementing `Fn() -> T`. Requires `Self: Clone`. Custom
    /// implementations can override this method for optimization.
    ///
    /// # Returns
    ///
    /// A closure implementing `Fn() -> T`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::Supplier;
    ///
    /// let closure = || 42;
    /// let fn_closure = closure.to_fn();
    /// assert_eq!(fn_closure(), 42);
    /// assert_eq!(fn_closure(), 42);
    /// ```
    fn to_fn(&self) -> impl Fn() -> T
    where
        Self: Clone + 'static,
    {
        self.clone().into_fn()
    }

    /// Converts to `BoxSupplierOnce` without consuming self
    ///
    /// **⚠️ Requires Clone**: This method requires `Self` to implement `Clone`.
    /// Clones the current supplier and converts the clone to a one-time supplier.
    ///
    /// # Returns
    ///
    /// Returns a `BoxSupplierOnce<T>`
    fn to_once(&self) -> BoxSupplierOnce<T>
    where
        Self: Clone + 'static,
    {
        self.clone().into_once()
    }
}
