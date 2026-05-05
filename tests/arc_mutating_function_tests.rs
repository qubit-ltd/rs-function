/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::ArcMutatingFunction;

#[test]
fn test_arc_mutating_function_observable_behavior() {
    let type_name = std::any::type_name::<ArcMutatingFunction<i32, i32>>();
    assert!(type_name.contains("ArcMutatingFunction"), "{type_name}");
}
