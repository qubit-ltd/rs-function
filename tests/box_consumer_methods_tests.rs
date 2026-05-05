/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::{
    BoxConsumer,
    Consumer,
};
use std::sync::{
    Arc,
    atomic::{
        AtomicI32,
        Ordering,
    },
};

#[test]
fn test_box_consumer_methods_observable_behavior() {
    let count = Arc::new(AtomicI32::new(0));
    let captured = Arc::clone(&count);
    let consumer = BoxConsumer::new(move |value: &i32| {
        captured.store(*value, Ordering::SeqCst);
    });
    consumer.accept(&42);
    assert_eq!(count.load(Ordering::SeqCst), 42);
}
