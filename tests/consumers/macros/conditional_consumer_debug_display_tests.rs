// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxConsumer;
use qubit_function::Consumer;

#[test]
fn test_conditional_consumer_debug_display_formatting() {
    let consumer = BoxConsumer::new(|_value: &i32| {});
    let conditional_consumer = consumer.when(|value: &i32| *value > 0);

    let debug_output = format!("{conditional_consumer:?}");
    assert!(debug_output.contains("BoxConditionalConsumer"));
    assert!(debug_output.contains("consumer"));
    assert!(debug_output.contains("predicate"));

    let display_output = format!("{conditional_consumer}");
    assert!(display_output.starts_with("BoxConditionalConsumer("));
}
