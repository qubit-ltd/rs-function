// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxMutatingFunctionOnce;
use qubit_function::MutatingFunctionOnce;

#[test]
fn test_box_mutating_function_once_mutates_input() {
    let function = BoxMutatingFunctionOnce::new(|value: &mut i32| {
        *value *= 2;
        *value
    });
    let mut value = 6;
    assert_eq!(function.apply(&mut value), 12);
    assert_eq!(value, 12);
}
