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
//! Defines the `BoxBinaryOperatorOnce` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// Type Aliases for BinaryOperatorOnce (BiTransformerOnce<T, T, T>)
// ============================================================================

/// Type alias for `BoxBiTransformerOnce<T, T, T>`
///
/// Represents a one-time use binary operator that takes two values of type `T`
/// and produces a value of the same type `T`. Equivalent to Java's
/// `BinaryOperator<T>` with consuming semantics (FnOnce).
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BoxBinaryOperatorOnce, BiTransformerOnce};
///
/// let add: BoxBinaryOperatorOnce<i32> = BoxBinaryOperatorOnce::new(|x, y| x + y);
/// assert_eq!(add.apply(20, 22), 42);
/// ```
///
pub type BoxBinaryOperatorOnce<T> = BoxBiTransformerOnce<T, T, T>;
