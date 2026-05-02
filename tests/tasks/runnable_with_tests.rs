/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
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

#[test]
fn test_runnable_with_default_conversions_cover_all_targets() {
    let mut input = 1;

    let mut boxed = RunnableWith::into_box(AddRunnableWith { amount: 1 });
    boxed.run_with(&mut input).expect("boxed should run");
    assert_eq!(input, 2);

    let mut rc = RunnableWith::into_rc(AddRunnableWith { amount: 2 });
    rc.run_with(&mut input).expect("rc should run");
    assert_eq!(input, 4);

    let mut arc = RunnableWith::into_arc(AddRunnableWith { amount: 3 });
    arc.run_with(&mut input).expect("arc should run");
    assert_eq!(input, 7);

    let mut function = RunnableWith::into_fn(AddRunnableWith { amount: 4 });
    function(&mut input).expect("function should run");
    assert_eq!(input, 11);

    let source = AddRunnableWith { amount: 5 };
    let mut boxed = source.to_box();
    boxed.run_with(&mut input).expect("to_box should run");
    assert_eq!(input, 16);

    let mut rc = source.to_rc();
    rc.run_with(&mut input).expect("to_rc should run");
    assert_eq!(input, 21);

    let mut arc = source.to_arc();
    arc.run_with(&mut input).expect("to_arc should run");
    assert_eq!(input, 26);

    let mut function = source.to_fn();
    function(&mut input).expect("to_fn should run");
    assert_eq!(input, 31);
}

#[test]
fn test_box_runnable_with_conversions_preserve_behavior_and_name() {
    let mut input = 0;
    let boxed = BoxRunnableWith::new_with_name("box", |value: &mut i32| {
        *value += 1;
        Ok::<(), io::Error>(())
    });
    let mut same_box = RunnableWith::into_box(boxed);
    assert_eq!(same_box.name(), Some("box"));
    same_box.run_with(&mut input).expect("box should run");
    assert_eq!(input, 1);

    let boxed = BoxRunnableWith::new_with_name("rc", |value: &mut i32| {
        *value += 2;
        Ok::<(), io::Error>(())
    });
    let mut rc = RunnableWith::into_rc(boxed);
    assert_eq!(rc.name(), Some("rc"));
    rc.run_with(&mut input).expect("rc should run");
    assert_eq!(input, 3);

    let boxed = BoxRunnableWith::new(|value: &mut i32| {
        *value += 3;
        Ok::<(), io::Error>(())
    });
    let mut function = RunnableWith::into_fn(boxed);
    function(&mut input).expect("function should run");
    assert_eq!(input, 6);

    let boxed = BoxRunnableWith::new_with_name("callable", |value: &mut i32| {
        *value += 4;
        Ok::<(), io::Error>(())
    });
    let mut callable = RunnableWith::into_callable_with(boxed);
    assert_eq!(callable.name(), Some("callable"));
    callable
        .call_with(&mut input)
        .expect("callable conversion should run");
    assert_eq!(input, 10);
}

#[test]
fn test_box_runnable_with_into_callable_preserves_error() {
    let mut input = 0;
    let boxed = BoxRunnableWith::<i32, io::Error>::new_with_name("callable", |_value| {
        Err(io::Error::other("callable failed"))
    });

    let mut callable = RunnableWith::into_callable_with(boxed);
    let error = callable
        .call_with(&mut input)
        .expect_err("callable conversion should preserve errors");

    assert_eq!(callable.name(), Some("callable"));
    assert_eq!(error.to_string(), "callable failed");
    assert_eq!(input, 0);
}

#[test]
fn test_rc_runnable_with_conversions_preserve_shared_state() {
    let count = Rc::new(Cell::new(0));
    let captured = Rc::clone(&count);
    let shared = RcRunnableWith::new_with_name("shared", move |input: &mut i32| {
        *input += 1;
        captured.set(captured.get() + 1);
        Ok::<(), io::Error>(())
    });
    let mut input = 0;

    let mut boxed = shared.clone().into_box();
    assert_eq!(boxed.name(), Some("shared"));
    boxed.run_with(&mut input).expect("box should run");

    let mut rc = shared.clone().into_rc();
    assert_eq!(rc.name(), Some("shared"));
    rc.run_with(&mut input).expect("rc should run");

    let mut function = shared.clone().into_fn();
    function(&mut input).expect("function should run");

    let mut boxed = shared.to_box();
    boxed.run_with(&mut input).expect("to_box should run");

    let mut rc = shared.to_rc();
    rc.run_with(&mut input).expect("to_rc should run");

    let mut function = shared.to_fn();
    function(&mut input).expect("to_fn should run");

    assert_eq!(count.get(), 6);
    assert_eq!(input, 6);
}

#[test]
fn test_arc_runnable_with_conversions_preserve_shared_state() {
    let count = Arc::new(AtomicUsize::new(0));
    let captured = Arc::clone(&count);
    let shared = ArcRunnableWith::new_with_name("shared", move |input: &mut i32| {
        *input += 2;
        captured.fetch_add(1, Ordering::SeqCst);
        Ok::<(), io::Error>(())
    });
    let mut input = 0;

    let mut boxed = shared.clone().into_box();
    assert_eq!(boxed.name(), Some("shared"));
    boxed.run_with(&mut input).expect("box should run");

    let mut rc = shared.clone().into_rc();
    assert_eq!(rc.name(), Some("shared"));
    rc.run_with(&mut input).expect("rc should run");

    let mut arc = shared.clone().into_arc();
    assert_eq!(arc.name(), Some("shared"));
    arc.run_with(&mut input).expect("arc should run");

    let mut function = shared.clone().into_fn();
    function(&mut input).expect("function should run");

    let mut boxed = shared.to_box();
    boxed.run_with(&mut input).expect("to_box should run");

    let mut rc = shared.to_rc();
    rc.run_with(&mut input).expect("to_rc should run");

    let mut arc = shared.to_arc();
    arc.run_with(&mut input).expect("to_arc should run");

    let mut function = shared.to_fn();
    function(&mut input).expect("to_fn should run");

    assert_eq!(count.load(Ordering::SeqCst), 8);
    assert_eq!(input, 16);
}

#[test]
fn test_box_runnable_with_combinators_cover_error_branches() {
    let mut input = 0;
    let next_ran = Rc::new(Cell::new(false));
    let next_ran_capture = Rc::clone(&next_ran);
    let mut chained =
        BoxRunnableWith::<i32, io::Error>::new(|_value| Err(io::Error::other("first failed")))
            .and_then(move |value: &mut i32| {
                next_ran_capture.set(true);
                *value += 1;
                Ok::<(), io::Error>(())
            });
    let error = chained
        .run_with(&mut input)
        .expect_err("and_then should short-circuit");
    assert_eq!(error.to_string(), "first failed");
    assert!(!next_ran.get());
    assert_eq!(input, 0);

    let callable_ran = Rc::new(Cell::new(false));
    let callable_ran_capture = Rc::clone(&callable_ran);
    let mut callable =
        BoxRunnableWith::<i32, io::Error>::new(|_value| Err(io::Error::other("prepare failed")))
            .then_callable_with(move |value: &mut i32| {
                callable_ran_capture.set(true);
                Ok::<i32, io::Error>(*value + 1)
            });
    let error = callable
        .call_with(&mut input)
        .expect_err("then_callable_with should short-circuit");
    assert_eq!(error.to_string(), "prepare failed");
    assert!(!callable_ran.get());
    assert_eq!(input, 0);
}
