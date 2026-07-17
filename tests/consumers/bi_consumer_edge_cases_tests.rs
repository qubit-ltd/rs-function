// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
/// Tests for BiConsumer types
use qubit_function::{
    ArcBiConsumer,
    BiConsumer,
    BoxBiConsumer,
    RcBiConsumer,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

#[cfg(test)]
mod edge_cases_tests {
    use super::{
        Arc,
        ArcBiConsumer,
        BiConsumer,
        BoxBiConsumer,
        Rc,
        RcBiConsumer,
        RefCell,
    };

    #[test]
    fn test_noop_multiple_calls() {
        let consumer = BoxBiConsumer::<i32, i32>::noop();
        consumer.accept(&5, &3);
        consumer.accept(&10, &20);
        consumer.accept(&1, &2);
        // Should do nothing
    }

    #[test]
    fn test_and_then_with_noop() {
        let counter = Arc::new(std::sync::Mutex::new(0));
        let c = counter.clone();
        let consumer =
            qubit_function::BoxBiConsumer::new(move |_x: &i32, _y: &i32| {
                *c.lock().expect("mutex should not be poisoned") += 1;
            })
            .and_then(BoxBiConsumer::noop());
        consumer.accept(&5, &3);
        assert_eq!(*counter.lock().expect("mutex should not be poisoned"), 1);
    }

    #[test]
    fn test_complex_chain() {
        let counter = Arc::new(std::sync::Mutex::new(0));
        let c1 = counter.clone();
        let c2 = counter.clone();
        let c3 = counter.clone();
        let consumer =
            qubit_function::BoxBiConsumer::new(move |_x: &i32, _y: &i32| {
                *c1.lock().expect("mutex should not be poisoned") += 1;
            })
            .and_then(move |_x: &i32, _y: &i32| {
                *c2.lock().expect("mutex should not be poisoned") += 1;
            })
            .and_then(BoxBiConsumer::noop())
            .and_then(move |_x: &i32, _y: &i32| {
                *c3.lock().expect("mutex should not be poisoned") += 1;
            });
        consumer.accept(&5, &3);
        assert_eq!(*counter.lock().expect("mutex should not be poisoned"), 3);
    }

    #[test]
    fn test_with_different_types() {
        let counter = Arc::new(std::sync::Mutex::new(String::new()));
        let c = counter.clone();
        let consumer =
            qubit_function::BoxBiConsumer::new(move |s: &String, n: &i32| {
                *c.lock().expect("mutex should not be poisoned") =
                    format!("{}: {}", s, n);
            });
        consumer.accept(&"Count".to_string(), &42);
        assert_eq!(
            *counter.lock().expect("mutex should not be poisoned"),
            "Count: 42"
        );
    }

    #[test]
    fn test_arc_consumer_multiple_threads() {
        let counter = Arc::new(std::sync::Mutex::new(0));
        let c = counter.clone();
        let consumer = ArcBiConsumer::new(move |x: &i32, y: &i32| {
            *c.lock().expect("mutex should not be poisoned") += x + y;
        });

        let handles: Vec<_> = (0..10)
            .map(|i| {
                let cons = consumer.clone();
                std::thread::spawn(move || {
                    cons.accept(&i, &1);
                })
            })
            .collect();

        for handle in handles {
            handle.join().expect("thread should not panic");
        }

        // Sum of (0+1) + (1+1) + ... + (9+1) = 55
        assert_eq!(*counter.lock().expect("mutex should not be poisoned"), 55);
    }

    #[test]
    fn test_rc_consumer_multiple_clones() {
        let counter = Rc::new(RefCell::new(0));
        let c = counter.clone();
        let consumer = RcBiConsumer::new(move |x: &i32, y: &i32| {
            *c.borrow_mut() += x + y;
        });

        let cons1 = consumer.clone();
        let cons2 = consumer.clone();
        let cons3 = consumer.clone();

        cons1.accept(&1, &2);
        cons2.accept(&3, &4);
        cons3.accept(&5, &6);

        assert_eq!(*counter.borrow(), 21); // 3 + 7 + 11
    }

    #[test]
    fn test_name_with_and_then() {
        let mut consumer1 =
            qubit_function::BoxBiConsumer::new(|_x: &i32, _y: &i32| {});
        consumer1.set_name("first");
        let consumer2 =
            qubit_function::BoxBiConsumer::new(|_x: &i32, _y: &i32| {});
        let chained = consumer1.and_then(consumer2);
        // Name is not preserved through and_then
        assert_eq!(chained.name(), None);
    }
}

// ============================================================================
// Conversion Tests
// ============================================================================

// ============================================================================
// Name Tests - Testing name() and set_name() methods
// ============================================================================
