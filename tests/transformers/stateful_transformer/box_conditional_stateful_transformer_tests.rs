// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxStatefulTransformer;
use qubit_function::StatefulTransformer;

#[test]
fn test_box_conditional_stateful_transformer_selects_matching_branch() {
    let transformer = BoxStatefulTransformer::new(|value: i32| value * 2);
    let mut conditional = transformer.when(|value: &i32| *value > 0).or_else(|value: i32| -value);

    assert_eq!(conditional.apply(5), 10);
    assert_eq!(conditional.apply(-5), 5);
}
