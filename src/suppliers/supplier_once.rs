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
//! let value = once.get();
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

#[path = "supplier_once/supplier_once.rs"]
mod supplier_once_trait;
pub use supplier_once_trait::SupplierOnce;
mod box_supplier_once;
pub use box_supplier_once::BoxSupplierOnce;
