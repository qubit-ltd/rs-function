// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `ArcUnaryOperator` public type.

use super::ArcTransformer;

/// A shared thread-safe unary transformation closed over one type.
///
/// Represents a thread-safe unary operator that transforms a value of type `T`
/// to another value of the same type `T`. Equivalent to Java's
/// `UnaryOperator<T>` with shared, thread-safe ownership.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{ArcUnaryOperator, Transformer};
///
/// let double: ArcUnaryOperator<i32> = ArcUnaryOperator::new(|x| x * 2);
/// let double_clone = double.clone();
/// assert_eq!(double.apply(21), 42);
/// assert_eq!(double_clone.apply(21), 42);
/// ```
pub type ArcUnaryOperator<T> = ArcTransformer<T, T>;
