/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::RcBiTransformer;

#[test]
fn test_rc_bi_transformer_observable_behavior() {
    let type_name = std::any::type_name::<RcBiTransformer<i32, i32, i32>>();
    assert!(type_name.contains("RcBiTransformer"), "{type_name}");
}
