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

use qubit_function::ArcRunnable;
use qubit_function::Runnable;

use crate::internal::isolated_test::run_isolated;

#[test]
fn test_arc_runnable_clone_shares_callback_state() {
    let call_count = Arc::new(AtomicUsize::new(0));
    let callback_call_count = Arc::clone(&call_count);
    let mut runnable = ArcRunnable::new(move || {
        callback_call_count.fetch_add(1, Ordering::SeqCst);
        Ok::<(), std::io::Error>(())
    });
    let mut cloned_runnable = runnable.clone();

    assert!(runnable.run().is_ok());
    assert!(cloned_runnable.run().is_ok());
    assert_eq!(call_count.load(Ordering::SeqCst), 2);
}

#[test]
fn test_arc_runnable_serializes_shared_state_across_threads() {
    let qualified = concat!(
        module_path!(),
        "::",
        "test_arc_runnable_serializes_shared_state_across_threads"
    );
    let (_, exact_name) = qualified.split_once("::").expect("test path includes crate prefix");
    run_isolated(exact_name, || {
        let sequence = Cell::new(0_usize);
        let active = Arc::new(AtomicUsize::new(0));
        let records = Arc::new(Mutex::new(Vec::new()));
        let callback_active = Arc::clone(&active);
        let callback_records = Arc::clone(&records);
        let runnable = ArcRunnable::new(move || {
            assert_eq!(callback_active.fetch_add(1, Ordering::SeqCst), 0);
            std::thread::yield_now();
            let next = sequence.get() + 1;
            sequence.set(next);
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
                    for _ in 0..32 {
                        clone.run().expect("run should succeed");
                    }
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
fn test_arc_runnable_recovers_after_panic() {
    let qualified = concat!(module_path!(), "::", "test_arc_runnable_recovers_after_panic");
    let (_, exact_name) = qualified.split_once("::").expect("test path includes crate prefix");
    run_isolated(exact_name, || {
        let count = Cell::new(0);
        let mirrored_count = Arc::new(AtomicUsize::new(0));
        let callback_mirrored_count = Arc::clone(&mirrored_count);
        let mut first = ArcRunnable::new(move || {
            let next = count.get() + 1;
            count.set(next);
            callback_mirrored_count.store(next, Ordering::SeqCst);
            assert_ne!(next, 1, "intentional first-call panic");
            Ok::<(), std::io::Error>(())
        });
        let mut second = first.clone();
        let caught = std::panic::catch_unwind(AssertUnwindSafe(|| first.run()));
        assert!(caught.is_err(), "first callback must panic");
        second.run().expect("clone remains callable");
        assert_eq!(mirrored_count.load(Ordering::SeqCst), 2);
    });
}
