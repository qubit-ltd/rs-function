// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::ArcMutator;
use qubit_function::Mutator;

#[test]
fn test_arc_mutator_clone_applies_mutation() {
    let mutator = ArcMutator::new(|value: &mut i32| *value += 1);
    let cloned_mutator = mutator.clone();
    let mut value = 41;
    mutator.apply(&mut value);
    cloned_mutator.apply(&mut value);
    assert_eq!(value, 43);
}
