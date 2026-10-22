// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # ConsumerOnce Types
//!
//! Provides implementations of one-time consumer interfaces for executing
//! one-time operations that accept a single input parameter but return no
//! result.
//!
//! It is similar to the `FnOnce(&T)` trait in the standard library.
//!
//! This module provides a unified `ConsumerOnce` trait and two concrete
//! implementations:
//!
//! - **`BoxConsumerOnce<T>`**: Box-based single ownership implementation
//! - **`BoxConditionalConsumerOnce<T>`**: conditional single ownership
//!
//! # Why No Arc/Rc Variants?
//!
//! Unlike reusable [`Consumer`](crate::consumers::Consumer) implementations,
//! this module does **not** provide `ArcConsumerOnce` or `RcConsumerOnce`
//! implementations. This is a design decision based on the fact that `FnOnce`
//! semantics require single ownership at the call site, while `Arc` and `Rc`
//! are meant to preserve shared ownership across clones.
//!
//! # Design Philosophy
//!
//! ConsumerOnce uses `FnOnce(&T)` semantics for truly one-time consumption
//! operations.
//!
//! Unlike Consumer, ConsumerOnce consumes itself on first call. Suitable for
//! initialization callbacks, cleanup callbacks, and similar scenarios.

mod box_consumer_once;
pub use box_consumer_once::BoxConsumerOnce;
mod box_conditional_consumer_once;
pub use box_conditional_consumer_once::BoxConditionalConsumerOnce;
#[path = "consumer_once/consumer_once.rs"]
mod consumer_once_trait;
pub use consumer_once_trait::ConsumerOnce;
