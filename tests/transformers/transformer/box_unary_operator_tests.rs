// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::{
    BoxUnaryOperator,
    Transformer,
};

#[test]
fn test_box_unary_operator_alias() {
    let operator = BoxUnaryOperator::new(|value: i32| value + 1);
    assert_eq!(operator.apply(41), 42);
}
