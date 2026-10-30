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
fn test_arc_conditional_consumer_clone_shares_callback() {
    let total = Arc::new(AtomicI32::new(0));
    let callback_total = Arc::clone(&total);
    let original = ArcConsumer::new({
        let calls = AtomicI32::new(0);
        move |value: &i32| {
            let call = calls.fetch_add(1, Ordering::SeqCst) + 1;
            callback_total.fetch_add(*value * call, Ordering::SeqCst);
        }
    })
    .when(|value: &i32| *value > 0);
    let cloned = original.clone();

    original.accept(&2);
    cloned.accept(&-10);
    cloned.accept(&3);
    assert_eq!(total.load(Ordering::SeqCst), 8);
}
