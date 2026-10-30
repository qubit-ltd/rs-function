// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

mod bi_transformer;
mod bi_transformer_arc_bi_transformer_once_tests;
mod bi_transformer_arc_bi_transformer_tests;
mod bi_transformer_box_bi_transformer_once_tests;
mod bi_transformer_box_conditional_tests;
mod bi_transformer_closure_bi_transformer_tests;
#[cfg(feature = "rc")]
mod bi_transformer_conditional_transformer_display_debug_tests;
mod bi_transformer_custom_bi_transformer_tests;
mod bi_transformer_edge_cases_tests;
mod bi_transformer_once;
#[cfg(feature = "once")]
mod bi_transformer_once_box_bi_transformer_once_tests;
#[cfg(feature = "once")]
mod bi_transformer_once_complex_transformation_tests;
#[cfg(feature = "once")]
mod bi_transformer_once_conditional_tests;
#[cfg(feature = "once")]
mod bi_transformer_once_edge_case_tests;
#[cfg(feature = "once")]
mod bi_transformer_once_ownership_tests;
#[cfg(feature = "once")]
mod bi_transformer_once_tests;
#[cfg(feature = "once")]
mod bi_transformer_once_type_tests;
#[cfg(feature = "rc")]
mod bi_transformer_rc_bi_transformer_once_tests;
#[cfg(feature = "rc")]
mod bi_transformer_rc_bi_transformer_tests;
#[cfg(feature = "rc")]
mod bi_transformer_rc_conditional_tests;
mod bi_transformer_tests;
mod bi_transformer_trait_usage_tests;
mod macros;
mod stateful_bi_transformer;
#[cfg(all(feature = "rc", feature = "stateful"))]
mod stateful_bi_transformer_box_stateful_bi_transformer_display_with_name_tests;
#[cfg(all(feature = "rc", feature = "stateful"))]
mod stateful_bi_transformer_conditional_stateful_bi_transformer_display_debug_tests;
#[cfg(all(feature = "rc", feature = "stateful"))]
mod stateful_bi_transformer_rc_stateful_bi_transformer_when_preserves_original_tests;
#[cfg(all(feature = "rc", feature = "stateful"))]
mod stateful_bi_transformer_tests;
mod stateful_transformer;
#[cfg(all(feature = "rc", feature = "stateful"))]
mod stateful_transformer_box_stateful_transformer_display_with_name_tests;
#[cfg(all(feature = "rc", feature = "stateful"))]
mod stateful_transformer_conditional_stateful_transformer_display_debug_tests;
#[cfg(all(feature = "rc", feature = "stateful"))]
mod stateful_transformer_rc_conditional_mapper_clone_tests;
#[cfg(all(feature = "rc", feature = "stateful"))]
mod stateful_transformer_tests;
mod transformer;
mod transformer_arc_transformer_tests;
#[cfg(feature = "rc")]
mod transformer_complex_composition_tests;
mod transformer_edge_cases_tests;
mod transformer_once;
#[cfg(all(feature = "rc", feature = "once"))]
mod transformer_once_tests;
#[cfg(feature = "rc")]
mod transformer_rc_conditional_tests;
mod transformer_tests;
mod transformer_to_conversion_tests;
mod transformer_trait_usage_tests;
#[cfg(feature = "rc")]
mod transformer_transformer_once_tests;
#[cfg(feature = "rc")]
mod transformer_type_conversion_tests;
