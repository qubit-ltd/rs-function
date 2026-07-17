// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::{
    RcStatefulBinaryOperator,
    StatefulBiTransformer,
};

#[test]
fn test_rc_stateful_binary_operator_alias() {
    let mut calls = 0;
    let mut operator = RcStatefulBinaryOperator::new(move |a: i32, b: i32| {
        calls += 1;
        a + b + calls - 1
    });
    assert_eq!(operator.apply(20, 22), 42);
}
