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
fn test_mutator_debug_and_display_include_mutator_name() {
    let mutator = BoxMutator::new(|value: &mut i32| *value += 1);
    assert_eq!(
        format!("{mutator:?}"),
        "BoxMutator { name: None, function: \"<function>\" }"
    );
    assert_eq!(format!("{mutator}"), "BoxMutator");

    let named_mutator = BoxMutator::new_with_name("increment", |value: &mut i32| *value += 1);
    assert_eq!(
        format!("{named_mutator:?}"),
        "BoxMutator { name: Some(\"increment\"), function: \"<function>\" }"
    );
    assert_eq!(format!("{named_mutator}"), "BoxMutator(increment)");
}
