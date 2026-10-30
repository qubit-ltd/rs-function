// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxRunnableOnce;
use qubit_function::RunnableOnce;

#[test]
fn test_box_runnable_once_moves_capture_into_callback() {
    let value = String::from("ran");
    let runnable = BoxRunnableOnce::new(move || {
        assert_eq!(value, "ran");
        Ok::<_, std::io::Error>(())
    });

    runnable.run_once().expect("task should succeed");
}
