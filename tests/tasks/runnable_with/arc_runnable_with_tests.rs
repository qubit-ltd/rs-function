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

use qubit_function::ArcRunnableWith;
use qubit_function::RunnableWith;

use crate::internal::isolated_test::run_isolated;

#[test]
fn test_arc_runnable_with_runs_callback_on_mutable_input() {
    let mut runnable = ArcRunnableWith::new(|input: &mut i32| {
        *input += 1;
        Ok::<(), std::io::Error>(())
    });
    let mut input = 0;

    assert!(runnable.run_with(&mut input).is_ok());
    assert_eq!(input, 1);
}

#[test]
fn test_arc_runnable_with_serializes_shared_state_across_threads() {
    let qualified = concat!(
        module_path!(),
        "::",
        "test_arc_runnable_with_serializes_shared_state_across_threads"
    );
    let (_, exact_name) = qualified.split_once("::").expect("test path includes crate prefix");
    run_isolated(exact_name, || {
        let sequence = Cell::new(0_usize);
        let active = Arc::new(AtomicUsize::new(0));
        let records = Arc::new(Mutex::new(Vec::new()));
        let callback_active = Arc::clone(&active);
        let callback_records = Arc::clone(&records);
        let runnable = ArcRunnableWith::new(move |input: &mut usize| {
            assert_eq!(callback_active.fetch_add(1, Ordering::SeqCst), 0);
            std::thread::yield_now();
            let next = sequence.get() + 1;
            sequence.set(next);
            *input += 1;
            callback_records.lock().expect("record lock").push(next);
            assert_eq!(callback_active.fetch_sub(1, Ordering::SeqCst), 1);
            Ok::<(), std::io::Error>(())
        });
        let barrier = Arc::new(Barrier::new(4));
        let workers = (0..4)
            .map(|_| {
                let mut clone = runnable.clone();
                let barrier = Arc::clone(&barrier);
                std::thread::spawn(move || {
                    barrier.wait();
                    let mut input = 0;
                    for _ in 0..32 {
                        clone.run_with(&mut input).expect("run should succeed");
                    }
                    assert_eq!(input, 32);
                })
            })
            .collect::<Vec<_>>();
        for worker in workers {
            worker.join().expect("worker should succeed");
        }
        let mut actual = records.lock().expect("record lock").clone();
        actual.sort_unstable();
        assert_eq!(actual, (1..=128).collect::<Vec<_>>());
        assert_eq!(active.load(Ordering::SeqCst), 0);
    });
}

#[test]
fn test_arc_runnable_with_recovers_after_panic() {
    let qualified = concat!(module_path!(), "::", "test_arc_runnable_with_recovers_after_panic");
    let (_, exact_name) = qualified.split_once("::").expect("test path includes crate prefix");
    run_isolated(exact_name, || {
        let count = Cell::new(0);
        let mirrored_count = Arc::new(AtomicUsize::new(0));
        let callback_mirrored_count = Arc::clone(&mirrored_count);
        let mut first = ArcRunnableWith::new(move |input: &mut usize| {
            *input += 1;
            let next = count.get() + 1;
            count.set(next);
            callback_mirrored_count.store(next, Ordering::SeqCst);
            assert_ne!(next, 1, "intentional first-call panic");
            Ok::<(), std::io::Error>(())
        });
        let mut second = first.clone();
        let mut input = 0;
        let caught = std::panic::catch_unwind(AssertUnwindSafe(|| first.run_with(&mut input)));
        assert!(caught.is_err(), "first callback must panic");
        second.run_with(&mut input).expect("clone remains callable");
        assert_eq!(mirrored_count.load(Ordering::SeqCst), 2);
        assert_eq!(input, 2);
    });
}
