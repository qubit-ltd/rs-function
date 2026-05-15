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
//! Defines the `StatefulBinaryOperator` public type.

use super::StatefulBiTransformer;

// ============================================================================
// StatefulBinaryOperator Trait - Marker trait for StatefulBiTransformer<T, T, T>
// ============================================================================

/// StatefulBinaryOperator trait - marker trait for stateful binary operators
///
/// A binary operator takes two values of type `T` and produces a value of the
/// same type `T`. This trait extends `StatefulBiTransformer<T, T, T>` to provide
/// semantic clarity for same-type binary operations. Equivalent to Java's
/// `BinaryOperator<T>` which extends `BiFunction<T, T, T>`.
///
/// # Automatic Implementation
///
/// This trait is automatically implemented for all types that implement
/// `StatefulBiTransformer<T, T, T>`, so you don't need to implement it manually.
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
/// use qubit_function::{StatefulBinaryOperator, StatefulBiTransformer};
///
/// fn reduce<T, O>(values: Vec<T>, initial: T, mut op: O) -> T
/// where
///     O: StatefulBinaryOperator<T>,
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
/// use qubit_function::{BoxStatefulBinaryOperator, StatefulBiTransformer};
///
/// fn create_adder() -> BoxStatefulBinaryOperator<i32> {
///     BoxStatefulBinaryOperator::new(|x, y| x + y)
/// }
///
/// let mut op = create_adder();
/// assert_eq!(op.apply(20, 22), 42);
/// ```
///
pub trait StatefulBinaryOperator<T>: StatefulBiTransformer<T, T, T> {}

/// Blanket implementation of StatefulBinaryOperator for all StatefulBiTransformer<T, T, T>
///
/// This automatically implements `StatefulBinaryOperator<T>` for any type that
/// implements `StatefulBiTransformer<T, T, T>`.
///
impl<F, T> StatefulBinaryOperator<T> for F
where
    F: StatefulBiTransformer<T, T, T>,
{
    // empty
}
