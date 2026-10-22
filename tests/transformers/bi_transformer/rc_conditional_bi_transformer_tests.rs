// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BiTransformer;
use qubit_function::RcBiTransformer;

#[test]
fn test_rc_conditional_bi_transformer_observable_behavior() {
    let add = RcBiTransformer::new(|left: i32, right: i32| left + right);
    let multiply = RcBiTransformer::new(|left: i32, right: i32| left * right);
    let conditional = add.when(|left: &i32, _right: &i32| *left > 0);
    let transformer = conditional.or_else(multiply);

    assert_eq!(transformer.apply(2, 3), 5);
    assert_eq!(transformer.apply(-2, 3), -6);
}
