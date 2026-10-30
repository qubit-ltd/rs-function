// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxConditionalStatefulFunction;
use qubit_function::BoxStatefulFunction;
use qubit_function::StatefulFunction;

#[test]
fn test_box_conditional_stateful_function_selects_branch() {
    let function = BoxStatefulFunction::new(|value: &i32| value * 2);
    let conditional: BoxConditionalStatefulFunction<i32, i32> = function.when(|value: &i32| *value > 0);
    let mut conditional = conditional.or_else(|value: &i32| -value);
    assert_eq!(conditional.apply(&3), 6);
    assert_eq!(conditional.apply(&-3), 3);
}
