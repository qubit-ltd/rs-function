/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Comparator Abstraction
//!
//! Provides a Rust implementation similar to Java's `Comparator` interface
//! for comparison operations and chaining.
//!
//! ## Design Overview
//!
//! This module adopts the **Trait + Multiple Implementations** design
//! pattern, which is the most flexible and elegant approach for
//! implementing comparators in Rust. It achieves a perfect balance
//! between semantic clarity, type safety, and API flexibility.
//!
//! ### Core Components
//!
//! 1. **`Comparator<T>` trait**: A minimalist unified interface that only
//!    defines the core `compare` method and type conversion methods
//!    (`into_*`). It does NOT include chaining methods like
//!    `then_comparing`, etc.
//!
//! 2. **Three Concrete Struct Implementations**:
//!    - [`BoxComparator<T>`]: Box-based single ownership implementation
//!      for one-time use scenarios
//!    - [`ArcComparator<T>`]: Arc-based thread-safe shared ownership
//!      implementation for multi-threaded scenarios
//!    - [`RcComparator<T>`]: Rc-based single-threaded shared ownership
//!      implementation for single-threaded reuse
//!
//! 3. **Specialized Composition Methods**: Each struct implements its own
//!    inherent methods (`reversed`, `then_comparing`, etc.) that return
//!    the same concrete type, preserving their specific characteristics
//!    (e.g., `ArcComparator` compositions remain `ArcComparator` and stay
//!    cloneable and thread-safe).
//!
//! 4. **Extension Trait for Closures**: The `FnComparatorOps<T>`
//!    extension trait provides composition methods for all closures and
//!    function pointers, returning `BoxComparator<T>` to initiate method
//!    chaining.
//!
//! 5. **Unified Trait Implementation**: All closures and the three
//!    structs implement the `Comparator<T>` trait, enabling them to be
//!    handled uniformly by generic functions.
//!
//! ## Ownership Model Coverage
//!
//! The three implementations correspond to three typical ownership
//! scenarios:
//!
//! | Type | Ownership | Clonable | Thread-Safe | API | Use Case |
//! |:-----|:----------|:---------|:------------|:----|:---------|
//! | [`BoxComparator`] | Single | ❌ | ❌ | consumes `self` | One-time |
//! | [`ArcComparator`] | Shared | ✅ | ✅ | borrows `&self` | Multi-thread |
//! | [`RcComparator`] | Shared | ✅ | ❌ | borrows `&self` | Single-thread |
//!
//! ## Key Design Advantages
//!
//! ### 1. Type Preservation through Specialization
//!
//! By implementing composition methods on concrete structs rather than in
//! the trait, each type maintains its specific characteristics through
//! composition:
//!
//! ```rust
//! use qubit_function::comparator::{Comparator, ArcComparator};
//! use std::cmp::Ordering;
//!
//! let arc_cmp = ArcComparator::new(|a: &i32, b: &i32| a.cmp(b));
//! let another = ArcComparator::new(|a: &i32, b: &i32| b.cmp(a));
//!
//! // Composition returns ArcComparator, preserving clonability and
//! // thread-safety
//! let combined = arc_cmp.then_comparing(&another);
//! let cloned = combined.clone();  // ✅ Still cloneable
//!
//! // Original comparators remain usable
//! assert_eq!(arc_cmp.compare(&5, &3), Ordering::Greater);
//! ```
//!
//! ### 2. Elegant API without Explicit Cloning
//!
//! `ArcComparator` and `RcComparator` use `&self` in their composition
//! methods, providing a natural experience without requiring explicit
//! `.clone()` calls:
//!
//! ```rust
//! use qubit_function::comparator::{Comparator, ArcComparator};
//!
//! let cmp = ArcComparator::new(|a: &i32, b: &i32| a.cmp(b));
//!
//! // No need for explicit clone()
//! let reversed = cmp.reversed();
//! let chained = cmp.then_comparing(&ArcComparator::new(|a: &i32, b: &i32| b.cmp(a)));
//!
//! // cmp is still available
//! cmp.compare(&1, &2);
//! ```
//!
//! ### 3. Efficient Closure Composition
//!
//! The `FnComparatorOps` extension trait allows direct composition on
//! closures:
//!
//! ```rust
//! use qubit_function::comparator::{Comparator, FnComparatorOps, BoxComparator};
//! use std::cmp::Ordering;
//!
//! let cmp = (|a: &i32, b: &i32| a.cmp(b))
//!     .reversed()
//!     .then_comparing(BoxComparator::new(|a: &i32, b: &i32| b.cmp(a)));
//!
//! assert_eq!(cmp.compare(&5, &3), Ordering::Less);
//! ```
//!
//! ## Usage Examples
//!
//! ### Basic Comparison
//!
//! ```rust
//! use qubit_function::comparator::{Comparator, BoxComparator};
//! use std::cmp::Ordering;
//!
//! let cmp = BoxComparator::new(|a: &i32, b: &i32| a.cmp(b));
//! assert_eq!(cmp.compare(&5, &3), Ordering::Greater);
//! ```
//!
//! ### Reversed Comparison
//!
//! ```rust
//! use qubit_function::comparator::{Comparator, BoxComparator};
//! use std::cmp::Ordering;
//!
//! let cmp = BoxComparator::new(|a: &i32, b: &i32| a.cmp(b));
//! let rev = cmp.reversed();
//! assert_eq!(rev.compare(&5, &3), Ordering::Less);
//! ```
//!
//! ### Chained Comparison
//!
//! ```rust
//! use qubit_function::comparator::{Comparator, BoxComparator};
//! use std::cmp::Ordering;
//!
//! #[derive(Debug)]
//! struct Person {
//!     name: String,
//!     age: i32,
//! }
//!
//! let by_name = BoxComparator::new(|a: &Person, b: &Person| {
//!     a.name.cmp(&b.name)
//! });
//! let by_age = BoxComparator::new(|a: &Person, b: &Person| {
//!     a.age.cmp(&b.age)
//! });
//! let cmp = by_name.then_comparing(by_age);
//!
//! let p1 = Person { name: "Alice".to_string(), age: 30 };
//! let p2 = Person { name: "Alice".to_string(), age: 25 };
//! assert_eq!(cmp.compare(&p1, &p2), Ordering::Greater);
//! ```
//!
//! ## Author
//!
//! Haixing Hu
use std::cmp::Ordering;
use std::rc::Rc;
use std::sync::Arc;

