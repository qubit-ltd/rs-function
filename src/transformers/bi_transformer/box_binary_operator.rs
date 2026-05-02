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
//! Defines the `BoxBinaryOperator` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// Type Aliases for BinaryOperator (BiTransformer<T, T, T>)
// ============================================================================

/// Type alias for `BoxBiTransformer<T, T, T>`
///
/// Represents a binary operator that takes two values of type `T` and produces
/// a value of the same type `T`, with single ownership semantics. Equivalent to
/// Java's `BinaryOperator<T>`.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BoxBinaryOperator, BiTransformer};
///
/// let add: BoxBinaryOperator<i32> = BoxBinaryOperator::new(|x, y| x + y);
/// assert_eq!(add.apply(20, 22), 42);
/// ```
///
pub type BoxBinaryOperator<T> = BoxBiTransformer<T, T, T>;
