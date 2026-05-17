/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::FnBiConsumerOps;
use std::sync::atomic::{
    AtomicI32,
    Ordering,
};

#[test]
fn test_fn_bi_consumer_ops_observable_behavior() {
    fn assert_ops<F: FnBiConsumerOps<i32, i32>>(_: &F) {}

    let count = AtomicI32::new(0);
    let consumer = |left: &i32, right: &i32| count.store(left + right, Ordering::SeqCst);
    assert_ops(&consumer);
    consumer(&20, &22);
    assert_eq!(count.load(Ordering::SeqCst), 42);
}
