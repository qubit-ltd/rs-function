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
mod arc_non_mutating_consumer_tests {
    use super::Arc;
    use super::ArcConsumer;
    use super::Consumer;

    #[test]
    fn test_new_and_accept() {
        let consumer = ArcConsumer::new(|x: &i32| {
            std::hint::black_box(x);
        });
        consumer.accept(&5);
    }

    #[test]
    fn test_clone() {
        let counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let c = counter.clone();
        let consumer = ArcConsumer::new(move |_x: &i32| {
            c.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        });

        let clone = consumer.clone();
        consumer.accept(&5);
        clone.accept(&10);

        assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 2);
    }

    #[test]
    fn test_and_then() {
        let counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let c1 = counter.clone();
        let c2 = counter.clone();

        let first = ArcConsumer::new(move |_x: &i32| {
            c1.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        });

        let second = ArcConsumer::new(move |_x: &i32| {
            c2.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        });

        let chained = first.and_then(second.clone());
        chained.accept(&5);

        assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 2);

        // Original consumers remain usable
        first.accept(&10);
        second.accept(&15);
        assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 4);
    }

    #[test]
    fn test_name() {
        let mut consumer = ArcConsumer::new(|_x: &i32| {});
        assert_eq!(consumer.name(), None);

        consumer.set_name("test_consumer");
        assert_eq!(consumer.name(), Some("test_consumer"));
    }

    #[test]
    fn test_debug() {
        let consumer = ArcConsumer::new(|_x: &i32| {});
        let debug_str = format!("{:?}", consumer);
        assert!(debug_str.contains("ArcConsumer"));
    }

    #[test]
    fn test_display() {
        let mut consumer = ArcConsumer::new(|_x: &i32| {});
        assert_eq!(format!("{}", consumer), "ArcConsumer");

        consumer.set_name("my_consumer");
        assert_eq!(format!("{}", consumer), "ArcConsumer(my_consumer)");
    }

    #[test]
    fn test_thread_safety() {
        let counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let c = counter.clone();
        let consumer = ArcConsumer::new(move |_x: &i32| {
            c.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        });

        let handles: Vec<_> = (0..10)
            .map(|i| {
                let consumer_clone = consumer.clone();
                std::thread::spawn(move || {
                    consumer_clone.accept(&i);
                })
            })
            .collect();

        for handle in handles {
            handle.join().expect("thread should not panic");
        }

        assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 10);
    }
}
