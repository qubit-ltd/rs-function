// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxMutator;
use qubit_function::Mutator;

#[test]
fn test_conditional_mutator_applies_mutation() {
    let conditional = BoxMutator::new(|value: &mut i32| *value += 1).when(|value: &i32| *value > 0);

    let debug = format!("{conditional:?}");
    assert!(debug.starts_with("BoxConditionalMutator {"), "{debug}");
    assert!(debug.contains("mutator:"), "{debug}");
    assert!(debug.contains("predicate:"), "{debug}");
    assert_eq!(
        format!("{conditional}"),
        "BoxConditionalMutator(BoxMutator, BoxPredicate(unnamed))"
    );

    let mut value = 41;
    conditional.apply(&mut value);
    assert_eq!(value, 42);

    let mut rejected = 0;
    conditional.apply(&mut rejected);
    assert_eq!(rejected, 0);
}
