// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::ArcStatefulMutator;
use qubit_function::StatefulMutator;

#[test]
fn test_arc_conditional_stateful_mutator_applies_only_when_predicate_matches() {
    let mutator = ArcStatefulMutator::new(|value: &mut i32| *value += 1);
    let mut conditional = mutator.when(|value: &i32| *value > 0);

    let mut matching_value = 1;
    conditional.apply(&mut matching_value);
    assert_eq!(matching_value, 2);

    let mut non_matching_value = 0;
    conditional.apply(&mut non_matching_value);
    assert_eq!(non_matching_value, 0);
}
