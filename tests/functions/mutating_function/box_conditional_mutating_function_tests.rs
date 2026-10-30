// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxConditionalMutatingFunction;
use qubit_function::BoxMutatingFunction;
use qubit_function::MutatingFunction;

#[test]
fn test_box_conditional_mutating_function_selects_branch() {
    let function = BoxMutatingFunction::new(|value: &mut i32| {
        *value += 2;
        *value
    });
    let conditional: BoxConditionalMutatingFunction<i32, i32> = function.when(|value: &i32| *value > 0);
    let conditional = conditional.or_else(|value: &mut i32| {
        *value -= 2;
        *value
    });
    let mut positive = 5;
    let mut negative = -5;
    assert_eq!(conditional.apply(&mut positive), 7);
    assert_eq!(conditional.apply(&mut negative), -7);
}
