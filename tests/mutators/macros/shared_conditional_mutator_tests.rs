// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::ArcMutator;
use qubit_function::Mutator;

#[test]
fn test_shared_conditional_mutator_observable_behavior() {
    let mutator = ArcMutator::new(|value: &mut i32| *value += 1)
        .when(|value: &i32| *value > 0)
        .or_else(ArcMutator::new(|value: &mut i32| *value -= 1));

    let mut positive_value = 41;
    mutator.apply(&mut positive_value);
    assert_eq!(positive_value, 42);

    let mut negative_value = -41;
    mutator.apply(&mut negative_value);
    assert_eq!(negative_value, -42);
}
