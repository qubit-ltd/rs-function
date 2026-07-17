// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `ArcBinaryMutatingFunction` public type.

use super::ArcBiMutatingFunction;

/// A shared thread-safe mutating binary function with matching input types.
///
/// Represents a thread-safe binary mutating function that takes two values of
/// type `T` and produces a value of type `R`. Similar to Java's `BiFunction<T,
/// T, R>` with shared, thread-safe ownership and mutable references.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{ArcBinaryMutatingFunction, BiMutatingFunction};
///
/// let swap_and_sum: ArcBinaryMutatingFunction<i32, i32> = ArcBinaryMutatingFunction::new(|x: &mut i32, y: &mut i32| {
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
pub type ArcBinaryMutatingFunction<T, R> = ArcBiMutatingFunction<T, T, R>;
