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
    Predicate,
    RcPredicate,
};

#[test]
fn test_rc_predicate_observable_behavior() {
    let type_name = std::any::type_name::<RcPredicate<i32>>();
    assert!(type_name.contains("RcPredicate"), "{type_name}");
}

#[test]
fn test_rc_predicate_not_operator_observable_behavior() {
    let owned_negated = !RcPredicate::new(|value: &i32| *value > 0);
    assert!(!owned_negated.test(&5));
    assert!(owned_negated.test(&-5));

    let original = RcPredicate::new(|value: &i32| *value > 0);
    let borrowed_negated = !&original;
    assert!(!borrowed_negated.test(&5));
    assert!(borrowed_negated.test(&-5));
    assert!(original.test(&5));
}
