/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::FnConsumerOps;
use std::sync::atomic::{
    AtomicI32,
    Ordering,
};

#[test]
fn test_fn_consumer_ops_observable_behavior() {
    fn assert_ops<F: FnConsumerOps<i32>>(_: &F) {}

    let count = AtomicI32::new(0);
    let consumer = |value: &i32| count.store(*value, Ordering::SeqCst);
    assert_ops(&consumer);
    consumer(&42);
    assert_eq!(count.load(Ordering::SeqCst), 42);
}
