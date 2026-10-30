// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::cell::RefCell;
use std::rc::Rc;

use qubit_function::RcStatefulTester;
use qubit_function::StatefulTester;

#[test]
fn test_rc_stateful_tester_observable_behavior() {
    let mut calls = 0;
    let mut tester = RcStatefulTester::new(move || {
        calls += 1;
        calls == 1
    });
    assert!(tester.test(), "the first callback result should be true");
    assert!(
        !tester.test(),
        "the callback should return false after its state changes"
    );
}

#[test]
fn test_rc_stateful_tester_self_composition() {
    let calls = Rc::new(RefCell::new(Vec::new()));
    let recorded = Rc::clone(&calls);
    let tester = RcStatefulTester::new(move || {
        let mut values = recorded.borrow_mut();
        let next = values.len() + 1;
        values.push(next);
        next == 1
    });
    let mut combined = tester.and(tester.clone());
    assert!(!combined.test());
    assert_eq!(*calls.borrow(), vec![1, 2]);

    let calls = Rc::new(RefCell::new(Vec::new()));
    let recorded = Rc::clone(&calls);
    let tester = RcStatefulTester::new(move || {
        let mut values = recorded.borrow_mut();
        let next = values.len() + 1;
        values.push(next);
        false
    });
    let mut combined = tester.and(tester.clone());
    assert!(!combined.test());
    assert_eq!(*calls.borrow(), vec![1]);

    let calls = Rc::new(RefCell::new(Vec::new()));
    let recorded = Rc::clone(&calls);
    let tester = RcStatefulTester::new(move || {
        let mut values = recorded.borrow_mut();
        let next = values.len() + 1;
        values.push(next);
        next == 2
    });
    let mut combined = tester.or(tester.clone());
    assert!(combined.test());
    assert_eq!(*calls.borrow(), vec![1, 2]);

    let calls = Rc::new(RefCell::new(Vec::new()));
    let recorded = Rc::clone(&calls);
    let tester = RcStatefulTester::new(move || {
        let mut values = recorded.borrow_mut();
        let next = values.len() + 1;
        values.push(next);
        true
    });
    let mut combined = tester.or(tester.clone());
    assert!(combined.test());
    assert_eq!(*calls.borrow(), vec![1]);
}
