// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for the `ConsumerOnce` trait and its implementations.

use std::sync::Arc;
use std::sync::Mutex;

use qubit_function::BoxConsumerOnce;
use qubit_function::ConsumerOnce;

// ============================================================================
// BoxConsumerOnce Tests
// ============================================================================

#[cfg(test)]
mod box_consumer_once_tests {
    use super::Arc;
    use super::BoxConsumerOnce;
    use super::ConsumerOnce;
    use super::Mutex;

    #[test]
    fn test_box_consumer_once_new_invokes_callback() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let consumer = BoxConsumerOnce::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });
        consumer.accept(&5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);
    }

    #[test]
    fn test_box_consumer_once_accept_passes_input_to_callback() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let consumer = BoxConsumerOnce::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x * 2);
        });
        consumer.accept(&5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![10]);
    }

    #[test]
    fn test_box_consumer_once_and_then_invokes_callbacks_in_order() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();
        let chained = BoxConsumerOnce::new(move |x: &i32| {
            l1.lock().expect("mutex should not be poisoned").push(*x * 2);
        })
        .and_then(move |x: &i32| {
            l2.lock().expect("mutex should not be poisoned").push(*x + 10);
        });
        chained.accept(&5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![10, 15]);
    }

    #[test]
    fn test_box_consumer_once_and_then_multiple_invokes_callbacks_in_order() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();
        let l3 = log.clone();
        let chained = BoxConsumerOnce::new(move |x: &i32| {
            l1.lock().expect("mutex should not be poisoned").push(*x * 2);
        })
        .and_then(move |x: &i32| {
            l2.lock().expect("mutex should not be poisoned").push(*x + 10);
        })
        .and_then(move |x: &i32| {
            l3.lock().expect("mutex should not be poisoned").push(*x - 1);
        });
        chained.accept(&5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![10, 15, 4]);
    }

    #[test]
    fn test_box_consumer_once_noop_accepts_input_without_panicking() {
        let noop = BoxConsumerOnce::<i32>::noop();
        noop.accept(&42);
        // Should not panic
    }

    #[test]
    fn test_box_consumer_once_new_with_name_exposes_name_and_invokes_callback() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let consumer = BoxConsumerOnce::new_with_name("test_consumer", move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });
        assert_eq!(consumer.name(), Some("test_consumer"));
        consumer.accept(&5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);
    }

    // print and print_with methods have been removed

    #[test]
    fn test_box_consumer_once_when_invokes_callback_when_condition_is_true() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let consumer = BoxConsumerOnce::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x + 1);
        });
        let conditional = consumer.when(|x: &i32| *x > 0);
        conditional.accept(&5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![6]);
    }

    #[test]
    fn test_box_consumer_once_when_skips_callback_when_condition_is_false() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let consumer = BoxConsumerOnce::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x + 1);
        });
        let conditional = consumer.when(|x: &i32| *x > 0);
        conditional.accept(&-5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), Vec::<i32>::new());
    }

    #[test]
    fn test_box_consumer_once_when_or_else_invokes_first_callback_when_true() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();
        let consumer = BoxConsumerOnce::new(move |x: &i32| {
            l1.lock().expect("mutex should not be poisoned").push(*x + 1);
        });
        let conditional = consumer.when(|x: &i32| *x > 0).or_else(move |x: &i32| {
            l2.lock().expect("mutex should not be poisoned").push(*x - 1);
        });
        conditional.accept(&5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![6]);
    }

    #[test]
    fn test_box_consumer_once_when_or_else_invokes_fallback_when_false() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();
        let consumer = BoxConsumerOnce::new(move |x: &i32| {
            l1.lock().expect("mutex should not be poisoned").push(*x + 1);
        });
        let conditional = consumer.when(|x: &i32| *x > 0).or_else(move |x: &i32| {
            l2.lock().expect("mutex should not be poisoned").push(*x - 1);
        });
        conditional.accept(&-5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![-6]);
    }
}

// ============================================================================
// Closure Tests
// ============================================================================
