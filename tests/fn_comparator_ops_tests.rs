/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::FnComparatorOps;
use std::cmp::Ordering;

#[test]
fn test_fn_comparator_ops_observable_behavior() {
    fn assert_ops<F: FnComparatorOps<i32>>(_: &F) {}

    let comparator = |left: &i32, right: &i32| left.cmp(right);
    assert_ops(&comparator);
    assert_eq!(comparator(&2, &1), Ordering::Greater);
}
