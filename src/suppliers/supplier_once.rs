// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # SupplierOnce Types
//!
//! Provides one-time supplier implementations that generate and
//! return values without taking any input parameters, consuming
//! themselves in the process.
//!
//! # Overview
//!
//! A **SupplierOnce** is a functional abstraction similar to
//! `Supplier`, but can only be called once. The `get()` method
//! consumes `self`, ensuring the supplier cannot be reused.
//!
//! # Key Characteristics
//!
//! - **Single use**: Can only call `get()` once
//! - **Consumes self**: The method takes ownership of `self`
//! - **Holds `FnOnce`**: Can capture and move non-cloneable values
//! - **Type-system guaranteed**: Prevents multiple calls at compile time
//!
//! # Use Cases
//!
//! 1. **Lazy initialization**: Delay expensive computation until needed
//! 2. **One-time resource consumption**: Generate value by consuming a resource
//! 3. **Move-only closures**: Hold closures that capture moved values
//!
//! # Examples
//!
//! ## Lazy Initialization
//!
//! ```rust
//! use qubit_function::{BoxSupplierOnce, SupplierOnce};
//!
//! let once = BoxSupplierOnce::new(|| {
//!     println!("Expensive initialization");
//!     42
//! });
//!
//! let value = once.get(); // Only initializes once
//! assert_eq!(value, 42);
//! ```
//!
//! ## Moving Captured Values
//!
//! ```rust
//! use qubit_function::{BoxSupplierOnce, SupplierOnce};
//!
//! let resource = String::from("data");
//! let once = BoxSupplierOnce::new(move || resource);
//!
//! let value = once.get();
//! assert_eq!(value, "data");
//! ```
use crate::macros::impl_closure_once_trait;
#[cfg(feature = "combinators")]
use crate::predicates::predicate::Predicate;
use crate::suppliers::macros::{
    impl_box_supplier_methods,
    impl_supplier_common_methods,
    impl_supplier_debug_display,
};
#[cfg(feature = "combinators")]
use crate::transformers::transformer::Transformer;

mod box_supplier_once;
pub use box_supplier_once::BoxSupplierOnce;

// ==========================================================================
// SupplierOnce Trait
// ==========================================================================

/// One-time supplier trait: generates a value consuming self.
///
/// Similar to `Supplier`, but can only be called once. The `get()`
/// method consumes `self`, ensuring the supplier cannot be reused.
///
/// # Key Characteristics
///
/// - **Single use**: Can only call `get()` once
/// - **Consumes self**: The method takes ownership of `self`
/// - **Holds `FnOnce`**: Can capture and move non-cloneable values
/// - **Type-system guaranteed**: Prevents multiple calls at compile time
///
/// # Use Cases
///
/// 1. **Lazy initialization**: Delay expensive computation until needed
/// 2. **One-time resource consumption**: Generate value by consuming a resource
/// 3. **Move-only closures**: Hold closures that capture moved values
///
/// # Examples
///
/// ## Lazy Initialization
///
/// ```rust
/// use qubit_function::{BoxSupplierOnce, SupplierOnce};
///
/// let once = BoxSupplierOnce::new(|| {
///     println!("Expensive computation");
///     42
/// });
///
/// let value = once.get(); // Prints: Expensive computation
/// assert_eq!(value, 42);
/// // once is now consumed and cannot be used again
/// ```
///
/// ## Resource Consumption
///
/// ```rust
/// use qubit_function::{BoxSupplierOnce, SupplierOnce};
///
/// let resource = String::from("data");
/// let once = BoxSupplierOnce::new(move || {
///     resource // Move the resource
/// });
///
/// let value = once.get();
/// assert_eq!(value, "data");
/// ```
pub trait SupplierOnce<T> {
    /// Generates and returns the value, consuming self.
    ///
    /// This method can only be called once because it consumes
    /// `self`. This ensures type-system level guarantee that the
    /// supplier won't be called multiple times.
    ///
    /// # Returns
    ///
    /// The generated value of type `T`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BoxSupplierOnce, SupplierOnce};
    ///
    /// let once = BoxSupplierOnce::new(|| 42);
    /// assert_eq!(once.get(), 42);
    /// // once is consumed here
    /// ```
    fn get(self) -> T;
}
