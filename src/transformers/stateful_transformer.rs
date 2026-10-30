// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # StatefulTransformer Types
//!
//! Provides Rust implementations of stateful transformer traits for stateful
//! value transformation. StatefulTransformers consume input values (taking
//! ownership) and produce output values while allowing internal state
//! modification. This is analogous to `FnMut(T) -> R` in Rust's standard
//! library.
//!
//! This module provides the `StatefulTransformer<T, R>` trait and three
//! implementations:
//!
//! - [`BoxStatefulTransformer`]: Single ownership, not cloneable
//! - [`ArcStatefulTransformer`]: Thread-safe shared ownership, cloneable
//! - `RcStatefulTransformer`: Single-threaded shared ownership, cloneable;
//!   available with the `rc` feature.

mod box_stateful_transformer;
pub use box_stateful_transformer::BoxStatefulTransformer;
#[cfg(feature = "rc")]
mod rc_stateful_transformer;
#[cfg(feature = "rc")]
pub use rc_stateful_transformer::RcStatefulTransformer;
mod arc_stateful_transformer;
pub use arc_stateful_transformer::ArcStatefulTransformer;
mod box_conditional_stateful_transformer;
pub use box_conditional_stateful_transformer::BoxConditionalStatefulTransformer;
#[cfg(feature = "rc")]
mod rc_conditional_stateful_transformer;
#[cfg(feature = "rc")]
pub use rc_conditional_stateful_transformer::RcConditionalStatefulTransformer;
mod arc_conditional_stateful_transformer;
pub use arc_conditional_stateful_transformer::ArcConditionalStatefulTransformer;

#[path = "stateful_transformer/stateful_transformer.rs"]
mod stateful_transformer_trait;
pub use stateful_transformer_trait::StatefulTransformer;
