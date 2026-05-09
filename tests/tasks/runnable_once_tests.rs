/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

//! Unit tests for RunnableOnce and BoxRunnableOnce.

use std::{
    cell::Cell,
    io,
    rc::Rc,
    sync::{
        Arc,
        atomic::{
            AtomicBool,
            AtomicUsize,
            Ordering,
        },
    },
};

use qubit_function::{
    BoxRunnableOnce,
    CallableOnce,
    LocalBoxRunnableOnce,
    RunnableOnce,
    SupplierOnce,
};

#[derive(Clone)]
struct ClonedRunnableOnce {
    flag: Rc<Cell<bool>>,
}

impl RunnableOnce<io::Error> for ClonedRunnableOnce {
    fn run(self) -> Result<(), io::Error> {
        self.flag.set(true);
        Ok(())
    }
}

struct FlagCallableOnce {
    flag: Rc<Cell<bool>>,
}

impl CallableOnce<i32, io::Error> for FlagCallableOnce {
    fn call(self) -> Result<i32, io::Error> {
        self.flag.set(true);
        Ok(42)
    }
}

#[derive(Clone)]
struct SendClonedRunnableOnce {
    events: Arc<AtomicUsize>,
}

impl RunnableOnce<&'static str> for SendClonedRunnableOnce {
    fn run(self) -> Result<(), &'static str> {
        self.events.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
}

fn assert_send<T: Send>() {}

#[test]
fn test_runnable_once_closure_run_returns_success() {
    let flag = Rc::new(Cell::new(false));
    let captured = Rc::clone(&flag);
    let task = move || {
        captured.set(true);
        Ok::<(), io::Error>(())
    };

    task.run().expect("runnable-once closure should succeed");
    assert!(flag.get());
}

#[test]
fn test_runnable_once_closure_run_returns_error() {
    let task = || Err::<(), _>(io::Error::other("failed"));

    let error = task.run().expect_err("runnable-once closure should fail");
    assert_eq!(error.kind(), io::ErrorKind::Other);
    assert_eq!(error.to_string(), "failed");
}

#[test]
fn test_runnable_once_closure_into_box_executes_once() {
    let task = || Ok::<(), io::Error>(());

    let boxed = RunnableOnce::into_box(task);
    boxed.run().expect("boxed runnable-once should succeed");
}

#[test]
fn test_runnable_once_closure_into_fn_returns_fn_once() {
    let task = || Ok::<(), io::Error>(());
    let function = RunnableOnce::into_fn(task);

    function().expect("runnable-once function should succeed");
}

#[test]
fn test_runnable_once_to_box_clones_runnable() {
    let flag = Rc::new(Cell::new(false));
    let task = ClonedRunnableOnce {
        flag: Rc::clone(&flag),
    };

    let first = task.to_local_box();
    first.run().expect("boxed clone should succeed");
    assert!(flag.get());

    flag.set(false);
    let second = task.to_local_box();
    second
        .run()
        .expect("original runnable should remain reusable");
    assert!(flag.get());
}

#[test]
fn test_runnable_once_to_fn_clones_runnable() {
    let flag = Rc::new(Cell::new(false));
    let task = ClonedRunnableOnce {
        flag: Rc::clone(&flag),
    };

    let function = task.to_fn();
    function().expect("cloned runnable should succeed");
    assert!(flag.get());
}

#[test]
fn test_runnable_once_default_into_callable_returns_unit() {
    let flag = Rc::new(Cell::new(false));
    let task = ClonedRunnableOnce {
        flag: Rc::clone(&flag),
    };

    let callable = RunnableOnce::into_local_callable(task);
    callable.call().expect("unit callable should succeed");
    assert!(flag.get());
}

#[test]
fn test_box_runnable_once_new_and_run() {
    let flag = Arc::new(AtomicBool::new(false));
    let captured = Arc::clone(&flag);
    let task = BoxRunnableOnce::new(move || {
        captured.store(true, Ordering::SeqCst);
        Ok::<(), io::Error>(())
    });

    task.run().expect("box runnable-once should succeed");
    assert!(flag.load(Ordering::SeqCst));
}

#[test]
fn test_box_runnable_once_is_send_task_object() {
    assert_send::<BoxRunnableOnce<io::Error>>();
}

#[test]
fn test_local_box_runnable_once_allows_non_send_capture() {
    let flag = Rc::new(Cell::new(false));
    let captured = Rc::clone(&flag);
    let task = LocalBoxRunnableOnce::new(move || {
        captured.set(true);
        Ok::<(), io::Error>(())
    });

    task.run()
        .expect("local runnable-once should allow local capture");
    assert!(flag.get());
}

#[test]
fn test_box_runnable_once_name_management() {
    let mut task = BoxRunnableOnce::<io::Error>::new_with_name("cleanup", || Ok(()));
    assert_eq!(task.name(), Some("cleanup"));
    assert_eq!(task.to_string(), "BoxRunnableOnce(cleanup)");
    assert!(format!("{task:?}").contains("cleanup"));

    task.set_name("renamed");
    assert_eq!(task.name(), Some("renamed"));

    task.clear_name();
    assert_eq!(task.name(), None);
    assert_eq!(task.to_string(), "BoxRunnableOnce");
}

