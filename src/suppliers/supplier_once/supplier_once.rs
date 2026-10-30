// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `SupplierOnce` trait.

/// A supplier that produces one value when consumed.
///
/// Unlike [`crate::Supplier`], this trait consumes its implementation when
/// `get` is called. Implementations can therefore move captured, non-cloneable
/// values into the generated result.
///
/// # Type Parameters
///
/// - `T`: The type of value generated when the supplier is consumed.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BoxSupplierOnce, SupplierOnce};
///
/// let supplier = BoxSupplierOnce::new(|| 42);
/// let value = supplier.get();
/// assert_eq!(value, 42);
/// ```
pub trait SupplierOnce<T> {
    /// Generates and returns the value, consuming the supplier.
    ///
    /// Consuming `self` makes the single-use contract enforceable by the type
    /// system. The returned value is the result of the stored computation.
    ///
    /// # Returns
    ///
    /// The generated value of type `T`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BoxSupplierOnce, SupplierOnce};
    ///
    /// let supplier = BoxSupplierOnce::new(|| 42);
    /// assert_eq!(supplier.get(), 42);
    /// ```
    #[must_use = "the supplied value should be used"]
    fn get(self) -> T;
}
