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

use qubit_function::ArcStatefulBiConsumer;
use qubit_function::StatefulBiConsumer;

#[test]
fn test_arc_stateful_bi_consumer_retains_callback_state() {
    let calls = Arc::new(AtomicI32::new(0));
    let captured = Arc::clone(&calls);
    let mut consumer = ArcStatefulBiConsumer::new(move |left: &i32, right: &i32| {
        captured.store(left + right, Ordering::SeqCst);
    });
    consumer.accept(&5, &3);
    consumer.accept(&2, &4);
    assert_eq!(calls.load(Ordering::SeqCst), 6);
}
