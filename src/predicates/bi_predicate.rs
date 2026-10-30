// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # BiPredicate Abstraction
//!
//! Provides a Rust implementation similar to Java's `BiPredicate`
//! interface for testing whether two values satisfy a condition.
//!
//! A bi-predicate tests two values through shared references. Implementations
//! may still use interior mutability or observe external state.
//!
//! ## Design Philosophy
//!
//! `BiPredicate<T, U>` receives both inputs through shared references and is
//! called through `&self`. This does not permit mutation through those
//! references, but `Fn` callbacks may still use interior mutability or external
//! side effects. With the `stateful` feature, `StatefulBiPredicate` provides a
//! native `FnMut` callback and `&mut self` receiver. Logical composition
//! belongs to the concrete wrappers, whose ownership models differ.
//!
//! ## Type Selection Guide
//!
//! | Scenario | Recommended type | Reason |
//! | --- | --- | --- |
//! | Single ownership | `BoxBiPredicate` | Heap-backed type erasure |
//! | Single-threaded sharing | `RcBiPredicate` | Shared ownership without thread-safety overhead |
//! | Multi-threaded sharing | `ArcBiPredicate` | Thread-safe shared ownership |
//! | Mutable callback state | `BoxStatefulBiPredicate` or `ArcStatefulBiPredicate` (`stateful`); `RcStatefulBiPredicate` also requires `rc` | Native `FnMut` with `&mut self` |
//!
//! ## Examples
//!
//! ### Closures
//!
//! ```
//! use qubit_function::BiPredicate;
//!
//! let positive_sum = |x: &i32, y: &i32| x + y > 0;
//! assert!(positive_sum.test(&5, &3));
//! assert!(!positive_sum.test(&-3, &-7));
//! ```
//!
//! ### Box-based composition
//!
//! ```
//! use qubit_function::{BiPredicate, BoxBiPredicate};
//!
//! let positive_and_larger = BoxBiPredicate::new(|x: &i32, y: &i32| x + y > 0)
//!     .and(|x: &i32, y: &i32| x > y);
//! assert!(positive_and_larger.test(&10, &5));
//! assert!(!positive_and_larger.test(&3, &8));
//! ```
//!
//! `RcBiPredicate` is available with the `rc` feature. `ArcBiPredicate` can
//! be cloned and shared between threads when its callback bounds are met. The
//! stateful wrappers are available with `stateful`; the Rc variant also
//! requires `rc`.

pub(super) use crate::predicates::macros::constants::ALWAYS_FALSE_NAME;
pub(super) use crate::predicates::macros::constants::ALWAYS_TRUE_NAME;

mod bi_predicate_fn;
#[path = "bi_predicate/bi_predicate.rs"]
mod bi_predicate_trait;
mod send_sync_bi_predicate_fn;

pub(super) use bi_predicate_fn::BiPredicateFn;
pub use bi_predicate_trait::BiPredicate;
pub(super) use send_sync_bi_predicate_fn::SendSyncBiPredicateFn;

mod box_bi_predicate;
pub use box_bi_predicate::BoxBiPredicate;
mod arc_bi_predicate;
pub use arc_bi_predicate::ArcBiPredicate;
#[cfg(feature = "rc")]
mod rc_bi_predicate;
#[cfg(feature = "rc")]
pub use rc_bi_predicate::RcBiPredicate;
