// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Tests for Consumer types

use std::rc::Rc;
use std::sync::Arc;

use qubit_function::ArcConsumer;
use qubit_function::BoxConsumer;
use qubit_function::Consumer;
use qubit_function::RcConsumer;

#[cfg(test)]
mod box_non_mutating_consumer_tests {
    use super::Arc;
    use super::ArcConsumer;
    use super::BoxConsumer;
    use super::Consumer;

    #[test]
    fn test_new_and_accept() {
        let consumer = BoxConsumer::new(|x: &i32| {
            std::hint::black_box(x);
        });
        consumer.accept(&5);
    }

    #[test]
    fn test_and_then() {
        let counter = Arc::new(std::sync::Mutex::new(0));
        let c1 = counter.clone();
        let c2 = counter.clone();
        let chained = BoxConsumer::new(move |_x: &i32| {
            *c1.lock().expect("mutex should not be poisoned") += 1;
        })
        .and_then(move |_x: &i32| {
            *c2.lock().expect("mutex should not be poisoned") += 1;
        });

        chained.accept(&5);
        assert_eq!(*counter.lock().expect("mutex should not be poisoned"), 2);
    }

    #[test]
    fn test_and_then_with_box_consumer() {
        let counter = Arc::new(std::sync::Mutex::new(0));
        let c1 = counter.clone();
        let c2 = counter.clone();

        let first = BoxConsumer::new(move |_x: &i32| {
            *c1.lock().expect("mutex should not be poisoned") += 1;
        });

        let second = BoxConsumer::new(move |_x: &i32| {
            *c2.lock().expect("mutex should not be poisoned") += 1;
        });

        let chained = first.and_then(second);
        chained.accept(&5);
        assert_eq!(*counter.lock().expect("mutex should not be poisoned"), 2);
    }

    #[test]
    fn test_and_then_multiple_chains() {
        let counter = Arc::new(std::sync::Mutex::new(0));
        let c1 = counter.clone();
        let c2 = counter.clone();
        let c3 = counter.clone();

        let chained = BoxConsumer::new(move |_x: &i32| {
            *c1.lock().expect("mutex should not be poisoned") += 1;
        })
        .and_then(move |_x: &i32| {
            *c2.lock().expect("mutex should not be poisoned") += 1;
        })
        .and_then(move |_x: &i32| {
            *c3.lock().expect("mutex should not be poisoned") += 1;
        });

        chained.accept(&5);
        assert_eq!(*counter.lock().expect("mutex should not be poisoned"), 3);
    }

    #[test]
    fn test_noop() {
        let noop = BoxConsumer::<i32>::noop();
        noop.accept(&42);
        // Should not panic
    }

    #[test]
    fn test_arc_noop() {
        let noop = ArcConsumer::<i32>::noop();
        noop.accept(&42);
        // Should not panic
    }

    #[test]
    fn test_name() {
        let mut consumer = BoxConsumer::<i32>::noop();
        assert_eq!(consumer.name(), None);

        consumer.set_name("test_consumer");
        assert_eq!(consumer.name(), Some("test_consumer"));
    }

    #[test]
    fn test_debug() {
        let consumer = BoxConsumer::<i32>::noop();
        let debug_str = format!("{:?}", consumer);
        assert!(debug_str.contains("BoxConsumer"));
    }

    #[test]
    fn test_display() {
        let mut consumer = BoxConsumer::<i32>::noop();
        assert_eq!(format!("{}", consumer), "BoxConsumer");

        consumer.set_name("my_consumer");
        assert_eq!(format!("{}", consumer), "BoxConsumer(my_consumer)");
    }

    #[test]
    fn test_with_different_types() {
        let string_consumer = BoxConsumer::new(|s: &String| {
            std::hint::black_box(s);
        });
        string_consumer.accept(&"Hello".to_string());

        let vec_consumer = BoxConsumer::new(|v: &Vec<i32>| {
            std::hint::black_box(v.len());
        });
        vec_consumer.accept(&vec![1, 2, 3]);
    }
}
