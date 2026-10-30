// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Comprehensive tests for FunctionOnce trait and BoxFunctionOnce

use qubit_function::BoxFunctionOnce;
use qubit_function::FunctionOnce;

// ============================================================================
// FunctionOnce Trait Tests - Core Functionality
// ============================================================================

#[test]
fn test_box_function_once_debug_display() {
    // Test Debug and Display for BoxFunctionOnce without name
    let double = BoxFunctionOnce::new(|x: &i32| x * 2);
    let debug_str = format!("{:?}", double);
    assert_eq!(
        debug_str, "BoxFunctionOnce { name: None, function: \"<function>\" }",
        "Debug output should expose the wrapper fields and opaque callback"
    );

    let display_str = format!("{}", double);
    assert_eq!(display_str, "BoxFunctionOnce");

    // Test Debug and Display for BoxFunctionOnce with name
    let named_double = BoxFunctionOnce::new_with_name("double", |x: &i32| x * 2);
    let named_debug_str = format!("{:?}", named_double);
    assert_eq!(
        named_debug_str, "BoxFunctionOnce { name: Some(\"double\"), function: \"<function>\" }",
        "named Debug output should include the wrapper name and opaque callback"
    );

    let named_display_str = format!("{}", named_double);
    assert_eq!(named_display_str, "BoxFunctionOnce(double)");
}

// ============================================================================
// FunctionOnce Name Management Tests
// ============================================================================

#[test]
fn test_box_function_once_name_methods() {
    // Test new_with_name, name(), and set_name()
    let mut double = BoxFunctionOnce::new_with_name("double_func_once", |x: &i32| x * 2);

    // Test name() returns the initial name
    assert_eq!(
        double.name(),
        Some("double_func_once"),
        "new_with_name should preserve the supplied name"
    );

    // Test set_name() changes the name
    double.set_name("modified_double_once");
    assert_eq!(
        double.name(),
        Some("modified_double_once"),
        "set_name should replace the stored name"
    );

    // Test that function still works after name change
    assert_eq!(
        double.apply(&5),
        10,
        "renaming the wrapper should not change its callback behavior"
    );
}

// ============================================================================
// ConditionalFunctionOnce Debug and Display Tests
// ============================================================================

#[test]
fn test_box_conditional_function_once_debug_display() {
    // Test Debug and Display for BoxConditionalFunctionOnce without name
    let double = BoxFunctionOnce::new(|x: &i32| x * 2);
    let conditional = double.when(|x: &i32| *x > 0);

    let debug_str = format!("{:?}", conditional);
    assert_eq!(
        debug_str,
        concat!(
            "BoxConditionalFunctionOnce { function: ",
            "BoxFunctionOnce { name: None, function: \"<function>\" }, ",
            "predicate: BoxPredicate { name: None, function: \"<function>\" } }"
        ),
        "conditional Debug output should expose its function and predicate"
    );

    let display_str = format!("{}", conditional);
    assert_eq!(
        display_str, "BoxConditionalFunctionOnce(BoxFunctionOnce, BoxPredicate(unnamed))",
        "conditional Display output should include both wrapped callbacks"
    );

    // Test Debug and Display for BoxConditionalFunctionOnce with name
    let triple = BoxFunctionOnce::new_with_name("triple_func_once", |x: &i32| x * 3);
    let named_conditional = triple.when(|x: &i32| *x % 2 == 0);

    let named_debug_str = format!("{:?}", named_conditional);
    assert_eq!(
        named_debug_str,
        concat!(
            "BoxConditionalFunctionOnce { function: ",
            "BoxFunctionOnce { name: Some(\"triple_func_once\"), ",
            "function: \"<function>\" }, ",
            "predicate: BoxPredicate { name: None, function: \"<function>\" } }"
        ),
        "named conditional Debug output should preserve the function name and fields"
    );

    let named_display_str = format!("{}", named_conditional);
    assert_eq!(
        named_display_str, "BoxConditionalFunctionOnce(BoxFunctionOnce(triple_func_once), BoxPredicate(unnamed))",
        "named conditional Display output should include both wrapped callbacks"
    );
}
