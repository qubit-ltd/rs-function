/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! Unit tests for runnable-with task types.

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
    ArcRunnableWith,
    BoxRunnableWith,
    CallableWith,
    RcRunnableWith,
    RunnableWith,
};

#[derive(Clone)]
struct AddRunnableWith {
    amount: i32,
}

impl RunnableWith<i32, io::Error> for AddRunnableWith {
    fn run_with(&mut self, input: &mut i32) -> Result<(), io::Error> {
        *input += self.amount;
        Ok(())
    }
}

#[test]
fn test_runnable_with_closure_run_with_returns_success() {
    let mut input = 10;
    let mut task = |value: &mut i32| {
        *value += 5;
        Ok::<(), io::Error>(())
    };

    RunnableWith::run_with(&mut task, &mut input).expect("runnable-with closure should succeed");

    assert_eq!(input, 15);
}

#[test]
fn test_runnable_with_closure_run_with_returns_error() {
    let mut input = 10;
    let mut task = |_value: &mut i32| Err::<(), _>(io::Error::other("failed"));

    let error = RunnableWith::run_with(&mut task, &mut input)
        .expect_err("runnable-with closure should fail");

    assert_eq!(error.kind(), io::ErrorKind::Other);
    assert_eq!(error.to_string(), "failed");
    assert_eq!(input, 10);
}

#[test]
fn test_runnable_with_closure_into_box_executes_repeatedly() {
    let mut task = RunnableWith::into_box(|input: &mut i32| {
        *input += 1;
        Ok::<(), io::Error>(())
    });
    let mut input = 1;

    task.run_with(&mut input)
        .expect("boxed runnable-with should succeed");
    task.run_with(&mut input)
        .expect("boxed runnable-with should execute again");

    assert_eq!(input, 3);
}

#[test]
fn test_runnable_with_closure_into_fn_returns_fn_mut() {
    let task = |input: &mut i32| {
        *input *= 2;
        Ok::<(), io::Error>(())
    };
    let mut function = RunnableWith::into_fn(task);
    let mut input = 6;

    function(&mut input).expect("function should succeed");

    assert_eq!(input, 12);
}

#[test]
fn test_runnable_with_to_box_clones_runnable() {
    let mut input = 10;
    let mut task = AddRunnableWith { amount: 3 };
    let mut boxed = task.to_box();

    boxed
        .run_with(&mut input)
        .expect("boxed clone should succeed");
    task.run_with(&mut input)
        .expect("original runnable-with should remain usable");

    assert_eq!(input, 16);
}

#[test]
fn test_runnable_with_default_into_callable_returns_unit() {
    let mut input = 10;
    let task = AddRunnableWith { amount: 4 };
    let mut callable = RunnableWith::into_callable_with(task);

    callable
        .call_with(&mut input)
        .expect("default callable-with should succeed");

    assert_eq!(input, 14);
}

#[test]
fn test_box_runnable_with_name_management() {
    let mut task = BoxRunnableWith::<i32, io::Error>::new_with_name("adjust", |input: &mut i32| {
        *input += 1;
        Ok(())
    });

    assert_eq!(task.name(), Some("adjust"));
    assert_eq!(task.to_string(), "BoxRunnableWith(adjust)");
    assert!(format!("{task:?}").contains("adjust"));

    task.set_name("renamed");
    assert_eq!(task.name(), Some("renamed"));

    task.clear_name();
    assert_eq!(task.name(), None);
    assert_eq!(task.to_string(), "BoxRunnableWith");
}

#[test]
fn test_box_runnable_with_and_then_runs_in_order() {
    let first = BoxRunnableWith::new(|input: &mut i32| {
        *input += 2;
        Ok::<(), io::Error>(())
    });
    let mut chained = first.and_then(|input: &mut i32| {
        *input *= 3;
        Ok::<(), io::Error>(())
    });
    let mut input = 4;

    chained
        .run_with(&mut input)
        .expect("chained runnable-with should succeed");

    assert_eq!(input, 18);
}

#[test]
fn test_box_runnable_with_then_callable_runs_after_success() {
    let first = BoxRunnableWith::new_with_name("prepare", |input: &mut i32| {
        *input += 2;
        Ok::<(), io::Error>(())
    });
    let mut callable = first.then_callable_with(|input: &mut i32| {
        *input *= 2;
        Ok::<i32, io::Error>(*input)
    });
    let mut input = 5;

    assert_eq!(callable.name(), Some("prepare"));
    assert_eq!(
        callable
            .call_with(&mut input)
            .expect("callable-with should succeed"),
        14
    );
    assert_eq!(input, 14);
}

#[test]
fn test_rc_runnable_with_shares_state_between_clones() {
    let count = Rc::new(Cell::new(0));
    let captured = Rc::clone(&count);
    let mut shared = RcRunnableWith::new(move |input: &mut i32| {
        *input += 1;
        captured.set(captured.get() + 1);
        Ok::<(), io::Error>(())
    });
    let mut clone = shared.clone();
    let mut input = 0;

    shared.run_with(&mut input).expect("first call");
    clone.run_with(&mut input).expect("second call");

    assert_eq!(count.get(), 2);
    assert_eq!(input, 2);
}

#[test]
fn test_runnable_with_to_rc_clones_source() {
    let mut source = AddRunnableWith { amount: 2 };
    let mut shared = source.to_rc();
    let mut clone = shared.clone();
    let mut input = 0;

    shared
        .run_with(&mut input)
        .expect("shared runnable-with should succeed");
    clone
        .run_with(&mut input)
        .expect("shared clone should succeed");
    source
        .run_with(&mut input)
        .expect("original should remain usable");

    assert_eq!(input, 6);
}

#[test]
fn test_arc_runnable_with_shares_state_between_clones() {
    let count = Arc::new(AtomicUsize::new(0));
    let captured = Arc::clone(&count);
    let mut shared = ArcRunnableWith::new(move |input: &mut i32| {
        *input += 2;
        captured.fetch_add(1, Ordering::SeqCst);
        Ok::<(), io::Error>(())
    });
    let mut clone = shared.clone();
    let mut input = 0;

    shared.run_with(&mut input).expect("first call");
    clone.run_with(&mut input).expect("second call");

    assert_eq!(count.load(Ordering::SeqCst), 2);
    assert_eq!(input, 4);
}
