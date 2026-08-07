// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BiFunction;
use qubit_function::BoxBinaryFunction;

#[test]
fn test_box_binary_function_alias() {
    let function = BoxBinaryFunction::new(|a: &i32, b: &i32| a + b);
    assert_eq!(function.apply(&20, &22), 42);
}
