// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::sync::Arc;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;

use qubit_function::ArcConsumer;
use qubit_function::Consumer;

#[test]
fn test_consumer_clone_preserves_callback_behavior() {
    let count = Arc::new(AtomicI32::new(0));
    let captured = Arc::clone(&count);
    let consumer = ArcConsumer::new(move |value: &i32| {
        captured.store(*value, Ordering::SeqCst);
    });
    let cloned_consumer = consumer.clone();
    cloned_consumer.accept(&42);
    assert_eq!(count.load(Ordering::SeqCst), 42);
}
