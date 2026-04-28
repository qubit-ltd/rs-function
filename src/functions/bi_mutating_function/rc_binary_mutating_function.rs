/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `RcBinaryMutatingFunction` public type.

#![allow(unused_imports)]

use super::*;

/// Type alias for `RcBiMutatingFunction<T, T, R>`
///
/// Represents a single-threaded binary mutating function that takes two values of type `T`
/// and produces a value of type `R`. Similar to Java's `BiFunction<T, T, R>`
/// with shared, single-threaded ownership and mutable references.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{RcBinaryMutatingFunction, BiMutatingFunction};
///
/// let swap_and_sum: RcBinaryMutatingFunction<i32, i32> = RcBinaryMutatingFunction::new(|x, y| {
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
pub type RcBinaryMutatingFunction<T, R> = RcBiMutatingFunction<T, T, R>;
