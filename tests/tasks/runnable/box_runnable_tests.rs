// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use qubit_function::BoxRunnable;
use qubit_function::Runnable;

#[test]
fn test_box_runnable_reuses_mutable_callback_state() {
    let call_count = Arc::new(AtomicUsize::new(0));
    let callback_call_count = Arc::clone(&call_count);
    let mut runnable = BoxRunnable::new(move || {
        callback_call_count.fetch_add(1, Ordering::SeqCst);
        Ok::<(), std::io::Error>(())
    });

    assert!(runnable.run().is_ok());
    assert!(runnable.run().is_ok());
    assert_eq!(
        call_count.load(Ordering::SeqCst),
        2,
        "the callback should run on every invocation"
    );
}
