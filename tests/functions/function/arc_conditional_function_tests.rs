// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::ArcFunction;
use qubit_function::Function;

#[test]
fn test_arc_conditional_function_applies_matching_branch() {
    let double = ArcFunction::new(|value: &i32| value * 2);
    let identity = ArcFunction::<i32, i32>::identity();
    let conditional: ArcFunction<i32, i32> = double.when(|value: &i32| *value > 0).or_else(identity);

    assert_eq!(conditional.apply(&5), 10);
    assert_eq!(conditional.apply(&-5), -5);
}
