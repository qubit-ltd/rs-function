// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::ArcPredicate;
use qubit_function::Predicate;

#[test]
fn test_arc_predicate_observable_behavior() {
    let predicate = ArcPredicate::new(|value: &i32| *value % 2 == 0);
    assert!(predicate.test(&4));
    assert!(!predicate.test(&3));
}

#[test]
fn test_arc_predicate_not_operator_observable_behavior() {
    let owned_negated = !ArcPredicate::new(|value: &i32| *value > 0);
    assert!(!owned_negated.test(&5));
    assert!(owned_negated.test(&-5));

    let original = ArcPredicate::new(|value: &i32| *value > 0);
    let borrowed_negated = !&original;
    assert!(!borrowed_negated.test(&5));
    assert!(borrowed_negated.test(&-5));
    assert!(original.test(&5));
}
