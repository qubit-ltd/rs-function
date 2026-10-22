// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Verifies cloning and using a conditional two-argument function.

use qubit_function::RcBiFunction;
use qubit_function::BiFunction;
use qubit_function::RcBiPredicate;

/// Confirms a cloned conditional function retains its selected behavior.
#[test]
fn test_three_param_conditional_clone_macro_coverage() {
    let add = RcBiFunction::new(|x: &i32, y: &i32| *x + *y);
    let multiply = RcBiFunction::new(|x: &i32, y: &i32| *x * *y);
    let pred = RcBiPredicate::new(|x: &i32, y: &i32| *x > 0 && *y > 0);

    let conditional = add.when(pred);

    let cloned = conditional.clone();

    let func = cloned.or_else(multiply);

    assert_eq!(func.apply(&3, &4), 7);
}
