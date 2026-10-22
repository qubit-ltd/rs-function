// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::sync::Arc;
use std::sync::Mutex;

use qubit_function::BoxConsumer;
use qubit_function::Consumer;

#[test]
fn test_box_conditional_consumer_observable_behavior() {
    let accepted_values = Arc::new(Mutex::new(Vec::new()));
    let accepted_values_handle = Arc::clone(&accepted_values);
    let consumer = BoxConsumer::new(move |value: &i32| {
        accepted_values_handle
            .lock()
            .expect("accepted values mutex should not be poisoned")
            .push(*value);
    });
    let conditional_consumer = consumer.when(|value: &i32| *value > 0);

    conditional_consumer.accept(&1);
    conditional_consumer.accept(&-1);

    assert_eq!(
        *accepted_values
            .lock()
            .expect("accepted values mutex should not be poisoned"),
        vec![1]
    );
}
