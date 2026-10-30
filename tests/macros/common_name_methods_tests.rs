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
fn test_box_function_name_lifecycle_preserves_callback() {
    let mut function = BoxFunction::new(|value: &i32| value + 1).with_name("first");
    assert_eq!(function.name(), Some("first"));
    assert_eq!(function.to_string(), "BoxFunction(first)");
    function.set_name("renamed");
    assert_eq!(function.name(), Some("renamed"));
    assert_eq!(function.to_string(), "BoxFunction(renamed)");
    function.clear_name();
    assert_eq!(function.name(), None);
    assert_eq!(function.to_string(), "BoxFunction");
    assert_eq!(function.apply(&41), 42);
}
