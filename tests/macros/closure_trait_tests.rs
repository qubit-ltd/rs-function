// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::Function;

#[test]
fn test_function_blanket_impl_accepts_borrowed_capture() {
    let prefix = String::from("value=");
    let callback = |value: &i32| format!("{prefix}{value}");
    assert_eq!(Function::apply(&callback, &42), "value=42");
    assert_eq!(prefix, "value=");
}
