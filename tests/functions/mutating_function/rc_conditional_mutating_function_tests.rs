// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::MutatingFunction;
use qubit_function::RcMutatingFunction;

#[test]
fn test_rc_conditional_mutating_function_applies_matching_branch() {
    let increment = RcMutatingFunction::new(|value: &mut i32| {
        *value += 1;
        *value
    });
    let conditional = increment.when(|value: &i32| *value > 0);
    let function = conditional.or_else(|value: &mut i32| *value);

    let mut positive = 5;
    assert_eq!(function.apply(&mut positive), 6);
    assert_eq!(positive, 6);

    let mut negative = -5;
    assert_eq!(function.apply(&mut negative), -5);
    assert_eq!(negative, -5);
}
