// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Predicates Module
//!
//! This module provides predicate-related functional programming abstractions
//! for testing values and returning boolean results.

pub mod bi_predicate;
pub(crate) mod macros;
pub mod predicate;
#[cfg(feature = "stateful")]
#[path = "stateful_bi_predicate_module.rs"]
pub mod stateful_bi_predicate;
#[cfg(feature = "stateful")]
pub mod stateful_predicate;

pub use bi_predicate::ArcBiPredicate;
pub use bi_predicate::BiPredicate;
pub use bi_predicate::BoxBiPredicate;
#[cfg(feature = "rc")]
pub use bi_predicate::RcBiPredicate;
pub use predicate::ArcPredicate;
pub use predicate::BoxPredicate;
pub use predicate::Predicate;
#[cfg(feature = "rc")]
pub use predicate::RcPredicate;
#[cfg(feature = "stateful")]
pub use stateful_bi_predicate::ArcStatefulBiPredicate;
#[cfg(feature = "stateful")]
pub use stateful_bi_predicate::BoxStatefulBiPredicate;
#[cfg(all(feature = "stateful", feature = "rc"))]
pub use stateful_bi_predicate::RcStatefulBiPredicate;
#[cfg(feature = "stateful")]
pub use stateful_bi_predicate::StatefulBiPredicate;
#[cfg(feature = "stateful")]
pub use stateful_predicate::ArcStatefulPredicate;
#[cfg(feature = "stateful")]
pub use stateful_predicate::BoxStatefulPredicate;
#[cfg(all(feature = "stateful", feature = "rc"))]
pub use stateful_predicate::RcStatefulPredicate;
#[cfg(feature = "stateful")]
pub use stateful_predicate::StatefulPredicate;
