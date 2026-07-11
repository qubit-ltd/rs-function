// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Predicates Module
//!
//! This module provides predicate-related functional programming abstractions
//! for testing values and returning boolean results.

pub mod bi_predicate;
pub(crate) mod macros;
pub mod predicate;
#[cfg(feature = "stateful")]
pub mod stateful_bi_predicate;
#[cfg(feature = "stateful")]
pub mod stateful_predicate;

pub use bi_predicate::*;
pub use predicate::*;
#[cfg(feature = "stateful")]
pub use stateful_bi_predicate::*;
#[cfg(feature = "stateful")]
pub use stateful_predicate::*;
