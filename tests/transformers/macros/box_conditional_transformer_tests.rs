// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxTransformer;
use qubit_function::Transformer;

#[test]
fn test_box_conditional_transformer_macro_behavior() {
    let transformer = BoxTransformer::new(|value: i32| value + 1)
        .when(|value: &i32| *value > 0)
        .or_else(|_: i32| 0);
    assert_eq!(transformer.apply(41), 42);
    assert_eq!(transformer.apply(-1), 0);
}
