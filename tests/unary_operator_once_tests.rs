/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::UnaryOperatorOnce;

#[test]
fn test_unary_operator_once_observable_behavior() {
    fn assert_operator<F: UnaryOperatorOnce<i32>>(_: &F) {}

    let operator = |value: i32| value + 1;
    assert_operator(&operator);
    assert_eq!(operator(41), 42);
}
