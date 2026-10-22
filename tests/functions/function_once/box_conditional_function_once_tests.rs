// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::any::type_name;

use qubit_function::BoxConditionalFunctionOnce;

#[test]
fn test_box_conditional_function_once_observable_behavior() {
    let actual_type_name = type_name::<BoxConditionalFunctionOnce<i32, i32>>();
    assert!(
        actual_type_name.contains("BoxConditionalFunctionOnce"),
        "{actual_type_name}"
    );
}
