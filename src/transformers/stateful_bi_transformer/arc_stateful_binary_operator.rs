/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `ArcStatefulBinaryOperator` public type.

#![allow(unused_imports)]

use super::*;

/// Type alias for `ArcStatefulBiTransformer<T, T, T>`
///
/// Represents a thread-safe binary operator that takes two values of type `T`
/// and produces a value of the same type `T`. Equivalent to Java's
/// `BinaryOperator<T>` with shared, thread-safe ownership.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{ArcStatefulBinaryOperator, StatefulBiTransformer};
///
/// let multiply: ArcStatefulBinaryOperator<i32> = ArcStatefulBinaryOperator::new(|x, y| x * y);
/// let mut multiply_clone = multiply.clone();
/// let mut multiply = multiply;
/// assert_eq!(multiply.apply(6, 7), 42);
/// assert_eq!(multiply_clone.apply(6, 7), 42);
/// ```
///
/// # Author
///
/// Haixing Hu
pub type ArcStatefulBinaryOperator<T> = ArcStatefulBiTransformer<T, T, T>;
