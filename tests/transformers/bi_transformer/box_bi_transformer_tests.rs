/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::BoxBiTransformer;

#[test]
fn test_box_bi_transformer_observable_behavior() {
    let type_name = std::any::type_name::<BoxBiTransformer<i32, i32, i32>>();
    assert!(type_name.contains("BoxBiTransformer"), "{type_name}");
}
