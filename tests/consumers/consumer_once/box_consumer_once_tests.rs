// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::sync::Arc;
use std::sync::Mutex;

use qubit_function::BoxConsumerOnce;
use qubit_function::ConsumerOnce;

#[test]
fn test_box_consumer_once_observable_behavior() {
    let consumed_value = Arc::new(Mutex::new(None));
    let consumer = BoxConsumerOnce::new({
        let consumed_value = Arc::clone(&consumed_value);
        move |value: &i32| {
            *consumed_value.lock().expect("mutex should not be poisoned") = Some(value + 1);
        }
    });

    consumer.accept(&41);

    assert_eq!(
        *consumed_value.lock().expect("mutex should not be poisoned"),
        Some(42),
        "the consumer should invoke its callback with the accepted value"
    );
}
