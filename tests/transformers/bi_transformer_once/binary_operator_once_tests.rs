/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::BinaryOperatorOnce;

#[test]
fn test_binary_operator_once_observable_behavior() {
    fn assert_operator<F: BinaryOperatorOnce<i32>>(_: &F) {}

    let operator = |left: i32, right: i32| left + right;
    assert_operator(&operator);
    assert_eq!(operator(20, 22), 42);
}
