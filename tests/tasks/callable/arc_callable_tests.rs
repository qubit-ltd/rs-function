// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::cell::Cell;
use std::io::Error;
use std::panic::AssertUnwindSafe;
use std::sync::Arc;
use std::sync::Barrier;
use std::sync::Mutex;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use qubit_function::ArcCallable;
use qubit_function::Callable;

use crate::internal::isolated_test::run_isolated;

#[test]
fn test_arc_callable_invokes_mutable_callback() {
    let mut count = 0;
    let mut callable = ArcCallable::<i32, Error>::new(move || {
        count += 1;
        Ok(count)
    });

    assert_eq!(callable.call().expect("call should succeed"), 1);
    assert_eq!(callable.call().expect("call should succeed"), 2);
}

#[test]
fn test_arc_callable_serializes_shared_state_across_threads() {
    let qualified = concat!(
        module_path!(),
        "::",
        "test_arc_callable_serializes_shared_state_across_threads"
    );
    let (_, exact_name) = qualified.split_once("::").expect("test path includes crate prefix");
    run_isolated(exact_name, || {
        let sequence = Cell::new(0_usize);
        let active = Arc::new(AtomicUsize::new(0));
        let records = Arc::new(Mutex::new(Vec::new()));
        let callback_active = Arc::clone(&active);
        let callback_records = Arc::clone(&records);
        let callable = ArcCallable::new(move || {
            assert_eq!(callback_active.fetch_add(1, Ordering::SeqCst), 0);
            std::thread::yield_now();
            let next = sequence.get() + 1;
            sequence.set(next);
            callback_records.lock().expect("record lock").push(next);
            assert_eq!(callback_active.fetch_sub(1, Ordering::SeqCst), 1);
            Ok::<usize, Error>(next)
        });
        let barrier = Arc::new(Barrier::new(4));
        let workers = (0..4)
            .map(|_| {
                let mut clone = callable.clone();
                let barrier = Arc::clone(&barrier);
                std::thread::spawn(move || {
                    barrier.wait();
                    (0..32)
                        .map(|_| clone.call().expect("call should succeed"))
                        .collect::<Vec<_>>()
                })
            })
            .collect::<Vec<_>>();
        let mut returned = Vec::new();
        for worker in workers {
            returned.extend(worker.join().expect("worker should succeed"));
        }
        let mut actual = records.lock().expect("record lock").clone();
        actual.sort_unstable();
        assert_eq!(actual, (1..=128).collect::<Vec<_>>());
        returned.sort_unstable();
        assert_eq!(returned, (1..=128).collect::<Vec<_>>());
        assert_eq!(active.load(Ordering::SeqCst), 0);
    });
}

#[test]
fn test_arc_callable_recovers_after_panic() {
    let qualified = concat!(module_path!(), "::", "test_arc_callable_recovers_after_panic");
    let (_, exact_name) = qualified.split_once("::").expect("test path includes crate prefix");
    run_isolated(exact_name, || {
        let count = Cell::new(0);
        let mut first = ArcCallable::new(move || {
            let next = count.get() + 1;
            count.set(next);
            assert_ne!(next, 1, "intentional first-call panic");
            Ok::<usize, Error>(next)
        });
        let mut second = first.clone();
        let caught = std::panic::catch_unwind(AssertUnwindSafe(|| first.call()));
        assert!(caught.is_err(), "first callback must panic");
        assert_eq!(second.call().expect("clone remains callable"), 2);
    });
}
