// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::cell::Cell;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use qubit_function::ArcPredicate;
use qubit_function::Predicate;
use qubit_function::RcPredicate;

#[test]
fn test_arc_predicate_clone_shares_callback() {
    let calls = Arc::new(AtomicUsize::new(0));
    let captured_calls = Arc::clone(&calls);
    let predicate = ArcPredicate::new(move |value: &i32| {
        captured_calls.fetch_add(1, Ordering::SeqCst);
        *value > 0
    });
    let cloned = predicate.clone();

    assert!(predicate.test(&1));
    assert!(!cloned.test(&0));
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}

#[test]
fn test_rc_predicate_clone_shares_callback() {
    let calls = std::rc::Rc::new(Cell::new(0));
    let captured_calls = std::rc::Rc::clone(&calls);
    let predicate = RcPredicate::new(move |value: &i32| {
        captured_calls.set(captured_calls.get() + 1);
        *value > 0
    });
    let cloned = predicate.clone();

    assert!(predicate.test(&1));
    assert!(!cloned.test(&0));
    assert_eq!(calls.get(), 2);
}
