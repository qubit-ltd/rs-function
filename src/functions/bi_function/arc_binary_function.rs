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
//! Defines the `ArcBinaryFunction` public type.

use super::ArcBiFunction;

/// Type alias for `ArcBiFunction<T, T, R>`
///
/// Represents a thread-safe binary function that takes two values of type `T`
/// and produces a value of type `R`. Similar to Java's `BiFunction<T, T, R>`
/// with shared, thread-safe ownership.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{ArcBinaryFunction, BiFunction};
///
/// let multiply: ArcBinaryFunction<i32, i32> = ArcBinaryFunction::new(|x, y| *x * *y);
/// let multiply_clone = multiply.clone();
/// assert_eq!(multiply.apply(&6, &7), 42);
/// assert_eq!(multiply_clone.apply(&6, &7), 42);
/// ```
///
pub type ArcBinaryFunction<T, R> = ArcBiFunction<T, T, R>;
