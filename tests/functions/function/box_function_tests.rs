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
fn test_box_function_applies_callback_repeatedly() {
    let add_one = BoxFunction::new(|value: &i32| value + 1);

    assert_eq!(add_one.apply(&1), 2);
    assert_eq!(add_one.apply(&2), 3);
}
