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
fn test_predicate_constants_observable_behavior() {
    let always_true = BoxPredicate::<i32>::always_true();
    let always_false = BoxPredicate::<i32>::always_false();

    assert_eq!(always_true.name(), "always_true");
    assert_eq!(always_false.name(), "always_false");
    assert!(always_true.test(&1));
    assert!(!always_false.test(&1));
}
