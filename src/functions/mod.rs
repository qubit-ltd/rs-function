// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Functions Module
//!
//! This module provides function-related functional programming abstractions
//! for transforming values from one type to another with reference semantics.

pub mod bi_function;
#[cfg(feature = "once")]
pub mod bi_function_once;
pub mod bi_mutating_function;
#[cfg(feature = "once")]
pub mod bi_mutating_function_once;
pub mod function;
#[cfg(feature = "once")]
pub mod function_once;
pub(crate) mod macros;
pub mod mutating_function;
#[cfg(feature = "once")]
pub mod mutating_function_once;
#[cfg(feature = "stateful")]
pub mod stateful_function;
#[cfg(feature = "stateful")]
pub mod stateful_mutating_function;

pub use bi_function::*;
#[cfg(feature = "once")]
pub use bi_function_once::*;
pub use bi_mutating_function::*;
#[cfg(feature = "once")]
pub use bi_mutating_function_once::*;
pub use function::*;
#[cfg(feature = "once")]
pub use function_once::*;
pub use mutating_function::*;
#[cfg(feature = "once")]
pub use mutating_function_once::*;
#[cfg(feature = "stateful")]
pub use stateful_function::*;
#[cfg(feature = "stateful")]
pub use stateful_mutating_function::*;
