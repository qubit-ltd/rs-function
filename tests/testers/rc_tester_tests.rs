/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::{
    RcTester,
    Tester,
};

#[test]
fn test_rc_tester_observable_behavior() {
    let type_name = std::any::type_name::<RcTester>();
    assert!(type_name.contains("RcTester"), "{type_name}");
}

#[test]
fn test_rc_tester_not_operator_observable_behavior() {
    let owned_negated = !RcTester::new(|| true);
    assert!(!owned_negated.test());

    let original = RcTester::new(|| true);
    let borrowed_negated = !&original;
    assert!(!borrowed_negated.test());
    assert!(original.test());
}
