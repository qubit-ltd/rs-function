/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `RcComparator` public type.

#![allow(unused_imports)]

use super::*;

/// An Rc-based single-threaded comparator with shared ownership.
///
/// `RcComparator` wraps a comparator function in an `Rc`, providing
/// single-threaded shared ownership semantics. It is cloneable and uses
/// `&self` in composition operations.
///
/// # Type Parameters
///
/// * `T` - The type of values being compared
///
/// # Examples
///
/// ```rust
/// use qubit_function::comparator::{Comparator, RcComparator};
/// use std::cmp::Ordering;
///
/// let cmp = RcComparator::new(|a: &i32, b: &i32| a.cmp(b));
/// let cloned = cmp.clone();
/// assert_eq!(cmp.compare(&5, &3), Ordering::Greater);
/// assert_eq!(cloned.compare(&5, &3), Ordering::Greater);
/// ```
///
/// # Author
///
/// Haixing Hu
#[derive(Clone)]
pub struct RcComparator<T> {
    pub(super) function: Rc<dyn Fn(&T, &T) -> Ordering>,
}

impl<T> RcComparator<T> {
    /// Creates a new `RcComparator` from a closure.
    ///
    /// # Parameters
    ///
    /// * `f` - The closure to wrap
    ///
    /// # Returns
    ///
    /// A new `RcComparator` instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::comparator::RcComparator;
    ///
    /// let cmp = RcComparator::new(|a: &i32, b: &i32| a.cmp(b));
    /// ```
    #[inline]
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(&T, &T) -> Ordering + 'static,
    {
        Self {
            function: Rc::new(f),
        }
    }

    /// Returns a comparator that imposes the reverse ordering.
    ///
    /// # Returns
    ///
    /// A new `RcComparator` that reverses the comparison order.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::comparator::{Comparator, RcComparator};
    /// use std::cmp::Ordering;
    ///
    /// let cmp = RcComparator::new(|a: &i32, b: &i32| a.cmp(b));
    /// let rev = cmp.reversed();
    /// assert_eq!(rev.compare(&5, &3), Ordering::Less);
    /// assert_eq!(cmp.compare(&5, &3), Ordering::Greater); // cmp still works
    /// ```
    #[inline]
    pub fn reversed(&self) -> Self
    where
        T: 'static,
    {
        let self_fn = self.function.clone();
        RcComparator::new(move |a, b| self_fn(b, a))
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
    /// A new `RcComparator` that chains this comparator with another.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::comparator::{Comparator, RcComparator};
    /// use std::cmp::Ordering;
    ///
    /// let cmp1 = RcComparator::new(|a: &i32, b: &i32| {
    ///     (a % 2).cmp(&(b % 2))
    /// });
    /// let cmp2 = RcComparator::new(|a: &i32, b: &i32| a.cmp(b));
    /// let chained = cmp1.then_comparing(&cmp2);
    /// assert_eq!(chained.compare(&4, &2), Ordering::Greater);
    /// ```
    #[inline]
    pub fn then_comparing(&self, other: &Self) -> Self
    where
        T: 'static,
    {
        let first = self.function.clone();
        let second = other.function.clone();
        RcComparator::new(move |a, b| match first(a, b) {
            Ordering::Equal => second(a, b),
            ord => ord,
        })
    }

    /// Returns a comparator that compares values by a key extracted by the
    /// given function.
    ///
    /// # Parameters
    ///
    /// * `key_fn` - A function that extracts a comparable key from values
    ///
    /// # Returns
    ///
    /// A new `RcComparator` that compares by the extracted key.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::comparator::{Comparator, RcComparator};
    /// use std::cmp::Ordering;
    ///
    /// #[derive(Debug)]
    /// struct Person {
    ///     name: String,
    ///     age: i32,
    /// }
    ///
    /// let by_age = RcComparator::comparing(|p: &Person| &p.age);
    /// let p1 = Person { name: "Alice".to_string(), age: 30 };
    /// let p2 = Person { name: "Bob".to_string(), age: 25 };
    /// assert_eq!(by_age.compare(&p1, &p2), Ordering::Greater);
    /// ```
    #[inline]
    pub fn comparing<K, F>(key_fn: F) -> Self
    where
        K: Ord,
        F: Fn(&T) -> &K + 'static,
    {
        RcComparator::new(move |a, b| key_fn(a).cmp(key_fn(b)))
    }

    /// Converts this comparator into a closure.
    ///
    /// # Returns
    ///
    /// A closure that implements `Fn(&T, &T) -> Ordering`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::comparator::{Comparator, RcComparator};
    /// use std::cmp::Ordering;
    ///
    /// let cmp = RcComparator::new(|a: &i32, b: &i32| a.cmp(b));
    /// let func = cmp.into_fn();
    /// assert_eq!(func(&5, &3), Ordering::Greater);
    /// ```
    #[inline]
    pub fn into_fn(self) -> impl Fn(&T, &T) -> Ordering {
        move |a: &T, b: &T| (self.function)(a, b)
    }
}

impl<T> Comparator<T> for RcComparator<T> {
    #[inline]
    fn compare(&self, a: &T, b: &T) -> Ordering {
        (self.function)(a, b)
    }
}
