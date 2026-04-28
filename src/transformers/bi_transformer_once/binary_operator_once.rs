/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `BinaryOperatorOnce` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// BinaryOperatorOnce Trait - Marker trait for BiTransformerOnce<T, T, T>
// ============================================================================

/// BinaryOperatorOnce trait - marker trait for one-time use binary operators
///
/// A one-time use binary operator takes two values of type `T` and produces a
/// value of the same type `T`, consuming self in the process. This trait
/// extends `BiTransformerOnce<T, T, T>` to provide semantic clarity for
/// same-type binary operations with consuming semantics. Equivalent to Java's
/// `BinaryOperator<T>` but with FnOnce semantics.
///
/// # Automatic Implementation
///
/// This trait is automatically implemented for all types that implement
/// `BiTransformerOnce<T, T, T>`, so you don't need to implement it manually.
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
/// use qubit_function::{BinaryOperatorOnce, BiTransformerOnce};
///
/// fn combine<T, O>(a: T, b: T, op: O) -> T
/// where
///     O: BinaryOperatorOnce<T>,
/// {
///     op.apply(a, b)
/// }
///
/// let multiply = |x: i32, y: i32| x * y;
/// assert_eq!(combine(6, 7, multiply), 42);
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait BinaryOperatorOnce<T>: BiTransformerOnce<T, T, T> {}

/// Blanket implementation of BinaryOperatorOnce for all BiTransformerOnce<T, T, T>
///
/// This automatically implements `BinaryOperatorOnce<T>` for any type that
/// implements `BiTransformerOnce<T, T, T>`.
///
/// # Author
///
/// Haixing Hu
impl<F, T> BinaryOperatorOnce<T> for F
where
    F: BiTransformerOnce<T, T, T>,
{
    // empty
}
