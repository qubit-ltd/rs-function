// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::io::Error;

use qubit_function::RcRunnable;
use qubit_function::Runnable;

#[test]
fn test_rc_runnable_executes_callback() {
    let count = std::rc::Rc::new(std::cell::Cell::new(0));
    let captured_count = std::rc::Rc::clone(&count);
    let mut runnable = RcRunnable::<Error>::new(move || {
        captured_count.set(captured_count.get() + 1);
        Ok(())
    });

    runnable.run().expect("task should succeed");
    assert_eq!(count.get(), 1);
}
