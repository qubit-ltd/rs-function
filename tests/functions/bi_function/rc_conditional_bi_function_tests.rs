// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BiFunction;
use qubit_function::RcBiFunction;

#[test]
fn test_rc_conditional_bi_function_selects_callback_by_predicate() {
    let add = RcBiFunction::new(|first: &i32, second: &i32| first + second);
    let multiply = RcBiFunction::new(|first: &i32, second: &i32| first * second);
    let conditional = add.when(|first: &i32, _second: &i32| *first > 0).or_else(multiply);

    assert_eq!(conditional.apply(&5, &3), 8);
    assert_eq!(conditional.apply(&-5, &3), -15);
}
