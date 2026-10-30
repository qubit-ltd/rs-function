// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BiFunctionOnce;
use qubit_function::BoxBiFunctionOnce;

#[test]
fn test_box_bi_function_once_invokes_owned_callback() {
    let function = BoxBiFunctionOnce::new(|left: &i32, right: &i32| left + right);
    assert_eq!(function.apply(&6, &4), 10);
}
