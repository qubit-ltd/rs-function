// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::RcStatefulTransformer;
use qubit_function::StatefulTransformer;

#[test]
fn test_rc_conditional_stateful_transformer_observable_behavior() {
    let mut transformer = RcStatefulTransformer::new(|value: i32| value * 2)
        .when(|value: &i32| *value > 0)
        .or_else(|value: i32| -value);

    assert_eq!(transformer.apply(5), 10);
    assert_eq!(transformer.apply(-5), 5);
}
