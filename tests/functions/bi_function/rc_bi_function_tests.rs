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
fn test_rc_bi_function_applies_shared_callback() {
    let function = RcBiFunction::new(|left: &i32, right: &i32| left - right);
    assert_eq!(function.apply(&9, &4), 5);
}
