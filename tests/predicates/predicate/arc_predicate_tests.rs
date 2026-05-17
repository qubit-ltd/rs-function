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
    ArcPredicate,
    Predicate,
};

#[test]
fn test_arc_predicate_observable_behavior() {
    let type_name = std::any::type_name::<ArcPredicate<i32>>();
    assert!(type_name.contains("ArcPredicate"), "{type_name}");
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
