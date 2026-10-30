// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the private non-mutating bi-consumer callback signature.

/// Callback signature shared by the `BoxBiConsumer` and `RcBiConsumer` types.
///
/// The callback observes both inputs through shared references and returns no
/// value; the wrappers own or share the callback according to their respective
/// ownership models.
pub(in crate::consumers::bi_consumer) type BiConsumerFn<T, U> = dyn Fn(&T, &U);
