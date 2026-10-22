// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcStatefulBinaryOperator` public type.

use super::RcStatefulBiTransformer;

/// A single-threaded, cloneable binary operator whose calls share mutable
/// callback state.
///
/// This alias specializes [`RcStatefulBiTransformer`] so both consumed inputs
/// and the output have type `T`. Clones share the same callback through
/// reference counting; the operator is not thread-safe.
///
/// # Type Parameters
///
/// * `T` - The type consumed for both inputs and returned as the result.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{RcStatefulBinaryOperator, StatefulBiTransformer};
///
/// let max: RcStatefulBinaryOperator<i32> = RcStatefulBinaryOperator::new(
///     |x, y| if x > y { x } else { y },
/// );
/// let mut max_clone = max.clone();
/// let mut max = max;
/// assert_eq!(max.apply(30, 42), 42);
/// assert_eq!(max_clone.apply(30, 42), 42);
/// ```
pub type RcStatefulBinaryOperator<T> = RcStatefulBiTransformer<T, T, T>;
