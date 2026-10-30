// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxTransformer;
use qubit_function::Transformer;

#[test]
fn test_box_conditional_transformer_when_or_else_with_closure() {
    let double = BoxTransformer::new(|value: i32| value * 2);
    let negate = BoxTransformer::new(|value: i32| -value);
    let conditional = double.when(|value: &i32| *value > 0).or_else(negate);

    assert_eq!(conditional.apply(21), 42);
    assert_eq!(conditional.apply(-21), 21);
    assert_eq!(conditional.apply(0), 0);
}
