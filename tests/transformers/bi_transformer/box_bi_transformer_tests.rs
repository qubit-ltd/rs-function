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
fn test_box_bi_transformer_observable_behavior() {
    let add = BoxBiTransformer::new(|first: i32, second: i32| first + second);

    assert_eq!(add.apply(20, 22), 42);
}
