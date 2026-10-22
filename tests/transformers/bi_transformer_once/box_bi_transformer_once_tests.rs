// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BiTransformerOnce;
use qubit_function::BoxBiTransformerOnce;

#[test]
fn test_box_bi_transformer_once_applies_transformation() {
    let transformer = BoxBiTransformerOnce::new(|first: i32, second: i32| first + second);

    assert_eq!(transformer.apply(19, 23), 42);
}
