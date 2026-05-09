/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

//! Unit tests for LocalBoxCallableOnce.

use std::{
    io,
    rc::Rc,
};

use qubit_function::{
    CallableOnce,
    LocalBoxCallableOnce,
    RunnableOnce,
    SupplierOnce,
};

#[test]
fn test_local_box_callable_once_new_allows_non_send_capture() {
    let text = Rc::new(String::from("local"));
    let captured = Rc::clone(&text);
    let task = LocalBoxCallableOnce::new(move || Ok::<String, io::Error>(captured.to_string()));

    assert_eq!(
        task.call()
            .expect("local callable-once should allow non-send capture"),
        "local"
    );
}

#[test]
fn test_local_box_callable_once_from_supplier() {
    let text = Rc::new(String::from("supplier"));
    let captured = Rc::clone(&text);
    let supplier = move || Ok::<String, io::Error>(captured.to_string());

    let task = LocalBoxCallableOnce::from_supplier(supplier);

    assert_eq!(
        SupplierOnce::get(task).expect("supplier-backed local callable should succeed"),
        "supplier"
    );
}

#[test]
fn test_local_box_callable_once_map_and_then_support_local_captures() {
    let suffix = Rc::new(String::from("-mapped"));
    let mapped_suffix = Rc::clone(&suffix);
    let task = LocalBoxCallableOnce::new(|| Ok::<String, io::Error>(String::from("local")))
        .map(move |value| format!("{value}{mapped_suffix}"));

    let next_suffix = Rc::clone(&suffix);
    let chained = task.and_then(move |value| Ok(format!("{value}{next_suffix}")));

    assert_eq!(
        chained
            .call()
            .expect("chained local callable should succeed"),
        "local-mapped-mapped"
    );
}

#[test]
fn test_local_box_callable_once_map_err_transforms_local_error() {
    let prefix = Rc::new(String::from("local"));
    let captured = Rc::clone(&prefix);
    let task = LocalBoxCallableOnce::new(|| Err::<i32, _>(io::Error::other("raw")));

    let mapped = task.map_err(move |error| format!("{captured}: {error}"));

    assert_eq!(
        mapped
            .call()
            .expect_err("local map_err should transform error"),
        "local: raw"
    );
}

#[test]
fn test_local_box_callable_once_into_local_box_returns_self() {
    let task = LocalBoxCallableOnce::new(|| Ok::<i32, io::Error>(7));
    let boxed = CallableOnce::into_local_box(task);

    assert_eq!(boxed.call().expect("local boxed callable should run"), 7);
}

#[test]
fn test_local_box_callable_once_into_fn_extracts_function() {
    let text = Rc::new(String::from("local"));
    let captured = Rc::clone(&text);
    let task = LocalBoxCallableOnce::new(move || Ok::<String, io::Error>(captured.to_string()));

    let function = CallableOnce::into_fn(task);

    assert_eq!(
        function().expect("local callable function should run"),
        "local"
    );
}

#[test]
fn test_local_box_callable_once_into_local_runnable_preserves_name() {
    let task = LocalBoxCallableOnce::new_with_name("compute", || Ok::<i32, io::Error>(1));
    let runnable = CallableOnce::into_local_runnable(task);

    assert_eq!(runnable.name(), Some("compute"));
    runnable
        .run()
        .expect("local runnable converted from callable should run");
}
