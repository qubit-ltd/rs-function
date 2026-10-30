// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::ArcPredicate;
use qubit_function::Predicate;

#[test]
fn test_shared_predicate_clone_preserves_behavior() {
    let predicate = ArcPredicate::new(|value: &i32| *value > 0);
    let cloned = predicate.clone();
    assert!(predicate.test(&1));
    assert!(!cloned.test(&0));
}
