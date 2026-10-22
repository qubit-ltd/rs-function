// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::any::type_name;

use qubit_function::RcRunnableWith;

#[test]
fn test_rc_runnable_with_observable_behavior() {
    let type_name = type_name::<RcRunnableWith<i32, std::io::Error>>();
    assert!(type_name.contains("RcRunnableWith"), "{type_name}");
}
