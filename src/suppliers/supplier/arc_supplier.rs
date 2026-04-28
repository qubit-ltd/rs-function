/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `ArcSupplier` public type.

#![allow(unused_imports)]

use super::*;

// ======================================================================
// ArcSupplier - Thread-safe Shared Ownership Implementation
// ======================================================================

/// Thread-safe shared ownership stateless supplier.
///
/// Uses `Arc<dyn Fn() -> T + Send + Sync>` for thread-safe shared
/// ownership. **Lock-free** - no `Mutex` needed! Can be cloned and
/// sent across threads with excellent concurrent performance.
///
/// # Ownership Model
///
/// Methods borrow `&self` instead of consuming `self`. The
/// original supplier remains usable after method calls:
///
/// ```rust
/// use qubit_function::{ArcSupplier, Supplier};
///
/// let source = ArcSupplier::new(|| 10);
/// let mapped = source.map(|x| x * 2);
/// // source is still usable here!
/// ```
///
/// # Lock-Free Performance
///
/// Unlike `ArcStatefulSupplier`, this implementation doesn't need `Mutex`.
/// Multiple threads can call `get()` concurrently without lock
/// contention, making it ideal for high-concurrency scenarios.
///
/// # Examples
///
/// ## Thread-safe Factory
///
/// ```rust
/// use qubit_function::{ArcSupplier, Supplier};
/// use std::thread;
///
/// let factory = ArcSupplier::new(|| {
///     String::from("Hello")
/// });
///
/// let f1 = factory.clone();
/// let f2 = factory.clone();
///
/// let h1 = thread::spawn(move || f1.get());
/// let h2 = thread::spawn(move || f2.get());
///
/// assert_eq!(h1.join().unwrap(), "Hello");
/// assert_eq!(h2.join().unwrap(), "Hello");
/// ```
///
/// ## Reusable Transformations
///
/// ```rust
/// use qubit_function::{ArcSupplier, Supplier};
///
/// let base = ArcSupplier::new(|| 10);
/// let doubled = base.map(|x| x * 2);
/// let tripled = base.map(|x| x * 3);
///
/// // All remain usable
/// assert_eq!(base.get(), 10);
/// assert_eq!(doubled.get(), 20);
/// assert_eq!(tripled.get(), 30);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct ArcSupplier<T> {
    pub(super) function: Arc<dyn Fn() -> T + Send + Sync>,
    pub(super) name: Option<String>,
}

impl<T> ArcSupplier<T> {
    // Generates: new(), new_with_name(), name(), set_name()
    // Note: constant() is NOT generated here, implemented separately below
    crate::macros::impl_common_new_methods!(
        (Fn() -> T + Send + Sync + 'static),
        |f| Arc::new(f),
        "supplier"
    );

    crate::macros::impl_common_name_methods!("supplier");

    // Generates: map(), filter(), zip()
    impl_shared_supplier_methods!(ArcSupplier<T>, Supplier, (arc));
}

// Separate impl block for constant() with stricter T: Sync bound
impl<T> ArcSupplier<T> {
    /// Creates a supplier that returns a constant value.
    ///
    /// Creates a supplier that always returns the same value. Useful for
    /// default values or placeholder implementations.
    ///
    /// **Note:** This method requires `T: Sync` because the constant value
    /// is captured by a `Fn` closure which needs to be `Sync` for `Arc`.
    ///
    /// # Parameters
    ///
    /// * `value` - The constant value to return
    ///
    /// # Returns
    ///
    /// Returns a new supplier instance that returns the constant value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{ArcSupplier, Supplier};
    ///
    /// let supplier = ArcSupplier::constant(42);
    /// assert_eq!(supplier.get(), 42);
    /// assert_eq!(supplier.get(), 42); // Can be called multiple times
    /// ```
    pub fn constant(value: T) -> Self
    where
        T: Clone + Send + Sync + 'static,
    {
        Self::new(move || value.clone())
    }
}

// Generates: Debug and Display implementations for ArcSupplier<T>
impl_supplier_debug_display!(ArcSupplier<T>);

// Generates: Clone implementation for ArcSupplier<T>
impl_supplier_clone!(ArcSupplier<T>);

impl<T> Supplier<T> for ArcSupplier<T> {
    fn get(&self) -> T {
        (self.function)()
    }

    // Use macro to implement conversion methods
    impl_arc_conversions!(
        ArcSupplier<T>,
        BoxSupplier,
        RcSupplier,
        BoxSupplierOnce,
        Fn() -> T
    );
}
