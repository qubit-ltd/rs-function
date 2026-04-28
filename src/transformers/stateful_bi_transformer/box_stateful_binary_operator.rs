/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `BoxStatefulBinaryOperator` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// Type Aliases for StatefulBinaryOperator (StatefulBiTransformer<T, T, T>)
// ============================================================================

/// Type alias for `BoxStatefulBiTransformer<T, T, T>`
///
/// Represents a binary operator that takes two values of type `T` and produces
/// a value of the same type `T`, with single ownership semantics. Equivalent to
/// Java's `BinaryOperator<T>`.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BoxStatefulBinaryOperator, StatefulBiTransformer};
///
/// let add: BoxStatefulBinaryOperator<i32> = BoxStatefulBinaryOperator::new(|x, y| x + y);
/// let mut add = add;
/// assert_eq!(add.apply(20, 22), 42);
/// ```
///
/// # Author
///
/// Haixing Hu
pub type BoxStatefulBinaryOperator<T> = BoxStatefulBiTransformer<T, T, T>;
