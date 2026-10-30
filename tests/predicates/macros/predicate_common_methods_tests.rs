// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxPredicate;
use qubit_function::Predicate;

#[test]
fn test_predicate_common_methods_observable_behavior() {
    let predicate = BoxPredicate::new(|value: &i32| *value > 0);
    assert!(predicate.test(&1), "positive values should pass the predicate");
    assert!(!predicate.test(&0), "zero should fail the positive-value predicate");
}
