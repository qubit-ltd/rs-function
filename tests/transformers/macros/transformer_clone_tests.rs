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
fn test_transformer_clone_observable_behavior() {
    let transformer = ArcTransformer::new(|value: i32| value + 1);
    let cloned = transformer.clone();

    assert_eq!(transformer.apply(41), 42);
    assert_eq!(cloned.apply(41), 42);
}
