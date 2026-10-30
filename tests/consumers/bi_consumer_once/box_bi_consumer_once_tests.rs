// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::TryRecvError;
use std::sync::mpsc::channel;

use qubit_function::BiConsumerOnce;
use qubit_function::BoxBiConsumerOnce;

#[test]
fn test_box_bi_consumer_once_accepts_inputs_once() {
    let output = Arc::new(Mutex::new(Vec::new()));
    let captured = Arc::clone(&output);
    let consumer = BoxBiConsumerOnce::new(move |left: &i32, right: &i32| {
        captured
            .lock()
            .expect("mutex should not be poisoned")
            .push(left + right);
    });
    consumer.accept(&5, &3);
    assert_eq!(*output.lock().expect("mutex should not be poisoned"), vec![8]);
}

#[test]
fn test_true_fn_once_callback_paths() {
    let callback = || {
        let (sender, receiver) = channel();
        let captured = String::from("owned");
        let consume = move |first: &i32, second: &i32| {
            assert_eq!((*first, *second), (1, 2));
            sender.send(captured).expect("receiver remains alive");
        };
        (consume, receiver)
    };

    let (direct, received) = callback();
    BiConsumerOnce::accept(direct, &1, &2);
    assert_eq!(received.recv().expect("direct callback sends capture"), "owned");

    let (source, received) = callback();
    BoxBiConsumerOnce::new(source).accept(&1, &2);
    assert_eq!(received.recv().expect("boxed callback sends capture"), "owned");

    let (first, first_received) = callback();
    let (after, after_received) = callback();
    BoxBiConsumerOnce::new(first).and_then(after).accept(&1, &2);
    assert_eq!(first_received.recv().expect("first callback runs"), "owned");
    assert_eq!(after_received.recv().expect("after callback runs"), "owned");

    for enabled in [true, false] {
        let (first, first_received) = callback();
        let (fallback, fallback_received) = callback();
        BoxBiConsumerOnce::new(first)
            .when(move |_: &i32, _: &i32| enabled)
            .or_else(fallback)
            .accept(&1, &2);
        let (selected, skipped) = if enabled {
            (first_received, fallback_received)
        } else {
            (fallback_received, first_received)
        };
        assert_eq!(selected.recv().expect("selected branch runs"), "owned");
        assert!(matches!(skipped.try_recv(), Err(TryRecvError::Disconnected)));
    }
}
