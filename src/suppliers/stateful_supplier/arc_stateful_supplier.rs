/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
// qubit-style: allow explicit-imports
//! Defines the `ArcStatefulSupplier` public type.

use super::{
    Arc,
    BoxStatefulSupplier,
    BoxSupplierOnce,
    Mutex,
    Predicate,
    RcStatefulSupplier,
    StatefulSupplier,
    Transformer,
    impl_arc_conversions,
    impl_closure_trait,
    impl_shared_supplier_methods,
    impl_supplier_clone,
    impl_supplier_debug_display,
};

// ==========================================================================
// ArcStatefulSupplier - Thread-safe Shared Ownership Implementation
// ==========================================================================

/// Thread-safe shared ownership supplier.
///
/// Uses `Arc<Mutex<dyn FnMut() -> T + Send>>` for thread-safe
/// shared ownership. Can be cloned and sent across threads.
///
/// # Ownership Model
///
/// Methods borrow `&self` instead of consuming `self`. The original
/// supplier remains usable after method calls:
///
/// ```rust
/// use qubit_function::{ArcStatefulSupplier, StatefulSupplier};
///
/// let source = ArcStatefulSupplier::new(|| 10);
/// let mapped = source.map(|x| x * 2);
/// // source is still usable here!
/// ```
///
/// # Examples
///
/// ## Thread-safe Counter
///
/// ```rust
/// use qubit_function::{ArcStatefulSupplier, StatefulSupplier};
/// use std::sync::{Arc, Mutex};
/// use std::thread;
///
/// let counter = Arc::new(Mutex::new(0));
/// let counter_clone = Arc::clone(&counter);
///
/// let supplier = ArcStatefulSupplier::new(move || {
///     let mut c = counter_clone.lock().expect("mutex should not be poisoned");
///     *c += 1;
///     *c
/// });
///
/// let mut s1 = supplier.clone();
/// let mut s2 = supplier.clone();
///
/// let h1 = thread::spawn(move || s1.get());
/// let h2 = thread::spawn(move || s2.get());
///
/// let v1 = h1.join().expect("thread should not panic");
/// let v2 = h2.join().expect("thread should not panic");
/// assert!(v1 != v2);
/// ```
///
/// ## Reusable Transformations
///
/// ```rust
/// use qubit_function::{ArcStatefulSupplier, StatefulSupplier};
///
/// let base = ArcStatefulSupplier::new(|| 10);
/// let doubled = base.map(|x| x * 2);
/// let tripled = base.map(|x| x * 3);
///
/// // All remain usable
/// let mut b = base;
/// let mut d = doubled;
/// let mut t = tripled;
/// assert_eq!(b.get(), 10);
/// assert_eq!(d.get(), 20);
/// assert_eq!(t.get(), 30);
/// ```
///
pub struct ArcStatefulSupplier<T> {
    pub(super) function: Arc<Mutex<dyn FnMut() -> T + Send>>,
    pub(super) name: Option<String>,
}

impl<T> ArcStatefulSupplier<T> {
    // Generates: new(), new_with_name(), name(), set_name()
    // Note: constant() is NOT generated here, implemented separately below
    crate::macros::impl_common_new_methods!(
        (FnMut() -> T + Send + 'static),
        |f| Arc::new(Mutex::new(f)),
        "supplier"
    );

    crate::macros::impl_common_name_methods!("supplier");

    // Generates: map(), filter(), zip()
    impl_shared_supplier_methods!(ArcStatefulSupplier<T>, StatefulSupplier, (arc));
}

// Separate impl block for constant() and memoize() with stricter T: Send bound
impl<T> ArcStatefulSupplier<T> {
    /// Creates a supplier that returns a constant value.
    ///
    /// Creates a supplier that always returns the same value. Useful for
    /// default values or placeholder implementations.
    ///
    /// **Note:** This method requires `T: Send` because the constant value
    /// is captured by a `FnMut` closure which will be stored in an `Arc<Mutex<...>>`.
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
    /// use qubit_function::{ArcStatefulSupplier, StatefulSupplier};
    ///
    /// let mut supplier = ArcStatefulSupplier::constant(42);
    /// assert_eq!(supplier.get(), 42);
    /// assert_eq!(supplier.get(), 42); // Can be called multiple times
    /// ```
    pub fn constant(value: T) -> Self
    where
        T: Clone + Send + 'static,
    {
        Self::new(move || value.clone())
    }

    /// Creates a memoizing supplier.
    ///
    /// **Note:** This method requires `T: Send` because the cached value
    /// needs to be shared across threads via `Arc<Mutex<...>>`.
    ///
    /// # Returns
    ///
    /// A new memoized `ArcStatefulSupplier<T>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{ArcStatefulSupplier, StatefulSupplier};
    /// use std::sync::{Arc, Mutex};
    ///
    /// let call_count = Arc::new(Mutex::new(0));
    /// let call_count_clone = Arc::clone(&call_count);
    /// let source = ArcStatefulSupplier::new(move || {
    ///     let mut c = call_count_clone.lock().expect("mutex should not be poisoned");
    ///     *c += 1;
    ///     42
    /// });
    /// let memoized = source.memoize();
    ///
    /// let mut s = memoized;
    /// assert_eq!(s.get(), 42); // Calls underlying function
    /// assert_eq!(s.get(), 42); // Returns cached value
    /// assert_eq!(*call_count.lock().expect("mutex should not be poisoned"), 1);
    /// ```
    pub fn memoize(&self) -> ArcStatefulSupplier<T>
    where
        T: Clone + Send + 'static,
    {
        let self_fn = Arc::clone(&self.function);
        let cache: Arc<Mutex<Option<T>>> = Arc::new(Mutex::new(None));
        ArcStatefulSupplier {
            function: Arc::new(Mutex::new(move || {
                let mut cache_guard = cache.lock();
                if let Some(ref cached) = *cache_guard {
                    cached.clone()
                } else {
                    let value = self_fn.lock()();
                    *cache_guard = Some(value.clone());
                    value
                }
            })),
            name: None,
        }
    }
}

// Generates: Debug and Display implementations for ArcStatefulSupplier<T>
impl_supplier_debug_display!(ArcStatefulSupplier<T>);

// Generates: Clone implementation for ArcStatefulSupplier<T>
impl_supplier_clone!(ArcStatefulSupplier<T>);

impl<T> StatefulSupplier<T> for ArcStatefulSupplier<T> {
    fn get(&mut self) -> T {
        (self.function.lock())()
    }

    // Use macro to implement conversion methods
    impl_arc_conversions!(
        ArcStatefulSupplier<T>,
        BoxStatefulSupplier,
        RcStatefulSupplier,
        BoxSupplierOnce,
        FnMut() -> T
    );
}

// ==========================================================================
// Implement StatefulSupplier for Closures
// ==========================================================================

// Implement StatefulSupplier<T> for any type that implements FnMut() -> T
impl_closure_trait!(
    StatefulSupplier<T>,
    get,
    BoxSupplierOnce,
    FnMut() -> T
);
