// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::RcStatefulMutatingFunction;
use qubit_function::StatefulMutatingFunction;

#[test]
fn test_rc_stateful_mutating_function_observable_behavior() {
    let mut call_count = 0;
    let mut function = RcStatefulMutatingFunction::new(move |value: &mut i32| {
        call_count += 1;
        *value += 1;
        call_count
    });

    let mut value = 5;
    assert_eq!(function.apply(&mut value), 1);
    assert_eq!(value, 6);
    assert_eq!(function.apply(&mut value), 2);
    assert_eq!(value, 7);
}

#[test]
fn test_rc_and_then_stateful_shares_pipeline_state() {
    let mut source_calls = 0;
    let mut source = RcStatefulMutatingFunction::new_with_name("source", move |input: &mut i32| {
        source_calls += 1;
        *input += source_calls;
        *input
    });
    let mut mapper_calls = 0;
    let mut pipeline = source.and_then_stateful(move |result: &i32| {
        mapper_calls += 1;
        *result + mapper_calls
    });
    let mut clone = pipeline.clone();
    let mut input = 5;
    assert_eq!(pipeline.apply(&mut input), 7);
    assert_eq!(clone.apply(&mut input), 10);
    assert_eq!(source.apply(&mut input), 11);
    assert_eq!(pipeline.name(), None);
    assert_eq!(source.name(), Some("source"));
}
