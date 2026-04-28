/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `FnComparatorOps` public type.

#![allow(unused_imports)]

use super::*;

/// Extension trait providing composition methods for closures and function
/// pointers.
///
/// This trait is automatically implemented for all closures and function
/// pointers with the signature `Fn(&T, &T) -> Ordering`, allowing them to
/// be composed directly without explicit wrapping.
///
/// # Examples
///
/// ```rust
/// use qubit_function::comparator::{Comparator, FnComparatorOps, BoxComparator};
/// use std::cmp::Ordering;
///
/// let cmp = (|a: &i32, b: &i32| a.cmp(b))
///     .reversed()
///     .then_comparing(BoxComparator::new(|a: &i32, b: &i32| b.cmp(a)));
///
/// assert_eq!(cmp.compare(&5, &3), Ordering::Less);
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait FnComparatorOps<T>: Fn(&T, &T) -> Ordering + Sized {
    /// Returns a comparator that imposes the reverse ordering.
    ///
    /// # Returns
    ///
    /// A new `BoxComparator` that reverses the comparison order.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::comparator::{Comparator, FnComparatorOps};
    /// use std::cmp::Ordering;
    ///
    /// let rev = (|a: &i32, b: &i32| a.cmp(b)).reversed();
    /// assert_eq!(rev.compare(&5, &3), Ordering::Less);
    /// ```
    #[inline]
    fn reversed(self) -> BoxComparator<T>
    where
        Self: 'static,
        T: 'static,
    {
        BoxComparator::new(self).reversed()
    }

    /// Returns a comparator that uses this comparator first, then another
    /// comparator if this one considers the values equal.
    ///
    /// # Parameters
    ///
    /// * `other` - The comparator to use for tie-breaking
    ///
    /// # Returns
    ///
    /// A new `BoxComparator` that chains this comparator with another.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::comparator::{Comparator, FnComparatorOps, BoxComparator};
    /// use std::cmp::Ordering;
    ///
    /// let cmp = (|a: &i32, b: &i32| (a % 2).cmp(&(b % 2)))
    ///     .then_comparing(BoxComparator::new(|a: &i32, b: &i32| a.cmp(b)));
    /// assert_eq!(cmp.compare(&4, &2), Ordering::Greater);
    /// ```
    #[inline]
    fn then_comparing(self, other: BoxComparator<T>) -> BoxComparator<T>
    where
        Self: 'static,
        T: 'static,
    {
        BoxComparator::new(self).then_comparing(other)
    }
}

impl<T, F> FnComparatorOps<T> for F where F: Fn(&T, &T) -> Ordering {}
