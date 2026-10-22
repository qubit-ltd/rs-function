// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::sync::Arc;
use std::sync::Mutex;

use qubit_function::ArcConditionalStatefulBiConsumer;
use qubit_function::ArcStatefulBiConsumer;
use qubit_function::StatefulBiConsumer;

#[test]
fn test_arc_conditional_stateful_bi_consumer_accepts_when_predicate_is_true() {
    let log = Arc::new(Mutex::new(Vec::new()));
    let captured_log = Arc::clone(&log);
    let consumer = ArcStatefulBiConsumer::new(move |first: &i32, second: &i32| {
        captured_log
            .lock()
            .expect("mutex should not be poisoned")
            .push(*first + *second);
    });
    let mut conditional = consumer.when(|first: &i32, _second: &i32| *first > 0);

    conditional.accept(&5, &3);

    assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8]);
}

#[test]
fn test_arc_conditional_stateful_bi_consumer_skips_when_predicate_is_false() {
    let log = Arc::new(Mutex::new(Vec::new()));
    let captured_log = Arc::clone(&log);
    let consumer = ArcStatefulBiConsumer::new(move |first: &i32, second: &i32| {
        captured_log
            .lock()
            .expect("mutex should not be poisoned")
            .push(*first + *second);
    });
    let mut conditional = consumer.when(|first: &i32, _second: &i32| *first > 0);

    conditional.accept(&-5, &3);

    assert!(log.lock().expect("mutex should not be poisoned").is_empty());
}

#[test]
fn test_arc_conditional_stateful_bi_consumer_clones_share_consumer_state() {
    let log = Arc::new(Mutex::new(Vec::new()));
    let captured_log = Arc::clone(&log);
    let consumer = ArcStatefulBiConsumer::new(move |first: &i32, second: &i32| {
        captured_log
            .lock()
            .expect("mutex should not be poisoned")
            .push(*first + *second);
    });
    let conditional = consumer.when(|first: &i32, _second: &i32| *first > 0);
    let mut first_clone = conditional.clone();
    let mut second_clone = conditional.clone();

    first_clone.accept(&5, &3);
    second_clone.accept(&10, &2);

    assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8, 12]);
}

#[test]
fn test_arc_conditional_stateful_bi_consumer_or_else_handles_false_predicate() {
    let log = Arc::new(Mutex::new(Vec::new()));
    let consumer_log = Arc::clone(&log);
    let fallback_log = Arc::clone(&log);
    let consumer = ArcStatefulBiConsumer::new(move |first: &i32, second: &i32| {
        consumer_log
            .lock()
            .expect("mutex should not be poisoned")
            .push(*first + *second);
    });
    let conditional = consumer.when(|first: &i32, _second: &i32| *first > 0);
    let mut with_fallback = conditional.or_else(move |first: &i32, second: &i32| {
        fallback_log
            .lock()
            .expect("mutex should not be poisoned")
            .push(*first * *second);
    });

    with_fallback.accept(&5, &3);
    with_fallback.accept(&-5, &3);

    assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8, -15]);
}

#[test]
fn test_arc_conditional_stateful_bi_consumer_formats_debug_and_display() {
    let consumer = ArcStatefulBiConsumer::new(|_first: &i32, _second: &i32| {});
    let conditional = consumer.when(|first: &i32, _second: &i32| *first > 0);
    let debug_output = format!("{conditional:?}");
    let display_output = format!("{conditional}");

    assert!(debug_output.contains("ArcConditionalStatefulBiConsumer"));
    assert!(debug_output.contains("consumer"));
    assert!(debug_output.contains("predicate"));
    assert!(display_output.contains("ArcConditionalStatefulBiConsumer"));
}

#[test]
fn test_arc_conditional_stateful_bi_consumer_observable_behavior() {
    let type_name = std::any::type_name::<ArcConditionalStatefulBiConsumer<i32, i32>>();
    assert!(type_name.contains("ArcConditionalStatefulBiConsumer"), "{type_name}");
}
