// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::RcStatefulMutator;
use qubit_function::StatefulMutator;

#[test]
fn test_rc_stateful_mutator_clones_share_callback_state() {
    let mut calls = 0;
    let mut mutator = RcStatefulMutator::new(move |value: &mut i32| {
        calls += 1;
        *value += calls;
    });
    let mut cloned_mutator = mutator.clone();

    let mut value = 10;
    mutator.apply(&mut value);
    cloned_mutator.apply(&mut value);

    assert_eq!(value, 13, "cloned mutators should share callback state");
}
