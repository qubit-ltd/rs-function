// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

#[cfg(feature = "stateful")]
use qubit_function::ArcStatefulTransformer;
use qubit_function::ArcTransformer;
#[cfg(feature = "stateful")]
use qubit_function::BoxStatefulTransformer;
use qubit_function::BoxTransformer;
#[cfg(feature = "once")]
use qubit_function::BoxTransformerOnce;
#[cfg(all(feature = "rc", feature = "stateful"))]
use qubit_function::RcStatefulTransformer;
#[cfg(feature = "rc")]
use qubit_function::RcTransformer;
#[cfg(feature = "stateful")]
use qubit_function::StatefulTransformer;
use qubit_function::Transformer;
#[cfg(feature = "once")]
use qubit_function::TransformerOnce;

#[test]
fn test_box_transformer_identity_infers_type_from_result() {
    let identity: BoxTransformer<String, String> = BoxTransformer::identity();
    assert_eq!(identity.apply(String::from("item")), "item");

    let _: BoxTransformer<i32, i32> = BoxTransformer::<i32, i32>::identity();
}

#[test]
fn test_arc_transformer_identity_infers_type_from_result() {
    let identity: ArcTransformer<String, String> = ArcTransformer::identity();
    assert_eq!(identity.apply(String::from("item")), "item");

    let _: ArcTransformer<i32, i32> = ArcTransformer::<i32, i32>::identity();
}

#[test]
fn test_box_transformer_identity_moves_non_clone_input() {
    struct Token(u32);

    let identity: BoxTransformer<Token, Token> = BoxTransformer::identity();
    assert_eq!(identity.apply(Token(7)).0, 7);
}

#[cfg(feature = "rc")]
#[test]
fn test_rc_transformer_identity_infers_type_from_result() {
    let identity: RcTransformer<String, String> = RcTransformer::identity();
    assert_eq!(identity.apply(String::from("item")), "item");

    let _: RcTransformer<i32, i32> = RcTransformer::<i32, i32>::identity();
}

#[cfg(feature = "stateful")]
#[test]
fn test_box_stateful_transformer_identity_infers_type_from_result() {
    let mut identity: BoxStatefulTransformer<String, String> = BoxStatefulTransformer::identity();
    assert_eq!(identity.apply(String::from("item")), "item");

    let _: BoxStatefulTransformer<i32, i32> = BoxStatefulTransformer::<i32, i32>::identity();
}

#[cfg(feature = "stateful")]
#[test]
fn test_arc_stateful_transformer_identity_infers_type_from_result() {
    let mut identity: ArcStatefulTransformer<String, String> = ArcStatefulTransformer::identity();
    assert_eq!(identity.apply(String::from("item")), "item");

    let _: ArcStatefulTransformer<i32, i32> = ArcStatefulTransformer::<i32, i32>::identity();
}

#[cfg(all(feature = "rc", feature = "stateful"))]
#[test]
fn test_rc_stateful_transformer_identity_infers_type_from_result() {
    let mut identity: RcStatefulTransformer<String, String> = RcStatefulTransformer::identity();
    assert_eq!(identity.apply(String::from("item")), "item");

    let _: RcStatefulTransformer<i32, i32> = RcStatefulTransformer::<i32, i32>::identity();
}

#[cfg(feature = "once")]
#[test]
fn test_box_transformer_once_identity_infers_type_from_result() {
    let identity: BoxTransformerOnce<String, String> = BoxTransformerOnce::identity();
    assert_eq!(identity.apply(String::from("item")), "item");

    let _: BoxTransformerOnce<i32, i32> = BoxTransformerOnce::<i32, i32>::identity();
}
