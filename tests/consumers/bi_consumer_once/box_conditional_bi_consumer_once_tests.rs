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
use qubit_function::BoxConditionalBiConsumerOnce;

#[test]
fn test_box_conditional_bi_consumer_once_selects_branch() {
    let output = Arc::new(Mutex::new(Vec::new()));
    let apply = |left: i32, right: i32| {
        let then_output = Arc::clone(&output);
        let else_output = Arc::clone(&output);
        let consumer = BoxBiConsumerOnce::new(move |left: &i32, right: &i32| {
            then_output
                .lock()
                .expect("mutex should not be poisoned")
                .push(left + right);
        });
        let conditional: BoxConditionalBiConsumerOnce<i32, i32> = consumer.when(|left: &i32, _right: &i32| *left > 0);
        conditional
            .or_else(move |left: &i32, right: &i32| {
                else_output
                    .lock()
                    .expect("mutex should not be poisoned")
                    .push(left * right);
            })
            .accept(&left, &right);
    };
    apply(5, 3);
    apply(-5, 3);
    assert_eq!(*output.lock().expect("mutex should not be poisoned"), vec![8, -15]);
}
