// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # BiConsumer Types
//!
//! Provides bi-consumer interface implementations for operations accepting
//! two input parameters without returning a result.
//!
//! It is similar to the `FnMut(&T, &U)` trait in the standard library.
//!
//! This module provides the `StatefulBiConsumer` trait and concrete regular
//! and conditional implementations based on different ownership models:
//!
//! - **`BoxStatefulBiConsumer<T, U>`**: Box-based single ownership
//! - **`ArcStatefulBiConsumer<T, U>`**: Arc<Mutex<>>-based thread-safe shared
//!   ownership
//! - **`RcStatefulBiConsumer<T, U>`**: Rc<RefCell<>>-based single-threaded
//!   shared ownership
//!
//! # Design Philosophy
//!
//! BiConsumer uses `FnMut(&T, &U)` semantics: can modify its own state but
//! does NOT modify input values.
//!
//! Suitable for statistics, accumulation, and event processing scenarios
//! involving two parameters.

mod box_stateful_bi_consumer;
pub use box_stateful_bi_consumer::BoxStatefulBiConsumer;
#[path = "stateful_bi_consumer/stateful_bi_consumer.rs"]
mod stateful_bi_consumer_trait;
pub use stateful_bi_consumer_trait::StatefulBiConsumer;
#[cfg(feature = "rc")]
mod rc_stateful_bi_consumer;
#[cfg(feature = "rc")]
pub use rc_stateful_bi_consumer::RcStatefulBiConsumer;
mod arc_stateful_bi_consumer;
pub use arc_stateful_bi_consumer::ArcStatefulBiConsumer;
mod box_conditional_stateful_bi_consumer;
pub use box_conditional_stateful_bi_consumer::BoxConditionalStatefulBiConsumer;
mod arc_conditional_stateful_bi_consumer;
pub use arc_conditional_stateful_bi_consumer::ArcConditionalStatefulBiConsumer;
#[cfg(feature = "rc")]
mod rc_conditional_stateful_bi_consumer;
#[cfg(feature = "rc")]
pub use rc_conditional_stateful_bi_consumer::RcConditionalStatefulBiConsumer;
