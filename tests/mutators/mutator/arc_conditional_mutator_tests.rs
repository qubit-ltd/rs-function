// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::ArcConditionalMutator;
use qubit_function::ArcMutator;
use qubit_function::Mutator;

#[test]
fn test_arc_conditional_mutator_applies_only_when_predicate_matches() {
    let mutator = ArcMutator::new(|value: &mut i32| *value += 1);
    let conditional: ArcConditionalMutator<i32> = mutator.when(|value: &i32| *value > 0);

    let mut accepted = 1;
    conditional.apply(&mut accepted);
    assert_eq!(accepted, 2);

    let mut rejected = 0;
    conditional.apply(&mut rejected);
    assert_eq!(rejected, 0);
}
