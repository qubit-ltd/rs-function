// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxFunction;
use qubit_function::Function;

#[test]
fn test_function_identity_method_observable_behavior() {
    let function = BoxFunction::<i32, i32>::identity();
    assert_eq!(function.apply(&42), 42);
}
