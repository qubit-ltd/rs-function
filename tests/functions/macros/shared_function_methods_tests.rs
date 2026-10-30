// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::ArcFunction;
use qubit_function::Function;

#[test]
fn test_shared_function_methods_observable_behavior() {
    let double = ArcFunction::new(|value: &i32| value * 2);
    let to_string = ArcFunction::new(|value: &i32| value.to_string());
    let chained = double.and_then(to_string);
    assert_eq!(chained.apply(&21), "42");
}
