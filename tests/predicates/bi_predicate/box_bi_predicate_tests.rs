// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BiPredicate;
use qubit_function::BoxBiPredicate;

#[test]
fn test_box_bi_predicate_evaluates_both_values() {
    let predicate = BoxBiPredicate::new(|left: &i32, right: &i32| left < right);
    assert!(predicate.test(&3, &5));
    assert!(!predicate.test(&5, &3));
}
