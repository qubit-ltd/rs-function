/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Predicates Module
//!
//! This module provides predicate-related functional programming abstractions
//! for testing values and returning boolean results.
//!

pub mod bi_predicate;
#[doc(hidden)]
pub mod macros;
pub mod predicate;

pub use bi_predicate::{
    ArcBiPredicate,
    BiPredicate,
    BoxBiPredicate,
    FnBiPredicateOps,
    RcBiPredicate,
};
pub use predicate::{
    ArcPredicate,
    BoxPredicate,
    FnPredicateOps,
    Predicate,
    RcPredicate,
};
