/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::FnStatefulSupplierOps;

#[test]
fn test_fn_stateful_supplier_ops_observable_behavior() {
    fn assert_ops<F: FnStatefulSupplierOps<i32>>(_: &F) {}

    let mut value = 41;
    let mut supplier = || {
        value += 1;
        value
    };
    assert_ops(&supplier);
    assert_eq!(supplier(), 42);
}
