// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Unit tests for StatefulConsumer types

use qubit_function::{
    ArcConsumer,
    ArcStatefulConsumer,
    BoxConsumer,
    BoxStatefulConsumer,
    Consumer,
    RcConsumer,
    RcStatefulConsumer,
    StatefulConsumer,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{
    Arc,
    Mutex,
};

// ============================================================================
// BoxConsumer Tests
// ============================================================================

#[cfg(test)]
mod test_conversions {
    use super::{
        Rc,
        RcStatefulConsumer,
        RefCell,
        StatefulConsumer,
    };

    // RcConsumer cannot be converted to ArcConsumer because Rc is not Send

    // ============================================================================
    // RcConsumer ConsumerOnce Tests
    // ============================================================================

    #[test]
    fn test_accept_once() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l = log.clone();
        let mut consumer = RcStatefulConsumer::new(move |x: &i32| {
            l.borrow_mut().push(*x);
        });
        consumer.accept(&5);
        assert_eq!(*log.borrow(), vec![5]);
    }

    #[test]
    fn test_accept_once_with_different_types() {
        // String
        let log = Rc::new(RefCell::new(String::new()));
        let l = log.clone();
        let mut consumer = RcStatefulConsumer::new(move |s: &String| {
            *l.borrow_mut() = format!("Got: {}", s);
        });
        let text = String::from("hello");
        consumer.accept(&text);
        assert_eq!(*log.borrow(), "Got: hello");

        // Vec
        let log = Rc::new(RefCell::new(0));
        let l = log.clone();
        let mut consumer = RcStatefulConsumer::new(move |v: &Vec<i32>| {
            *l.borrow_mut() = v.len();
        });
        let numbers = vec![1, 2, 3];
        consumer.accept(&numbers);
        assert_eq!(*log.borrow(), 3);

        // bool
        let log = Rc::new(RefCell::new(String::new()));
        let l = log.clone();
        let mut consumer = RcStatefulConsumer::new(move |b: &bool| {
            *l.borrow_mut() = if *b { "true" } else { "false" }.to_string();
        });
        let flag = true;
        consumer.accept(&flag);
        assert_eq!(*log.borrow(), "true");
    }

    #[test]
    fn test_consumer_once_with_state_modification() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l = log.clone();
        let mut counter = 0;
        let mut consumer = RcStatefulConsumer::new(move |x: &i32| {
            counter += 1;
            l.borrow_mut().push(*x + counter);
        });
        consumer.accept(&10);
        assert_eq!(*log.borrow(), vec![11]); // 10 + 1
    }

    #[test]
    fn test_consumer_once_consumes_self() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l = log.clone();
        let mut consumer = RcStatefulConsumer::new(move |x: &i32| {
            l.borrow_mut().push(*x);
        });

        consumer.accept(&5);
        assert_eq!(*log.borrow(), vec![5]);
    }
}

// ============================================================================
// Unified Interface Tests
// ============================================================================
