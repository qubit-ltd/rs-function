// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # StatefulFunction Types
//!
//! Provides Rust implementations of stateful function traits for transforming
//! borrowed input values while allowing internal state modification.
//!
//! It is similar to the `FnMut(&T) -> R` trait in the standard library.
//!
//! This module provides the `StatefulFunction<T, R>` trait and three
//! implementations:
//!
//! - [`BoxStatefulFunction`]: Single ownership, not cloneable
//! - [`ArcStatefulFunction`]: Thread-safe shared ownership, cloneable
//! - [`RcStatefulFunction`]: Single-threaded shared ownership, cloneable

mod arc_conditional_stateful_function;
pub use arc_conditional_stateful_function::ArcConditionalStatefulFunction;
mod arc_stateful_function;
pub use arc_stateful_function::ArcStatefulFunction;
mod box_conditional_stateful_function;
pub use box_conditional_stateful_function::BoxConditionalStatefulFunction;
mod box_stateful_function;
pub use box_stateful_function::BoxStatefulFunction;
#[cfg(feature = "rc")]
mod rc_conditional_stateful_function;
#[cfg(feature = "rc")]
pub use rc_conditional_stateful_function::RcConditionalStatefulFunction;
#[cfg(feature = "rc")]
mod rc_stateful_function;
#[cfg(feature = "rc")]
pub use rc_stateful_function::RcStatefulFunction;
#[path = "stateful_function/stateful_function.rs"]
mod stateful_function_trait;
pub use stateful_function_trait::StatefulFunction;
