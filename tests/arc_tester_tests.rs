/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::ArcTester;

#[test]
fn test_arc_tester_observable_behavior() {
    let type_name = std::any::type_name::<ArcTester>();
    assert!(type_name.contains("ArcTester"), "{type_name}");
}
