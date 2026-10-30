// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::sync::Arc;
use std::sync::Mutex;

use qubit_function::BoxConditionalStatefulBiConsumer;
use qubit_function::BoxStatefulBiConsumer;
use qubit_function::StatefulBiConsumer;

#[test]
fn test_box_conditional_stateful_bi_consumer_selects_branch() {
    let output = Arc::new(Mutex::new(Vec::new()));
    let then_output = Arc::clone(&output);
    let else_output = Arc::clone(&output);
    let consumer = BoxStatefulBiConsumer::new(move |left: &i32, right: &i32| {
        then_output
            .lock()
            .expect("mutex should not be poisoned")
            .push(left + right);
    });
    let conditional: BoxConditionalStatefulBiConsumer<i32, i32> = consumer.when(|left: &i32, _right: &i32| *left > 0);
    let mut conditional = conditional.or_else(move |left: &i32, right: &i32| {
        else_output
            .lock()
            .expect("mutex should not be poisoned")
            .push(left * right);
    });
    conditional.accept(&5, &3);
    conditional.accept(&-5, &3);
    assert_eq!(*output.lock().expect("mutex should not be poisoned"), vec![8, -15]);
}
