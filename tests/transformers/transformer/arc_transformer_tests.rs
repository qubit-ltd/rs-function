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
fn test_arc_transformer_observable_behavior() {
    let double = ArcTransformer::new(|value: i32| value * 2);

    assert_eq!(double.apply(21), 42);
}
