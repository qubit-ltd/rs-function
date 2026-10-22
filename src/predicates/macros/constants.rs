// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! # Predicate Constants
//!
//! Defines constants used by Predicate and BiPredicate.

/// Stable name assigned to predicates created by `always_true` constructors.
///
/// The value is exposed through each predicate's `name()` method and can be
/// used to identify predicates that always evaluate to `true`.
pub const ALWAYS_TRUE_NAME: &str = "always_true";

/// Stable name assigned to predicates created by `always_false` constructors.
///
/// The value is exposed through each predicate's `name()` method and can be
/// used to identify predicates that always evaluate to `false`.
pub const ALWAYS_FALSE_NAME: &str = "always_false";
