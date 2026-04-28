/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `BinaryOperator` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// BinaryOperator Trait - Marker trait for BiTransformer<T, T, T>
// ============================================================================

/// BinaryOperator trait - marker trait for binary operators
///
/// A binary operator takes two values of type `T` and produces a value of the
/// same type `T`. This trait extends `BiTransformer<T, T, T>` to provide
/// semantic clarity for same-type binary operations. Equivalent to Java's
/// `BinaryOperator<T>` which extends `BiFunction<T, T, T>`.
///
/// # Automatic Implementation
///
/// This trait is automatically implemented for all types that implement
/// `BiTransformer<T, T, T>`, so you don't need to implement it manually.
///
/// # Type Parameters
///
/// * `T` - The type of both input values and the output value
///
/// # Examples
///
/// ## Using in generic constraints
///
/// ```rust
/// use qubit_function::{BinaryOperator, BiTransformer};
///
/// fn reduce<T, O>(values: Vec<T>, initial: T, op: O) -> T
/// where
///     O: BinaryOperator<T>,
///     T: Clone,
/// {
///     values.into_iter().fold(initial, |acc, val| op.apply(acc, val))
/// }
///
/// let sum = |a: i32, b: i32| a + b;
/// assert_eq!(reduce(vec![1, 2, 3, 4], 0, sum), 10);
/// ```
///
/// ## With concrete types
///
/// ```rust
/// use qubit_function::{BoxBinaryOperator, BinaryOperator, BiTransformer};
///
/// fn create_adder() -> BoxBinaryOperator<i32> {
///     BoxBinaryOperator::new(|x, y| x + y)
/// }
///
/// let op = create_adder();
/// assert_eq!(op.apply(20, 22), 42);
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait BinaryOperator<T>: BiTransformer<T, T, T> {}

/// Blanket implementation of BinaryOperator for all BiTransformer<T, T, T>
///
/// This automatically implements `BinaryOperator<T>` for any type that
/// implements `BiTransformer<T, T, T>`.
///
/// # Author
///
/// Haixing Hu
impl<F, T> BinaryOperator<T> for F
where
    F: BiTransformer<T, T, T>,
{
    // empty
}
