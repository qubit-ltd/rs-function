/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
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
//! 1. **Returns Ownership**: `StatefulSupplier` returns `T` (not `&T`) to
//!    avoid lifetime issues
//! 2. **Uses `&mut self`**: Typical scenarios (counters, generators)
//!    require state modification
//! 3. **Separate stateless API**: `Supplier` covers lock-free stateless
//!    factories, while `StatefulSupplier` covers stateful generation
//!
//! # Three Implementations
//!
//! - **`BoxStatefulSupplier<T>`**: Single ownership using `Box<dyn FnMut()
//!   -> T>`. Zero overhead, cannot be cloned. Best for one-time use
//!   and builder patterns.
//!
//! - **`ArcStatefulSupplier<T>`**: Thread-safe shared ownership using
//!   `Arc<Mutex<dyn FnMut() -> T + Send>>`. Can be cloned and sent
//!   across threads. Higher overhead due to locking.
//!
//! - **`RcStatefulSupplier<T>`**: Single-threaded shared ownership using
//!   `Rc<RefCell<dyn FnMut() -> T>>`. Can be cloned but not sent
//!   across threads. Lower overhead than `ArcStatefulSupplier`.
//!
//! # Comparison with Other Functional Abstractions
//!
//! | Type      | Input | Output | self      | Modifies? | Use Case      |
//! |-----------|-------|--------|-----------|-----------|---------------|
//! | Supplier  | None  | `T`    | `&mut`    | Yes       | Factory       |
//! | Consumer  | `&T`  | `()`   | `&mut`    | Yes       | Observer      |
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
//! ```rust
//! use qubit_function::{BoxStatefulSupplier, StatefulSupplier};
//!
//! let mut pipeline = BoxStatefulSupplier::new(|| 10)
//!     .map(|x| x * 2)
//!     .map(|x| x + 5);
//!
//! assert_eq!(pipeline.get(), 25);
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
//!     let mut c = counter_clone.lock().unwrap();
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
//! let v1 = h1.join().unwrap();
//! let v2 = h2.join().unwrap();
//!
//! assert!(v1 != v2);
//! assert_eq!(*counter.lock().unwrap(), 2);
//! ```
//!
//! # Author
//!
//! Haixing Hu
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::macros::{
    impl_arc_conversions,
    impl_box_conversions,
    impl_closure_trait,
    impl_rc_conversions,
};
use crate::predicates::predicate::Predicate;
use crate::suppliers::{
    macros::{
        impl_box_supplier_methods,
        impl_shared_supplier_methods,
        impl_supplier_clone,
        impl_supplier_common_methods,
        impl_supplier_debug_display,
    },
    supplier_once::BoxSupplierOnce,
};
use crate::transformers::transformer::Transformer;

