// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the private thread-safe bi-consumer callback signature.

/// Callback signature used by `ArcBiConsumer` for shared access to two inputs.
///
/// The callback can be invoked repeatedly through a shared reference and can
/// be shared across threads. It receives immutable borrows of both inputs;
/// any mutation or external effects must come from the callback's captured
/// state or other shared resources.
pub(in crate::consumers::bi_consumer) type ThreadSafeBiConsumerFn<T, U> = dyn Fn(&T, &U) + Send + Sync;
