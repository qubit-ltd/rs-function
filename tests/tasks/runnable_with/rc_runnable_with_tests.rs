// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::RcRunnableWith;
use qubit_function::RunnableWith;

#[test]
fn test_rc_runnable_with_mutates_input() {
    let mut runnable = RcRunnableWith::new(|value: &mut i32| {
        *value += 1;
        Ok::<_, std::io::Error>(())
    });
    let mut input = 41;

    runnable.run_with(&mut input).expect("task should succeed");
    assert_eq!(input, 42);
}
