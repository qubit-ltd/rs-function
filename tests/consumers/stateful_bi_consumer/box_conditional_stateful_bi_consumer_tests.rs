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

#[cfg(test)]
mod box_conditional_bi_consumer_tests {
    use std::sync::Arc;
    use std::sync::Mutex;

    use qubit_function::BoxStatefulBiConsumer;
    use qubit_function::StatefulBiConsumer;

    #[test]
    fn test_accept_when_true() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let callback_log = Arc::clone(&log);
        let consumer = BoxStatefulBiConsumer::new(move |x: &i32, y: &i32| {
            callback_log.lock().expect("mutex should not be poisoned").push(*x + *y);
        });
        let mut conditional = consumer.when(|x: &i32, _y: &i32| *x > 0);

        conditional.accept(&5, &3);

        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8]);
    }

    #[test]
    fn test_accept_when_false() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let callback_log = Arc::clone(&log);
        let consumer = BoxStatefulBiConsumer::new(move |x: &i32, y: &i32| {
            callback_log.lock().expect("mutex should not be poisoned").push(*x + *y);
        });
        let mut conditional = consumer.when(|x: &i32, _y: &i32| *x > 0);

        conditional.accept(&-5, &3);

        assert_eq!(*log.lock().expect("mutex should not be poisoned"), Vec::<i32>::new());
    }

    #[test]
    fn test_and_then() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let first_log = Arc::clone(&log);
        let second_log = Arc::clone(&log);
        let consumer = BoxStatefulBiConsumer::new(move |x: &i32, y: &i32| {
            first_log.lock().expect("mutex should not be poisoned").push(*x + *y);
        });
        let conditional = consumer.when(|x: &i32, _y: &i32| *x > 0);
        let mut chained = conditional.and_then(move |x: &i32, y: &i32| {
            second_log.lock().expect("mutex should not be poisoned").push(*x * *y);
        });

        chained.accept(&5, &3);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8, 15]);

        chained.accept(&-5, &3);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8, 15, -15]);
    }

    #[test]
    fn test_or_else() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let first_log = Arc::clone(&log);
        let second_log = Arc::clone(&log);
        let consumer = BoxStatefulBiConsumer::new(move |x: &i32, y: &i32| {
            first_log.lock().expect("mutex should not be poisoned").push(*x + *y);
        });
        let mut conditional = consumer
            .when(|x: &i32, _y: &i32| *x > 0)
            .or_else(move |x: &i32, y: &i32| {
                second_log.lock().expect("mutex should not be poisoned").push(*x * *y);
            });

        conditional.accept(&5, &3);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8]);

        conditional.accept(&-5, &3);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8, -15]);
    }

    #[test]
    fn test_with_always_true_predicate() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let callback_log = Arc::clone(&log);
        let consumer = BoxStatefulBiConsumer::new(move |x: &i32, y: &i32| {
            callback_log.lock().expect("mutex should not be poisoned").push(*x + *y);
        });
        let mut conditional = consumer.when(|_: &i32, _: &i32| true);

        conditional.accept(&5, &3);
        conditional.accept(&-5, &3);

        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8, -2]);
    }

    #[test]
    fn test_with_always_false_predicate() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let callback_log = Arc::clone(&log);
        let consumer = BoxStatefulBiConsumer::new(move |x: &i32, y: &i32| {
            callback_log.lock().expect("mutex should not be poisoned").push(*x + *y);
        });
        let mut conditional = consumer.when(|_: &i32, _: &i32| false);

        conditional.accept(&5, &3);
        conditional.accept(&-5, &3);

        assert_eq!(*log.lock().expect("mutex should not be poisoned"), Vec::<i32>::new());
    }

    #[test]
    fn test_with_complex_predicate() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let callback_log = Arc::clone(&log);
        let consumer = BoxStatefulBiConsumer::new(move |x: &i32, y: &i32| {
            callback_log.lock().expect("mutex should not be poisoned").push(*x + *y);
        });
        let mut conditional = consumer.when(|x: &i32, y: &i32| *x > 0 && *y > 0 && *x + *y < 10);

        conditional.accept(&2, &3);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);

        conditional.accept(&5, &10);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);
    }

    #[test]
    fn test_debug() {
        let consumer = BoxStatefulBiConsumer::<i32, i32>::noop();
        let conditional = consumer.when(|x: &i32, _y: &i32| *x > 0);
        let debug_str = format!("{conditional:?}");

        assert!(debug_str.contains("BoxConditionalStatefulBiConsumer"));
        assert!(debug_str.contains("consumer"));
        assert!(debug_str.contains("predicate"));
    }

    #[test]
    fn test_display() {
        let consumer = BoxStatefulBiConsumer::<i32, i32>::noop();
        let conditional = consumer.when(|x: &i32, _y: &i32| *x > 0);
        let display_str = format!("{conditional}");

        assert!(display_str.contains("BoxConditionalStatefulBiConsumer"));
    }
}
