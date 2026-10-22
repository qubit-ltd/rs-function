// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxStatefulMutator;
use qubit_function::StatefulMutator;

#[test]
fn test_box_stateful_mutator_applies_stateful_callback_in_sequence() {
    let mut calls = 0;
    let mut mutator = BoxStatefulMutator::new(move |value: &mut i32| {
        calls += 1;
        *value += calls;
    });

    let mut value = 10;
    mutator.apply(&mut value);
    assert_eq!(value, 11, "the first callback call should add one");

    mutator.apply(&mut value);
    assert_eq!(value, 13, "the second callback call should add two");
}
