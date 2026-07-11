// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Transformers Module
//!
//! This module provides transformer-related functional programming
//! abstractions for converting values from one type to another, including
//! single-parameter transformers, bi-transformers, and their stateful
//! variants.

pub mod bi_transformer;
#[cfg(feature = "once")]
pub mod bi_transformer_once;
pub(crate) mod macros;
#[cfg(feature = "stateful")]
pub mod stateful_bi_transformer;
#[cfg(feature = "stateful")]
pub mod stateful_transformer;
pub mod transformer;
#[cfg(feature = "once")]
pub mod transformer_once;

pub use bi_transformer::*;
#[cfg(feature = "once")]
pub use bi_transformer_once::*;
#[cfg(feature = "stateful")]
pub use stateful_bi_transformer::*;
#[cfg(feature = "stateful")]
pub use stateful_transformer::*;
pub use transformer::*;
#[cfg(feature = "once")]
pub use transformer_once::*;
