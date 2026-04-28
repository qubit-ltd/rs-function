/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `ArcBinaryMutatingFunction` public type.

#![allow(unused_imports)]

use super::*;

/// Type alias for `ArcBiMutatingFunction<T, T, R>`
///
/// Represents a thread-safe binary mutating function that takes two values of type `T`
/// and produces a value of type `R`. Similar to Java's `BiFunction<T, T, R>`
/// with shared, thread-safe ownership and mutable references.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{ArcBinaryMutatingFunction, BiMutatingFunction};
///
/// let swap_and_sum: ArcBinaryMutatingFunction<i32, i32> = ArcBinaryMutatingFunction::new(|x, y| {
///     let temp = *x;
///     *x = *y;
///     *y = temp;
///     *x + *y
/// });
/// let swap_clone = swap_and_sum.clone();
/// let mut a = 5;
/// let mut b = 10;
/// assert_eq!(swap_and_sum.apply(&mut a, &mut b), 15);
/// assert_eq!(swap_clone.apply(&mut a, &mut b), 15);
/// ```
///
/// # Author
///
/// Haixing Hu
pub type ArcBinaryMutatingFunction<T, R> = ArcBiMutatingFunction<T, T, R>;
