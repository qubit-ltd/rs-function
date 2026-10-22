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
fn test_function_constant_method_returns_constant_value() {
    let function = BoxFunction::constant(42);
    assert_eq!(function.apply(&0), 42);
    assert_eq!(function.apply(&100), 42);
}
