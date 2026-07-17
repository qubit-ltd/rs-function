// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::{
    ArcStatefulBiTransformer,
    ArcStatefulBinaryOperator,
    ArcStatefulTransformer,
    BoxBiPredicate,
    BoxStatefulBiTransformer,
    BoxStatefulBinaryOperator,
    BoxStatefulTransformer,
    RcStatefulBiTransformer,
    RcStatefulBinaryOperator,
    RcStatefulTransformer,
    StatefulBiTransformer,
    StatefulBinaryOperator,
};

#[test]
fn test_box_stateful_bi_transformer_display_with_name() {
    let mut counter = 0;
    let transformer = BoxStatefulBiTransformer::new_with_name(
        "add_counter",
        move |x: i32, y: i32| {
            counter += 1;
            x + y + counter
        },
    );
    let display_str = format!("{}", transformer);
    assert_eq!(display_str, "BoxStatefulBiTransformer(add_counter)");
}

#[test]
fn test_box_stateful_bi_transformer_display_without_name() {
    let mut counter = 0;
    let transformer = BoxStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    });
    let display_str = format!("{}", transformer);
    assert_eq!(display_str, "BoxStatefulBiTransformer");
}

#[test]
fn test_rc_stateful_bi_transformer_display_with_name() {
    let mut counter = 0;
    let transformer = RcStatefulBiTransformer::new_with_name(
        "add_counter",
        move |x: i32, y: i32| {
            counter += 1;
            x + y + counter
        },
    );
    let display_str = format!("{}", transformer);
    assert_eq!(display_str, "RcStatefulBiTransformer(add_counter)");
}

#[test]
fn test_rc_stateful_bi_transformer_display_without_name() {
    let mut counter = 0;
    let transformer = RcStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    });
    let display_str = format!("{}", transformer);
    assert_eq!(display_str, "RcStatefulBiTransformer");
}

#[test]
fn test_arc_stateful_bi_transformer_display_with_name() {
    let mut counter = 0;
    let transformer = ArcStatefulBiTransformer::new_with_name(
        "add_counter",
        move |x: i32, y: i32| {
            counter += 1;
            x + y + counter
        },
    );
    let display_str = format!("{}", transformer);
    assert_eq!(display_str, "ArcStatefulBiTransformer(add_counter)");
}

#[test]
fn test_arc_stateful_bi_transformer_display_without_name() {
    let mut counter = 0;
    let transformer = ArcStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    });
    let display_str = format!("{}", transformer);
    assert_eq!(display_str, "ArcStatefulBiTransformer");
}
