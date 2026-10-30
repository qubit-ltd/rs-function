// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::sync::Arc;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;

use qubit_function::ArcFunction;
use qubit_function::Function;

#[test]
fn test_function_clone_shares_callback_state() {
    let total = Arc::new(AtomicI32::new(0));
    let callback_total = Arc::clone(&total);
    let function = ArcFunction::new({
        let calls = AtomicI32::new(0);
        move |value: &i32| {
            callback_total.fetch_add(*value, Ordering::SeqCst);
            calls.fetch_add(1, Ordering::SeqCst) + 1
        }
    });
    let cloned = function.clone();

    assert_eq!(function.apply(&2), 1);
    assert_eq!(cloned.apply(&3), 2);
    assert_eq!(total.load(Ordering::SeqCst), 5);
}
