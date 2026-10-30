// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxConditionalMutatingFunctionOnce;
use qubit_function::BoxMutatingFunctionOnce;
use qubit_function::MutatingFunctionOnce;

#[test]
fn test_box_conditional_mutating_function_once_selects_branch() {
    let apply = |mut value: i32| {
        let function = BoxMutatingFunctionOnce::new(|value: &mut i32| {
            *value += 2;
            *value
        });
        let conditional: BoxConditionalMutatingFunctionOnce<i32, i32> = function.when(|value: &i32| *value > 0);
        conditional
            .or_else(|value: &mut i32| {
                *value -= 2;
                *value
            })
            .apply(&mut value)
    };
    assert_eq!(apply(5), 7);
    assert_eq!(apply(-5), -7);
}
