// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxMutatingFunction;
use qubit_function::MutatingFunction;

#[test]
fn test_box_mutating_function_updates_input_and_returns_result() {
    let function = BoxMutatingFunction::new(|value: &mut i32| {
        *value *= 2;
        *value
    });
    let mut value = 5;

    assert_eq!(function.apply(&mut value), 10);
    assert_eq!(value, 10);
}
