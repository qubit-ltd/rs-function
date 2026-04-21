/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! Unit tests for runnable task types.

use std::{
    cell::Cell,
    io,
    rc::Rc,
};

use qubit_function::{
    BoxRunnable,
    Callable,
    Runnable,
    SupplierOnce,
};

#[derive(Clone)]
struct ClonedRunnable {
    flag: Rc<Cell<bool>>,
}

impl Runnable<io::Error> for ClonedRunnable {
    fn run(&mut self) -> Result<(), io::Error> {
        self.flag.set(true);
        Ok(())
    }
}

#[test]
fn test_runnable_closure_run_returns_success() {
    let flag = Rc::new(Cell::new(false));
    let captured = Rc::clone(&flag);
    let mut task = move || {
        captured.set(true);
        Ok::<(), io::Error>(())
    };

    task.run().expect("runnable closure should succeed");
    assert!(flag.get());
}

#[test]
fn test_runnable_closure_run_returns_error() {
    let mut task = || Err::<(), _>(io::Error::other("failed"));

    let error = task.run().expect_err("runnable closure should fail");
    assert_eq!(error.kind(), io::ErrorKind::Other);
    assert_eq!(error.to_string(), "failed");
}

#[test]
fn test_runnable_closure_into_box_executes_once() {
    let flag = Rc::new(Cell::new(false));
    let captured = Rc::clone(&flag);
    let task = move || {
        captured.set(true);
        Ok::<(), io::Error>(())
    };

    let mut boxed = Runnable::into_box(task);

    boxed.run().expect("boxed runnable should succeed");
    assert!(flag.get());
}

#[test]
fn test_runnable_closure_into_fn_returns_fn_once() {
    let task = || Ok::<(), io::Error>(());

    let mut function = Runnable::into_fn(task);

    function().expect("runnable function should succeed");
}

#[test]
fn test_runnable_to_box_clones_runnable() {
    let flag = Rc::new(Cell::new(false));
    let mut task = ClonedRunnable {
        flag: Rc::clone(&flag),
    };

    let mut boxed = task.to_box();

    boxed.run().expect("boxed clone should succeed");
    assert!(flag.get());

    flag.set(false);
    task.run().expect("original runnable should remain usable");
    assert!(flag.get());
}

#[test]
fn test_runnable_to_fn_clones_runnable() {
    let flag = Rc::new(Cell::new(false));
    let mut task = ClonedRunnable {
        flag: Rc::clone(&flag),
    };

    let mut function = task.to_fn();

    function().expect("cloned runnable should succeed");
    assert!(flag.get());

    flag.set(false);
    drop(function);
    task.run().expect("original runnable should remain usable");
    assert!(flag.get());
}

#[test]
fn test_runnable_default_into_callable_returns_unit() {
    let flag = Rc::new(Cell::new(false));
    let task = ClonedRunnable {
        flag: Rc::clone(&flag),
    };

    let mut callable = Runnable::into_callable(task);

    callable.call().expect("default callable should succeed");
    assert!(flag.get());
}

#[test]
fn test_box_runnable_new_and_run() {
    let flag = Rc::new(Cell::new(false));
    let captured = Rc::clone(&flag);
    let mut task = BoxRunnable::new(move || {
        captured.set(true);
        Ok::<(), io::Error>(())
    });

    task.run().expect("box runnable should succeed");
    assert!(flag.get());
}

#[test]
fn test_box_runnable_name_management() {
    let mut task = BoxRunnable::<io::Error>::new_with_name("cleanup", || Ok(()));

    assert_eq!(task.name(), Some("cleanup"));
    assert_eq!(task.to_string(), "BoxRunnable(cleanup)");
    assert!(format!("{task:?}").contains("cleanup"));

    task.set_name("renamed");
    assert_eq!(task.name(), Some("renamed"));

    task.clear_name();
    assert_eq!(task.name(), None);
    assert_eq!(task.to_string(), "BoxRunnable");
}

#[test]
fn test_box_runnable_into_box_returns_self() {
    let task = BoxRunnable::new(|| Ok::<(), io::Error>(()));

    let mut boxed = Runnable::into_box(task);

    boxed.run().expect("boxed runnable should succeed");
}

#[test]
fn test_box_runnable_into_fn_extracts_function() {
    let task = BoxRunnable::new(|| Ok::<(), io::Error>(()));

    let mut function = Runnable::into_fn(task);

    function().expect("runnable function should succeed");
}

#[test]
fn test_box_runnable_from_supplier() {
    let supplier = || Ok::<(), io::Error>(());

    let mut task = BoxRunnable::from_supplier(supplier);

    task.run().expect("supplier-backed runnable should succeed");
}

#[test]
fn test_box_runnable_implements_supplier_once() {
    let task = BoxRunnable::new(|| Ok::<(), io::Error>(()));

    let result = SupplierOnce::get(task);

    result.expect("supplier runnable should succeed");
}

#[test]
fn test_box_runnable_and_then_runs_next_on_success() {
    let events = Rc::new(Cell::new(0));
    let first_events = Rc::clone(&events);
    let second_events = Rc::clone(&events);
    let first = BoxRunnable::new(move || {
        first_events.set(1);
        Ok::<(), io::Error>(())
    });
    let second = move || {
        second_events.set(2);
        Ok::<(), io::Error>(())
    };

    let mut chained = first.and_then(second);

    chained.run().expect("chained runnable should succeed");
    assert_eq!(events.get(), 2);
}

#[test]
fn test_box_runnable_and_then_skips_next_on_error() {
    let events = Rc::new(Cell::new(0));
    let second_events = Rc::clone(&events);
    let first = BoxRunnable::new(|| Err::<(), _>(io::Error::other("stop")));
    let second = move || {
        second_events.set(2);
        Ok::<(), io::Error>(())
    };

    let mut chained = first.and_then(second);

    assert_eq!(
        chained
            .run()
            .expect_err("chained runnable should preserve error")
            .to_string(),
        "stop",
    );
    assert_eq!(events.get(), 0);
}

#[test]
fn test_box_runnable_then_callable_runs_callable_on_success() {
    let task = BoxRunnable::new_with_name("prepare", || Ok::<(), io::Error>(()));
    let callable = || Ok::<i32, io::Error>(42);

    let mut chained = task.then_callable(callable);

    assert_eq!(chained.name(), Some("prepare"));
    assert_eq!(chained.call().expect("callable should succeed"), 42);
}

#[test]
fn test_runnable_into_callable_returns_unit_callable() {
    let task = BoxRunnable::new_with_name("cleanup", || Ok::<(), io::Error>(()));

    let mut callable = task.into_callable();

    assert_eq!(callable.name(), Some("cleanup"));
    callable.call().expect("unit callable should succeed");
}
