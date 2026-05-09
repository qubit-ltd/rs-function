/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

//! Unit tests for callable task types.

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
    ArcCallable,
    BoxCallable,
    BoxCallableOnce,
    Callable,
    CallableOnce,
    RcCallable,
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

#[derive(Clone)]
struct SharedCallable {
    count: Rc<Cell<u32>>,
}

impl Callable<u32, io::Error> for SharedCallable {
    fn call(&mut self) -> Result<u32, io::Error> {
        self.count.set(self.count.get() + 1);
        Ok(self.count.get())
    }
}

#[derive(Clone)]
struct SharedCallableForArc {
    count: Arc<AtomicUsize>,
}

impl Callable<u32, io::Error> for SharedCallableForArc {
    fn call(&mut self) -> Result<u32, io::Error> {
        let value = self.count.fetch_add(1, Ordering::SeqCst) + 1;
        Ok(value as u32)
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
fn test_box_callable_into_local_once_preserves_name() {
    let task = BoxCallable::new_with_name("compute", || Ok::<i32, io::Error>(9));

    let once = Callable::into_local_once(task);

    assert_eq!(once.name(), Some("compute"));
    assert_eq!(once.call().expect("local once should succeed"), 9);
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

#[test]
fn test_callable_into_rc_preserves_state_and_name() {
    let shared = SharedCallable {
        count: Rc::new(Cell::new(0)),
    };
    let shared = BoxCallable::new_with_name("count", move || {
        let mut current = shared.count.get();
        current += 1;
        shared.count.set(current);
        Ok::<i32, io::Error>(current as i32)
    });
    let mut shared = Callable::into_rc(shared);
    assert_eq!(shared.name(), Some("count"));

    let value = shared
        .call()
        .expect("into_rc should execute first call through shared closure");
    let value2 = shared
        .clone()
        .call()
        .expect("into_rc clone should execute second call");
    assert_eq!(value2, 2);
    assert_eq!(value, 1);
}

#[test]
fn test_callable_to_rc_reuses_source_via_clone() {
    let count = Rc::new(Cell::new(0));
    let mut source = SharedCallable {
        count: Rc::clone(&count),
    };
    let mut shared = source.to_rc();
    let mut shared_clone = shared.clone();

    shared.call().expect("to_rc should execute");
    shared_clone.call().expect("to_rc clone should execute");
    source.call().expect("original source should remain usable");

    assert_eq!(count.get(), 3);
}

#[test]
fn test_callable_into_arc_preserves_state() {
    let task = SharedCallableForArc {
        count: Arc::new(AtomicUsize::new(0)),
    };
    let mut shared = Callable::into_arc(task);

    assert_eq!(shared.call().expect("first into_arc call"), 1);
    assert_eq!(shared.call().expect("second into_arc call"), 2);
}

#[test]
fn test_callable_to_arc_reuses_source_via_clone() {
    let count = Arc::new(AtomicUsize::new(0));
    let mut source = SharedCallableForArc {
        count: Arc::clone(&count),
    };
    let mut shared = source.to_arc();
    let mut shared_clone = shared.clone();

    shared.call().expect("to_arc should execute");
    shared_clone.call().expect("to_arc clone should execute");
    source.call().expect("original source should remain usable");

    assert_eq!(count.load(Ordering::SeqCst), 3);
}

#[test]
fn test_arc_callable_local_once_conversions_preserve_name() {
    let task = ArcCallable::new_with_name("shared", || Ok::<i32, io::Error>(31));

    let once = Callable::into_local_once(task.clone());

    assert_eq!(once.name(), Some("shared"));
    assert_eq!(once.call().expect("arc local once should succeed"), 31);

    let once = task.to_local_once();

    assert_eq!(once.name(), Some("shared"));
    assert_eq!(once.call().expect("arc local clone should succeed"), 31);
}

#[test]
fn test_rc_callable_local_once_conversions_preserve_name() {
    let task = RcCallable::new_with_name("shared", || Ok::<i32, io::Error>(37));

    let once = Callable::into_local_once(task.clone());

    assert_eq!(once.name(), Some("shared"));
    assert_eq!(once.call().expect("rc local once should succeed"), 37);

    let once = task.to_local_once();

    assert_eq!(once.name(), Some("shared"));
    assert_eq!(once.call().expect("rc local clone should succeed"), 37);
}

#[test]
fn test_callable_into_once_from_reusable_callable() {
    let count = Rc::new(Cell::new(0));
    let count_clone = Rc::clone(&count);
    let once = Callable::into_local_once(move || {
        let mut state = count_clone.get() as i32;
        state += 1;
        count_clone.set(state as u32);
        Ok::<i32, io::Error>(state)
    });

    assert_eq!(once.call().expect("into_once should execute"), 1);
    assert_eq!(count.get(), 1);
}

#[test]
fn test_callable_to_once_produces_repeatable_once_callables() {
    let task = SharedCallable {
        count: Rc::new(Cell::new(0)),
    };
    let first = task.to_local_once();
    let second = task.to_local_once();

    assert_eq!(first.call().expect("first once should execute"), 1);
    assert_eq!(second.call().expect("second once should execute"), 2);
}

#[test]
fn test_box_callable_into_rc() {
    let task = BoxCallable::new(|| Ok::<i32, io::Error>(21));
    let mut rc_task = task.into_rc();

    assert_eq!(rc_task.call().expect("rc callable should succeed"), 21);
    assert_eq!(
        rc_task
            .call()
            .expect("rc callable clone should reuse state"),
        21
    );
}

#[test]
fn test_rc_callable_from_supplier() {
    let count = Rc::new(Cell::new(0));
    let captured = Rc::clone(&count);
    let mut task = RcCallable::from_supplier(move || {
        captured.set(captured.get() + 1);
        Ok::<i32, io::Error>(captured.get())
    });

    assert_eq!(task.call().expect("rc supplier should execute"), 1);
    assert_eq!(task.call().expect("rc supplier should execute again"), 2);
    assert_eq!(count.get(), 2);
}

#[test]
fn test_arc_callable_from_supplier() {
    let count = Arc::new(AtomicUsize::new(0));
    let captured = Arc::clone(&count);
    let mut task = ArcCallable::from_supplier(move || {
        captured.fetch_add(1, Ordering::SeqCst);
        Ok::<i32, io::Error>(captured.load(Ordering::SeqCst) as i32)
    });

    assert_eq!(task.call().expect("arc supplier should execute"), 1);
    assert_eq!(task.call().expect("arc supplier should execute again"), 2);
    assert_eq!(count.load(Ordering::SeqCst), 2);
}

#[derive(Clone)]
struct TextCallable {
    value: String,
}

impl Callable<String, &'static str> for TextCallable {
    fn call(&mut self) -> Result<String, &'static str> {
        Ok(self.value.clone())
    }
}

#[test]
fn test_callable_default_conversions_with_text_error_type() {
    let task = TextCallable {
        value: "payload".to_string(),
    };

    let mut boxed = Callable::into_box(task.clone());
    assert_eq!(
        boxed.call().expect("boxed conversion should succeed"),
        "payload"
    );

    let mut shared_rc = Callable::into_rc(task.clone());
    assert_eq!(
        shared_rc.call().expect("rc conversion should succeed"),
        "payload",
    );

    let mut shared_arc = Callable::into_arc(task.clone());
    assert_eq!(
        shared_arc.call().expect("arc conversion should succeed"),
        "payload",
    );

    let mut function = Callable::into_fn(task.clone());
    assert_eq!(function().expect("fn conversion should succeed"), "payload",);

    let once = Callable::into_once(task.clone());
    assert_eq!(
        once.call().expect("once conversion should succeed"),
        "payload"
    );

    let once_from_ref = task.to_once();
    assert_eq!(
        once_from_ref.call().expect("to_once should succeed"),
        "payload"
    );

    let mut runnable = Callable::into_runnable(task);
    runnable.run().expect("runnable conversion should succeed");
}

#[test]
fn test_box_callable_combinators_with_text_error_type() {
    let mut mapped = BoxCallable::new(|| Ok::<i32, &'static str>(5)).map(|v| v + 7);
    assert_eq!(mapped.call().expect("map should succeed"), 12);

    let mut mapped_err = BoxCallable::new(|| Err::<i32, _>("raw")).map_err(|e| format!("E:{e}"));
    assert_eq!(
        mapped_err
            .call()
            .expect_err("map_err should transform error"),
        "E:raw",
    );

    let mut chained = BoxCallable::new(|| Ok::<i32, &'static str>(3))
        .and_then(|v| Ok::<i32, &'static str>(v * 4));
    assert_eq!(chained.call().expect("and_then should succeed"), 12);
}

#[test]
fn test_rc_and_arc_callable_into_runnable_keep_error_type() {
    let rc_task = RcCallable::from_supplier(|| Ok::<i32, &'static str>(1));
    let mut rc_runnable = Callable::into_runnable(rc_task);
    rc_runnable.run().expect("rc runnable should succeed");

    let arc_task = ArcCallable::from_supplier(|| Ok::<i32, &'static str>(1));
    let mut arc_runnable = Callable::into_runnable(arc_task);
    arc_runnable.run().expect("arc runnable should succeed");
}
