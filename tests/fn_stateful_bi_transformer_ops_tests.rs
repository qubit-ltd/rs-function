/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::FnStatefulBiTransformerOps;

#[test]
fn test_fn_stateful_bi_transformer_ops_observable_behavior() {
    fn assert_ops<F: FnStatefulBiTransformerOps<i32, i32, i32>>(_: &F) {}

    let mut calls = 0;
    let mut transformer = |left: i32, right: i32| {
        calls += 1;
        left + right + calls
    };
    assert_ops(&transformer);
    assert_eq!(transformer(20, 21), 42);
}
