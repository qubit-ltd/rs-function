// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::cell::Cell;
use std::rc::Rc;

use qubit_function::BoxStatefulConsumer;
use qubit_function::StatefulConsumer;

#[test]
fn test_box_stateful_consumer_observable_behavior() {
    let consumed_value = Rc::new(Cell::new(None));
    let callback_value = Rc::clone(&consumed_value);
    let mut consumer = BoxStatefulConsumer::new(move |value: &i32| {
        callback_value.set(Some(*value));
    });

    consumer.accept(&42);
    drop(consumer);

    assert_eq!(consumed_value.get(), Some(42));
}
