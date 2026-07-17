// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `RcUnaryOperator` public type.

use super::RcTransformer;

/// A locally shared unary transformation closed over one type.
///
/// Represents a single-threaded unary operator that transforms a value of type
/// `T` to another value of the same type `T`. Equivalent to Java's
/// `UnaryOperator<T>` with shared, single-threaded ownership.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{RcUnaryOperator, Transformer};
///
/// let negate: RcUnaryOperator<i32> = RcUnaryOperator::new(|x: i32| -x);
/// let negate_clone = negate.clone();
/// assert_eq!(negate.apply(42), -42);
/// assert_eq!(negate_clone.apply(42), -42);
/// ```
pub type RcUnaryOperator<T> = RcTransformer<T, T>;
