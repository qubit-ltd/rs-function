// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxTransformer;

#[test]
fn test_conditional_transformer_debug_display_observable_behavior() {
    let conditional = BoxTransformer::new(|value: i32| value + 1).when(|value: &i32| *value > 0);

    assert_eq!(
        format!("{}", conditional),
        "BoxConditionalTransformer(BoxTransformer, BoxPredicate(unnamed))"
    );
    assert_eq!(
        format!("{:?}", conditional),
        "BoxConditionalTransformer { transformer: BoxTransformer { name: None, function: \"<function>\" }, predicate: BoxPredicate { name: None, function: \"<function>\" } }"
    );
}
