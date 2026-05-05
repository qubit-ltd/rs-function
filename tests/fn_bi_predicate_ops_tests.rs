/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::FnBiPredicateOps;

#[test]
fn test_fn_bi_predicate_ops_observable_behavior() {
    fn assert_ops<F: FnBiPredicateOps<i32, i32>>(_: &F) {}

    let predicate = |left: &i32, right: &i32| left < right;
    assert_ops(&predicate);
    assert!(predicate(&1, &2));
}
