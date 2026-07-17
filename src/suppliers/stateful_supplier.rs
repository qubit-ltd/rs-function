// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # StatefulSupplier Types
//!
//! Provides stateful supplier implementations that generate and return values
//! without taking input while allowing mutable internal state.
//!
//! # Overview
//!
//! A **StatefulSupplier** is a functional abstraction equivalent to
//! `FnMut() -> T`: it generates values without accepting input and may update
//! its own internal state between calls. It is useful for counters,
//! sequences, generators, and memoized computations.
//!
//! For stateless factories and constants that only need `Fn() -> T`, use
//! [`Supplier`](crate::Supplier).
//!
//! # Core Design Principles
//!
//! 1. **Returns Ownership**: `StatefulSupplier` returns `T` (not `&T`) to avoid
//!    lifetime issues
//! 2. **Uses `&mut self`**: Typical scenarios (counters, generators) require
//!    state modification
//! 3. **Separate shared-receiver API**: `Supplier` covers `Fn` factories that
//!    do not require `&mut self`, while `StatefulSupplier` covers `FnMut`
//!    generation
//!
//! # Three Implementations
//!
//! - **`BoxStatefulSupplier<T>`**: Single ownership using `Box<dyn FnMut() ->
//!   T>`. Uses one heap allocation and dynamic dispatch, cannot be cloned.
//!
//! - **`ArcStatefulSupplier<T>`**: Thread-safe shared ownership using
//!   `Arc<Mutex<dyn FnMut() -> T + Send>>`. Can be cloned and sent across
//!   threads. Higher overhead due to locking.
//!
//! - **`RcStatefulSupplier<T>`**: Single-threaded shared ownership using
//!   `Rc<RefCell<dyn FnMut() -> T>>`. Can be cloned but not sent across
//!   threads. Lower overhead than `ArcStatefulSupplier`.
//!
//! # Comparison with Other Functional Abstractions
//!
//! | Type      | Input | Output | self      | Modifies? | Use Case      |
//! |-----------|-------|--------|-----------|-----------|---------------|
//! | Supplier  | None  | `T`    | `&self`   | No        | Factory       |
//! | StatefulSupplier | None | `T` | `&mut self` | Yes | Stateful factory |
//! | Consumer  | `&T`  | `()`   | `&self`   | No        | Observer      |
//! | Predicate | `&T`  | `bool` | `&self`   | No        | Filter        |
//! | Function  | `&T`  | `R`    | `&self`   | No        | Transform     |
//!
//! # Examples
//!
//! ## Basic Counter
//!
//! ```rust
//! use qubit_function::{BoxStatefulSupplier, StatefulSupplier};
//!
//! let mut counter = 0;
//! let mut supplier = BoxStatefulSupplier::new(move || {
//!     counter += 1;
//!     counter
//! });
//!
//! assert_eq!(supplier.get(), 1);
//! assert_eq!(supplier.get(), 2);
//! assert_eq!(supplier.get(), 3);
//! ```
//!
//! ## Method Chaining
//!
//!
//! ```rust
//! # {
//! use qubit_function::{BoxStatefulSupplier, StatefulSupplier};
//!
//! let mut pipeline = BoxStatefulSupplier::new(|| 10)
//!     .map(|x| x * 2)
//!     .map(|x| x + 5);
//!
//! assert_eq!(pipeline.get(), 25);
//! # }
//! ```
//!
//! ## Thread-safe Sharing
//!
//! ```rust
//! use qubit_function::{ArcStatefulSupplier, StatefulSupplier};
//! use std::sync::{Arc, Mutex};
//! use std::thread;
//!
//! let counter = Arc::new(Mutex::new(0));
//! let counter_clone = Arc::clone(&counter);
//!
//! let supplier = ArcStatefulSupplier::new(move || {
//!     let mut c = counter_clone.lock().expect("mutex should not be poisoned");
//!     *c += 1;
//!     *c
//! });
//!
//! let mut s1 = supplier.clone();
//! let mut s2 = supplier.clone();
//!
//! let h1 = thread::spawn(move || s1.get());
//! let h2 = thread::spawn(move || s2.get());
//!
//! let v1 = h1.join().expect("thread should not panic");
//! let v2 = h2.join().expect("thread should not panic");
//!
//! assert!(v1 != v2);
//! assert_eq!(*counter.lock().expect("mutex should not be poisoned"), 2);
//! ```

mod box_stateful_supplier;
pub use box_stateful_supplier::BoxStatefulSupplier;
#[cfg(feature = "rc")]
mod rc_stateful_supplier;
#[cfg(feature = "rc")]
pub use rc_stateful_supplier::RcStatefulSupplier;
mod arc_stateful_supplier;
pub use arc_stateful_supplier::ArcStatefulSupplier;

// ==========================================================================
// Supplier Trait
// ==========================================================================

/// Supplier trait: generates and returns values without input.
///
/// The core abstraction for value generation. Similar to Java's
/// `Supplier<T>` interface, it produces values without taking any
/// input parameters.
///
/// # Key Characteristics
///
/// - **No input parameters**: The caller supplies no arguments
/// - **Mutable access**: Uses `&mut self` to allow state changes
/// - **Returns ownership**: Returns `T` (not `&T`) to avoid lifetime issues
/// - **Can modify state**: Commonly used for counters, sequences, and
///   generators
///
/// # Automatically Implemented for Closures
///
/// All `FnMut() -> T` closures automatically implement this trait,
/// enabling seamless integration with both raw closures and wrapped
/// supplier types.
///
/// # Examples
///
/// ## Using with Generic Functions
///
/// ```rust
/// use qubit_function::{StatefulSupplier, BoxStatefulSupplier};
///
/// fn call_twice<S: StatefulSupplier<i32>>(supplier: &mut S) -> (i32, i32) {
///     (supplier.get(), supplier.get())
/// }
///
/// let mut s = BoxStatefulSupplier::new(|| 42);
/// assert_eq!(call_twice(&mut s), (42, 42));
///
/// let mut closure = || 100;
/// assert_eq!(call_twice(&mut closure), (100, 100));
/// ```
///
/// ## Stateful Supplier
///
/// ```rust
/// use qubit_function::StatefulSupplier;
///
/// let mut counter = 0;
/// let mut stateful = || {
///     counter += 1;
///     counter
/// };
///
/// assert_eq!(stateful.get(), 1);
/// assert_eq!(stateful.get(), 2);
/// ```
pub trait StatefulSupplier<T> {
    /// Generates and returns the next value.
    ///
    /// Executes the underlying function and returns the generated
    /// value. Uses `&mut self` because suppliers typically involve
    /// state changes (counters, sequences, etc.).
    ///
    /// # Returns
    ///
    /// The generated value of type `T`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{StatefulSupplier, BoxStatefulSupplier};
    ///
    /// let mut supplier = BoxStatefulSupplier::new(|| 42);
    /// assert_eq!(supplier.get(), 42);
    /// ```
    #[must_use = "the supplied value should be used"]
    fn get(&mut self) -> T;
}

crate::macros::impl_closure_trait!(
    StatefulSupplier<T>,
    get,
    FnMut() -> T
);
