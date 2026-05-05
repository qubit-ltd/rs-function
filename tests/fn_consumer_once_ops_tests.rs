/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::FnConsumerOnceOps;
use std::cell::Cell;

#[test]
fn test_fn_consumer_once_ops_observable_behavior() {
    fn assert_ops<F: FnConsumerOnceOps<i32>>(_: &F) {}

    let count = Cell::new(0);
    let consumer = |value: &i32| count.set(*value);
    assert_ops(&consumer);
    consumer(&42);
    assert_eq!(count.get(), 42);
}
