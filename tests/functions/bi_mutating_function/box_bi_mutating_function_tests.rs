// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BiMutatingFunction;
use qubit_function::BoxBiMutatingFunction;

#[test]
fn test_box_bi_mutating_function_observable_behavior() {
    let function = BoxBiMutatingFunction::new(|first: &mut i32, second: &mut i32| {
        *first += *second;
        *second *= 2;
        *first + *second
    });
    let mut first = 3;
    let mut second = 4;

    assert_eq!(function.apply(&mut first, &mut second), 15);
    assert_eq!(first, 7);
    assert_eq!(second, 8);
}
