// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

mod bi_predicate;
mod bi_predicate_2_tests;
#[cfg(feature = "rc")]
mod bi_predicate_3_tests;
#[cfg(feature = "rc")]
mod bi_predicate_4_tests;
#[cfg(feature = "rc")]
mod bi_predicate_tests;
mod macros;
mod predicate;
#[cfg(feature = "rc")]
mod predicate_always_predicates_tests;
mod predicate_closure_predicate_tests;
mod predicate_custom_predicate_tests;
mod predicate_different_types_tests;
#[cfg(feature = "rc")]
mod predicate_display_debug_tests;
#[cfg(feature = "rc")]
mod predicate_generic_function_tests;
#[cfg(feature = "rc")]
mod predicate_interior_mutability_tests;
#[cfg(feature = "rc")]
mod predicate_logical_operations_tests;
#[cfg(feature = "rc")]
mod predicate_not_composition_tests;
mod predicate_parameter_types_2_tests;
#[cfg(feature = "rc")]
mod predicate_parameter_types_tests;
#[cfg(feature = "rc")]
mod predicate_tests;
mod stateful_bi_predicate;
#[cfg(all(feature = "rc", feature = "stateful"))]
mod stateful_bi_predicate_tests;
mod stateful_predicate;
#[cfg(feature = "stateful")]
mod stateful_predicate_tests;
