// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # BiConsumer Types
//!
//! Provides bi-consumer interfaces for operations invoked through `&self` with
//! two shared input references. The API grants no direct mutable access to the
//! wrapper or inputs, but callbacks may use interior mutability or external
//! side effects.
//!
//! It is similar to the `Fn(&T, &U)` trait in the standard library.
//!
//! This module provides a unified `BiConsumer` trait and three
//! concrete implementations based on different ownership models:
//!
//! - **`BoxBiConsumer<T, U>`**: Box-based single ownership
//! - **`ArcBiConsumer<T, U>`**: Arc-based thread-safe shared ownership
//! - **`RcBiConsumer<T, U>`**: Rc-based single-threaded shared ownership
//!
//! # Design Philosophy
//!
//! BiConsumer uses `Fn(&T, &U)` semantics: it is invoked through `&self` and
//! receives shared references to both input values.
//!
//! Suitable for observation, logging, and notification scenarios with two
//! parameters. Compared to `StatefulBiConsumer`, `BiConsumer` does not require
//! wrapper-level interior mutability (`Mutex`/`RefCell`), making it more
//! efficient and easier to share.

mod box_bi_consumer;
pub use box_bi_consumer::BoxBiConsumer;
#[path = "bi_consumer/bi_consumer.rs"]
mod bi_consumer_trait;
pub use bi_consumer_trait::BiConsumer;
mod internal;
#[cfg(feature = "rc")]
mod rc_bi_consumer;
#[cfg(feature = "rc")]
pub use rc_bi_consumer::RcBiConsumer;
mod arc_bi_consumer;
pub use arc_bi_consumer::ArcBiConsumer;
mod box_conditional_bi_consumer;
pub use box_conditional_bi_consumer::BoxConditionalBiConsumer;
mod arc_conditional_bi_consumer;
pub use arc_conditional_bi_consumer::ArcConditionalBiConsumer;
#[cfg(feature = "rc")]
mod rc_conditional_bi_consumer;
#[cfg(feature = "rc")]
pub use rc_conditional_bi_consumer::RcConditionalBiConsumer;
