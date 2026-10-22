// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use qubit_function::ArcConsumer;
use qubit_function::Consumer;

#[test]
fn test_arc_consumer_observable_behavior() {
    let accepted_values = Arc::new(AtomicUsize::new(0));
    let accepted_values_handle = Arc::clone(&accepted_values);
    let consumer = ArcConsumer::new(move |_: &i32| {
        accepted_values_handle.fetch_add(1, Ordering::SeqCst);
    });
    let cloned_consumer = consumer.clone();

    consumer.accept(&1);
    cloned_consumer.accept(&2);

    assert_eq!(accepted_values.load(Ordering::SeqCst), 2);
}
