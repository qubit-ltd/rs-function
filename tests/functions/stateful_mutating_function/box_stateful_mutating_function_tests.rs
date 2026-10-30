// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxStatefulFunction;
use qubit_function::BoxStatefulMutatingFunction;
use qubit_function::Function;
use qubit_function::StatefulMutatingFunction;

#[test]
fn test_box_stateful_mutating_function_retains_state() {
    let mut calls = 0;
    let mut function = BoxStatefulMutatingFunction::new(move |value: &mut i32| {
        calls += 1;
        *value += calls;
        *value
    });
    let mut value = 5;
    assert_eq!(function.apply(&mut value), 6);
    assert_eq!(function.apply(&mut value), 8);
}

#[test]
fn test_box_and_then_stateful_accepts_wrapped_mapper() {
    let mut source_calls = 0;
    let source = BoxStatefulMutatingFunction::new_with_name("source", move |input: &mut i32| {
        source_calls += 1;
        *input += source_calls;
        *input
    });
    let mut mapper_calls = 0;
    let mapper = BoxStatefulFunction::new(move |result: &i32| {
        mapper_calls += 1;
        *result + mapper_calls
    });
    let mut pipeline = source.and_then_stateful(mapper);
    let mut input = 5;
    assert_eq!(pipeline.name(), None);
    assert_eq!(pipeline.apply(&mut input), 7);
    assert_eq!(pipeline.apply(&mut input), 10);
    assert_eq!(input, 8);
}

struct DoubleResult;

impl Function<i32, i32> for DoubleResult {
    fn apply(&self, value: &i32) -> i32 {
        *value * 2
    }
}

#[test]
fn test_box_existing_and_then_accepts_custom_function() {
    let source = BoxStatefulMutatingFunction::new(|input: &mut i32| {
        *input += 1;
        *input
    });
    let mut pipeline = source.and_then(DoubleResult);
    assert_eq!(pipeline.apply(&mut 5), 12);
}
