/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::FnBiFunctionOps;

#[test]
fn test_fn_bi_function_ops_observable_behavior() {
    fn assert_ops<F: FnBiFunctionOps<i32, i32, i32>>(_: &F) {}

    let function = |left: &i32, right: &i32| left + right;
    assert_ops(&function);
    assert_eq!(function(&20, &22), 42);
}
