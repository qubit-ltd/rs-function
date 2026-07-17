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
mod test_unified_interface {
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

    fn apply_consumer<C: Consumer<i32>>(consumer: &mut C, value: &i32) -> i32 {
        consumer.accept(value);
        *value // Return original value since Consumer doesn't modify input
    }

    #[test]
    fn test_with_box_consumer() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let mut consumer = BoxConsumer::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x * 2);
        });
        let result = apply_consumer(&mut consumer, &5);
        assert_eq!(result, 5);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![10]
        );
    }

    #[test]
    fn test_with_arc_consumer() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let mut consumer = ArcConsumer::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x * 2);
        });
        let result = apply_consumer(&mut consumer, &5);
        assert_eq!(result, 5);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![10]
        );
    }

    #[test]
    fn test_with_rc_consumer() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l = log.clone();
        let mut consumer = RcConsumer::new(move |x: &i32| {
            l.borrow_mut().push(*x * 2);
        });
        let result = apply_consumer(&mut consumer, &5);
        assert_eq!(result, 5);
        assert_eq!(*log.borrow(), vec![10]);
    }
}

// ============================================================================
// BoxConsumer chaining test
// ============================================================================
