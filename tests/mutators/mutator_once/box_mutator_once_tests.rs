// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxMutatorOnce;
use qubit_function::MutatorOnce;

#[test]
fn test_box_mutator_once_observable_behavior() {
    let increment = 3;
    let mutator = BoxMutatorOnce::new(move |value: &mut i32| *value += increment);
    let mut value = 4;

    mutator.apply(&mut value);

    assert_eq!(value, 7, "BoxMutatorOnce should apply its callback once");
}
