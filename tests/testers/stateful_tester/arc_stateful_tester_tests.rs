// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::sync::Arc;
use std::sync::Mutex;

use qubit_function::ArcStatefulTester;
use qubit_function::StatefulTester;

use crate::internal::isolated_test::run_isolated;

#[test]
fn test_arc_stateful_tester_observable_behavior() {
    let mut calls = 0;
    let mut tester = ArcStatefulTester::new(move || {
        calls += 1;
        calls == 1
    });
    assert!(tester.test(), "the first call should return true");
    assert!(!tester.test(), "the second call should return false");
}

#[test]
fn test_arc_stateful_tester_self_composition() {
    let qualified = concat!(module_path!(), "::", "test_arc_stateful_tester_self_composition");
    let (_, exact_name) = qualified.split_once("::").expect("test path includes crate prefix");
    run_isolated(exact_name, || {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let recorded = Arc::clone(&calls);
        let tester = ArcStatefulTester::new(move || {
            let mut values = recorded.lock().expect("record lock");
            let next = values.len() + 1;
            values.push(next);
            next == 1
        });
        let mut combined = tester.and(tester.clone());
        assert!(!combined.test());
        assert_eq!(*calls.lock().expect("record lock"), vec![1, 2]);

        let calls = Arc::new(Mutex::new(Vec::new()));
        let recorded = Arc::clone(&calls);
        let tester = ArcStatefulTester::new(move || {
            let mut values = recorded.lock().expect("record lock");
            let next = values.len() + 1;
            values.push(next);
            false
        });
        let mut combined = tester.and(tester.clone());
        assert!(!combined.test());
        assert_eq!(*calls.lock().expect("record lock"), vec![1]);

        let calls = Arc::new(Mutex::new(Vec::new()));
        let recorded = Arc::clone(&calls);
        let tester = ArcStatefulTester::new(move || {
            let mut values = recorded.lock().expect("record lock");
            let next = values.len() + 1;
            values.push(next);
            next == 2
        });
        let mut combined = tester.or(tester.clone());
        assert!(combined.test());
        assert_eq!(*calls.lock().expect("record lock"), vec![1, 2]);

        let calls = Arc::new(Mutex::new(Vec::new()));
        let recorded = Arc::clone(&calls);
        let tester = ArcStatefulTester::new(move || {
            let mut values = recorded.lock().expect("record lock");
            let next = values.len() + 1;
            values.push(next);
            true
        });
        let mut combined = tester.or(tester.clone());
        assert!(combined.test());
        assert_eq!(*calls.lock().expect("record lock"), vec![1]);
    });
}
