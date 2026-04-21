/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! Unit tests for callable task types.

use std::io;

use qubit_function::{
    BoxCallable,
    BoxCallableOnce,
    Callable,
    Runnable,
    SupplierOnce,
};

#[derive(Clone)]
struct ClonedCallable {
    value: i32,
}

impl Callable<i32, io::Error> for ClonedCallable {
    fn call(&mut self) -> Result<i32, io::Error> {
        Ok(self.value)
    }
}

#[test]
fn test_callable_closure_call_returns_success_value() {
    let mut task = || Ok::<i32, io::Error>(42);

    assert_eq!(
        Callable::call(&mut task).expect("callable closure should succeed"),
        42
    );
}

#[test]
fn test_callable_closure_call_returns_error() {
    let mut task = || Err::<i32, _>(io::Error::other("failed"));

    let error = Callable::call(&mut task).expect_err("callable closure should fail");
    assert_eq!(error.kind(), io::ErrorKind::Other);
    assert_eq!(error.to_string(), "failed");
}

#[test]
fn test_callable_closure_into_box_executes_once() {
    let data = String::from("payload");
    let task = move || Ok::<String, io::Error>(data.clone());

    let mut boxed = Callable::into_box(task);

    assert_eq!(
        boxed.call().expect("boxed callable should succeed"),
        "payload",
    );
}

#[test]
fn test_callable_closure_into_fn_returns_fn_once() {
    let task = || Ok::<i32, io::Error>(7);

    let mut function = Callable::into_fn(task);

    assert_eq!(function().expect("callable function should succeed"), 7);
}

#[test]
fn test_callable_to_box_clones_callable() {
    let mut task = ClonedCallable { value: 11 };

    let mut boxed = task.to_box();

    assert_eq!(boxed.call().expect("boxed clone should succeed"), 11);
    assert_eq!(
        task.call().expect("original callable should remain usable"),
        11,
    );
}

#[test]
fn test_callable_to_fn_clones_callable() {
    let mut task = ClonedCallable { value: 13 };

    let mut function = task.to_fn();

    assert_eq!(function().expect("cloned callable should succeed"), 13);
    drop(function);
    assert_eq!(
        task.call().expect("original callable should remain usable"),
        13,
    );
}

#[test]
fn test_callable_default_into_runnable_discards_success_value() {
    let task = ClonedCallable { value: 17 };

    let mut runnable = Callable::into_runnable(task);

    runnable.run().expect("default runnable should succeed");
}

#[test]
fn test_box_callable_new_and_call() {
    let mut task = BoxCallable::new(|| Ok::<i32, io::Error>(21));

    assert_eq!(task.call().expect("box callable should succeed"), 21);
}

#[test]
fn test_box_callable_name_management() {
    let mut task = BoxCallable::<i32, io::Error>::new_with_name("compute", || Ok(1));

    assert_eq!(task.name(), Some("compute"));
    assert_eq!(task.to_string(), "BoxCallable(compute)");
    assert!(format!("{task:?}").contains("compute"));

    task.set_name("renamed");
    assert_eq!(task.name(), Some("renamed"));

    task.clear_name();
    assert_eq!(task.name(), None);
    assert_eq!(task.to_string(), "BoxCallable");
}

#[test]
fn test_box_callable_into_box_returns_self() {
    let task = BoxCallable::new(|| Ok::<i32, io::Error>(5));

    let mut boxed = Callable::into_box(task);

    assert_eq!(boxed.call().expect("box callable should succeed"), 5);
}

#[test]
fn test_box_callable_into_fn_extracts_function() {
    let task = BoxCallable::new(|| Ok::<i32, io::Error>(8));

    let mut function = Callable::into_fn(task);

    assert_eq!(function().expect("function should succeed"), 8);
}

#[test]
fn test_box_callable_from_supplier() {
    let supplier = || Ok::<i32, io::Error>(34);

    let mut task = BoxCallable::from_supplier(supplier);

    assert_eq!(
        task.call()
            .expect("supplier-backed callable should succeed"),
        34,
    );
}

#[test]
fn test_box_callable_implements_supplier_once() {
    let task = BoxCallableOnce::new(|| Ok::<i32, io::Error>(55));

    let result = SupplierOnce::get(task);

    assert_eq!(result.expect("supplier callable should succeed"), 55);
}

#[test]
fn test_box_callable_map_transforms_success_value() {
    let task = BoxCallable::new_with_name("compute", || Ok::<i32, io::Error>(10));

    let mut mapped = task.map(|value| value * 2);

    assert_eq!(mapped.name(), Some("compute"));
    assert_eq!(mapped.call().expect("mapped callable should succeed"), 20);
}

#[test]
fn test_box_callable_map_err_transforms_error_value() {
    let task = BoxCallable::new(|| Err::<i32, _>(io::Error::other("raw")));

    let mut mapped = task.map_err(|error| error.to_string());

    assert_eq!(
        mapped.call().expect_err("mapped callable should fail"),
        "raw",
    );
}

#[test]
fn test_box_callable_and_then_runs_next_on_success() {
    let task = BoxCallable::new(|| Ok::<i32, io::Error>(4));

    let mut chained = task.and_then(|value| Ok(value * 3));

    assert_eq!(chained.call().expect("chained callable should succeed"), 12,);
}

#[test]
fn test_box_callable_and_then_skips_next_on_error() {
    let task = BoxCallable::new(|| Err::<i32, _>(io::Error::other("stop")));

    let mut chained = task.and_then(|_| Ok::<i32, io::Error>(99));

    assert_eq!(
        chained
            .call()
            .expect_err("chained callable should preserve error")
            .to_string(),
        "stop",
    );
}

#[test]
fn test_callable_into_runnable_discards_success_value() {
    let task = BoxCallable::new_with_name("compute", || Ok::<i32, io::Error>(42));

    let mut runnable = task.into_runnable();

    assert_eq!(runnable.name(), Some("compute"));
    runnable.run().expect("runnable should succeed");
}
