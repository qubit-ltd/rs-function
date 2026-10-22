// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxStatefulConsumer;
use qubit_function::StatefulConsumer;

#[test]
fn test_box_stateful_consumer_observable_behavior() {
    let mut consumed_value = None;
    let mut consumer = BoxStatefulConsumer::new(|value: &i32| {
        consumed_value = Some(*value);
    });

    consumer.accept(&42);
    drop(consumer);

    assert_eq!(consumed_value, Some(42));
}
