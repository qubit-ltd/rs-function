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
mod consumer_once_trait_tests {
    use super::{
        Arc,
        ArcConsumer,
        BoxConsumer,
        Consumer,
        Mutex,
        Rc,
        RcConsumer,
        RefCell,
    };

    #[test]
    fn test_box_consumer_accept_once() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let consumer = BoxConsumer::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });

        consumer.accept(&100);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![100]
        );
    }

    #[test]
    fn test_arc_consumer_accept_once() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let consumer = ArcConsumer::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x * 3);
        });

        consumer.accept(&7);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![21]
        );
    }

    #[test]
    fn test_rc_consumer_accept_once() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l = log.clone();
        let consumer = RcConsumer::new(move |x: &i32| {
            l.borrow_mut().push(*x - 5);
        });

        consumer.accept(&15);
        assert_eq!(*log.borrow(), vec![10]);
    }
}

// ============================================================================
// BoxStatefulConsumer and_then Tests
// ============================================================================
