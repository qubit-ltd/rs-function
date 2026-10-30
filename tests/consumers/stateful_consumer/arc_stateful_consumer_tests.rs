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

use qubit_function::ArcStatefulConsumer;
use qubit_function::StatefulConsumer;

#[test]
fn test_arc_stateful_consumer_retains_callback_state() {
    let calls = Arc::new(AtomicI32::new(0));
    let captured = Arc::clone(&calls);
    let mut consumer = ArcStatefulConsumer::new(move |_value: &i32| {
        let next = captured.load(Ordering::SeqCst) + 1;
        captured.store(next, Ordering::SeqCst);
    });
    consumer.accept(&5);
    consumer.accept(&3);
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}