#[test]
fn test_box_runnable_once_into_box_returns_self() {
    let task = BoxRunnableOnce::new(|| Ok::<(), io::Error>(()));
    let boxed = RunnableOnce::into_box(task);
    boxed
        .run()
        .expect("boxed runnable conversion should succeed");
}

#[test]
fn test_box_runnable_once_into_fn_extracts_function() {
    let task = BoxRunnableOnce::new(|| Ok::<(), io::Error>(()));
    let function = RunnableOnce::into_fn(task);

    function().expect("runnable-once function should succeed");
}

#[test]
fn test_box_runnable_once_implements_supplier_once() {
    let task = BoxRunnableOnce::new(|| Ok::<(), io::Error>(()));

    let result = SupplierOnce::get(task);

    result.expect("supplier once runnable should succeed");
}

#[test]
fn test_box_runnable_once_and_then_runs_next_on_success() {
    let events = Arc::new(AtomicUsize::new(0));
    let first_events = Arc::clone(&events);
    let second_events = Arc::clone(&events);
    let first = BoxRunnableOnce::new(move || {
        first_events.store(1, Ordering::SeqCst);
        Ok::<(), io::Error>(())
    });
    let second = move || {
        second_events.store(2, Ordering::SeqCst);
        Ok::<(), io::Error>(())
    };

    let chained = first.and_then(second);
    chained.run().expect("chained runnable-once should succeed");
    assert_eq!(events.load(Ordering::SeqCst), 2);
}

#[test]
fn test_box_runnable_once_and_then_skips_next_on_error() {
    let events = Arc::new(AtomicUsize::new(0));
    let second_events = Arc::clone(&events);
    let first = BoxRunnableOnce::new(|| Err::<(), _>(io::Error::other("stop")));
    let second = move || {
        second_events.store(2, Ordering::SeqCst);
        Ok::<(), io::Error>(())
    };

    let chained = first.and_then(second);
    assert_eq!(
        chained
            .run()
            .expect_err("chained runnable should preserve error")
            .to_string(),
        "stop",
    );
    assert_eq!(events.load(Ordering::SeqCst), 0);
}

#[test]
fn test_box_runnable_once_combinators_cover_branches_with_same_next_types() {
    let success_flag = Rc::new(Cell::new(false));
    let first = LocalBoxRunnableOnce::new(|| Ok::<(), io::Error>(()));
    let chained = first.and_then(ClonedRunnableOnce {
        flag: Rc::clone(&success_flag),
    });
    chained
        .run()
        .expect("concrete and_then next should run after success");
    assert!(success_flag.get());

    let error_flag = Rc::new(Cell::new(false));
    let first = LocalBoxRunnableOnce::new(|| Err::<(), _>(io::Error::other("stop")));
    let chained = first.and_then(ClonedRunnableOnce {
        flag: Rc::clone(&error_flag),
    });
    assert_eq!(
        chained
            .run()
            .expect_err("concrete and_then next should be skipped")
            .to_string(),
        "stop",
    );
    assert!(!error_flag.get());

    let success_flag = Rc::new(Cell::new(false));
    let first = LocalBoxRunnableOnce::new(|| Ok::<(), io::Error>(()));
    let callable = first.then_callable(FlagCallableOnce {
        flag: Rc::clone(&success_flag),
    });
    assert_eq!(
        callable
            .call()
            .expect("concrete callable should run after success"),
        42
    );
    assert!(success_flag.get());

    let error_flag = Rc::new(Cell::new(false));
    let first = LocalBoxRunnableOnce::new(|| Err::<(), _>(io::Error::other("prepare failed")));
    let callable = first.then_callable(FlagCallableOnce {
        flag: Rc::clone(&error_flag),
    });
    assert_eq!(
        callable
            .call()
            .expect_err("concrete callable should be skipped")
            .to_string(),
        "prepare failed",
    );
    assert!(!error_flag.get());
}

#[test]
fn test_box_runnable_once_then_callable_runs_callable_on_success() {
    let task = BoxRunnableOnce::new_with_name("prepare", || Ok::<(), io::Error>(()));
    let callable = || Ok::<i32, io::Error>(42);

    let chained = task.then_callable(callable);
    assert_eq!(chained.name(), Some("prepare"));
    assert_eq!(chained.call().expect("callable should succeed"), 42);
}

#[test]
fn test_box_runnable_once_then_callable_skips_callable_on_error() {
    let callable_ran = Arc::new(AtomicBool::new(false));
    let callable_ran_capture = Arc::clone(&callable_ran);
    let task = BoxRunnableOnce::<io::Error>::new(|| Err(io::Error::other("prepare failed")));
    let callable = move || {
        callable_ran_capture.store(true, Ordering::SeqCst);
        Ok::<i32, io::Error>(42)
    };

    let chained = task.then_callable(callable);
    let error = chained
        .call()
        .expect_err("then_callable should preserve runnable error");

    assert_eq!(error.to_string(), "prepare failed");
    assert!(!callable_ran.load(Ordering::SeqCst));
}

