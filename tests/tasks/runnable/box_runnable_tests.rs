// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxRunnable;
use qubit_function::Runnable;

#[test]
fn test_box_runnable_reuses_mutable_callback_state() {
    let mut call_count = 0;
    let mut runnable = BoxRunnable::new(|| {
        call_count += 1;
        Ok::<(), std::io::Error>(())
    });

    assert_eq!(runnable.run(), Ok(()));
    assert_eq!(runnable.run(), Ok(()));
    assert_eq!(call_count, 2, "the callback should run on every invocation");
}
