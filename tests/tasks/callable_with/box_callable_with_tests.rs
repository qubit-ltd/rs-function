// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Tests for the boxed callable-with adapter.

use std::io;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use qubit_function::BoxCallableWith;
use qubit_function::CallableWith;

#[test]
fn test_box_callable_with_name_management() {
    let mut task = BoxCallableWith::<i32, i32, io::Error>::new_with_name("adjust", |input: &mut i32| Ok(*input + 1));

    assert_eq!(task.name(), Some("adjust"));
    assert_eq!(task.to_string(), "BoxCallableWith(adjust)");
    assert!(format!("{task:?}").contains("adjust"));

    task.set_name("renamed");
    assert_eq!(task.name(), Some("renamed"));

    task.clear_name();
    assert_eq!(task.name(), None);
    assert_eq!(task.to_string(), "BoxCallableWith");
}

#[test]
fn test_box_callable_with_map_transforms_success_value() {
    let task = BoxCallableWith::new_with_name("compute", |input: &mut i32| {
        *input += 2;
        Ok::<i32, io::Error>(*input)
    });
    let mut mapped = task.map(|value| value * 3);
    let mut input = 5;

    assert_eq!(mapped.name(), Some("compute"));
    assert_eq!(
        mapped
            .call_with(&mut input)
            .expect("mapped callable-with should succeed"),
        21
    );
    assert_eq!(input, 7);
}

#[test]
fn test_box_callable_with_map_err_transforms_error() {
    let task = BoxCallableWith::<i32, i32, io::Error>::new(|_input: &mut i32| Err(io::Error::other("original")));
    let mut mapped = task.map_err(|error| error.to_string());
    let mut input = 0;

    let error = mapped
        .call_with(&mut input)
        .expect_err("mapped error should be returned");

    assert_eq!(error, "original");
}

#[test]
fn test_box_callable_with_and_then_receives_value_and_input() {
    let task = BoxCallableWith::new(|input: &mut i32| {
        *input += 2;
        Ok::<i32, io::Error>(*input)
    });
    let mut chained = task.and_then(|value, input: &mut i32| {
        *input += value;
        Ok::<i32, io::Error>(*input)
    });
    let mut input = 4;

    assert_eq!(
        chained
            .call_with(&mut input)
            .expect("chained callable-with should succeed"),
        12
    );
    assert_eq!(input, 12);
}

#[test]
fn test_box_callable_with_combinators_cover_error_branches() {
    let mut input = 0;
    let mut mapped =
        BoxCallableWith::<i32, i32, io::Error>::new(|_value: &mut i32| Err(io::Error::other("map source failed")))
            .map(|value| value + 1);
    let error = mapped
        .call_with(&mut input)
        .expect_err("map should propagate source errors");
    assert_eq!(error.to_string(), "map source failed");

    let mut map_err_success =
        BoxCallableWith::<i32, i32, io::Error>::new(|value: &mut i32| Ok(*value)).map_err(|error| error.to_string());
    assert_eq!(
        map_err_success
            .call_with(&mut input)
            .expect("map_err should preserve success"),
        0
    );

    let next_ran = Arc::new(AtomicBool::new(false));
    let next_ran_capture = Arc::clone(&next_ran);
    let mut chained =
        BoxCallableWith::<i32, i32, io::Error>::new(|_value: &mut i32| Err(io::Error::other("first failed"))).and_then(
            move |value, input| {
                *input += value;
                next_ran_capture.store(true, Ordering::SeqCst);
                Ok::<i32, io::Error>(*input)
            },
        );
    let error = chained
        .call_with(&mut input)
        .expect_err("and_then should short-circuit");

    assert_eq!(error.to_string(), "first failed");
    assert!(!next_ran.load(Ordering::SeqCst));
    assert_eq!(input, 0);
}
