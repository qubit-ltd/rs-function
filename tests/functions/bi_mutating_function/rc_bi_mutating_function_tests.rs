// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::RcBiMutatingFunction;
use qubit_function::BiMutatingFunction;

#[test]
fn test_rc_bi_mutating_function_observable_behavior() {
    let function = RcBiMutatingFunction::new(|first: &mut i32, second: &mut i32| {
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
