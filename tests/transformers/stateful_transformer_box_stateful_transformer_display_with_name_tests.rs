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
fn test_box_stateful_transformer_display_with_name() {
    let mut counter = 0;
    let transformer = BoxStatefulTransformer::new_with_name("counter", move |x: i32| {
        counter += 1;
        x + counter
    });
    let display_str = format!("{}", transformer);
    assert_eq!(display_str, "BoxStatefulTransformer(counter)");
}

#[test]
fn test_box_stateful_transformer_display_without_name() {
    let mut counter = 0;
    let transformer = BoxStatefulTransformer::new(move |x: i32| {
        counter += 1;
        x + counter
    });
    let display_str = format!("{}", transformer);
    assert_eq!(display_str, "BoxStatefulTransformer");
}

#[test]
fn test_rc_stateful_transformer_display_with_name() {
    let mut counter = 0;
    let transformer = RcStatefulTransformer::new_with_name("counter", move |x: i32| {
        counter += 1;
        x + counter
    });
    let display_str = format!("{}", transformer);
    assert_eq!(display_str, "RcStatefulTransformer(counter)");
}

#[test]
fn test_rc_stateful_transformer_display_without_name() {
    let mut counter = 0;
    let transformer = RcStatefulTransformer::new(move |x: i32| {
        counter += 1;
        x + counter
    });
    let display_str = format!("{}", transformer);
    assert_eq!(display_str, "RcStatefulTransformer");
}

#[test]
fn test_arc_stateful_transformer_display_with_name() {
    let mut counter = 0;
    let transformer = ArcStatefulTransformer::new_with_name("counter", move |x: i32| {
        counter += 1;
        x + counter
    });
    let display_str = format!("{}", transformer);
    assert_eq!(display_str, "ArcStatefulTransformer(counter)");
}

#[test]
fn test_arc_stateful_transformer_display_without_name() {
    let mut counter = 0;
    let transformer = ArcStatefulTransformer::new(move |x: i32| {
        counter += 1;
        x + counter
    });
    let display_str = format!("{}", transformer);
    assert_eq!(display_str, "ArcStatefulTransformer");
}
