// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxConditionalFunctionOnce;
use qubit_function::BoxFunctionOnce;
use qubit_function::FunctionOnce;

#[test]
fn test_box_conditional_function_once_selects_branch() {
    let apply = |value: i32| {
        let function = BoxFunctionOnce::new(|value: &i32| value * 2);
        let conditional: BoxConditionalFunctionOnce<i32, i32> = function.when(|value: &i32| *value > 0);
        conditional.or_else(|value: &i32| -value).apply(&value)
    };
    assert_eq!(apply(5), 10);
    assert_eq!(apply(-5), 5);
}
