/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `BoxComparator` public type.

#![allow(unused_imports)]

use super::*;

/// A boxed comparator with single ownership.
///
/// `BoxComparator` wraps a comparator function in a `Box`, providing single
/// ownership semantics. It is not cloneable and consumes `self` in
/// composition operations.
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
///
/// # Author
///
/// Haixing Hu
pub struct BoxComparator<T> {
    pub(super) function: Box<dyn Fn(&T, &T) -> Ordering>,
}

impl<T> BoxComparator<T> {
    /// Creates a new `BoxComparator` from a closure.
    ///
    /// # Parameters
    ///
    /// * `f` - The closure to wrap
    ///
    /// # Returns
    ///
    /// A new `BoxComparator` instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::comparator::BoxComparator;
    ///
    /// let cmp = BoxComparator::new(|a: &i32, b: &i32| a.cmp(b));
    /// ```
    #[inline]
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(&T, &T) -> Ordering + 'static,
    {
        Self {
            function: Box::new(f),
        }
    }

    /// Returns a comparator that imposes the reverse ordering.
    ///
    /// # Returns
    ///
    /// A new `BoxComparator` that reverses the comparison order.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::comparator::{Comparator, BoxComparator};
    /// use std::cmp::Ordering;
    ///
    /// let cmp = BoxComparator::new(|a: &i32, b: &i32| a.cmp(b));
    /// let rev = cmp.reversed();
    /// assert_eq!(rev.compare(&5, &3), Ordering::Less);
    /// ```
    #[inline]
    pub fn reversed(self) -> Self
    where
        T: 'static,
    {
        BoxComparator::new(move |a, b| (self.function)(b, a))
    }

    /// Returns a comparator that uses this comparator first, then another
    /// comparator if this one considers the values equal.
    ///
    /// # Parameters
    ///
    /// * `other` - The comparator to use for tie-breaking. **Note: This
    ///   parameter is passed by value and will transfer ownership.** If you
    ///   need to preserve the original comparator, clone it first (if it
    ///   implements `Clone`). Can be:
    ///   - A `BoxComparator<T>`
    ///   - An `RcComparator<T>`
    ///   - An `ArcComparator<T>`
    ///   - Any type implementing `Comparator<T>`
    ///
    /// # Returns
    ///
    /// A new `BoxComparator` that chains this comparator with another.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::comparator::{Comparator, BoxComparator};
    /// use std::cmp::Ordering;
    ///
    /// #[derive(Debug)]
    /// struct Person {
    ///     name: String,
    ///     age: i32,
    /// }
    ///
    /// let by_name = BoxComparator::new(|a: &Person, b: &Person| {
    ///     a.name.cmp(&b.name)
    /// });
    /// let by_age = BoxComparator::new(|a: &Person, b: &Person| {
    ///     a.age.cmp(&b.age)
    /// });
    ///
    /// // by_age is moved here
    /// let cmp = by_name.then_comparing(by_age);
    ///
    /// let p1 = Person { name: "Alice".to_string(), age: 30 };
    /// let p2 = Person { name: "Alice".to_string(), age: 25 };
    /// assert_eq!(cmp.compare(&p1, &p2), Ordering::Greater);
    /// // by_age.compare(&p1, &p2); // Would not compile - moved
    /// ```
    #[inline]
    pub fn then_comparing(self, other: Self) -> Self
    where
        T: 'static,
    {
        BoxComparator::new(move |a, b| match (self.function)(a, b) {
            Ordering::Equal => (other.function)(a, b),
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
    /// A new `BoxComparator` that compares by the extracted key.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::comparator::{Comparator, BoxComparator};
    /// use std::cmp::Ordering;
    ///
    /// #[derive(Debug)]
    /// struct Person {
    ///     name: String,
    ///     age: i32,
    /// }
    ///
    /// let by_age = BoxComparator::comparing(|p: &Person| &p.age);
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
        BoxComparator::new(move |a: &T, b: &T| key_fn(a).cmp(key_fn(b)))
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
    /// use qubit_function::comparator::{Comparator, BoxComparator};
    /// use std::cmp::Ordering;
    ///
    /// let cmp = BoxComparator::new(|a: &i32, b: &i32| a.cmp(b));
    /// let func = cmp.into_fn();
    /// assert_eq!(func(&5, &3), Ordering::Greater);
    /// ```
    #[inline]
    pub fn into_fn(self) -> impl Fn(&T, &T) -> Ordering {
        move |a: &T, b: &T| (self.function)(a, b)
    }
}

impl<T> Comparator<T> for BoxComparator<T> {
    #[inline]
    fn compare(&self, a: &T, b: &T) -> Ordering {
        (self.function)(a, b)
    }
}
