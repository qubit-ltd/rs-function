// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxFunctionOnce;
use qubit_function::FunctionOnce;

#[test]
fn test_closure_once_trait_moves_captured_value() {
    let captured = String::from("moved");
    let function = BoxFunctionOnce::new(move |_: &()| captured);
    assert_eq!(function.apply(&()), "moved");
}
