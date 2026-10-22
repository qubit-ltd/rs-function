// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::any::type_name;
use std::io::Error;

use qubit_function::ArcCallable;

#[test]
fn test_arc_callable_observable_behavior() {
    let callable_type_name = type_name::<ArcCallable<i32, Error>>();
    assert!(callable_type_name.contains("ArcCallable"), "{callable_type_name}");
}
