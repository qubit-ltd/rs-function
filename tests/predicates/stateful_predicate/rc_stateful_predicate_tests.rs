// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::{
    RcStatefulPredicate,
    StatefulPredicate,
};

#[test]
fn test_rc_stateful_predicate_observable_behavior() {
    let mut predicate = RcStatefulPredicate::new(|value: &i32| *value > 0);
    assert!(predicate.test(&1));
    assert!(!predicate.test(&-1));
}
