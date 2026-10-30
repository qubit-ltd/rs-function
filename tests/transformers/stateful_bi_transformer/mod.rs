// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

#[cfg(feature = "stateful")]
mod arc_conditional_stateful_bi_transformer_tests;
#[cfg(feature = "stateful")]
mod arc_stateful_bi_transformer_tests;
#[cfg(feature = "stateful")]
mod arc_stateful_binary_operator_tests;
#[cfg(feature = "stateful")]
mod box_conditional_stateful_bi_transformer_tests;
#[cfg(feature = "stateful")]
mod box_stateful_bi_transformer_tests;
#[cfg(feature = "stateful")]
mod box_stateful_binary_operator_tests;
#[cfg(all(feature = "rc", feature = "stateful"))]
mod rc_conditional_stateful_bi_transformer_tests;
#[cfg(all(feature = "rc", feature = "stateful"))]
mod rc_stateful_bi_transformer_tests;
#[cfg(all(feature = "rc", feature = "stateful"))]
mod rc_stateful_binary_operator_tests;
#[cfg(feature = "stateful")]
mod stateful_binary_operator_tests;
