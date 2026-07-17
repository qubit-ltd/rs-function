// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
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
