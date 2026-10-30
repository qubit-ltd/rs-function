// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Unit tests for callable-with task types.

use std::cell::Cell;
use std::io;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use qubit_function::ArcCallableWith;
use qubit_function::CallableWith;
use qubit_function::RcCallableWith;

#[derive(Clone)]
struct AddWith {
    amount: i32,
}

impl CallableWith<i32, i32, io::Error> for AddWith {
    fn call_with(&mut self, input: &mut i32) -> Result<i32, io::Error> {
        *input += self.amount;
        Ok(*input)
    }
}

#[derive(Clone)]
struct SharedCallableWith {
    count: Rc<Cell<u32>>,
}

impl CallableWith<i32, u32, io::Error> for SharedCallableWith {
    fn call_with(&mut self, input: &mut i32) -> Result<u32, io::Error> {
        *input += 1;
        self.count.set(self.count.get() + 1);
        Ok(self.count.get())
    }
}

#[test]
fn test_callable_with_closure_call_with_returns_success_value() {
    let mut value = 10;
    let mut task = |input: &mut i32| {
        *input += 5;
        Ok::<i32, io::Error>(*input)
    };

    assert_eq!(
        CallableWith::call_with(&mut task, &mut value).expect("callable-with closure should succeed"),
        15
    );
    assert_eq!(value, 15);
}

#[test]
fn test_callable_with_closure_call_with_returns_error() {
    let mut value = 10;
    let mut task = |_input: &mut i32| Err::<i32, _>(io::Error::other("failed"));

    let error = CallableWith::call_with(&mut task, &mut value).expect_err("callable-with closure should fail");

    assert_eq!(error.kind(), io::ErrorKind::Other);
    assert_eq!(error.to_string(), "failed");
    assert_eq!(value, 10);
}

#[test]
fn test_rc_callable_with_shares_state_between_clones() {
    let count = Rc::new(Cell::new(0));
    let captured = Rc::clone(&count);
    let mut shared = RcCallableWith::new(move |input: &mut i32| {
        *input += 1;
        captured.set(captured.get() + 1);
        Ok::<u32, io::Error>(captured.get())
    });
    let mut clone = shared.clone();
    let mut input = 0;

    assert_eq!(shared.call_with(&mut input).expect("first call"), 1);
    assert_eq!(clone.call_with(&mut input).expect("second call"), 2);
    assert_eq!(count.get(), 2);
    assert_eq!(input, 2);
}

#[test]
fn test_arc_callable_with_shares_state_between_clones() {
    let count = Arc::new(AtomicUsize::new(0));
    let captured = Arc::clone(&count);
    let mut shared = ArcCallableWith::new(move |input: &mut i32| {
        *input += 2;
        let value = captured.fetch_add(1, Ordering::SeqCst) + 1;
        Ok::<usize, io::Error>(value)
    });
    let mut clone = shared.clone();
    let mut input = 0;

    assert_eq!(shared.call_with(&mut input).expect("first call"), 1);
    assert_eq!(clone.call_with(&mut input).expect("second call"), 2);
    assert_eq!(count.load(Ordering::SeqCst), 2);
    assert_eq!(input, 4);
}
