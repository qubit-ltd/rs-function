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

use qubit_function::BiConsumer;
use qubit_function::BoxBiConsumer;

#[test]
fn test_box_bi_consumer_accepts_inputs() {
    let sum = Arc::new(AtomicI32::new(0));
    let captured = Arc::clone(&sum);
    let consumer = BoxBiConsumer::new(move |left: &i32, right: &i32| captured.store(left + right, Ordering::SeqCst));
    consumer.accept(&5, &3);
    assert_eq!(sum.load(Ordering::SeqCst), 8);
}
