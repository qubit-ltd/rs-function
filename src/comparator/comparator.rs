// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `Comparator` public trait.

use std::cmp::Ordering;

/// A trait for comparison operations.
///
/// This trait defines only the core comparison operation. It does NOT include
/// composition methods like `reversed` or
/// `then_comparing` to maintain a clean separation between the trait
/// interface and specialized implementations.
///
/// # Type Parameters
///
/// * `T` - The type of values being compared
///
/// # Examples
///
/// ```rust
/// use qubit_function::comparator::{Comparator, BoxComparator};
/// use std::cmp::Ordering;
///
/// let cmp = BoxComparator::new(|a: &i32, b: &i32| a.cmp(b));
/// assert_eq!(cmp.compare(&5, &3), Ordering::Greater);
/// ```
pub trait Comparator<T> {
    /// Compares two values and returns an ordering.
    ///
    /// # Parameters
    ///
    /// * `a` - The first value to compare
    /// * `b` - The second value to compare
    ///
    /// # Returns
    ///
    /// An `Ordering` indicating whether `a` is less than, equal to, or
    /// greater than `b`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::comparator::{Comparator, BoxComparator};
    /// use std::cmp::Ordering;
    ///
    /// let cmp = BoxComparator::new(|a: &i32, b: &i32| a.cmp(b));
    /// assert_eq!(cmp.compare(&5, &3), Ordering::Greater);
    /// assert_eq!(cmp.compare(&3, &5), Ordering::Less);
    /// assert_eq!(cmp.compare(&5, &5), Ordering::Equal);
    /// ```
    #[must_use = "the ordering result should be used"]
    fn compare(&self, a: &T, b: &T) -> Ordering;
}

/// Blanket implementation of `Comparator` for all closures and function
/// pointers.
///
/// This allows any closure or function with the signature
/// `Fn(&T, &T) -> Ordering` to be used as a comparator.
///
/// # Examples
///
/// ```rust
/// use qubit_function::comparator::Comparator;
/// use std::cmp::Ordering;
///
/// let cmp = |a: &i32, b: &i32| a.cmp(b);
/// assert_eq!(cmp.compare(&5, &3), Ordering::Greater);
/// ```
impl<T, F> Comparator<T> for F
where
    F: Fn(&T, &T) -> Ordering,
{
    #[inline(always)]
    fn compare(&self, a: &T, b: &T) -> Ordering {
        self(a, b)
    }
}
