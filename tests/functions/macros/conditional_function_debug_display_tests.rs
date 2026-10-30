// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxFunction;

#[test]
fn test_conditional_function_debug_display_observable_behavior() {
    let function = BoxFunction::new(|value: &i32| value + 1).when(|value: &i32| *value > 0);

    assert_eq!(
        format!("{function:?}"),
        "BoxConditionalFunction { function: BoxFunction { name: None, function: \"<function>\" }, predicate: BoxPredicate { name: None, function: \"<function>\" } }"
    );
    assert_eq!(
        format!("{function}"),
        "BoxConditionalFunction(BoxFunction, BoxPredicate(unnamed))"
    );
}
