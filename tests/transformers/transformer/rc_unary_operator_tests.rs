// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::{
    RcUnaryOperator,
    Transformer,
};

#[test]
fn test_rc_unary_operator_alias() {
    let operator = RcUnaryOperator::new(|value: i32| value + 1);
    assert_eq!(operator.apply(41), 42);
}
