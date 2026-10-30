// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::ArcConditionalStatefulMutatingFunction;
use qubit_function::ArcStatefulMutatingFunction;
use qubit_function::StatefulMutatingFunction;

#[test]
fn test_arc_conditional_stateful_mutating_function_selects_branch() {
    let function = ArcStatefulMutatingFunction::new(|value: &mut i32| {
        *value += 1;
        *value
    });
    let conditional: ArcConditionalStatefulMutatingFunction<i32, i32> = function.when(|value: &i32| *value > 0);
    let mut conditional = conditional.or_else(ArcStatefulMutatingFunction::<i32, i32>::identity());
    let mut positive = 4;
    let mut negative = -4;
    assert_eq!(conditional.apply(&mut positive), 5);
    assert_eq!(conditional.apply(&mut negative), -4);
}
