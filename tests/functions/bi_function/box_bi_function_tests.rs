// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BiFunction;
use qubit_function::BoxBiFunction;

#[test]
fn test_box_bi_function_applies_callback_to_both_inputs() {
    let function = BoxBiFunction::new(|first: &i32, second: &i32| first + second);

    assert_eq!(function.apply(&2, &3), 5);
}
