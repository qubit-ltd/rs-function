// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::sync::Arc;
use std::sync::Mutex;

use qubit_function::BoxConditionalConsumerOnce;
use qubit_function::BoxConsumerOnce;
use qubit_function::ConsumerOnce;

#[test]
fn test_box_conditional_consumer_once_selects_branch() {
    let output = Arc::new(Mutex::new(Vec::new()));
    let apply = |value: i32| {
        let then_output = Arc::clone(&output);
        let else_output = Arc::clone(&output);
        let consumer = BoxConsumerOnce::new(move |value: &i32| {
            then_output.lock().expect("mutex should not be poisoned").push(*value);
        });
        let conditional: BoxConditionalConsumerOnce<i32> = consumer.when(|value: &i32| *value > 0);
        conditional
            .or_else(move |value: &i32| {
                else_output.lock().expect("mutex should not be poisoned").push(-value);
            })
            .accept(&value);
    };
    apply(5);
    apply(-5);
    assert_eq!(*output.lock().expect("mutex should not be poisoned"), vec![5, 5]);
}
