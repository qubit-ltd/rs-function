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
//! Defines the `UnaryOperatorOnce` public type.

use super::TransformerOnce;

// ============================================================================
// UnaryOperatorOnce Trait - Marker trait for TransformerOnce<T, T>
// ============================================================================

/// UnaryOperatorOnce trait - marker trait for one-time use unary operators
///
/// A one-time use unary operator transforms a value of type `T` to another
/// value of the same type `T`, consuming self in the process. This trait
/// extends `TransformerOnce<T, T>` to provide semantic clarity for same-type
/// transformations with consuming semantics. Equivalent to Java's
/// `UnaryOperator<T>` but with FnOnce semantics.
///
/// # Automatic Implementation
///
/// This trait is automatically implemented for all types that implement
/// `TransformerOnce<T, T>`, so you don't need to implement it manually.
///
/// # Type Parameters
///
/// * `T` - The type of both input and output values
///
/// # Examples
///
/// ## Using in generic constraints
///
/// ```rust
/// use qubit_function::{UnaryOperatorOnce, TransformerOnce};
///
/// fn apply<T, O>(value: T, op: O) -> T
/// where
///     O: UnaryOperatorOnce<T>,
/// {
///     op.apply(value)
/// }
///
/// let double = |x: i32| x * 2;
/// assert_eq!(apply(21, double), 42);
/// ```
///
pub trait UnaryOperatorOnce<T>: TransformerOnce<T, T> {}

/// Blanket implementation of UnaryOperatorOnce for all TransformerOnce<T, T>
///
/// This automatically implements `UnaryOperatorOnce<T>` for any type that
/// implements `TransformerOnce<T, T>`.
///
impl<F, T> UnaryOperatorOnce<T> for F
where
    F: TransformerOnce<T, T>,
{
    // empty
}
