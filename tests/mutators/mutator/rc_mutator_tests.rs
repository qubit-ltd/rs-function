// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::Mutator;
use qubit_function::RcMutator;

#[test]
fn test_rc_mutator_applies_to_value() {
    let mutator = RcMutator::new(|value: &mut i32| *value *= 2);
    let mut value = 21;
    mutator.apply(&mut value);
    assert_eq!(value, 42);
}
