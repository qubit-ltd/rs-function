/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::StatefulTester;

#[derive(Clone)]
struct ThresholdTester {
    count: usize,
    threshold: usize,
}

impl StatefulTester for ThresholdTester {
    fn test(&mut self) -> bool {
        self.count += 1;
        self.count >= self.threshold
    }
}

#[test]
fn test_stateful_tester_closure_mutates_state() {
    let mut count = 0;
    let mut tester = move || {
        count += 1;
        count >= 2
    };

    assert!(!StatefulTester::test(&mut tester));
    assert!(StatefulTester::test(&mut tester));
}

#[test]
fn test_stateful_tester_into_fn_returns_fn_mut() {
    let tester = ThresholdTester {
        count: 0,
        threshold: 3,
    };
    let mut function = tester.into_fn();

    assert!(!function());
    assert!(!function());
    assert!(function());
}

#[test]
fn test_stateful_tester_into_mut_fn_returns_fn_mut() {
    let tester = ThresholdTester {
        count: 0,
        threshold: 2,
    };
    let mut function = tester.into_mut_fn();

    assert!(!function());
    assert!(function());
}

#[test]
fn test_stateful_tester_to_fn_uses_cloned_state() {
    let tester = ThresholdTester {
        count: 0,
        threshold: 2,
    };
    let mut function = tester.to_fn();

    assert!(!function());
    assert!(function());

    let mut original = tester.clone();
    assert!(!original.test());
}

#[test]
fn test_stateful_tester_to_mut_fn_uses_cloned_state() {
    let tester = ThresholdTester {
        count: 0,
        threshold: 2,
    };
    let mut function = tester.to_mut_fn();

    assert!(!function());
    assert!(function());

    let mut original = tester.clone();
    assert!(!original.test());
}
