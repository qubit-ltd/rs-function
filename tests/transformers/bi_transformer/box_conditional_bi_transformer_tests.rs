// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BiTransformer;
use qubit_function::BoxBiTransformer;

#[test]
fn test_box_conditional_bi_transformer_selects_matching_branch() {
    let transformer = BoxBiTransformer::new(|left: i32, right: i32| left - right);
    let conditional = transformer
        .when(|left: &i32, _right: &i32| *left > 0)
        .or_else(|left, right| left + right);

    assert_eq!(conditional.apply(9, 4), 5);
    assert_eq!(conditional.apply(-9, 4), -5);
}
