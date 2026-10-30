// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::sync::Arc;
use std::sync::Mutex;

use qubit_function::BiConsumerOnce;
use qubit_function::BoxBiConsumerOnce;

#[test]
fn test_box_bi_consumer_once_accepts_inputs_once() {
    let output = Arc::new(Mutex::new(Vec::new()));
    let captured = Arc::clone(&output);
    let consumer = BoxBiConsumerOnce::new(move |left: &i32, right: &i32| {
        captured
            .lock()
            .expect("mutex should not be poisoned")
            .push(left + right);
    });
    consumer.accept(&5, &3);
    assert_eq!(*output.lock().expect("mutex should not be poisoned"), vec![8]);
}
