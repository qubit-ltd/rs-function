// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::cell::Cell;
use std::panic::AssertUnwindSafe;
use std::sync::Arc;
use std::sync::Barrier;
use std::sync::Mutex;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use qubit_function::ArcCallableWith;
use qubit_function::CallableWith;

use crate::internal::isolated_test::run_isolated;

#[test]
fn test_arc_callable_with_mutates_input_and_returns_value() {
    let mut callable = ArcCallableWith::new(|value: &mut i32| {
        *value += 1;
        Ok::<_, std::io::Error>(*value)
    });
    let mut input = 41;

    assert_eq!(callable.call_with(&mut input).expect("call should succeed"), 42);
    assert_eq!(input, 42);
}

#[test]
fn test_arc_callable_with_serializes_shared_state_across_threads() {
    let qualified = concat!(
        module_path!(),
        "::",
        "test_arc_callable_with_serializes_shared_state_across_threads"
    );
    let (_, exact_name) = qualified.split_once("::").expect("test path includes crate prefix");
    run_isolated(exact_name, || {
        let sequence = Cell::new(0_usize);
        let active = Arc::new(AtomicUsize::new(0));
        let records = Arc::new(Mutex::new(Vec::new()));
        let callback_active = Arc::clone(&active);
        let callback_records = Arc::clone(&records);
        let callable = ArcCallableWith::new(move |input: &mut usize| {
            assert_eq!(callback_active.fetch_add(1, Ordering::SeqCst), 0);
            std::thread::yield_now();
            let next = sequence.get() + 1;
            sequence.set(next);
            *input += 1;
            callback_records.lock().expect("record lock").push(next);
            assert_eq!(callback_active.fetch_sub(1, Ordering::SeqCst), 1);
            Ok::<usize, std::io::Error>(next)
        });
        let barrier = Arc::new(Barrier::new(4));
        let workers = (0..4)
            .map(|_| {
                let mut clone = callable.clone();
                let barrier = Arc::clone(&barrier);
                std::thread::spawn(move || {
                    barrier.wait();
                    let mut input = 0;
                    let returned = (0..32)
                        .map(|_| clone.call_with(&mut input).expect("call should succeed"))
                        .collect::<Vec<_>>();
                    assert_eq!(input, 32);
                    returned
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
fn test_arc_callable_with_recovers_after_panic() {
    let qualified = concat!(module_path!(), "::", "test_arc_callable_with_recovers_after_panic");
    let (_, exact_name) = qualified.split_once("::").expect("test path includes crate prefix");
    run_isolated(exact_name, || {
        let count = Cell::new(0);
        let mut first = ArcCallableWith::new(move |input: &mut usize| {
            *input += 1;
            let next = count.get() + 1;
            count.set(next);
            assert_ne!(next, 1, "intentional first-call panic");
            Ok::<usize, std::io::Error>(next)
        });
        let mut second = first.clone();
        let mut input = 0;
        let caught = std::panic::catch_unwind(AssertUnwindSafe(|| first.call_with(&mut input)));
        assert!(caught.is_err(), "first callback must panic");
        assert_eq!(second.call_with(&mut input).expect("clone remains callable"), 2);
        assert_eq!(input, 2);
    });
}
