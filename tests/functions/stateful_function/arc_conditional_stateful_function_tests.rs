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
fn test_arc_conditional_stateful_function_observable_behavior() {
    let mut function = ArcStatefulFunction::new(|input: &i32| input * 2)
        .when(|input: &i32| *input > 0)
        .or_else(|input: &i32| -input);

    assert_eq!(
        function.apply(&5),
        10,
        "the conditional branch should handle positive input"
    );
    assert_eq!(
        function.apply(&-5),
        5,
        "the alternative branch should handle negative input"
    );
}
