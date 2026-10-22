// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::any::type_name;
use std::io::Error;

use qubit_function::RcRunnable;

#[test]
fn test_rc_runnable_observable_behavior() {
    let type_name = type_name::<RcRunnable<Error>>();
    assert!(type_name.contains("RcRunnable"), "{type_name}");
}
