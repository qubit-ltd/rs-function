// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::ArcCallableWith;
use qubit_function::CallableWith;

#[test]
fn test_arc_callable_with_mutates_input_and_returns_value() {
    let mut callable = ArcCallableWith::new(|value: &mut i32| {
        *value += 1;
        Ok::<_, std::io::Error>(*value)
    });
    let mut input = 41;

    assert_eq!(callable.call_with(&mut input).expect("call should succeed"), 42);
    assert_eq!(input, 42);
}
