// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxStatefulBiTransformer;
use qubit_function::StatefulBiTransformer;

#[test]
fn test_box_stateful_bi_transformer_retains_mutable_callback_state() {
    let mut count = 0;
    let mut transformer = BoxStatefulBiTransformer::new(move |left: i32, right: i32| {
        count += 1;
        left + right + count
    });

    assert_eq!(transformer.apply(5, 3), 9);
    assert_eq!(transformer.apply(2, 4), 8);
}
