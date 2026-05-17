/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::FnStatefulConsumerOps;

#[test]
fn test_fn_stateful_consumer_ops_observable_behavior() {
    fn assert_ops<F: FnStatefulConsumerOps<i32>>(_: &F) {}

    let mut total = 0;
    let mut consumer = |value: &i32| total += *value;
    assert_ops(&consumer);
    consumer(&42);
    assert_eq!(total, 42);
}
