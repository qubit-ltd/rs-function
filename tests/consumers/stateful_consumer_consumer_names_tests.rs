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
mod test_consumer_names {
    use super::{
        Arc,
        ArcConsumer,
        ArcStatefulConsumer,
        BoxConsumer,
        BoxStatefulConsumer,
        Consumer,
        Mutex,
        Rc,
        RcConsumer,
        RcStatefulConsumer,
        RefCell,
    };

    #[test]
    fn test_box_consumer_with_name() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let mut consumer = BoxConsumer::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });
        consumer.set_name("logger");
        assert_eq!(consumer.name(), Some("logger"));
        consumer.accept(&5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);
    }

    #[test]
    fn test_box_consumer_set_name() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let mut consumer = BoxStatefulConsumer::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });
        assert_eq!(consumer.name(), None);
        consumer.set_name("my_consumer");
        assert_eq!(consumer.name(), Some("my_consumer"));
    }

    #[test]
    fn test_arc_consumer_with_name() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let mut consumer = ArcConsumer::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });
        consumer.set_name("logger");
        assert_eq!(consumer.name(), Some("logger"));
        consumer.accept(&5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);
    }

    #[test]
    fn test_arc_consumer_set_name() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let mut consumer = ArcStatefulConsumer::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });
        assert_eq!(consumer.name(), None);
        consumer.set_name("my_consumer");
        assert_eq!(consumer.name(), Some("my_consumer"));
    }

    #[test]
    fn test_rc_consumer_with_name() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l = log.clone();
        let mut consumer = RcConsumer::new(move |x: &i32| {
            l.borrow_mut().push(*x);
        });
        consumer.set_name("logger");
        assert_eq!(consumer.name(), Some("logger"));
        consumer.accept(&5);
        assert_eq!(*log.borrow(), vec![5]);
    }

    #[test]
    fn test_rc_consumer_set_name() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l = log.clone();
        let mut consumer = RcStatefulConsumer::new(move |x: &i32| {
            l.borrow_mut().push(*x);
        });
        assert_eq!(consumer.name(), None);
        consumer.set_name("my_consumer");
        assert_eq!(consumer.name(), Some("my_consumer"));
    }
}

// ============================================================================
// Edge Cases Tests
// ============================================================================
