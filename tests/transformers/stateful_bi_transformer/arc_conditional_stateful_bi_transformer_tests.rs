// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::ArcStatefulBiTransformer;
use qubit_function::StatefulBiTransformer;

#[test]
fn test_arc_conditional_stateful_bi_transformer_selects_matching_branch() {
    let add = ArcStatefulBiTransformer::new(|first: i32, second: i32| first + second);
    let multiply = ArcStatefulBiTransformer::new(|first: i32, second: i32| first * second);
    let mut conditional = add.when(|first: &i32, _second: &i32| *first > 0).or_else(multiply);

    assert_eq!(conditional.apply(5, 3), 8);
    assert_eq!(conditional.apply(-5, 3), -15);
}
