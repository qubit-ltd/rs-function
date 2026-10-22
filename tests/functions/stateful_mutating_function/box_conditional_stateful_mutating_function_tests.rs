// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxStatefulMutatingFunction;
use qubit_function::StatefulMutatingFunction;

#[test]
fn test_box_conditional_stateful_mutating_function_observable_behavior() {
    let increment = BoxStatefulMutatingFunction::new(|value: &mut i32| {
        *value += 1;
        *value
    });
    let negate = BoxStatefulMutatingFunction::new(|value: &mut i32| {
        *value = -*value;
        *value
    });
    let mut conditional = increment.when(|value: &i32| *value > 0).or_else(negate);

    let mut positive = 5;
    let mut negative = -5;

    assert_eq!(conditional.apply(&mut positive), 6);
    assert_eq!(positive, 6);
    assert_eq!(conditional.apply(&mut negative), 5);
    assert_eq!(negative, 5);
}
