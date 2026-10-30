// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::Function;
use qubit_function::RcConditionalFunction;
use qubit_function::RcFunction;

#[test]
fn test_rc_conditional_function_selects_branch() {
    let function = RcFunction::new(|value: &i32| value * 2);
    let conditional: RcConditionalFunction<i32, i32> = function.when(|value: &i32| *value > 0);
    let conditional = conditional.or_else(|value: &i32| -value);
    assert_eq!(conditional.apply(&3), 6);
    assert_eq!(conditional.apply(&-3), 3);
}
