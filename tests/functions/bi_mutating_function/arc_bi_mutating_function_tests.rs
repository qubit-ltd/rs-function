// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::ArcBiMutatingFunction;
use qubit_function::BiMutatingFunction;

#[test]
fn test_arc_bi_mutating_function_mutates_inputs() {
    let function = ArcBiMutatingFunction::new(|left: &mut i32, right: &mut i32| {
        *left += *right;
        *left
    });
    let (mut left, mut right) = (5, 3);
    assert_eq!(function.apply(&mut left, &mut right), 8);
    assert_eq!((left, right), (8, 3));
}
