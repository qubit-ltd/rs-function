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
//! Defines the `BoxBinaryMutatingFunction` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// Type Aliases for BinaryMutatingOperator (BiMutatingFunction<T, U, R> where T == U)
// ============================================================================

/// Type alias for `BoxBiMutatingFunction<T, T, R>`
///
/// Represents a binary mutating function that takes two values of type `T` and produces
/// a value of type `R`, with single ownership semantics. Similar to Java's
/// `BiFunction<T, T, R>` but with mutable references.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BoxBinaryMutatingFunction, BiMutatingFunction};
///
/// let swap_and_sum: BoxBinaryMutatingFunction<i32, i32> = BoxBinaryMutatingFunction::new(|x, y| {
///     let temp = *x;
///     *x = *y;
///     *y = temp;
///     *x + *y
/// });
/// let mut a = 5;
/// let mut b = 10;
/// assert_eq!(swap_and_sum.apply(&mut a, &mut b), 15);
/// assert_eq!(a, 10);
/// assert_eq!(b, 5);
/// ```
///
pub type BoxBinaryMutatingFunction<T, R> = BoxBiMutatingFunction<T, T, R>;
