// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # StatefulBiPredicate Abstraction
//!
//! Provides bi-predicate wrappers for closures that implement
//! `FnMut(&T, &U) -> bool`. A stateful bi-predicate can update its own
//! internal state while testing two borrowed input values.
//!
//! Use [`BiPredicate`](crate::BiPredicate) for immutable
//! `Fn(&T, &U) -> bool` predicates and `StatefulBiPredicate` when the
//! predicate needs native `FnMut` semantics, such as counters, rolling
//! windows, sampling, or stateful filters over pairs of values.

use crate::predicates::macros::constants::ALWAYS_FALSE_NAME;
use crate::predicates::macros::constants::ALWAYS_TRUE_NAME;

#[path = "stateful_bi_predicate/arc_stateful_bi_predicate.rs"]
mod arc_stateful_bi_predicate;
pub use arc_stateful_bi_predicate::ArcStatefulBiPredicate;
#[path = "stateful_bi_predicate/box_stateful_bi_predicate.rs"]
mod box_stateful_bi_predicate;
pub use box_stateful_bi_predicate::BoxStatefulBiPredicate;
#[path = "stateful_bi_predicate.rs"]
mod stateful_bi_predicate_trait;
pub use stateful_bi_predicate_trait::StatefulBiPredicate;
#[cfg(feature = "rc")]
#[path = "stateful_bi_predicate/rc_stateful_bi_predicate.rs"]
mod rc_stateful_bi_predicate;
#[cfg(feature = "rc")]
pub use rc_stateful_bi_predicate::RcStatefulBiPredicate;
