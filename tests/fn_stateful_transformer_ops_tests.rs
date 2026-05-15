/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::FnStatefulTransformerOps;
use qubit_function::StatefulTransformer;

#[test]
fn test_fn_stateful_transformer_ops_observable_behavior() {
    fn assert_ops<F: FnStatefulTransformerOps<i32, i32>>(_: &F) {}

    let mut calls = 0;
    let mut transformer = |value: i32| {
        calls += 1;
        value + calls
    };
    assert_ops(&transformer);
    assert_eq!(transformer(41), 42);
}

#[test]
fn test_fn_stateful_transformer_ops_compose() {
    let mut calls = 0;
    let transformer = move |value: i32| {
        calls += 1;
        value * calls
    };

    let mut composed = transformer.compose(|value: i32| value + 1);

    assert_eq!(composed.apply(10), 11);
    assert_eq!(composed.apply(10), 22);
}
