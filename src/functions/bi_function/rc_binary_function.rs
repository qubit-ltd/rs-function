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
//! Defines the `RcBinaryFunction` public type.

#![allow(unused_imports)]

use super::*;

/// Type alias for `RcBiFunction<T, T, R>`
///
/// Represents a single-threaded binary function that takes two values of type `T`
/// and produces a value of type `R`. Similar to Java's `BiFunction<T, T, R>`
/// with shared, single-threaded ownership.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{RcBinaryFunction, BiFunction};
///
/// let max: RcBinaryFunction<i32, i32> = RcBinaryFunction::new(|x, y| if x > y { *x } else { *y });
/// let max_clone = max.clone();
/// assert_eq!(max.apply(&30, &42), 42);
/// assert_eq!(max_clone.apply(&30, &42), 42);
/// ```
///
pub type RcBinaryFunction<T, R> = RcBiFunction<T, T, R>;
