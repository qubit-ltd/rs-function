// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxMutator;
use qubit_function::Mutator;

#[test]
fn test_conditional_mutator_converts_closures_for_both_branches() {
    let mutator = BoxMutator::new(|value: &mut i32| *value += 1)
        .when(|value: &i32| *value > 0)
        .or_else(|value: &mut i32| *value -= 1);

    let mut positive_value = 41;
    mutator.apply(&mut positive_value);
    assert_eq!(positive_value, 42, "the predicate branch should run");

    let mut negative_value = -41;
    mutator.apply(&mut negative_value);
    assert_eq!(negative_value, -42, "the else branch should run");
}
