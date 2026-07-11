// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Consumers Module
//!
//! This module provides consumer-related functional programming abstractions,
//! including single-parameter consumers, bi-consumers, and their stateful
//! variants.

pub mod bi_consumer;
#[cfg(feature = "once")]
pub mod bi_consumer_once;
pub mod consumer;
#[cfg(feature = "once")]
pub mod consumer_once;
pub(crate) mod macros;
#[cfg(feature = "stateful")]
pub mod stateful_bi_consumer;
#[cfg(feature = "stateful")]
pub mod stateful_consumer;

pub use bi_consumer::*;
#[cfg(feature = "once")]
pub use bi_consumer_once::*;
pub use consumer::*;
#[cfg(feature = "once")]
pub use consumer_once::*;
#[cfg(feature = "stateful")]
pub use stateful_bi_consumer::*;
#[cfg(feature = "stateful")]
pub use stateful_consumer::*;
