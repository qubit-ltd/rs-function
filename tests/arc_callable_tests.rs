/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::ArcCallable;

#[test]
fn test_arc_callable_observable_behavior() {
    let type_name = std::any::type_name::<ArcCallable<i32, std::io::Error>>();
    assert!(type_name.contains("ArcCallable"), "{type_name}");
}
