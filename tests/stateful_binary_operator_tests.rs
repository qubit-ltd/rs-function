/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::StatefulBinaryOperator;

#[test]
fn test_stateful_binary_operator_observable_behavior() {
    fn assert_operator<F: StatefulBinaryOperator<i32>>(_: &F) {}

    let mut calls = 0;
    let mut operator = |left: i32, right: i32| {
        calls += 1;
        left + right + calls
    };
    assert_operator(&operator);
    assert_eq!(operator(20, 21), 42);
}
