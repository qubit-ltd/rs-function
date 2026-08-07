// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

/// Tests for BiConsumer types
use qubit_function::ArcBiConsumer;
/// Tests for BiConsumer types
use qubit_function::BiConsumer;
/// Tests for BiConsumer types
use qubit_function::BoxBiConsumer;
/// Tests for BiConsumer types
use qubit_function::RcBiConsumer;

#[cfg(test)]
mod box_non_mutating_bi_consumer_tests {
    use super::Arc;
    use super::BiConsumer;
    use super::BoxBiConsumer;

    #[test]
    fn test_new_and_accept() {
        let consumer = BoxBiConsumer::new(|x: &i32, y: &i32| {
            std::hint::black_box(x + y);
        });
        consumer.accept(&5, &3);
    }

    #[test]
    fn test_and_then() {
        let counter = Arc::new(std::sync::Mutex::new(0));
        let c1 = counter.clone();
        let c2 = counter.clone();
        let chained = BoxBiConsumer::new(move |_x: &i32, _y: &i32| {
            *c1.lock().expect("mutex should not be poisoned") += 1;
        })
        .and_then(move |_x: &i32, _y: &i32| {
            *c2.lock().expect("mutex should not be poisoned") += 1;
        });

        chained.accept(&5, &3);
        assert_eq!(*counter.lock().expect("mutex should not be poisoned"), 2);
    }

    #[test]
    fn test_noop() {
        let noop = BoxBiConsumer::<i32, i32>::noop();
        noop.accept(&42, &10);
        // Should not panic
    }

    #[test]
    fn test_name() {
        let mut consumer = BoxBiConsumer::<i32, i32>::noop();
        assert_eq!(consumer.name(), None);

        consumer.set_name("test_consumer");
        assert_eq!(consumer.name(), Some("test_consumer"));
    }

    #[test]
    fn test_debug() {
        let consumer = BoxBiConsumer::new(|_x: &i32, _y: &i32| {});
        let debug_str = format!("{:?}", consumer);
        assert!(debug_str.contains("BoxBiConsumer"));
    }

    #[test]
    fn test_display() {
        let consumer = BoxBiConsumer::new(|_x: &i32, _y: &i32| {});
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "BoxBiConsumer");
    }

    #[test]
    fn test_display_with_name() {
        let mut consumer = BoxBiConsumer::new(|_x: &i32, _y: &i32| {});
        consumer.set_name("my_consumer");
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "BoxBiConsumer(my_consumer)");
    }
}
