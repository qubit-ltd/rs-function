// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxFunctionOnce;
use qubit_function::FunctionOnce;

#[test]
fn test_box_function_once_invokes_owned_callback() {
    let function = BoxFunctionOnce::new(|value: &i32| value * 2);
    assert_eq!(function.apply(&6), 12);
}
