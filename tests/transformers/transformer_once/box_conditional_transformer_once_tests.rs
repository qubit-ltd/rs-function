// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxTransformerOnce;
use qubit_function::TransformerOnce;

#[test]
fn test_box_conditional_transformer_once_selects_matching_branch() {
    let conditional = BoxTransformerOnce::new(|value: i32| value * 2)
        .when(|value: &i32| *value > 0)
        .or_else(|value: i32| -value);

    assert_eq!(conditional.apply(5), 10);

    let conditional = BoxTransformerOnce::new(|value: i32| value * 2)
        .when(|value: &i32| *value > 0)
        .or_else(|value: i32| -value);

    assert_eq!(conditional.apply(-5), 5);
}
