// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::ArcRunnableWith;
use qubit_function::RunnableWith;

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
