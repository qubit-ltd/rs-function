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

use qubit_function::BoxConsumerOnce;
use qubit_function::ConsumerOnce;

#[test]
fn test_box_consumer_once_observable_behavior() {
    let consumed_value = Arc::new(Mutex::new(None));
    let consumer = BoxConsumerOnce::new({
        let consumed_value = Arc::clone(&consumed_value);
        move |value: &i32| {
            *consumed_value.lock().expect("mutex should not be poisoned") = Some(value + 1);
        }
    });

    consumer.accept(&41);

    assert_eq!(
        *consumed_value.lock().expect("mutex should not be poisoned"),
        Some(42),
        "the consumer should invoke its callback with the accepted value"
    );
}

#[test]
fn test_true_fn_once_callback_paths() {
    let callback = || {
        let (sender, receiver) = channel();
        let captured = String::from("owned");
        let consume = move |value: &i32| {
            assert_eq!(*value, 1);
            sender.send(captured).expect("receiver remains alive");
        };
        (consume, receiver)
    };

    let (direct, received) = callback();
    ConsumerOnce::accept(direct, &1);
    assert_eq!(received.recv().expect("direct callback sends capture"), "owned");

    let (source, received) = callback();
    BoxConsumerOnce::new(source).accept(&1);
    assert_eq!(received.recv().expect("boxed callback sends capture"), "owned");

    let (first, first_received) = callback();
    let (after, after_received) = callback();
    BoxConsumerOnce::new(first).and_then(after).accept(&1);
    assert_eq!(first_received.recv().expect("first callback runs"), "owned");
    assert_eq!(after_received.recv().expect("after callback runs"), "owned");

    for enabled in [true, false] {
        let (first, first_received) = callback();
        let (fallback, fallback_received) = callback();
        BoxConsumerOnce::new(first)
            .when(move |_: &i32| enabled)
            .or_else(fallback)
            .accept(&1);
        let (selected, skipped) = if enabled {
            (first_received, fallback_received)
        } else {
            (fallback_received, first_received)
        };
        assert_eq!(selected.recv().expect("selected branch runs"), "owned");
        assert!(matches!(skipped.try_recv(), Err(TryRecvError::Disconnected)));
    }
}
