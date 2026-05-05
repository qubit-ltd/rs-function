/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::FnStatefulFunctionOps;

#[test]
fn test_fn_stateful_function_ops_observable_behavior() {
    fn assert_ops<F: FnStatefulFunctionOps<i32, i32>>(_: &F) {}

    let mut calls = 0;
    let mut function = |value: &i32| {
        calls += 1;
        value + calls
    };
    assert_ops(&function);
    assert_eq!(function(&41), 42);
}
