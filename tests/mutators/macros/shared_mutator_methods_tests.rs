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
fn test_shared_mutator_methods_observable_behavior() {
    let first = ArcMutator::new(|value: &mut i32| *value *= 2);
    let after = ArcMutator::new(|value: &mut i32| *value += 1);
    let chained = first.and_then(after);
    let mut value = 41;
    chained.apply(&mut value);
    assert_eq!(value, 83);
}
