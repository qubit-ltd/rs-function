// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Unit tests for the predicate module.

use std::cell::RefCell;
use std::sync::Arc;
use std::sync::Mutex;

use qubit_function::predicates::ArcPredicate;
use qubit_function::predicates::BoxPredicate;
use qubit_function::predicates::Predicate;
use qubit_function::predicates::RcPredicate;

struct PositivePredicate;

impl Predicate<i32> for PositivePredicate {
    fn test(&self, value: &i32) -> bool {
        *value > 0
    }
}

#[test]
fn test_new_accepts_custom_predicate() {
    let boxed = BoxPredicate::new(PositivePredicate);
    let shared = ArcPredicate::new(PositivePredicate);

    assert!(boxed.test(&1));
    assert!(!shared.test(&-1));
}

#[test]
fn test_predicate_not_operator() {
    let boxed = !BoxPredicate::new(|x: &i32| *x > 0);
    assert!(!boxed.test(&5));
    assert!(boxed.test(&-5));

    let rc = RcPredicate::new(|x: &i32| *x > 0);
    let negated_rc = !&rc;
    assert!(!negated_rc.test(&5));
    assert!(negated_rc.test(&-5));
    assert!(rc.test(&5));

    let arc = ArcPredicate::new(|x: &i32| *x > 0);
    let negated_arc = !&arc;
    assert!(!negated_arc.test(&5));
    assert!(negated_arc.test(&-5));
    assert!(arc.test(&5));
}
