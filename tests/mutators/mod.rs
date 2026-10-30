// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

mod macros;
mod mutator;
#[cfg(feature = "rc")]
mod mutator_conditional_mutator_and_then_tests;
#[cfg(feature = "rc")]
mod mutator_conditional_mutator_debug_display_tests;
mod mutator_once;
#[cfg(feature = "once")]
mod mutator_once_tests;
#[cfg(feature = "rc")]
mod mutator_rc_mutator_tests;
mod mutator_tests;
#[cfg(feature = "rc")]
mod mutator_unified_interface_tests;
mod mutator_wrapper_composition_tests;
mod stateful_mutator;
#[cfg(all(feature = "rc", feature = "stateful"))]
mod stateful_mutator_complex_scenarios_tests;
#[cfg(all(feature = "rc", feature = "stateful"))]
mod stateful_mutator_conditional_execution_2_tests;
#[cfg(all(feature = "rc", feature = "stateful"))]
mod stateful_mutator_conditional_execution_tests;
#[cfg(feature = "stateful")]
mod stateful_mutator_custom_mutator_default_impl_tests;
#[cfg(feature = "stateful")]
mod stateful_mutator_custom_types_tests;
#[cfg(feature = "stateful")]
mod stateful_mutator_edge_cases_tests;
#[cfg(all(feature = "rc", feature = "stateful"))]
mod stateful_mutator_rc_mutator_tests;
#[cfg(feature = "stateful")]
mod stateful_mutator_tests;
#[cfg(all(feature = "rc", feature = "stateful"))]
mod stateful_mutator_unified_interface_tests;
#[cfg(feature = "once")]
mod stateful_mutator_wrapper_composition_tests;
