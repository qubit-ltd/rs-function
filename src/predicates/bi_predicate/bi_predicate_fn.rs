// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the shared callback type used by single-threaded bi-predicate
//! wrappers.

/// A callback that tests two borrowed values.
///
/// The callback receives shared references for the duration of each call; it
/// does not take ownership of either value or retain those references.
///
/// # Type Parameters
///
/// * `T` - The type of the first value tested by the callback.
/// * `U` - The type of the second value tested by the callback.
pub(in crate::predicates) type BiPredicateFn<T, U> = dyn Fn(&T, &U) -> bool;
