// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Tests for Consumer types

use qubit_function::{
    ArcConsumer,
    BoxConsumer,
    Consumer,
    RcConsumer,
};
use std::rc::Rc;
use std::sync::Arc;

#[cfg(test)]
mod closure_tests {
    use super::{
        Arc,
        Consumer,
    };

    #[test]
    fn test_closure_accept() {
        let closure = |x: &i32| {
            std::hint::black_box(x);
        };
        closure.accept(&5);
    }

    #[test]
    fn test_closure_and_then() {
        let counter = Arc::new(std::sync::Mutex::new(0));
        let c1 = counter.clone();
        let c2 = counter.clone();

        let chained = qubit_function::BoxConsumer::new(move |_x: &i32| {
            *c1.lock().expect("mutex should not be poisoned") += 1;
        })
        .and_then(move |_x: &i32| {
            *c2.lock().expect("mutex should not be poisoned") += 1;
        });

        chained.accept(&5);
        assert_eq!(*counter.lock().expect("mutex should not be poisoned"), 2);
    }

    #[test]
    fn test_closure_and_then_multiple() {
        let counter = Arc::new(std::sync::Mutex::new(0));
        let c1 = counter.clone();
        let c2 = counter.clone();
        let c3 = counter.clone();

        let chained = qubit_function::BoxConsumer::new(move |_x: &i32| {
            *c1.lock().expect("mutex should not be poisoned") += 1;
        })
        .and_then(move |_x: &i32| {
            *c2.lock().expect("mutex should not be poisoned") += 1;
        })
        .and_then(move |_x: &i32| {
            *c3.lock().expect("mutex should not be poisoned") += 1;
        });

        chained.accept(&5);
        assert_eq!(*counter.lock().expect("mutex should not be poisoned"), 3);
    }
}
