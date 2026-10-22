// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::any::type_name;

use qubit_function::BoxFunctionOnce;

#[test]
fn test_box_function_once_observable_behavior() {
    let actual_type_name = type_name::<BoxFunctionOnce<i32, i32>>();
    assert!(actual_type_name.contains("BoxFunctionOnce"), "{actual_type_name}");
}
