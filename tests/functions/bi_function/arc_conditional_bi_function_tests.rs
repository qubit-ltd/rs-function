// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::ArcBiFunction;
use qubit_function::ArcConditionalBiFunction;
use qubit_function::BiFunction;

#[test]
fn test_arc_conditional_bi_function_selects_branch() {
    let function = ArcBiFunction::new(|left: &i32, right: &i32| left + right);
    let conditional: ArcConditionalBiFunction<i32, i32, i32> =
        function.when(|left: &i32, right: &i32| *left > 0 && *right > 0);
    let conditional = conditional.or_else(|_left: &i32, _right: &i32| 0);
    assert_eq!(conditional.apply(&5, &3), 8);
    assert_eq!(conditional.apply(&-5, &3), 0);
}
