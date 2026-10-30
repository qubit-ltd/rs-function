// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxConsumer;

#[test]
fn test_conditional_consumer_debug_display_formatting() {
    let consumer = BoxConsumer::new(|_value: &i32| {});
    let conditional_consumer = consumer.when(|value: &i32| *value > 0);

    let debug_output = format!("{conditional_consumer:?}");
    assert!(
        debug_output.contains("BoxConditionalConsumer"),
        "Debug output omitted the conditional consumer type: {debug_output}"
    );
    assert!(
        debug_output.contains("consumer"),
        "Debug output omitted the consumer field: {debug_output}"
    );
    assert!(
        debug_output.contains("predicate"),
        "Debug output omitted the predicate field: {debug_output}"
    );

    let display_output = format!("{conditional_consumer}");
    assert!(
        display_output.starts_with("BoxConditionalConsumer("),
        "Display output did not start with the conditional consumer wrapper: {display_output}"
    );
}
