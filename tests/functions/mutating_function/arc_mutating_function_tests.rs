// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::ArcMutatingFunction;
use qubit_function::MutatingFunction;

#[test]
fn test_arc_mutating_function_mutates_input() {
    let function = ArcMutatingFunction::new(|value: &mut i32| {
        *value += 2;
        *value
    });
    let mut value = 5;
    assert_eq!(function.apply(&mut value), 7);
    assert_eq!(value, 7);
}
