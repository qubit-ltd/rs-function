/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::FnStatefulBiConsumerOps;

#[test]
fn test_fn_stateful_bi_consumer_ops_observable_behavior() {
    fn assert_ops<F: FnStatefulBiConsumerOps<i32, i32>>(_: &F) {}

    let mut total = 0;
    let mut consumer = |left: &i32, right: &i32| total += left + right;
    assert_ops(&consumer);
    consumer(&20, &22);
    assert_eq!(total, 42);
}
