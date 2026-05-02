/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
// qubit-style: allow explicit-imports
//! Defines the `RcStatefulBinaryOperator` public type.

#![allow(unused_imports)]

use super::*;

/// Type alias for `RcStatefulBiTransformer<T, T, T>`
///
/// Represents a single-threaded binary operator that takes two values of type
/// `T` and produces a value of the same type `T`. Equivalent to Java's
/// `BinaryOperator<T>` with shared, single-threaded ownership.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{RcStatefulBinaryOperator, StatefulBiTransformer};
///
/// let max: RcStatefulBinaryOperator<i32> = RcStatefulBinaryOperator::new(|x, y| if x > y { x } else { y });
/// let mut max_clone = max.clone();
/// let mut max = max;
/// assert_eq!(max.apply(30, 42), 42);
/// assert_eq!(max_clone.apply(30, 42), 42);
/// ```
///
pub type RcStatefulBinaryOperator<T> = RcStatefulBiTransformer<T, T, T>;
