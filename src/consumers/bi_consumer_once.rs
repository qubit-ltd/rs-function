// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # BiConsumerOnce Types
//!
//! Provides one-time bi-consumer interface implementations for operations
//! accepting two input parameters without returning a result.
//!
//! It is similar to the `FnOnce(&T, &U)` trait in the standard library.
//!
//! This module provides a unified `BiConsumerOnce` trait and two concrete
//! implementations:
//!
//! - **`BoxBiConsumerOnce<T, U>`**: Box-based single ownership implementation
//! - **`BoxConditionalBiConsumerOnce<T, U>`**: conditional single ownership
//!   implementation
//!
//! # Why No Arc/Rc Variants?
//!
//! Unlike reusable [`BiConsumer`](crate::consumers::BiConsumer)
//! implementations, this module does **not** provide `ArcBiConsumerOnce` or
//! `RcBiConsumerOnce` implementations. This is a design decision based on the
//! fact that `FnOnce` semantics require single ownership at the call site,
//! while `Arc` and `Rc` are meant to preserve shared ownership across clones.
//!
//! # Design Philosophy
//!
//! BiConsumerOnce uses `FnOnce(&T, &U)` semantics: for truly one-time
//! consumption operations.
//!
//! Unlike BiConsumer, BiConsumerOnce consumes itself on first call. Suitable
//! for initialization callbacks, cleanup callbacks, etc.

// ==========================================================================
// Type Aliases
// ==========================================================================

/// Callable once with references to both input values and no return value.
type BiConsumerOnceFn<T, U> = dyn FnOnce(&T, &U);

mod box_bi_consumer_once;
pub use box_bi_consumer_once::BoxBiConsumerOnce;
mod box_conditional_bi_consumer_once;
pub use box_conditional_bi_consumer_once::BoxConditionalBiConsumerOnce;
#[path = "bi_consumer_once/bi_consumer_once.rs"]
mod bi_consumer_once_trait;
pub use bi_consumer_once_trait::BiConsumerOnce;
