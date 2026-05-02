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
//! Defines the `RcBinaryOperator` public type.

#![allow(unused_imports)]

use super::*;

/// Type alias for `RcBiTransformer<T, T, T>`
///
/// Represents a single-threaded binary operator that takes two values of type
/// `T` and produces a value of the same type `T`. Equivalent to Java's
/// `BinaryOperator<T>` with shared, single-threaded ownership.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{RcBinaryOperator, BiTransformer};
///
/// let max: RcBinaryOperator<i32> = RcBinaryOperator::new(|x, y| if x > y { x } else { y });
/// let max_clone = max.clone();
/// assert_eq!(max.apply(30, 42), 42);
/// assert_eq!(max_clone.apply(30, 42), 42);
/// ```
///
pub type RcBinaryOperator<T> = RcBiTransformer<T, T, T>;
