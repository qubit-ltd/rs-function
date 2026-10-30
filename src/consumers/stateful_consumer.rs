// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Consumer Types
//!
//! Provides implementations of consumer interfaces for executing operations
//! that accept a shared reference to one input and return no result. This
//! module is available with the `stateful` feature.
//!
//! It is similar to the `FnMut(&T)` trait in the standard library.
//!
//! This module provides a unified `StatefulConsumer` trait and three concrete
//! implementations based on different ownership models:
//!
//! - **`BoxStatefulConsumer<T>`**: Box-based single ownership implementation
//!   for stored, reusable stateful callbacks
//! - **`ArcStatefulConsumer<T>`**: Thread-safe shared ownership implementation
//!   based on Arc<Mutex<>>
//! - **`RcStatefulConsumer<T>`**: Single-threaded shared ownership
//!   implementation based on Rc<RefCell<>>
//!
//! # Design Philosophy
//!
//! `StatefulConsumer` is called through `&mut self` and uses `FnMut(&T)`, so
//! it can update ordinary captured state. The shared input reference does not
//! allow direct mutation of the input; interior mutability remains possible.
//!
//! Suitable for statistics, accumulation, event handling, and other scenarios.

mod box_stateful_consumer;
pub use box_stateful_consumer::BoxStatefulConsumer;
#[path = "stateful_consumer/stateful_consumer.rs"]
mod stateful_consumer_trait;
pub use stateful_consumer_trait::StatefulConsumer;
#[cfg(feature = "rc")]
mod rc_stateful_consumer;
#[cfg(feature = "rc")]
pub use rc_stateful_consumer::RcStatefulConsumer;
mod arc_stateful_consumer;
pub use arc_stateful_consumer::ArcStatefulConsumer;
mod box_conditional_stateful_consumer;
pub use box_conditional_stateful_consumer::BoxConditionalStatefulConsumer;
mod arc_conditional_stateful_consumer;
pub use arc_conditional_stateful_consumer::ArcConditionalStatefulConsumer;
#[cfg(feature = "rc")]
mod rc_conditional_stateful_consumer;
#[cfg(feature = "rc")]
pub use rc_conditional_stateful_consumer::RcConditionalStatefulConsumer;
