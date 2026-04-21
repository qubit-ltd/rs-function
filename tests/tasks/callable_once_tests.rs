/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! Unit tests for CallableOnce and BoxCallableOnce.

use std::io;

use qubit_function::{
    BoxCallableOnce,
    CallableOnce,
    RunnableOnce,
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
fn test_callable_once_closure_into_box_executes_once() {
    let data = String::from("payload");
    let task = move || Ok::<String, io::Error>(data);

    let boxed = CallableOnce::into_box(task);

    assert_eq!(
        boxed.call().expect("boxed callable should succeed"),
        "payload"
    );
}

#[test]
fn test_callable_once_closure_into_fn_returns_fn_once() {
    let task = || Ok::<i32, io::Error>(7);

    let function = CallableOnce::into_fn(task);

    assert_eq!(function().expect("callable function should succeed"), 7);
}

#[test]
fn test_callable_once_to_box_clones_callable() {
    let task = ClonedCallableOnce { value: 11 };

    let boxed = task.to_box();
    assert_eq!(boxed.call().expect("boxed clone should succeed"), 11);

    let boxed = task.to_box();
    assert_eq!(
        boxed.call().expect("cloned callable should remain usable"),
        11
    );
}

#[test]
fn test_callable_once_to_fn_clones_callable() {
    let task = ClonedCallableOnce { value: 13 };

    let function = task.to_fn();
    assert_eq!(function().expect("cloned callable should succeed"), 13);
}

#[test]
fn test_callable_once_default_into_runnable_discards_success_value() {
    let task = ClonedCallableOnce { value: 17 };

    let runnable = CallableOnce::into_runnable(task);

    runnable.run().expect("default runnable should succeed");
}

#[test]
fn test_box_callable_once_new_and_call() {
    let task = BoxCallableOnce::new(|| Ok::<i32, io::Error>(21));
    assert_eq!(task.call().expect("box callable-once should succeed"), 21);
}

#[test]
fn test_box_callable_once_with_name() {
    let mut task = BoxCallableOnce::<i32, io::Error>::new_with_name("compute", || Ok(1));

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
fn test_box_callable_once_into_box_returns_self() {
    let task = BoxCallableOnce::new(|| Ok::<i32, io::Error>(5));
    let boxed = CallableOnce::into_box(task);
    assert_eq!(
        boxed
            .call()
            .expect("box callable-once conversion should succeed"),
        5
    );
}

#[test]
fn test_box_callable_once_into_fn_extracts_function() {
    let task = BoxCallableOnce::new(|| Ok::<i32, io::Error>(8));
    let function = CallableOnce::into_fn(task);
    assert_eq!(function().expect("function should succeed"), 8);
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
fn test_box_callable_once_map_transforms_success_value() {
    let task = BoxCallableOnce::new_with_name("compute", || Ok::<i32, io::Error>(10));

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
