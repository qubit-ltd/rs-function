/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::FnBiConsumerOnceOps;
use std::cell::Cell;

#[test]
fn test_fn_bi_consumer_once_ops_observable_behavior() {
    fn assert_ops<F: FnBiConsumerOnceOps<i32, i32>>(_: &F) {}

    let count = Cell::new(0);
    let consumer = |left: &i32, right: &i32| count.set(left + right);
    assert_ops(&consumer);
    consumer(&20, &22);
    assert_eq!(count.get(), 42);
}
