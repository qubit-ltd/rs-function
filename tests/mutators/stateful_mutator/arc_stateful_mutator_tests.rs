// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Unit tests for `ArcStatefulMutator`.

use std::thread;

use qubit_function::ArcStatefulMutator;
use qubit_function::StatefulMutator;

#[test]
fn test_new() {
    let mutator = ArcStatefulMutator::new(|x: &mut i32| *x += 1);
    let mut value = 5;
    let mut c = mutator;
    c.apply(&mut value);
    assert_eq!(value, 6, "the callback should increment the value");
}

#[test]
fn test_clone() {
    let mutator = ArcStatefulMutator::new(|x: &mut i32| *x *= 2);
    let clone1 = mutator.clone();
    let clone2 = mutator.clone();

    let mut value1 = 5;
    let mut c1 = clone1;
    c1.apply(&mut value1);
    assert_eq!(value1, 10, "the first clone should double the value");

    let mut value2 = 3;
    let mut c2 = clone2;
    c2.apply(&mut value2);
    assert_eq!(value2, 6, "the second clone should double the value");
}

#[test]
fn test_and_then() {
    let first = ArcStatefulMutator::new(|x: &mut i32| *x *= 2);
    let second = ArcStatefulMutator::new(|x: &mut i32| *x += 10);

    let chained = first.and_then(second);

    let mut value = 5;
    let mut c = chained;
    c.apply(&mut value);
    assert_eq!(value, 20, "the chained callbacks should run in order"); // (5 * 2) + 10

    // first and second are still usable
    let mut value2 = 3;
    let mut f = first;
    f.apply(&mut value2);
    assert_eq!(value2, 6, "the first mutator should remain usable after chaining");
}

#[test]
fn test_thread_safety() {
    let mutator = ArcStatefulMutator::new(|x: &mut i32| *x *= 2);
    let clone = mutator.clone();

    let handle = thread::spawn(move || {
        let mut value = 5;
        let mut c = clone;
        c.apply(&mut value);
        value
    });

    let mut value = 3;
    let mut c = mutator;
    c.apply(&mut value);
    assert_eq!(value, 6, "the original mutator should apply on the current thread");

    assert_eq!(
        handle.join().expect("thread should not panic"),
        10,
        "the cloned mutator should apply on the spawned thread"
    );
}

#[test]
fn test_noop() {
    let noop = ArcStatefulMutator::<i32>::noop();
    let mut value = 42;
    let mut m = noop;
    m.apply(&mut value);
    assert_eq!(value, 42, "the no-op mutator should leave the value unchanged");
}

#[test]
fn test_noop_clone() {
    let noop = ArcStatefulMutator::<i32>::noop();
    let clone1 = noop.clone();
    let clone2 = noop.clone();

    let mut value1 = 42;
    let mut m1 = clone1;
    m1.apply(&mut value1);
    assert_eq!(value1, 42, "the first no-op clone should leave the value unchanged");

    let mut value2 = 100;
    let mut m2 = clone2;
    m2.apply(&mut value2);
    assert_eq!(value2, 100, "the second no-op clone should leave the value unchanged");
}

#[test]
fn test_noop_chaining() {
    let noop = ArcStatefulMutator::<i32>::noop();
    let double = ArcStatefulMutator::new(|x: &mut i32| *x *= 2);

    let chained = noop.and_then(double);

    let mut value = 5;
    let mut c = chained;
    c.apply(&mut value);
    assert_eq!(value, 10, "the chained mutator should apply the doubling callback");
}

#[test]
fn test_new_with_name() {
    let mut mutator = ArcStatefulMutator::new_with_name("arc_stateful_test", |x: &mut i32| *x += 1);
    assert_eq!(
        mutator.name(),
        Some("arc_stateful_test"),
        "the constructor should retain the provided name"
    );

    let mut value = 5;
    mutator.apply(&mut value);
    assert_eq!(value, 6, "the named mutator should invoke its callback");
}

#[test]
fn test_new_with_optional_name_some() {
    let mut mutator =
        ArcStatefulMutator::new_with_optional_name(|x: &mut i32| *x += 1, Some("arc_stateful_optional".to_string()));
    assert_eq!(
        mutator.name(),
        Some("arc_stateful_optional"),
        "the constructor should retain the optional name"
    );

    let mut value = 5;
    mutator.apply(&mut value);
    assert_eq!(value, 6, "the optionally named mutator should invoke its callback");
}

#[test]
fn test_new_with_optional_name_none() {
    let mut mutator = ArcStatefulMutator::new_with_optional_name(|x: &mut i32| *x += 1, None);
    assert_eq!(mutator.name(), None, "the constructor should leave the name unset");

    let mut value = 5;
    mutator.apply(&mut value);
    assert_eq!(value, 6, "the unnamed mutator should invoke its callback");
}

#[test]
fn test_name_and_set_name() {
    let mut mutator = ArcStatefulMutator::new(|x: &mut i32| *x += 1);
    assert_eq!(mutator.name(), None, "new mutators should not have a name");

    mutator.set_name("arc_stateful_set_name");
    assert_eq!(
        mutator.name(),
        Some("arc_stateful_set_name"),
        "set_name should update the mutator name"
    );

    let mut value = 5;
    mutator.apply(&mut value);
    assert_eq!(value, 6, "renaming should not change callback behavior");
}

#[test]
fn test_arc_stateful_mutator_clones_share_callback_state() {
    let mut calls = 0;
    let mut mutator = ArcStatefulMutator::new(move |value: &mut i32| {
        calls += 1;
        *value += calls;
    });
    let mut cloned_mutator = mutator.clone();

    let mut value = 10;
    mutator.apply(&mut value);
    assert_eq!(value, 11, "the original mutator should apply the first call");

    cloned_mutator.apply(&mut value);
    assert_eq!(value, 13, "the clone should continue the shared call state");
}
