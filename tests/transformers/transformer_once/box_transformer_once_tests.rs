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
fn test_box_transformer_once_applies_transformation() {
    let double = BoxTransformerOnce::new(|value: i32| value * 2);

    assert_eq!(double.apply(21), 42);
}