mod box_comparator;
pub use box_comparator::BoxComparator;
mod arc_comparator;
pub use arc_comparator::ArcComparator;
mod rc_comparator;
pub use rc_comparator::RcComparator;
mod fn_comparator_ops;
pub use fn_comparator_ops::FnComparatorOps;

// ==========================================================================
// Type Aliases
// ==========================================================================

/// A trait for comparison operations.
///
/// This trait defines the core comparison operation and conversion methods.
/// It does NOT include composition methods like `reversed` or
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
///
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
    fn compare(&self, a: &T, b: &T) -> Ordering;

    /// Converts this comparator into a `BoxComparator`.
    ///
    /// # Returns
    ///
    /// A new `BoxComparator` wrapping this comparator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::comparator::{Comparator, BoxComparator};
    ///
    /// let cmp = BoxComparator::new(|a: &i32, b: &i32| a.cmp(b));
    /// let boxed = cmp.into_box();
    /// ```
    #[inline]
    fn into_box(self) -> BoxComparator<T>
    where
        Self: Sized + 'static,
    {
        BoxComparator::new(move |a, b| self.compare(a, b))
    }

    /// Converts this comparator into an `ArcComparator`.
    ///
    /// # Returns
    ///
    /// A new `ArcComparator` wrapping this comparator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::comparator::{Comparator, ArcComparator};
    ///
    /// let cmp = ArcComparator::new(|a: &i32, b: &i32| a.cmp(b));
    /// let arc = cmp.into_arc();
    /// ```
    #[inline]
    fn into_arc(self) -> ArcComparator<T>
    where
        Self: Sized + Send + Sync + 'static,
    {
        ArcComparator::new(move |a, b| self.compare(a, b))
    }

    /// Converts this comparator into an `RcComparator`.
    ///
    /// # Returns
    ///
    /// A new `RcComparator` wrapping this comparator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::comparator::{Comparator, RcComparator};
    ///
    /// let cmp = RcComparator::new(|a: &i32, b: &i32| a.cmp(b));
    /// let rc = cmp.into_rc();
    /// ```
    #[inline]
    fn into_rc(self) -> RcComparator<T>
    where
        Self: Sized + 'static,
    {
        RcComparator::new(move |a, b| self.compare(a, b))
    }

    /// Converts this comparator into a closure that implements
    /// `Fn(&T, &T) -> Ordering`.
    ///
    /// This method consumes the comparator and returns a closure that
    /// can be used anywhere a function or closure is expected.
    ///
    /// # Returns
    ///
    /// An implementation that can be called as `Fn(&T, &T) -> Ordering`.
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
    fn into_fn(self) -> impl Fn(&T, &T) -> Ordering
    where
        Self: Sized + 'static,
    {
        move |a: &T, b: &T| self.compare(a, b)
    }
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
    #[inline]
    fn compare(&self, a: &T, b: &T) -> Ordering {
        self(a, b)
    }
}
