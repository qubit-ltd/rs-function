// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxUnaryOperatorOnce;
use qubit_function::TransformerOnce;

#[test]
fn test_box_unary_operator_once_alias() {
    let operator = BoxUnaryOperatorOnce::new(|value: i32| value + 1);
    assert_eq!(operator.apply(41), 42);
}
