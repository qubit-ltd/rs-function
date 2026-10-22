// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::RcStatefulMutatingFunction;
use qubit_function::StatefulMutatingFunction;

#[test]
fn test_rc_stateful_mutating_function_observable_behavior() {
    let mut call_count = 0;
    let mut function = RcStatefulMutatingFunction::new(move |value: &mut i32| {
        call_count += 1;
        *value += 1;
        call_count
    });

    let mut value = 5;
    assert_eq!(function.apply(&mut value), 1);
    assert_eq!(value, 6);
    assert_eq!(function.apply(&mut value), 2);
    assert_eq!(value, 7);
}
