/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! Unit tests for callable-with task types.

use std::{
    cell::Cell,
    io,
    rc::Rc,
    sync::{
        Arc,
        atomic::{
            AtomicUsize,
            Ordering,
        },
    },
};

use qubit_function::{
    ArcCallableWith,
    BoxCallableWith,
    CallableWith,
    RcCallableWith,
    RunnableWith,
};

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
        CallableWith::call_with(&mut task, &mut value)
            .expect("callable-with closure should succeed"),
        15
    );
    assert_eq!(value, 15);
}

#[test]
fn test_callable_with_closure_call_with_returns_error() {
    let mut value = 10;
    let mut task = |_input: &mut i32| Err::<i32, _>(io::Error::other("failed"));

    let error = CallableWith::call_with(&mut task, &mut value)
        .expect_err("callable-with closure should fail");

    assert_eq!(error.kind(), io::ErrorKind::Other);
    assert_eq!(error.to_string(), "failed");
    assert_eq!(value, 10);
}

#[test]
fn test_callable_with_closure_into_box_executes_repeatedly() {
    let mut task = CallableWith::into_box(|input: &mut i32| {
        *input += 1;
        Ok::<i32, io::Error>(*input)
    });
    let mut value = 1;

    assert_eq!(
        task.call_with(&mut value)
            .expect("boxed callable-with should succeed"),
        2
    );
    assert_eq!(
        task.call_with(&mut value)
            .expect("boxed callable-with should execute again"),
        3
    );
}

#[test]
fn test_callable_with_closure_into_fn_returns_fn_mut() {
    let task = |input: &mut i32| {
        *input *= 2;
        Ok::<i32, io::Error>(*input)
    };
    let mut function = CallableWith::into_fn(task);
    let mut value = 6;

    assert_eq!(function(&mut value).expect("function should succeed"), 12);
}

#[test]
fn test_callable_with_to_box_clones_callable() {
    let mut value = 10;
    let mut task = AddWith { amount: 3 };
    let mut boxed = task.to_box();

    assert_eq!(
        boxed
            .call_with(&mut value)
            .expect("boxed clone should succeed"),
        13
    );
    assert_eq!(
        task.call_with(&mut value)
            .expect("original callable-with should remain usable"),
        16
    );
}

#[test]
fn test_callable_with_default_into_runnable_discards_success_value() {
    let mut value = 10;
    let task = AddWith { amount: 4 };
    let mut runnable = CallableWith::into_runnable_with(task);

    runnable
        .run_with(&mut value)
        .expect("default runnable-with should succeed");

    assert_eq!(value, 14);
}

#[test]
fn test_box_callable_with_name_management() {
    let mut task =
        BoxCallableWith::<i32, i32, io::Error>::new_with_name("adjust", |input: &mut i32| {
            Ok(*input + 1)
        });

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
    let task = BoxCallableWith::<i32, i32, io::Error>::new(|_input: &mut i32| {
        Err(io::Error::other("original"))
    });
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
fn test_callable_with_to_rc_clones_source() {
    let count = Rc::new(Cell::new(0));
    let mut source = SharedCallableWith {
        count: Rc::clone(&count),
    };
    let mut shared = source.to_rc();
    let mut shared_clone = shared.clone();
    let mut input = 0;

    assert_eq!(
        shared
            .call_with(&mut input)
            .expect("shared callable-with should succeed"),
        1
    );
    assert_eq!(
        shared_clone
            .call_with(&mut input)
            .expect("shared clone should succeed"),
        2
    );
    assert_eq!(
        source
            .call_with(&mut input)
            .expect("original should remain usable"),
        3
    );
    assert_eq!(input, 3);
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
