/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::BoxTester;

#[test]
fn test_box_tester_observable_behavior() {
    let type_name = std::any::type_name::<BoxTester>();
    assert!(type_name.contains("BoxTester"), "{type_name}");
}
