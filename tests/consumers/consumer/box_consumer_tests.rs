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

use qubit_function::BoxConsumer;
use qubit_function::Consumer;

#[test]
fn test_box_consumer_accepts_input() {
    let consumed = Arc::new(AtomicI32::new(0));
    let captured = Arc::clone(&consumed);
    let consumer = BoxConsumer::new(move |value: &i32| captured.store(*value, Ordering::SeqCst));
    consumer.accept(&7);
    assert_eq!(consumed.load(Ordering::SeqCst), 7);
}
