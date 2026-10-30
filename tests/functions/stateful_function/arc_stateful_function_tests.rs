// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::ArcStatefulFunction;
use qubit_function::StatefulFunction;

#[test]
fn test_arc_stateful_function_retains_callback_state() {
    let mut calls = 0;
    let mut function = ArcStatefulFunction::new(move |value: &i32| {
        calls += 1;
        value + calls
    });
    assert_eq!(function.apply(&5), 6);
    assert_eq!(function.apply(&5), 7);
}
