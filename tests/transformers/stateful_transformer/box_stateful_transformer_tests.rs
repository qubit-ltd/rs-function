// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxStatefulTransformer;
use qubit_function::StatefulTransformer;

#[test]
fn test_box_stateful_transformer_retains_mutable_callback_state() {
    let mut count = 0;
    let mut transformer = BoxStatefulTransformer::new(move |value: i32| {
        count += 1;
        value + count
    });

    assert_eq!(transformer.apply(10), 11);
    assert_eq!(transformer.apply(10), 12);
}
