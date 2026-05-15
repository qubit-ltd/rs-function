/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Suppliers Module
//!
//! This module provides supplier-related functional programming abstractions
//! for producing values without input parameters.
//!

pub(crate) mod macros;
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
