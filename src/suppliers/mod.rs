/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Suppliers Module
//!
//! This module provides supplier-related functional programming abstractions
//! for producing values without input parameters.
//!
//! # Author
//!
//! Haixing Hu

#[doc(hidden)]
pub mod macros;
pub mod stateful_supplier;
pub mod supplier;
pub mod supplier_once;

pub use stateful_supplier::{
    ArcStatefulSupplier,
    BoxStatefulSupplier,
    FnStatefulSupplierOps,
    RcStatefulSupplier,
    StatefulSupplier,
};
pub use supplier::{
    ArcSupplier,
    BoxSupplier,
    RcSupplier,
    Supplier,
};
pub use supplier_once::{
    BoxSupplierOnce,
    SupplierOnce,
};
