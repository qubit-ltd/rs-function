// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::{
    BiTransformer,
    BoxBinaryOperator,
};

#[test]
fn test_box_binary_operator_alias() {
    let operator = BoxBinaryOperator::new(|a: i32, b: i32| a + b);
    assert_eq!(operator.apply(20, 22), 42);
}
