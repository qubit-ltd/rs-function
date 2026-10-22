// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::ArcTransformer;
use qubit_function::Transformer;

#[test]
fn test_conditional_transformer_clone_observable_behavior() {
    let conditional = ArcTransformer::new(|value: i32| value + 1)
        .when(|value: &i32| *value > 0)
        .or_else(|value: i32| value - 1);
    let cloned = conditional.clone();

    assert_eq!(conditional.apply(41), 42);
    assert_eq!(conditional.apply(-41), -42);
    assert_eq!(cloned.apply(41), 42);
    assert_eq!(cloned.apply(-41), -42);
}
