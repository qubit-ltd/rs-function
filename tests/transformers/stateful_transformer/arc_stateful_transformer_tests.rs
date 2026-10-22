// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::ArcStatefulTransformer;
use qubit_function::StatefulTransformer;

#[test]
fn test_arc_stateful_transformer_applies_stateful_callback() {
    let mut call_count = 0;
    let mut transformer = ArcStatefulTransformer::new(move |value: i32| {
        call_count += 1;
        value + call_count
    });

    assert_eq!(transformer.apply(10), 11);
    assert_eq!(transformer.apply(10), 12);
}
