/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::{
    BoxPredicate,
    Predicate,
};

#[test]
fn test_predicate_debug_display_observable_behavior() {
    let predicate = BoxPredicate::new(|value: &i32| *value > 0);
    assert!(predicate.test(&1));
    assert!(!predicate.test(&0));
}
