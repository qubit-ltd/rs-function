/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
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
/// # Author
///
/// Haixing Hu
pub type BoxBinaryOperatorOnce<T> = BoxBiTransformerOnce<T, T, T>;
