// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Comparator Abstraction
//!
//! Provides a Rust implementation similar to Java's `Comparator` interface
//! for comparison operations and chaining.
//!
//! ## Design Overview
//!
//! This module separates the semantic [`Comparator`] contract from concrete
//! Box, Rc, and Arc ownership models.
//!
//! ### Core Components
//!
//! 1. **`Comparator<T>` trait**: A minimalist unified interface that only
//!    defines the core `compare` method. It does NOT include chaining methods
//!    like `then_comparing`, etc.
//!
//! 2. **Three Concrete Struct Implementations**:
//!    - [`BoxComparator<T>`]: Box-based single ownership implementation for
//!      stored, reusable comparison callbacks
//!    - [`ArcComparator<T>`]: Arc-based thread-safe shared ownership
//!      implementation for multi-threaded scenarios
//!    - `RcComparator<T>`: Rc-based single-threaded shared ownership
//!      implementation for single-threaded reuse
//!
//! 3. **Specialized Composition Methods**: Each struct implements its own
//!    inherent methods (`reversed`, `then_comparing`, etc.) that return the
//!    same concrete type, preserving their specific characteristics (e.g.,
//!    `ArcComparator` compositions remain `ArcComparator` and stay cloneable
//!    and thread-safe).
//!
//! 4. **Explicit Object Model**: Closures implement `Comparator<T>` directly,
//!    but chaining starts by constructing a Box, Rc, or Arc wrapper. This makes
//!    the ownership and sharing model visible at the call site.
//!
//! 5. **Unified Trait Implementation**: Compatible closures and the three
//!    structs implement the `Comparator<T>` trait, enabling them to be handled
//!    uniformly by generic functions.
//!
//! ## Ownership Model Coverage
//!
//! The three implementations correspond to three typical ownership
//! scenarios:
//!
//! | Type | Ownership | Clonable | Thread-Safe | API | Use Case |
//! |:-----|:----------|:---------|:------------|:----|:---------|
//! | [`BoxComparator`] | Single | ❌ | ❌ | consumes `self` | Owned reuse |
//! | [`ArcComparator`] | Shared | ✅ | ✅ | borrows `&self` | Multi-thread |
//! | `RcComparator` | Shared | ✅ | ❌ | borrows `&self` | Single-thread |
//!
//! ## Composition Properties
//!
//! ### 1. Type Preservation through Specialization
//!
//! By implementing composition methods on concrete structs rather than in
//! the trait, each type maintains its specific characteristics through
//! composition:
//!
//! ```rust
//! # {
//! use qubit_function::comparator::{Comparator, ArcComparator};
//! use std::cmp::Ordering;
//!
//! let arc_cmp = ArcComparator::new(|a: &i32, b: &i32| a.cmp(b));
//! let another = ArcComparator::new(|a: &i32, b: &i32| b.cmp(a));
//!
//! // Composition returns ArcComparator, preserving clonability and
//! // thread-safety
//! let combined = arc_cmp.then_comparing(another.clone());
//! let cloned = combined.clone();  // ✅ Still cloneable
//!
//! // Original comparators remain usable
//! assert_eq!(arc_cmp.compare(&5, &3), Ordering::Greater);
//! # }
//! ```
//!
//! ### 2. Borrowing Composition
//!
//! `ArcComparator` and `RcComparator` borrow the left comparator in their
//! composition methods. The right comparator is moved into the result, so
//! clone it at the call site when it must remain independently available:
//!
//! ```rust
//! # {
//! use qubit_function::comparator::{Comparator, ArcComparator};
//!
//! let cmp = ArcComparator::new(|a: &i32, b: &i32| a.cmp(b));
//!
//! // The left comparator remains available.
//! let reversed = cmp.reversed();
//! let chained = cmp.then_comparing(ArcComparator::new(|a: &i32, b: &i32| b.cmp(a)));
//!
//! // cmp is still available
//! cmp.compare(&1, &2);
//! # }
//! ```
//!
//! ### 3. Explicit Closure Composition
//!
//! Wrap a closure before composing it so the ownership model is explicit:
//!
//! ```rust
//! # {
//! use qubit_function::comparator::{Comparator, BoxComparator};
//! use std::cmp::Ordering;
//!
//! let cmp = BoxComparator::new(|a: &i32, b: &i32| a.cmp(b))
//!     .reversed()
//!     .then_comparing(BoxComparator::new(|a: &i32, b: &i32| b.cmp(a)));
//!
//! assert_eq!(cmp.compare(&5, &3), Ordering::Less);
//! # }
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
//! # {
//! use qubit_function::comparator::{Comparator, BoxComparator};
//! use std::cmp::Ordering;
//!
//! let cmp = BoxComparator::new(|a: &i32, b: &i32| a.cmp(b));
//! let rev = cmp.reversed();
//! assert_eq!(rev.compare(&5, &3), Ordering::Less);
//! # }
//! ```
//!
//! ### Chained Comparison
//!
//! ```rust
//! # {
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
//! # }
//! ```
#[allow(clippy::module_inception)]
mod comparator;
pub use comparator::Comparator;

mod box_comparator;
pub use box_comparator::BoxComparator;
mod arc_comparator;
pub use arc_comparator::ArcComparator;
#[cfg(feature = "rc")]
mod rc_comparator;
#[cfg(feature = "rc")]
pub use rc_comparator::RcComparator;
