// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

use qubit_function::BiConsumer;
use qubit_function::BoxBiConsumer;
use qubit_function::BoxConditionalBiConsumer;

#[test]
fn test_box_conditional_bi_consumer_selects_branch() {
    let output = Arc::new(Mutex::new(Vec::new()));
    let then_output = Arc::clone(&output);
    let else_output = Arc::clone(&output);
    let consumer = BoxBiConsumer::new(move |left: &i32, right: &i32| {
        then_output
            .lock()
            .expect("mutex should not be poisoned")
            .push(left + right);
    });
    let conditional: BoxConditionalBiConsumer<i32, i32> = consumer.when(|left: &i32, _right: &i32| *left > 0);
    let conditional = conditional.or_else(move |left: &i32, right: &i32| {
        else_output
            .lock()
            .expect("mutex should not be poisoned")
            .push(left * right);
    });
    conditional.accept(&5, &3);
    conditional.accept(&-5, &3);
    assert_eq!(*output.lock().expect("mutex should not be poisoned"), vec![8, -15]);
}

#[test]
fn test_box_conditional_bi_consumer_rejects_inputs() {
    let total = Rc::new(Cell::new(0));
    let callback_total = Rc::clone(&total);
    let consumer = BoxBiConsumer::new(move |first: &i32, second: &i32| {
        callback_total.set(first + second);
    });
    let conditional = consumer.when(|first: &i32, second: &i32| *first > 0 && *second > 0);

    conditional.accept(&5, &3);
    assert_eq!(total.get(), 8);

    conditional.accept(&-5, &3);
    assert_eq!(total.get(), 8, "consumer must skip inputs rejected by predicate");
}
