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

use qubit_function::ArcBiConsumer;
use qubit_function::BiConsumer;

#[test]
fn test_arc_conditional_bi_consumer_observable_behavior() {
    let total = Arc::new(AtomicUsize::new(0));
    let captured_total = Arc::clone(&total);
    let consumer = ArcBiConsumer::new(move |first: &usize, second: &usize| {
        captured_total.fetch_add(first + second, Ordering::SeqCst);
    });
    let conditional_consumer = consumer.when(|first: &usize, second: &usize| first + second > 10);

    conditional_consumer.accept(&2, &3);
    conditional_consumer.accept(&5, &7);

    assert_eq!(total.load(Ordering::SeqCst), 12);
}
