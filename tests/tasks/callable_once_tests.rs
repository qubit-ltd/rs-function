// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Unit tests for CallableOnce and BoxCallableOnce.

use std::{
    io,
    rc::Rc,
};

use qubit_function::{
    BoxCallableOnce,
    CallableOnce,
    LocalBoxCallableOnce,
    SupplierOnce,
};

#[derive(Clone)]
struct ClonedCallableOnce {
    value: i32,
}

impl CallableOnce<i32, io::Error> for ClonedCallableOnce {
    fn call(self) -> Result<i32, io::Error> {
        Ok(self.value)
    }
}

fn assert_send<T: Send>() {}

#[test]
fn test_callable_once_closure_call_returns_success_value() {
    let task = || Ok::<i32, io::Error>(42);

    assert_eq!(
        task.call().expect("callable-once closure should succeed"),
        42
    );
}

#[test]
fn test_callable_once_closure_call_returns_error() {
    let task = || Err::<i32, _>(io::Error::other("failed"));

    let error = task.call().expect_err("callable-once closure should fail");
    assert_eq!(error.kind(), io::ErrorKind::Other);
    assert_eq!(error.to_string(), "failed");
}

#[test]
fn test_box_callable_once_new_and_call() {
    let task = BoxCallableOnce::new(|| Ok::<i32, io::Error>(21));
    assert_eq!(task.call().expect("box callable-once should succeed"), 21);
}

#[test]
fn test_box_callable_once_is_send_task_object() {
    assert_send::<BoxCallableOnce<i32, io::Error>>();
}

#[test]
fn test_local_box_callable_once_allows_non_send_capture() {
    let text = Rc::new(String::from("local"));
    let captured = Rc::clone(&text);
    let task = LocalBoxCallableOnce::new(move || {
        Ok::<String, io::Error>(captured.to_string())
    });

    assert_eq!(
        task.call()
            .expect("local callable-once should allow local capture"),
        "local"
    );
}

#[test]
fn test_box_callable_once_with_name() {
    let mut task =
        BoxCallableOnce::<i32, io::Error>::new_with_name("compute", || Ok(1));

    assert_eq!(task.name(), Some("compute"));
    assert_eq!(task.to_string(), "BoxCallableOnce(compute)");
    assert!(format!("{task:?}").contains("compute"));

    task.set_name("renamed");
    assert_eq!(task.name(), Some("renamed"));

    task.clear_name();
    assert_eq!(task.name(), None);
    assert_eq!(task.to_string(), "BoxCallableOnce");
}

#[test]
fn test_box_callable_once_from_supplier() {
    let supplier = || Ok::<i32, io::Error>(34);
    let task = BoxCallableOnce::from_supplier(supplier);
    assert_eq!(
        task.call()
            .expect("supplier-backed callable should succeed"),
        34
    );
}

#[test]
fn test_box_callable_once_implements_supplier_once() {
    let task = BoxCallableOnce::new(|| Ok::<i32, io::Error>(55));

    let result = SupplierOnce::get(task);

    assert_eq!(result.expect("supplier once should succeed"), 55);
}

#[test]
fn test_box_callable_once_map_transforms_success_value() {
    let task =
        BoxCallableOnce::new_with_name("compute", || Ok::<i32, io::Error>(10));

    let mapped = task.map(|value| value * 2);

    assert_eq!(mapped.name(), Some("compute"));
    assert_eq!(mapped.call().expect("mapped callable should succeed"), 20);
}

#[test]
fn test_box_callable_once_map_err_transforms_error_value() {
    let task = BoxCallableOnce::new(|| Err::<i32, _>(io::Error::other("raw")));
    let mapped = task.map_err(|error| error.to_string());

    assert_eq!(
        mapped.call().expect_err("mapped callable should fail"),
        "raw"
    );
}

#[test]
fn test_box_callable_once_and_then_runs_next_on_success() {
    let task = BoxCallableOnce::new(|| Ok::<i32, io::Error>(4));
    let chained = task.and_then(|value| Ok(value * 3));

    assert_eq!(chained.call().expect("chained callable should succeed"), 12);
}

#[test]
fn test_box_callable_once_and_then_skips_next_on_error() {
    let task = BoxCallableOnce::new(|| Err::<i32, _>(io::Error::other("stop")));
    let chained = task.and_then(|_| Ok::<i32, io::Error>(99));

    assert_eq!(
        chained
            .call()
            .expect_err("chained callable should preserve error")
            .to_string(),
        "stop",
    );
}

#[derive(Clone)]
struct TextCallableOnce {
    value: String,
}

impl CallableOnce<String, &'static str> for TextCallableOnce {
    fn call(self) -> Result<String, &'static str> {
        Ok(self.value)
    }
}

#[test]
fn test_box_callable_once_combinators_with_text_error_type() {
    let mapped =
        BoxCallableOnce::new(|| Ok::<i32, &'static str>(6)).map(|v| v + 1);
    assert_eq!(mapped.call().expect("map should succeed"), 7);

    let mapped_err = BoxCallableOnce::new(|| Err::<i32, _>("raw"))
        .map_err(|e| format!("E:{e}"));
    assert_eq!(
        mapped_err
            .call()
            .expect_err("map_err should transform error"),
        "E:raw",
    );

    let chained = BoxCallableOnce::new(|| Ok::<i32, &'static str>(4))
        .and_then(|v| Ok::<i32, &'static str>(v * 2));
    assert_eq!(chained.call().expect("and_then should succeed"), 8);
}

#[test]
fn test_box_callable_once_from_supplier_with_text_error_type() {
    let task = BoxCallableOnce::from_supplier(|| {
        Ok::<String, &'static str>("supplied".to_string())
    });
    assert_eq!(
        task.call().expect("from_supplier should succeed"),
        "supplied",
    );
}
