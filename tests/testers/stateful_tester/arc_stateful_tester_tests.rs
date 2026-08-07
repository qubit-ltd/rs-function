// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::ArcStatefulTester;
use qubit_function::StatefulTester;

#[test]
fn test_arc_stateful_tester_observable_behavior() {
    let mut calls = 0;
    let mut tester = ArcStatefulTester::new(move || {
        calls += 1;
        calls == 1
    });
    assert!(tester.test());
    assert!(!tester.test());
}
