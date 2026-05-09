/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

//! Unit tests for LocalBoxRunnableOnce.

use std::{
    cell::Cell,
    io,
    rc::Rc,
};

use qubit_function::{
    CallableOnce,
    LocalBoxRunnableOnce,
    RunnableOnce,
    SupplierOnce,
};

#[test]
fn test_local_box_runnable_once_new_allows_non_send_capture() {
    let flag = Rc::new(Cell::new(false));
    let captured = Rc::clone(&flag);
    let task = LocalBoxRunnableOnce::new(move || {
        captured.set(true);
        Ok::<(), io::Error>(())
    });

    task.run()
        .expect("local runnable-once should allow non-send capture");
    assert!(flag.get());
}

#[test]
fn test_local_box_runnable_once_from_supplier() {
    let flag = Rc::new(Cell::new(false));
    let captured = Rc::clone(&flag);
    let supplier = move || {
        captured.set(true);
        Ok::<(), io::Error>(())
    };

    let task = LocalBoxRunnableOnce::from_supplier(supplier);

    SupplierOnce::get(task).expect("supplier-backed local runnable should succeed");
    assert!(flag.get());
}

#[test]
fn test_local_box_runnable_once_and_then_supports_local_next_task() {
    let events = Rc::new(Cell::new(0));
    let first_events = Rc::clone(&events);
    let second_events = Rc::clone(&events);
    let first = LocalBoxRunnableOnce::new(move || {
        first_events.set(1);
        Ok::<(), io::Error>(())
    });
    let second = move || {
        second_events.set(2);
        Ok::<(), io::Error>(())
    };

    let chained = first.and_then(second);

    chained.run().expect("chained local runnable should run");
    assert_eq!(events.get(), 2);
}

#[test]
fn test_local_box_runnable_once_into_local_box_returns_self() {
    let flag = Rc::new(Cell::new(false));
    let captured = Rc::clone(&flag);
    let task = LocalBoxRunnableOnce::new(move || {
        captured.set(true);
        Ok::<(), io::Error>(())
    });

    let local = RunnableOnce::into_local_box(task);

    local.run().expect("local runnable should run");
    assert!(flag.get());
}

#[test]
fn test_local_box_runnable_once_into_fn_extracts_function() {
    let flag = Rc::new(Cell::new(false));
    let captured = Rc::clone(&flag);
    let task = LocalBoxRunnableOnce::new(move || {
        captured.set(true);
        Ok::<(), io::Error>(())
    });

    let function = RunnableOnce::into_fn(task);

    function().expect("local runnable function should run");
    assert!(flag.get());
}

#[test]
fn test_local_box_runnable_once_then_callable_supports_local_callable() {
    let text = Rc::new(String::from("value"));
    let captured = Rc::clone(&text);
    let task = LocalBoxRunnableOnce::new(|| Ok::<(), io::Error>(()));
    let callable = move || Ok::<String, io::Error>(captured.to_string());

    let chained = task.then_callable(callable);

    assert_eq!(
        chained
            .call()
            .expect("local callable should run after runnable"),
        "value"
    );
}

#[test]
fn test_local_box_runnable_once_into_local_callable_preserves_name() {
    let task = LocalBoxRunnableOnce::<io::Error>::new_with_name("cleanup", || Ok(()));
    let callable = RunnableOnce::into_local_callable(task);

    assert_eq!(callable.name(), Some("cleanup"));
    callable
        .call()
        .expect("local callable converted from runnable should run");
}
