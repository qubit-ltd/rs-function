// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the callback type used by thread-safe bi-predicate wrappers.

/// A callback that tests two borrowed values and can be shared across threads.
///
/// # Type Parameters
///
/// * `T` - The type of the first value tested by the callback.
/// * `U` - The type of the second value tested by the callback.
pub(in crate::predicates) type SendSyncBiPredicateFn<T, U> = dyn Fn(&T, &U) -> bool + Send + Sync;
