// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::ArcStatefulTransformer;
use qubit_function::BoxStatefulTransformer;
use qubit_function::RcStatefulTransformer;

#[test]
fn test_box_conditional_stateful_transformer_display() {
    let mut counter = 0;
    let add = BoxStatefulTransformer::new(move |x: i32| {
        counter += 1;
        x + counter
    });
    let conditional = add.when(|x: &i32| *x > 0);
    let display_str = format!("{}", conditional);
    assert!(
        display_str.contains("BoxConditionalStatefulTransformer"),
        "unexpected display output: {display_str}"
    );
}

#[test]
fn test_box_conditional_stateful_transformer_display_no_name() {
    let mut counter = 0;
    let add = BoxStatefulTransformer::new(move |x: i32| {
        counter += 1;
        x + counter
    });
    let conditional = add.when(|x: &i32| *x > 0);
    let display_str = format!("{}", conditional);
    assert_eq!(
        display_str,
        "BoxConditionalStatefulTransformer(BoxStatefulTransformer, BoxPredicate(unnamed))"
    );
}

#[test]
fn test_box_conditional_stateful_transformer_debug() {
    let mut counter = 0;
    let add = BoxStatefulTransformer::new(move |x: i32| {
        counter += 1;
        x + counter
    });
    let conditional = add.when(|x: &i32| *x > 0);
    let debug_str = format!("{:?}", conditional);
    assert!(
        debug_str.contains("BoxConditionalStatefulTransformer"),
        "unexpected debug output: {debug_str}"
    );
}

#[test]
fn test_rc_conditional_stateful_transformer_display() {
    let mut counter = 0;
    let add = RcStatefulTransformer::new(move |x: i32| {
        counter += 1;
        x + counter
    });
    let conditional = add.when(|x: &i32| *x > 0);
    let display_str = format!("{}", conditional);
    assert!(
        display_str.contains("RcConditionalStatefulTransformer"),
        "unexpected display output: {display_str}"
    );
}

#[test]
fn test_rc_conditional_stateful_transformer_display_no_name() {
    let mut counter = 0;
    let add = RcStatefulTransformer::new(move |x: i32| {
        counter += 1;
        x + counter
    });
    let conditional = add.when(|x: &i32| *x > 0);
    let display_str = format!("{}", conditional);
    assert_eq!(
        display_str,
        "RcConditionalStatefulTransformer(RcStatefulTransformer, RcPredicate(unnamed))"
    );
}

#[test]
fn test_rc_conditional_stateful_transformer_debug() {
    let mut counter = 0;
    let add = RcStatefulTransformer::new(move |x: i32| {
        counter += 1;
        x + counter
    });
    let conditional = add.when(|x: &i32| *x > 0);
    let debug_str = format!("{:?}", conditional);
    assert!(
        debug_str.contains("RcConditionalStatefulTransformer"),
        "unexpected debug output: {debug_str}"
    );
}

#[test]
fn test_arc_conditional_stateful_transformer_display() {
    let mut counter = 0;
    let add = ArcStatefulTransformer::new(move |x: i32| {
        counter += 1;
        x + counter
    });
    let conditional = add.when(|x: &i32| *x > 0);
    let display_str = format!("{}", conditional);
    assert!(
        display_str.contains("ArcConditionalStatefulTransformer"),
        "unexpected display output: {display_str}"
    );
}

#[test]
fn test_arc_conditional_stateful_transformer_display_no_name() {
    let mut counter = 0;
    let add = ArcStatefulTransformer::new(move |x: i32| {
        counter += 1;
        x + counter
    });
    let conditional = add.when(|x: &i32| *x > 0);
    let display_str = format!("{}", conditional);
    assert_eq!(
        display_str,
        "ArcConditionalStatefulTransformer(ArcStatefulTransformer, ArcPredicate(unnamed))"
    );
}

#[test]
fn test_arc_conditional_stateful_transformer_debug() {
    let mut counter = 0;
    let add = ArcStatefulTransformer::new(move |x: i32| {
        counter += 1;
        x + counter
    });
    let conditional = add.when(|x: &i32| *x > 0);
    let debug_str = format!("{:?}", conditional);
    assert!(
        debug_str.contains("ArcConditionalStatefulTransformer"),
        "unexpected debug output: {debug_str}"
    );
}
