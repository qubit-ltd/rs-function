/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Mutators Module
//!
//! This module provides mutator-related functional programming abstractions
//! for modifying values in-place through mutable references.
//!

#[doc(hidden)]
pub mod macros;
pub mod mutator;
pub mod mutator_once;
pub mod stateful_mutator;

pub use mutator::{
    ArcConditionalMutator,
    ArcMutator,
    BoxConditionalMutator,
    BoxMutator,
    FnMutatorOps,
    Mutator,
    RcConditionalMutator,
    RcMutator,
};
pub use mutator_once::{
    BoxConditionalMutatorOnce,
    BoxMutatorOnce,
    FnMutatorOnceOps,
    MutatorOnce,
};
pub use stateful_mutator::{
    ArcConditionalStatefulMutator,
    ArcStatefulMutator,
    BoxConditionalStatefulMutator,
    BoxStatefulMutator,
    FnMutStatefulMutatorOps,
    RcConditionalStatefulMutator,
    RcStatefulMutator,
    StatefulMutator,
};
