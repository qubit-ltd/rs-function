/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `ArcBinaryOperator` public type.

#![allow(unused_imports)]

use super::*;

/// Type alias for `ArcBiTransformer<T, T, T>`
///
/// Represents a thread-safe binary operator that takes two values of type `T`
/// and produces a value of the same type `T`. Equivalent to Java's
/// `BinaryOperator<T>` with shared, thread-safe ownership.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{ArcBinaryOperator, BiTransformer};
///
/// let multiply: ArcBinaryOperator<i32> = ArcBinaryOperator::new(|x, y| x * y);
/// let multiply_clone = multiply.clone();
/// assert_eq!(multiply.apply(6, 7), 42);
/// assert_eq!(multiply_clone.apply(6, 7), 42);
/// ```
///
/// # Author
///
/// Haixing Hu
pub type ArcBinaryOperator<T> = ArcBiTransformer<T, T, T>;
