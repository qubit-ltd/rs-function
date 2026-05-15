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
//! Defines the `FnStatefulSupplierOps` public type.

use super::{
    BoxStatefulSupplier,
    Predicate,
    StatefulSupplier,
    Transformer,
};

// ==========================================================================
// Extension Trait for Closure Operations
// ==========================================================================

/// Extension trait providing supplier operations for closures
///
/// Provides composition methods (`map`, `filter`, `zip`, `memoize`) for
/// closures implementing `FnMut() -> T` without requiring explicit
/// wrapping in `BoxStatefulSupplier`.
///
/// This trait is automatically implemented for all closures and function
/// pointers that implement `FnMut() -> T`.
///
/// # Design Rationale
///
/// While closures automatically implement `Supplier<T>` through blanket
/// implementation, they don't have access to instance methods like
/// `map`, `filter`, and `zip`. This extension trait provides those
/// methods, returning `BoxStatefulSupplier` for maximum flexibility.
///
/// # Examples
///
/// ## Map transformation
///
/// ```rust
/// use qubit_function::{StatefulSupplier, FnStatefulSupplierOps};
///
/// let mut counter = 0;
/// let mut mapped = (move || {
///     counter += 1;
///     counter
/// }).map(|x| x * 2);
///
/// assert_eq!(mapped.get(), 2);
/// assert_eq!(mapped.get(), 4);
/// ```
///
/// ## Filter values
///
/// ```rust
/// use qubit_function::{StatefulSupplier, FnStatefulSupplierOps};
///
/// let mut counter = 0;
/// let mut filtered = (move || {
///     counter += 1;
///     counter
/// }).filter(|x: &i32| x % 2 == 0);
///
/// assert_eq!(filtered.get(), None);     // 1 is odd
/// assert_eq!(filtered.get(), Some(2));  // 2 is even
/// ```
///
/// ## Combine with zip
///
/// ```rust
/// use qubit_function::{StatefulSupplier, FnStatefulSupplierOps, BoxStatefulSupplier};
///
/// let first = || 42;
/// let second = BoxStatefulSupplier::new(|| "hello");
/// let mut zipped = first.zip(second);
///
/// assert_eq!(zipped.get(), (42, "hello"));
/// ```
///
pub trait FnStatefulSupplierOps<T>: FnMut() -> T + Sized {
    /// Maps the output using a transformation function.
    ///
    /// Consumes the closure and returns a new supplier that applies
    /// the mapper to each output.
    ///
    /// # Parameters
    ///
    /// * `mapper` - The mapper to apply to the output
    ///
    /// # Returns
    ///
    /// A new mapped `BoxStatefulSupplier<U>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{StatefulSupplier, FnStatefulSupplierOps};
    ///
    /// let mut mapped = (|| 10)
    ///     .map(|x| x * 2)
    ///     .map(|x| x + 5);
    /// assert_eq!(mapped.get(), 25);
    /// ```
    fn map<U, M>(self, mapper: M) -> BoxStatefulSupplier<U>
    where
        Self: 'static,
        M: Transformer<T, U> + 'static,
        U: 'static,
        T: 'static,
    {
        BoxStatefulSupplier::new(self).map(mapper)
    }

    /// Filters output based on a predicate.
    ///
    /// Returns a new supplier that returns `Some(value)` if the
    /// predicate is satisfied, `None` otherwise.
    ///
    /// # Parameters
    ///
    /// * `predicate` - The predicate to test the supplied value
    ///
    /// # Returns
    ///
    /// A new filtered `BoxStatefulSupplier<Option<T>>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{StatefulSupplier, FnStatefulSupplierOps};
    ///
    /// let mut counter = 0;
    /// let mut filtered = (move || {
    ///     counter += 1;
    ///     counter
    /// }).filter(|x: &i32| x % 2 == 0);
    ///
    /// assert_eq!(filtered.get(), None);     // 1 is odd
    /// assert_eq!(filtered.get(), Some(2));  // 2 is even
    /// ```
    fn filter<P>(self, predicate: P) -> BoxStatefulSupplier<Option<T>>
    where
        Self: 'static,
        P: Predicate<T> + 'static,
        T: 'static,
    {
        BoxStatefulSupplier::new(self).filter(predicate)
    }

    /// Combines this supplier with another, producing a tuple.
    ///
    /// Consumes both suppliers and returns a new supplier that
    /// produces `(T, U)` tuples.
    ///
    /// # Parameters
    ///
    /// * `other` - The other supplier to combine with. Can be any type
    ///   implementing `Supplier<U>`
    ///
    /// # Returns
    ///
    /// A new `BoxStatefulSupplier<(T, U)>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{StatefulSupplier, FnStatefulSupplierOps, BoxStatefulSupplier};
    ///
    /// let first = || 42;
    /// let second = BoxStatefulSupplier::new(|| "hello");
    /// let mut zipped = first.zip(second);
    ///
    /// assert_eq!(zipped.get(), (42, "hello"));
    /// ```
    fn zip<S, U>(self, other: S) -> BoxStatefulSupplier<(T, U)>
    where
        Self: 'static,
        S: StatefulSupplier<U> + 'static,
        U: 'static,
        T: 'static,
    {
        BoxStatefulSupplier::new(self).zip(other)
    }

    /// Creates a memoizing supplier.
    ///
    /// Returns a new supplier that caches the first value it
    /// produces. All subsequent calls return the cached value.
    ///
    /// # Returns
    ///
    /// A new memoized `BoxStatefulSupplier<T>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{StatefulSupplier, FnStatefulSupplierOps};
    ///
    /// let mut call_count = 0;
    /// let mut memoized = (move || {
    ///     call_count += 1;
    ///     42
    /// }).memoize();
    ///
    /// assert_eq!(memoized.get(), 42); // Calls underlying function
    /// assert_eq!(memoized.get(), 42); // Returns cached value
    /// ```
    fn memoize(self) -> BoxStatefulSupplier<T>
    where
        Self: 'static,
        T: Clone + 'static,
    {
        BoxStatefulSupplier::new(self).memoize()
    }
}

// Implement the extension trait for all closures
impl<T, F> FnStatefulSupplierOps<T> for F where F: FnMut() -> T + Sized {}
