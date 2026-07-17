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
mod rc_conditional_bi_consumer_tests {
    use super::{
        BiConsumer,
        Rc,
        RcBiConsumer,
    };
    use std::cell::RefCell;

    #[test]
    fn test_rc_conditional_and_then() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();

        let consumer = RcBiConsumer::new(move |x: &i32, y: &i32| {
            l1.borrow_mut().push(*x + *y);
        });

        let conditional = consumer.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let chained = conditional.and_then(move |x: &i32, y: &i32| {
            l2.borrow_mut().push(*x * *y);
        });

        chained.accept(&5, &3);
        assert_eq!(*log.borrow(), vec![8, 15]);

        chained.accept(&-5, &3);
        assert_eq!(*log.borrow(), vec![8, 15, -15]);
    }

    #[test]
    fn test_rc_conditional_or_else() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();

        let consumer = RcBiConsumer::new(move |x: &i32, y: &i32| {
            l1.borrow_mut().push(*x + *y);
        });

        let conditional = consumer
            .when(|x: &i32, y: &i32| *x > 0 && *y > 0)
            .or_else(move |x: &i32, y: &i32| {
                l2.borrow_mut().push(*x * *y);
            });

        conditional.accept(&5, &3);
        assert_eq!(*log.borrow(), vec![8]);

        conditional.accept(&-5, &3);
        assert_eq!(*log.borrow(), vec![8, -15]);
    }

    #[test]
    fn test_rc_conditional_accept() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l = log.clone();

        let consumer = RcBiConsumer::new(move |x: &i32, y: &i32| {
            l.borrow_mut().push(*x + *y);
        });

        let conditional = consumer.when(|x: &i32, y: &i32| *x > 0 && *y > 0);

        conditional.accept(&5, &3);
        assert_eq!(*log.borrow(), vec![8]);

        conditional.accept(&-5, &3);
        assert_eq!(*log.borrow(), vec![8]);
    }
}
