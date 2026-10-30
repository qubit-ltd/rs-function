// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::Mutator;
use qubit_function::RcConditionalMutator;
use qubit_function::RcMutator;

#[test]
fn test_rc_conditional_mutator_applies_only_when_predicate_matches() {
    let mutator = RcMutator::new(|value: &mut i32| *value += 1);
    let conditional: RcConditionalMutator<i32> = mutator.when(|value: &i32| *value > 0);

    let mut accepted = 1;
    conditional.apply(&mut accepted);
    assert_eq!(accepted, 2);

    let mut rejected = 0;
    conditional.apply(&mut rejected);
    assert_eq!(rejected, 0);
}
