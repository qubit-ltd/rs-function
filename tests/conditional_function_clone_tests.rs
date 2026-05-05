/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::{
    BoxFunction,
    Function,
};

#[test]
fn test_conditional_function_clone_observable_behavior() {
    let function = BoxFunction::new(|value: &i32| value + 1);
    assert_eq!(function.apply(&41), 42);
}
