// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::ArcBiFunction;
use qubit_function::BiFunction;

#[test]
fn test_arc_bi_function_applies_shared_callback() {
    let function = ArcBiFunction::new(|left: &i32, right: &i32| left - right);
    let cloned = function.clone();
    assert_eq!(function.apply(&9, &4), 5);
    assert_eq!(cloned.apply(&4, &9), -5);
}
