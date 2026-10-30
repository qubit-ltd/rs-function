// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Comprehensive tests for StatefulFunction trait and its implementations

use qubit_function::ArcStatefulFunction;
use qubit_function::BoxStatefulFunction;
use qubit_function::RcStatefulFunction;
use qubit_function::StatefulFunction;

// ============================================================================
// Conditional StatefulFunction Debug and Display Tests
// ============================================================================

#[test]
fn test_box_conditional_stateful_function_debug_display() {
    // Test Debug and Display for BoxConditionalStatefulFunction without name

    let mut double = BoxStatefulFunction::new(move |x: &i32| x * 2);
    // Call apply to test the function
    assert_eq!(double.apply(&5), 10);

    let conditional = double.when(|x: &i32| *x > 0);

    let debug_str = format!("{:?}", conditional);
    assert!(debug_str.contains("BoxConditionalStatefulFunction"), "{debug_str}");
    assert!(debug_str.contains("name"), "{debug_str}");
    assert!(debug_str.contains("function"), "{debug_str}");
    assert!(debug_str.contains("predicate"), "{debug_str}");

    let display_str = format!("{}", conditional);
    assert!(
        display_str.starts_with("BoxConditionalStatefulFunction("),
        "{display_str}"
    );
    assert!(display_str.contains("BoxStatefulFunction"), "{display_str}");
    assert!(display_str.contains("BoxPredicate"), "{display_str}");
    assert!(display_str.ends_with(")"), "{display_str}");

    // Test Debug and Display for BoxConditionalStatefulFunction with name
    let mut named_double = BoxStatefulFunction::new_with_name("stateful_double", |x: &i32| x * 2);
    // Call apply to test the function
    assert_eq!(named_double.apply(&3), 6);

    let named_conditional = named_double.when(|x: &i32| *x % 2 == 0);

    let named_debug_str = format!("{:?}", named_conditional);
    assert!(
        named_debug_str.contains("BoxConditionalStatefulFunction"),
        "{named_debug_str}"
    );
    assert!(named_debug_str.contains("name"), "{named_debug_str}");
    assert!(named_debug_str.contains("function"), "{named_debug_str}");
    assert!(named_debug_str.contains("predicate"), "{named_debug_str}");

    let named_display_str = format!("{}", named_conditional);
    assert!(
        named_display_str.starts_with("BoxConditionalStatefulFunction("),
        "{named_display_str}"
    );
    assert!(
        named_display_str.contains("BoxStatefulFunction(stateful_double)"),
        "{named_display_str}"
    );
    assert!(named_display_str.contains("BoxPredicate"), "{named_display_str}");
    assert!(named_display_str.ends_with(")"), "{named_display_str}");
}

#[test]
fn test_rc_conditional_stateful_function_debug_display() {
    // Test Debug and Display for RcConditionalStatefulFunction without name

    let mut double = RcStatefulFunction::new(move |x: &i32| x * 2);
    // Call apply to test the function
    assert_eq!(double.apply(&5), 10);

    let conditional = double.when(|x: &i32| *x > 0);

    let debug_str = format!("{:?}", conditional);
    assert!(debug_str.contains("RcConditionalStatefulFunction"), "{debug_str}");
    assert!(debug_str.contains("name"), "{debug_str}");
    assert!(debug_str.contains("function"), "{debug_str}");
    assert!(debug_str.contains("predicate"), "{debug_str}");

    let display_str = format!("{}", conditional);
    assert!(
        display_str.starts_with("RcConditionalStatefulFunction("),
        "{display_str}"
    );
    assert!(display_str.contains("RcStatefulFunction"), "{display_str}");
    assert!(display_str.contains("RcPredicate"), "{display_str}");
    assert!(display_str.ends_with(")"), "{display_str}");

    // Test Debug and Display for RcConditionalStatefulFunction with name
    let mut named_double = RcStatefulFunction::new_with_name("rc_stateful_double", |x: &i32| x * 2);
    // Call apply to test the function
    assert_eq!(named_double.apply(&3), 6);

    let named_conditional = named_double.when(|x: &i32| *x % 2 == 0);

    let named_debug_str = format!("{:?}", named_conditional);
    assert!(
        named_debug_str.contains("RcConditionalStatefulFunction"),
        "{named_debug_str}"
    );
    assert!(named_debug_str.contains("name"), "{named_debug_str}");
    assert!(named_debug_str.contains("function"), "{named_debug_str}");
    assert!(named_debug_str.contains("predicate"), "{named_debug_str}");

    let named_display_str = format!("{}", named_conditional);
    assert!(
        named_display_str.starts_with("RcConditionalStatefulFunction("),
        "{named_display_str}"
    );
    assert!(
        named_display_str.contains("RcStatefulFunction(rc_stateful_double)"),
        "{named_display_str}"
    );
    assert!(named_display_str.contains("RcPredicate"), "{named_display_str}");
    assert!(named_display_str.ends_with(")"), "{named_display_str}");
}

#[test]
fn test_arc_conditional_stateful_function_debug_display() {
    // Test Debug and Display for ArcConditionalStatefulFunction without name

    let mut double = ArcStatefulFunction::new(move |x: &i32| x * 2);
    // Call apply to test the function
    assert_eq!(double.apply(&5), 10);

    let conditional = double.when(|x: &i32| *x > 0);

    let debug_str = format!("{:?}", conditional);
    assert!(debug_str.contains("ArcConditionalStatefulFunction"), "{debug_str}");
    assert!(debug_str.contains("name"), "{debug_str}");
    assert!(debug_str.contains("function"), "{debug_str}");
    assert!(debug_str.contains("predicate"), "{debug_str}");

    let display_str = format!("{}", conditional);
    assert!(
        display_str.starts_with("ArcConditionalStatefulFunction("),
        "{display_str}"
    );
    assert!(display_str.contains("ArcStatefulFunction"), "{display_str}");
    assert!(display_str.contains("ArcPredicate"), "{display_str}");
    assert!(display_str.ends_with(")"), "{display_str}");

    // Test Debug and Display for ArcConditionalStatefulFunction with name
    let mut named_double = ArcStatefulFunction::new_with_name("arc_stateful_double", |x: &i32| x * 2);
    // Call apply to test the function
    assert_eq!(named_double.apply(&3), 6);

    let named_conditional = named_double.when(|x: &i32| *x % 2 == 0);

    let named_debug_str = format!("{:?}", named_conditional);
    assert!(
        named_debug_str.contains("ArcConditionalStatefulFunction"),
        "{named_debug_str}"
    );
    assert!(named_debug_str.contains("name"), "{named_debug_str}");
    assert!(named_debug_str.contains("function"), "{named_debug_str}");
    assert!(named_debug_str.contains("predicate"), "{named_debug_str}");

    let named_display_str = format!("{}", named_conditional);
    assert!(
        named_display_str.starts_with("ArcConditionalStatefulFunction("),
        "{named_display_str}"
    );
    assert!(
        named_display_str.contains("ArcStatefulFunction(arc_stateful_double)"),
        "{named_display_str}"
    );
    assert!(named_display_str.contains("ArcPredicate"), "{named_display_str}");
    assert!(named_display_str.ends_with(")"), "{named_display_str}");
}
