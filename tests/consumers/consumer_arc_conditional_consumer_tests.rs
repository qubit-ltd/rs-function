// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Tests for Consumer types

use qubit_function::{
    ArcConsumer,
    BoxConsumer,
    Consumer,
    RcConsumer,
};
use std::rc::Rc;
use std::sync::Arc;

#[cfg(test)]
mod arc_conditional_consumer_tests {
    use super::{
        Arc,
        ArcConsumer,
        Consumer,
    };
    use std::sync::atomic::{
        AtomicUsize,
        Ordering,
    };

    #[test]
    fn test_arc_conditional_and_then() {
        let counter = Arc::new(AtomicUsize::new(0));
        let c1 = counter.clone();
        let c2 = counter.clone();

        let consumer = ArcConsumer::new(move |_x: &i32| {
            c1.fetch_add(1, Ordering::SeqCst);
        });

        let conditional = consumer.when(|x: &i32| *x > 0);
        let chained = conditional.and_then(move |_x: &i32| {
            c2.fetch_add(10, Ordering::SeqCst);
        });

        chained.accept(&5);
        assert_eq!(counter.load(Ordering::SeqCst), 11);

        chained.accept(&-5);
        assert_eq!(counter.load(Ordering::SeqCst), 21);
    }

    #[test]
    fn test_arc_conditional_or_else() {
        let counter = Arc::new(AtomicUsize::new(0));
        let c1 = counter.clone();
        let c2 = counter.clone();

        let consumer = ArcConsumer::new(move |_x: &i32| {
            c1.fetch_add(1, Ordering::SeqCst);
        });

        let conditional =
            consumer.when(|x: &i32| *x > 0).or_else(move |_x: &i32| {
                c2.fetch_add(100, Ordering::SeqCst);
            });

        conditional.accept(&5);
        assert_eq!(counter.load(Ordering::SeqCst), 1);

        conditional.accept(&-5);
        assert_eq!(counter.load(Ordering::SeqCst), 101);
    }

    #[test]
    fn test_arc_conditional_accept() {
        let counter = Arc::new(AtomicUsize::new(0));
        let c = counter.clone();

        let consumer = ArcConsumer::new(move |_x: &i32| {
            c.fetch_add(1, Ordering::SeqCst);
        });

        let conditional = consumer.when(|x: &i32| *x > 0);

        conditional.accept(&5);
        assert_eq!(counter.load(Ordering::SeqCst), 1);

        conditional.accept(&-5);
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }
}
