// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Suppliers Module
//!
//! This module provides supplier-related functional programming abstractions
//! for producing values without input parameters.

pub(crate) mod macros;
#[cfg(feature = "stateful")]
pub mod stateful_supplier;
pub mod supplier;
#[cfg(feature = "once")]
pub mod supplier_once;

#[cfg(feature = "stateful")]
pub use stateful_supplier::*;
pub use supplier::*;
#[cfg(feature = "once")]
pub use supplier_once::*;
