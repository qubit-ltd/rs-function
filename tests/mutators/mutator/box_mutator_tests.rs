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
fn test_box_mutator_observable_behavior() {
    let mutator = BoxMutator::new(|value: &mut i32| *value *= 2);
    let mut value = 5;

    mutator.apply(&mut value);

    assert_eq!(value, 10, "BoxMutator should apply its callback to the value");
}
