// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::cell::RefCell;
use std::rc::Rc;

use qubit_function::RcStatefulBiPredicate;
use qubit_function::StatefulBiPredicate;

#[test]
fn test_rc_stateful_bi_predicate_observable_behavior() {
    let mut predicate = RcStatefulBiPredicate::new(|left: &i32, right: &i32| left < right);
    assert!(predicate.test(&1, &2));
    assert!(!predicate.test(&2, &1));
}

#[test]
fn test_rc_stateful_bi_predicate_self_composition() {
    let calls = Rc::new(RefCell::new(Vec::new()));
    let recorded = Rc::clone(&calls);
    let predicate = RcStatefulBiPredicate::new(move |left: &i32, right: &i32| {
        assert_eq!((*left, *right), (1, 2));
        let mut values = recorded.borrow_mut();
        let next = values.len() + 1;
        values.push(next);
        next == 1
    });
    let mut combined = predicate.and(predicate.clone());
    assert!(!combined.test(&1, &2));
    assert_eq!(*calls.borrow(), vec![1, 2]);

    let calls = Rc::new(RefCell::new(Vec::new()));
    let recorded = Rc::clone(&calls);
    let predicate = RcStatefulBiPredicate::new(move |left: &i32, right: &i32| {
        assert_eq!((*left, *right), (1, 2));
        let mut values = recorded.borrow_mut();
        let next = values.len() + 1;
        values.push(next);
        false
    });
    let mut combined = predicate.and(predicate.clone());
    assert!(!combined.test(&1, &2));
    assert_eq!(*calls.borrow(), vec![1]);

    let calls = Rc::new(RefCell::new(Vec::new()));
    let recorded = Rc::clone(&calls);
    let predicate = RcStatefulBiPredicate::new(move |left: &i32, right: &i32| {
        assert_eq!((*left, *right), (1, 2));
        let mut values = recorded.borrow_mut();
        let next = values.len() + 1;
        values.push(next);
        next == 2
    });
    let mut combined = predicate.or(predicate.clone());
    assert!(combined.test(&1, &2));
    assert_eq!(*calls.borrow(), vec![1, 2]);

    let calls = Rc::new(RefCell::new(Vec::new()));
    let recorded = Rc::clone(&calls);
    let predicate = RcStatefulBiPredicate::new(move |left: &i32, right: &i32| {
        assert_eq!((*left, *right), (1, 2));
        let mut values = recorded.borrow_mut();
        let next = values.len() + 1;
        values.push(next);
        true
    });
    let mut combined = predicate.or(predicate.clone());
    assert!(combined.test(&1, &2));
    assert_eq!(*calls.borrow(), vec![1]);
}
