// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::sync::Arc;
use std::sync::Mutex;

use qubit_function::ArcConditionalStatefulConsumer;
use qubit_function::ArcStatefulConsumer;
use qubit_function::StatefulConsumer;

#[test]
fn test_arc_conditional_stateful_consumer_selects_branch() {
    let output = Arc::new(Mutex::new(Vec::new()));
    let then_output = Arc::clone(&output);
    let else_output = Arc::clone(&output);
    let consumer = ArcStatefulConsumer::new(move |value: &i32| {
        then_output.lock().expect("mutex should not be poisoned").push(*value);
    });
    let conditional: ArcConditionalStatefulConsumer<i32> = consumer.when(|value: &i32| *value > 0);
    let mut conditional = conditional.or_else(move |value: &i32| {
        else_output.lock().expect("mutex should not be poisoned").push(-value);
    });
    conditional.accept(&5);
    conditional.accept(&-5);
    assert_eq!(*output.lock().expect("mutex should not be poisoned"), vec![5, 5]);
}
