/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Consumers Module
//!
//! This module provides consumer-related functional programming abstractions,
//! including single-parameter consumers, bi-consumers, and their stateful
//! variants.
//!
//! # Author
//!
//! Haixing Hu

pub mod bi_consumer;
pub mod bi_consumer_once;
pub mod consumer;
pub mod consumer_once;
pub mod macros;
pub mod stateful_bi_consumer;
pub mod stateful_consumer;

pub use bi_consumer::{
    ArcBiConsumer,
    ArcConditionalBiConsumer,
    BiConsumer,
    BoxBiConsumer,
    BoxConditionalBiConsumer,
    FnBiConsumerOps,
    RcBiConsumer,
    RcConditionalBiConsumer,
};
pub use bi_consumer_once::{
    BiConsumerOnce,
    BoxBiConsumerOnce,
    BoxConditionalBiConsumerOnce,
    FnBiConsumerOnceOps,
};
pub use consumer::{
    ArcConditionalConsumer,
    ArcConsumer,
    BoxConditionalConsumer,
    BoxConsumer,
    Consumer,
    FnConsumerOps,
    RcConditionalConsumer,
    RcConsumer,
};
pub use consumer_once::{
    BoxConditionalConsumerOnce,
    BoxConsumerOnce,
    ConsumerOnce,
    FnConsumerOnceOps,
};
pub use stateful_bi_consumer::{
    ArcConditionalStatefulBiConsumer,
    ArcStatefulBiConsumer,
    BoxConditionalStatefulBiConsumer,
    BoxStatefulBiConsumer,
    FnStatefulBiConsumerOps,
    RcConditionalStatefulBiConsumer,
    RcStatefulBiConsumer,
    StatefulBiConsumer,
};
pub use stateful_consumer::{
    ArcConditionalStatefulConsumer,
    ArcStatefulConsumer,
    BoxConditionalStatefulConsumer,
    BoxStatefulConsumer,
    FnStatefulConsumerOps,
    RcConditionalStatefulConsumer,
    RcStatefulConsumer,
    StatefulConsumer,
};
