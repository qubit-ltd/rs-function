// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Mutators Module
//!
//! This module provides mutator-related functional programming abstractions
//! for modifying values in-place through mutable references.

pub(crate) mod macros;
pub mod mutator;
#[cfg(feature = "once")]
pub mod mutator_once;
#[cfg(feature = "stateful")]
pub mod stateful_mutator;

pub use mutator::*;
#[cfg(feature = "once")]
pub use mutator_once::*;
#[cfg(feature = "stateful")]
pub use stateful_mutator::*;
