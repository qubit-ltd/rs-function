// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::ArcBiMutatingFunction;
use qubit_function::BiMutatingFunction;

#[test]
fn test_arc_conditional_bi_mutating_function_selects_and_applies_branch() {
    let increment_first = ArcBiMutatingFunction::new(|first: &mut i32, second: &mut i32| {
        *first += *second;
        *first
    });
    let multiply_first = ArcBiMutatingFunction::new(|first: &mut i32, second: &mut i32| {
        *first *= *second;
        *first
    });
    let conditional = increment_first
        .when(|first: &i32, _second: &i32| *first > 0)
        .or_else(multiply_first);

    let mut positive_first = 2;
    let mut positive_second = 3;
    assert_eq!(conditional.apply(&mut positive_first, &mut positive_second), 5);
    assert_eq!(positive_first, 5);
    assert_eq!(positive_second, 3);

    let mut negative_first = -2;
    let mut negative_second = 3;
    assert_eq!(conditional.apply(&mut negative_first, &mut negative_second), -6);
    assert_eq!(negative_first, -6);
    assert_eq!(negative_second, 3);
}
