/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::FnPredicateOps;

#[test]
fn test_fn_predicate_ops_observable_behavior() {
    fn assert_ops<F: FnPredicateOps<i32>>(_: &F) {}

    let predicate = |value: &i32| *value > 0;
    assert_ops(&predicate);
    assert!(predicate(&1));
}
