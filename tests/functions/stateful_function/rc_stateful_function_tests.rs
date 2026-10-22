// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::RcStatefulFunction;
use qubit_function::StatefulFunction;

#[test]
fn test_rc_stateful_function_clones_share_mutable_callback_state() {
    let mut function = RcStatefulFunction::new({
        let mut state = 0;
        move |input: &i32| {
            state += input;
            state
        }
    });
    let mut clone = function.clone();

    assert_eq!(function.apply(&2), 2, "the first call should update the state");
    assert_eq!(clone.apply(&3), 5, "the clone should observe the shared state");
}
