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
mod rc_conditional_consumer_tests {
    use super::{
        Consumer,
        Rc,
        RcConsumer,
    };
    use std::cell::RefCell;

    #[test]
    fn test_rc_conditional_and_then() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();

        let consumer = RcConsumer::new(move |x: &i32| {
            l1.borrow_mut().push(*x);
        });

        let conditional = consumer.when(|x: &i32| *x > 0);
        let chained = conditional.and_then(move |x: &i32| {
            l2.borrow_mut().push(*x * 2);
        });

        chained.accept(&5);
        assert_eq!(*log.borrow(), vec![5, 10]);

        chained.accept(&-5);
        assert_eq!(*log.borrow(), vec![5, 10, -10]);
    }

    #[test]
    fn test_rc_conditional_or_else() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();

        let consumer = RcConsumer::new(move |x: &i32| {
            l1.borrow_mut().push(*x);
        });

        let conditional =
            consumer.when(|x: &i32| *x > 0).or_else(move |x: &i32| {
                l2.borrow_mut().push(*x * 10);
            });

        conditional.accept(&5);
        assert_eq!(*log.borrow(), vec![5]);

        conditional.accept(&-5);
        assert_eq!(*log.borrow(), vec![5, -50]);
    }

    #[test]
    fn test_rc_conditional_accept() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l = log.clone();

        let consumer = RcConsumer::new(move |x: &i32| {
            l.borrow_mut().push(*x);
        });

        let conditional = consumer.when(|x: &i32| *x > 0);

        conditional.accept(&5);
        assert_eq!(*log.borrow(), vec![5]);

        conditional.accept(&-5);
        assert_eq!(*log.borrow(), vec![5]);
    }
}
