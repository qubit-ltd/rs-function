// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Mutators Module
//!
//! This module provides mutator-related functional programming abstractions
//! for modifying values in-place through mutable references.

pub(crate) mod macros;
pub mod mutator;
#[cfg(feature = "once")]
pub mod mutator_once;
#[cfg(feature = "stateful")]
pub mod stateful_mutator;

pub use mutator::ArcConditionalMutator;
pub use mutator::ArcMutator;
pub use mutator::BoxConditionalMutator;
pub use mutator::BoxMutator;
pub use mutator::Mutator;
#[cfg(feature = "rc")]
pub use mutator::RcConditionalMutator;
#[cfg(feature = "rc")]
pub use mutator::RcMutator;
#[cfg(feature = "once")]
pub use mutator_once::BoxConditionalMutatorOnce;
#[cfg(feature = "once")]
pub use mutator_once::BoxMutatorOnce;
#[cfg(feature = "once")]
pub use mutator_once::MutatorOnce;
#[cfg(feature = "stateful")]
pub use stateful_mutator::ArcConditionalStatefulMutator;
#[cfg(feature = "stateful")]
pub use stateful_mutator::ArcStatefulMutator;
#[cfg(feature = "stateful")]
pub use stateful_mutator::BoxConditionalStatefulMutator;
#[cfg(feature = "stateful")]
pub use stateful_mutator::BoxStatefulMutator;
#[cfg(all(feature = "stateful", feature = "rc"))]
pub use stateful_mutator::RcConditionalStatefulMutator;
#[cfg(all(feature = "stateful", feature = "rc"))]
pub use stateful_mutator::RcStatefulMutator;
#[cfg(feature = "stateful")]
pub use stateful_mutator::StatefulMutator;
