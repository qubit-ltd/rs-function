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
fn test_shared_consumer_methods_observable_behavior() {
    let first_calls = Arc::new(AtomicI32::new(0));
    let first_callback_calls = Arc::clone(&first_calls);
    let first = ArcConsumer::new(move |_value: &i32| {
        first_callback_calls.fetch_add(1, Ordering::SeqCst);
    });
    let after_calls = Arc::new(AtomicI32::new(0));
    let after_callback_calls = Arc::clone(&after_calls);
    let after = ArcConsumer::new(move |_value: &i32| {
        after_callback_calls.fetch_add(1, Ordering::SeqCst);
    });

    let chained = first.and_then(after);
    chained.accept(&42);

    assert_eq!(first_calls.load(Ordering::SeqCst), 1);
    assert_eq!(after_calls.load(Ordering::SeqCst), 1);
}
