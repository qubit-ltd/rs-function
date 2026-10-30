// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::cell::Cell;
use std::thread;

use qubit_function::ArcStatefulMutatingFunction;
use qubit_function::StatefulMutatingFunction;

#[test]
fn test_arc_stateful_mutating_function_retains_state() {
    let mut calls = 0;
    let mut function = ArcStatefulMutatingFunction::new(move |value: &mut i32| {
        calls += 1;
        *value += calls;
        *value
    });
    let mut value = 5;
    assert_eq!(function.apply(&mut value), 6);
    assert_eq!(function.apply(&mut value), 8);
}

#[test]
fn test_arc_and_then_stateful_accepts_send_non_sync_mapper() {
    let source = ArcStatefulMutatingFunction::new(|input: &mut i32| {
        *input += 1;
        *input
    });
    let calls = Cell::new(0);
    let mut mapper_calls = 0;
    let mut pipeline = source.and_then_stateful(move |result: &i32| {
        mapper_calls += 1;
        calls.set(mapper_calls);
        *result + calls.get()
    });
    let mut clone = pipeline.clone();
    let handle = thread::spawn(move || {
        let mut input = 5;
        clone.apply(&mut input)
    });
    assert_eq!(handle.join().expect("worker should complete"), 7);
    let mut input = 5;
    assert_eq!(pipeline.apply(&mut input), 8);
}
