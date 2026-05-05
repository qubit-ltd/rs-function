/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::FnFunctionOps;

#[test]
fn test_fn_function_ops_observable_behavior() {
    fn assert_ops<F: FnFunctionOps<i32, i32>>(_: &F) {}

    let function = |value: &i32| value + 1;
    assert_ops(&function);
    assert_eq!(function(&41), 42);
}
