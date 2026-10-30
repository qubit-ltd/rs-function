// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Qubit Function
//!
//! Provides functional programming abstractions for Rust, including:
//!
//! - **Function types**: Compute results from borrowed values
//! - **MutatingFunction types**: Mutate borrowed values and return results
//! - **BiFunction types**: Compute results from two borrowed values
//! - **BiMutatingFunction types**: Mutate two borrowed values and return
//!   results
//! - **Transformer types**: Transform values from type T to type R
//! - **UnaryOperator types**: Transform values of type T to the same type T
//! - **BiTransformer types**: Transform two values to produce a result
//! - **BinaryOperator types**: Transform two values of type T to produce a T
//! - **StatefulBinaryOperator types**: Stateful transform two values of type T
//!   to produce a T
//! - **Consumer types**: Functions that consume values without returning
//! - **BiConsumer types**: Functions that consume two values without returning
//! - **Predicate types**: Functions that test values and return boolean
//! - **StatefulPredicate types**: Stateful functions that test values and
//!   return boolean
//! - **BiPredicate types**: Functions that test two values and return boolean
//! - **StatefulBiPredicate types**: Stateful functions that test two values and
//!   return boolean
//! - **Supplier types**: Functions that produce values without input
//! - **Mutator types**: Functions that mutate values in place
//! - **Task types**: Fallible zero-argument and mutable-input actions and
//!   computations
//! - **Tester types**: Functions that test zero-argument conditions
//! - **Comparator types**: Functions that compare values and return ordering

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
//!
//! # Diagnostic names
//!
//! Diagnostic names identify a wrapper for logs and other diagnostics. Callable
//! `map` and `map_err` preserve the source name. Their `and_then` methods
//! return an unnamed wrapper. Supplier `map` preserves its source name, while
//! `filter` and `zip` return unnamed wrappers. Stateful supplier `map_stateful`
//! and `memoize`, comparator `reversed`, and predicate/tester logical negation
//! preserve the source name. Methods that combine independent sources, such as
//! `then_callable`, `then_callable_with`, and `then_comparing`, return unnamed
//! wrappers; names are not automatically concatenated.
//!
//! Cloning a wrapper copies its diagnostic name independently: changing the
//! name of one clone does not rename the other. Assign a name after composing
//! operations when the final wrapper needs a useful diagnostic name:
//!
//! ```
//! use qubit_function::{BoxCallable, Callable};
//!
//! let mut task = BoxCallable::new_with_name("load", || Ok::<i32, String>(20))
//!     .map(|value| value + 1)
//!     .and_then(|value| Ok(value * 2))
//!     .with_name("load-and-transform");
//! assert_eq!(task.name(), Some("load-and-transform"));
//! assert_eq!(task.call(), Ok(42));
//! ```

// Module declarations
pub mod comparator;
pub mod consumers;
pub mod functions;
pub(crate) mod internal;
pub(crate) mod macros;
pub mod mutators;
pub mod predicates;
pub mod suppliers;
pub mod tasks;
pub mod testers;
pub mod transformers;

pub use comparator::*;
pub use consumers::*;
pub use functions::*;
pub use mutators::*;
pub use predicates::*;
pub use suppliers::*;
pub use tasks::*;
pub use testers::*;
pub use transformers::*;
