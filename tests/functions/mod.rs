// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

mod bi_function;
mod bi_function_once;
#[cfg(feature = "once")]
mod bi_function_once_tests;
#[cfg(feature = "rc")]
mod bi_function_rc_conditional_bi_function_clone_tests;
#[cfg(feature = "rc")]
mod bi_function_tests;
mod bi_mutating_function;
mod bi_mutating_function_once;
#[cfg(feature = "once")]
mod bi_mutating_function_once_tests;
#[cfg(feature = "rc")]
mod bi_mutating_function_rc_conditional_bi_mutating_function_clone_tests;
#[cfg(feature = "rc")]
mod bi_mutating_function_tests;
mod box_function_apply_tests;
mod function;
#[cfg(feature = "rc")]
mod function_function_chain_multiple_tests;
mod function_once;
#[cfg(feature = "once")]
mod function_once_function_once_default_impl_tests;
#[cfg(all(feature = "rc", feature = "once"))]
mod function_once_tests;
#[cfg(all(feature = "rc", feature = "once"))]
mod function_tests;
mod macros;
mod mutating_function;
mod mutating_function_arc_mutating_function_tests;
#[cfg(feature = "rc")]
mod mutating_function_box_mutating_function_debug_display_tests;
mod mutating_function_closure_tests;
mod mutating_function_once;
#[cfg(feature = "once")]
mod mutating_function_once_tests;
#[cfg(feature = "rc")]
mod mutating_function_rc_mutating_function_tests;
mod mutating_function_tests;
mod stateful_function;
#[cfg(all(feature = "rc", feature = "stateful"))]
mod stateful_function_box_conditional_stateful_function_debug_display_tests;
#[cfg(all(feature = "rc", feature = "stateful"))]
mod stateful_function_rc_stateful_function_when_with_predicate_tests;
#[cfg(all(feature = "rc", feature = "stateful"))]
mod stateful_function_tests;
mod stateful_mutating_function;
#[cfg(feature = "stateful")]
mod stateful_mutating_function_arc_stateful_mutating_function_tests;
#[cfg(feature = "stateful")]
mod stateful_mutating_function_closure_tests;
#[cfg(all(feature = "rc", feature = "stateful"))]
mod stateful_mutating_function_rc_stateful_mutating_function_tests;
#[cfg(feature = "stateful")]
mod stateful_mutating_function_tests;
