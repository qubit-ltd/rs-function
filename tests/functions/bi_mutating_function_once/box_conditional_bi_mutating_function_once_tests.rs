// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BiMutatingFunctionOnce;
use qubit_function::BoxBiMutatingFunctionOnce;
use qubit_function::BoxConditionalBiMutatingFunctionOnce;

#[test]
fn test_box_conditional_bi_mutating_function_once_selects_branch() {
    let apply = |mut left: i32, mut right: i32| {
        let function = BoxBiMutatingFunctionOnce::new(|left: &mut i32, right: &mut i32| {
            *left += *right;
            *left
        });
        let conditional: BoxConditionalBiMutatingFunctionOnce<i32, i32, i32> =
            function.when(|left: &i32, right: &i32| *left > 0 && *right > 0);
        conditional
            .or_else(|left: &mut i32, right: &mut i32| {
                *left -= *right;
                *left
            })
            .apply(&mut left, &mut right)
    };
    assert_eq!(apply(5, 3), 8);
    assert_eq!(apply(-5, 3), -8);
}
