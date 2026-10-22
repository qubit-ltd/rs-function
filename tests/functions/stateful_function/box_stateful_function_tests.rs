// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxStatefulFunction;
use qubit_function::StatefulFunction;

#[test]
fn test_box_stateful_function_reuses_mutable_callback_state() {
    let mut call_count = 0;
    let mut function = BoxStatefulFunction::new(|input: &i32| {
        call_count += 1;
        input + call_count
    });

    assert_eq!(
        function.apply(&10),
        11,
        "the first call should observe the initial callback state"
    );
    assert_eq!(
        function.apply(&10),
        12,
        "the second call should observe the updated callback state"
    );
}
