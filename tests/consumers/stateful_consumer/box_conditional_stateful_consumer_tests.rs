// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::sync::Arc;
use std::sync::Mutex;

use qubit_function::BoxStatefulConsumer;
use qubit_function::StatefulConsumer;

#[test]
fn test_box_conditional_stateful_consumer_only_accepts_matching_values() {
    let accepted_values = Arc::new(Mutex::new(Vec::new()));
    let values_for_consumer = Arc::clone(&accepted_values);
    let mut consumer = BoxStatefulConsumer::new(move |value: &i32| {
        values_for_consumer
            .lock()
            .expect("accepted values mutex should not be poisoned")
            .push(*value);
    })
    .when(|value: &i32| *value > 0);

    consumer.accept(&5);
    consumer.accept(&-5);

    assert_eq!(
        *accepted_values
            .lock()
            .expect("accepted values mutex should not be poisoned"),
        vec![5],
    );
}
