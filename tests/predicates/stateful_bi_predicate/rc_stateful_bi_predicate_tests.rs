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

use crate::internal::isolated_test::run_isolated;

#[test]
fn test_rc_shared_bi_predicate_remaining_logical_contracts() {
    let qualified = concat!(
        module_path!(),
        "::",
        "test_rc_shared_bi_predicate_remaining_logical_contracts"
    );
    let (_, exact_name) = qualified.split_once("::").expect("test path includes crate prefix");
    run_isolated(exact_name, || {
        let inputs = [(false, false), (false, true), (true, false), (true, true)];
        let expected = [
            [true, true, true, false],
            [false, true, true, false],
            [true, false, false, false],
        ];
        for (operation, expected_values) in expected.iter().enumerate() {
            for (case, (left, right)) in inputs.into_iter().enumerate() {
                let calls = Rc::new(RefCell::new(Vec::new()));
                let recorded = Rc::clone(&calls);
                let source = RcStatefulBiPredicate::new(move |a: &i32, b: &i32| {
                    assert_eq!((*a, *b), (1, 2));
                    let mut log = recorded.borrow_mut();
                    let index = log.len();
                    log.push(index + 1);
                    [left, right][index]
                });
                let mut combined = match operation {
                    0 => source.nand(source.clone()),
                    1 => source.xor(source.clone()),
                    2 => source.nor(source.clone()),
                    _ => unreachable!(),
                };
                assert_eq!(
                    combined.test(&1, &2),
                    expected_values[case],
                    "operation={operation}, case={case}"
                );
                let expected_log = match operation {
                    0 if !left => vec![1],
                    2 if left => vec![1],
                    _ => vec![1, 2],
                };
                assert_eq!(
                    *calls.borrow(),
                    expected_log,
                    "operation={operation}, case={case}: callback order/short circuit"
                );
            }
        }
    });
}

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
