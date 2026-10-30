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
fn test_box_stateful_mutating_function_retains_state() {
    let mut calls = 0;
    let mut function = BoxStatefulMutatingFunction::new(move |value: &mut i32| {
        calls += 1;
        *value += calls;
        *value
    });
    let mut value = 5;
    assert_eq!(function.apply(&mut value), 6);
    assert_eq!(function.apply(&mut value), 8);
}