mod box_stateful_supplier;
pub use box_stateful_supplier::BoxStatefulSupplier;
mod rc_stateful_supplier;
pub use rc_stateful_supplier::RcStatefulSupplier;
mod arc_stateful_supplier;
pub use arc_stateful_supplier::ArcStatefulSupplier;
mod fn_stateful_supplier_ops;
pub use fn_stateful_supplier_ops::FnStatefulSupplierOps;

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
/// - **No input parameters**: Pure value generation
/// - **Mutable access**: Uses `&mut self` to allow state changes
/// - **Returns ownership**: Returns `T` (not `&T`) to avoid lifetime
///   issues
/// - **Can modify state**: Commonly used for counters, sequences,
///   and generators
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
///
/// # Author
///
/// Haixing Hu
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
    fn get(&mut self) -> T;

    /// Converts to `BoxStatefulSupplier`.
    ///
    /// This method has a default implementation that wraps the
    /// supplier in a `BoxStatefulSupplier`. Custom implementations can
    /// override this for more efficient conversions.
    ///
    /// # Returns
    ///
    /// A new `BoxStatefulSupplier<T>` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{StatefulSupplier, SupplierOnce};
    ///
    /// let closure = || 42;
    /// let mut boxed = StatefulSupplier::into_box(closure);
    /// assert_eq!(boxed.get(), 42);
    /// ```
    fn into_box(mut self) -> BoxStatefulSupplier<T>
    where
        Self: Sized + 'static,
    {
        BoxStatefulSupplier::new(move || self.get())
    }

    /// Converts to `RcStatefulSupplier`.
    ///
    /// This method has a default implementation that wraps the
    /// supplier in an `RcStatefulSupplier`. Custom implementations can
    /// override this for more efficient conversions.
    ///
    /// # Returns
    ///
    /// A new `RcStatefulSupplier<T>` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{StatefulSupplier, SupplierOnce};
    ///
    /// let closure = || 42;
    /// let mut rc = closure.into_rc();
    /// assert_eq!(rc.get(), 42);
    /// ```
    fn into_rc(mut self) -> RcStatefulSupplier<T>
    where
        Self: Sized + 'static,
    {
        RcStatefulSupplier::new(move || self.get())
    }

    /// Converts to `ArcStatefulSupplier`.
    ///
    /// This method has a default implementation that wraps the
    /// supplier in an `ArcStatefulSupplier`. Custom implementations can
    /// override this for more efficient conversions.
    ///
    /// # Returns
    ///
    /// A new `ArcStatefulSupplier<T>` instance
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{StatefulSupplier, SupplierOnce};
    ///
    /// let closure = || 42;
    /// let mut arc = closure.into_arc();
    /// assert_eq!(arc.get(), 42);
    /// ```
    fn into_arc(mut self) -> ArcStatefulSupplier<T>
    where
        Self: Sized + Send + 'static,
    {
        ArcStatefulSupplier::new(move || self.get())
    }

    /// Converts to a closure `FnMut() -> T`.
    ///
    /// This method wraps the supplier in a closure that calls the
    /// `get()` method when invoked. This allows using suppliers
    /// in contexts that expect `FnMut()` closures.
    ///
    /// # Returns
    ///
    /// A closure `impl FnMut() -> T`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{StatefulSupplier, BoxStatefulSupplier};
    ///
    /// let supplier = BoxStatefulSupplier::new(|| 42);
    /// let mut closure = supplier.into_fn();
    /// assert_eq!(closure(), 42);
    /// assert_eq!(closure(), 42);
    /// ```
    ///
    /// ## Using with functions that expect FnMut
    ///
    /// ```rust
    /// use qubit_function::{StatefulSupplier, BoxStatefulSupplier};
    ///
    /// fn call_fn_twice<F: FnMut() -> i32>(mut f: F) -> (i32, i32) {
    ///     (f(), f())
    /// }
    ///
    /// let supplier = BoxStatefulSupplier::new(|| 100);
    /// let closure = supplier.into_fn();
    /// assert_eq!(call_fn_twice(closure), (100, 100));
    /// ```
    fn into_fn(mut self) -> impl FnMut() -> T
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
    /// use qubit_function::{StatefulSupplier, SupplierOnce};
    ///
    /// let closure = || 42;
    /// let once = closure.into_once();
    /// assert_eq!(once.get(), 42);
    /// ```
    fn into_once(mut self) -> BoxSupplierOnce<T>
    where
        Self: Sized + 'static,
    {
        BoxSupplierOnce::new(move || self.get())
    }

    /// Creates a `BoxStatefulSupplier` from a cloned supplier.
    ///
    /// Uses `Clone` to obtain an owned copy and converts it into a
    /// `BoxStatefulSupplier`. Implementations can override this for a more
    /// efficient conversion.
    fn to_box(&self) -> BoxStatefulSupplier<T>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_box()
    }

    /// Creates an `RcStatefulSupplier` from a cloned supplier.
    ///
    /// Uses `Clone` to obtain an owned copy and converts it into an
    /// `RcStatefulSupplier`. Implementations can override it for better
    /// performance.
    fn to_rc(&self) -> RcStatefulSupplier<T>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_rc()
    }

    /// Creates an `ArcStatefulSupplier` from a cloned supplier.
    ///
    /// Requires the supplier and produced values to be `Send` so the
    /// resulting supplier can be shared across threads.
    fn to_arc(&self) -> ArcStatefulSupplier<T>
    where
        Self: Clone + Sized + Send + 'static,
    {
        self.clone().into_arc()
    }

    /// Creates a closure from a cloned supplier.
    ///
    /// The default implementation clones `self` and consumes the clone
    /// to produce a closure. Concrete suppliers can override it to
    /// avoid the additional clone.
    fn to_fn(&self) -> impl FnMut() -> T
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_fn()
    }

    /// Creates a `BoxSupplierOnce` from a cloned supplier
    ///
    /// Uses `Clone` to obtain an owned copy and converts it into a
    /// `BoxSupplierOnce`. Requires `Self: Clone`. Custom implementations
    /// can override this for better performance.
    fn to_once(&self) -> BoxSupplierOnce<T>
    where
        Self: Clone + Sized + 'static,
    {
        self.clone().into_once()
    }
}
