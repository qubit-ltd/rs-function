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
//! Defines the `BoxBinaryFunction` public type.

use super::BoxBiFunction;

// ============================================================================
// Type Aliases for BinaryOperator (BiFunction<T, T, R>)
// ============================================================================

/// Type alias for `BoxBiFunction<T, T, R>`
///
/// Represents a binary function that takes two values of type `T` and produces
/// a value of type `R`, with single ownership semantics. Similar to Java's
/// `BiFunction<T, T, R>` but with different type parameters.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BoxBinaryFunction, BiFunction};
///
/// let add: BoxBinaryFunction<i32, i32> = BoxBinaryFunction::new(|x, y| *x + *y);
/// assert_eq!(add.apply(&20, &22), 42);
/// ```
///
pub type BoxBinaryFunction<T, R> = BoxBiFunction<T, T, R>;
