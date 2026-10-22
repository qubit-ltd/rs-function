// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::RcStatefulBiTransformer;
use qubit_function::StatefulBiTransformer;

#[test]
fn test_rc_stateful_bi_transformer_applies_stateful_function() {
    let mut total = 0;
    let mut transformer = RcStatefulBiTransformer::new(move |first: i32, second: i32| {
        total += first + second;
        total
    });

    assert_eq!(transformer.apply(2, 3), 5);
    assert_eq!(transformer.apply(4, 1), 10);
}
