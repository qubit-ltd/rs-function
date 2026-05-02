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
//! Defines the `BoxUnaryOperatorOnce` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// Type Aliases for UnaryOperatorOnce (TransformerOnce<T, T>)
// ============================================================================

/// Type alias for `BoxTransformerOnce<T, T>`
///
/// Represents a one-time use unary operator that transforms a value of type `T`
/// to another value of the same type `T`. Equivalent to Java's `UnaryOperator<T>`
/// with consuming semantics (FnOnce).
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BoxUnaryOperatorOnce, TransformerOnce};
///
/// let increment: BoxUnaryOperatorOnce<i32> = BoxUnaryOperatorOnce::new(|x| x + 1);
/// assert_eq!(increment.apply(41), 42);
/// ```
///
pub type BoxUnaryOperatorOnce<T> = BoxTransformerOnce<T, T>;
