// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;

use qubit_function::ArcMutator;
use qubit_function::Mutator;

#[test]
fn test_arc_conditional_mutator_clone_preserves_predicate() {
    let original = ArcMutator::new({
        let calls = AtomicI32::new(0);
        move |value: &mut i32| {
            *value += calls.fetch_add(1, Ordering::SeqCst) + 1;
        }
    })
    .when(|value: &i32| *value > 0);
    let cloned = original.clone();
    let mut positive = 41;
    let mut negative = -1;
    original.apply(&mut positive);
    let mut second_positive = 41;
    cloned.apply(&mut second_positive);
    cloned.apply(&mut negative);
    assert_eq!(positive, 42);
    assert_eq!(second_positive, 43);
    assert_eq!(negative, -1);
}