#[test]
fn test_box_runnable_once_into_callable() {
    let task = BoxRunnableOnce::new_with_name("cleanup", || Ok::<(), io::Error>(()));
    let callable = RunnableOnce::into_callable(task);

    assert_eq!(callable.name(), Some("cleanup"));
    callable.call().expect("unit callable should succeed");
}

#[test]
fn test_box_runnable_once_local_conversions_preserve_name() {
    let task = BoxRunnableOnce::new_with_name("cleanup", || Ok::<(), io::Error>(()));

    let local = RunnableOnce::into_local_box(task);

    assert_eq!(local.name(), Some("cleanup"));
    local.run().expect("local runnable should succeed");

    let task = BoxRunnableOnce::new_with_name("cleanup", || Ok::<(), io::Error>(()));

    let callable = RunnableOnce::into_local_callable(task);

    assert_eq!(callable.name(), Some("cleanup"));
    callable
        .call()
        .expect("local callable conversion should succeed");
}

#[derive(Clone)]
struct TextRunnableOnce {
    events: Rc<Cell<u32>>,
}

impl RunnableOnce<&'static str> for TextRunnableOnce {
    fn run(self) -> Result<(), &'static str> {
        self.events.set(self.events.get() + 1);
        Ok(())
    }
}

#[test]
fn test_runnable_once_default_conversions_with_text_error_type() {
    let events = Rc::new(Cell::new(0));
    let task = TextRunnableOnce {
        events: Rc::clone(&events),
    };

    let boxed = RunnableOnce::into_local_box(task.clone());
    boxed.run().expect("into_box should succeed");

    let function = RunnableOnce::into_fn(task.clone());
    function().expect("into_fn should succeed");

    let boxed_from_ref = task.to_local_box();
    boxed_from_ref.run().expect("to_box should succeed");

    let function_from_ref = task.to_fn();
    function_from_ref().expect("to_fn should succeed");

    let callable = RunnableOnce::into_local_callable(task);
    callable.call().expect("into_callable should succeed");

    assert_eq!(events.get(), 5);
}

#[test]
fn test_send_runnable_once_default_send_conversions_with_text_error_type() {
    let events = Arc::new(AtomicUsize::new(0));
    let task = SendClonedRunnableOnce {
        events: Arc::clone(&events),
    };

    let boxed = task.to_box();
    boxed.run().expect("to_box should succeed");

    let callable = RunnableOnce::into_callable(task);
    callable.call().expect("into_callable should succeed");

    assert_eq!(events.load(Ordering::SeqCst), 2);
}

#[test]
fn test_box_runnable_once_from_supplier_with_text_error_type() {
    let task = BoxRunnableOnce::from_supplier(|| Ok::<(), &'static str>(()));
    task.run().expect("from_supplier should succeed");
}

#[test]
fn test_box_runnable_once_combinators_with_text_error_type() {
    let events = Rc::new(Cell::new(0));
    let first_events = Rc::clone(&events);
    let second_events = Rc::clone(&events);

    let first = LocalBoxRunnableOnce::new(move || {
        first_events.set(first_events.get() + 1);
        Ok::<(), &'static str>(())
    });
    let second = move || {
        second_events.set(second_events.get() + 1);
        Ok::<(), &'static str>(())
    };
    let chained = first.and_then(second);
    chained.run().expect("and_then should succeed");
    assert_eq!(events.get(), 2);

    let runnable = BoxRunnableOnce::new(|| Ok::<(), &'static str>(()));
    let callable = runnable.then_callable(|| Ok::<i32, &'static str>(9));
    assert_eq!(callable.call().expect("then_callable should succeed"), 9);

    let skipped = Rc::new(Cell::new(false));
    let skipped_capture = Rc::clone(&skipped);
    let first = LocalBoxRunnableOnce::new(|| Err::<(), &'static str>("stop"));
    let second = move || {
        skipped_capture.set(true);
        Ok::<(), &'static str>(())
    };
    let chained = first.and_then(second);
    assert_eq!(chained.run().expect_err("and_then should fail"), "stop");
    assert!(!skipped.get());

    let callable_ran = Rc::new(Cell::new(false));
    let callable_ran_capture = Rc::clone(&callable_ran);
    let runnable = LocalBoxRunnableOnce::new(|| Err::<(), &'static str>("prepare"));
    let callable = runnable.then_callable(move || {
        callable_ran_capture.set(true);
        Ok::<i32, &'static str>(9)
    });
    assert_eq!(
        callable
            .call()
            .expect_err("then_callable should preserve runnable error"),
        "prepare"
    );
    assert!(!callable_ran.get());
}
