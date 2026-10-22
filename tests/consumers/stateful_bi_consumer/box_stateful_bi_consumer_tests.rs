// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxStatefulBiConsumer;
use qubit_function::StatefulBiConsumer;

#[test]
fn test_box_stateful_bi_consumer_accepts_both_values() {
    let mut received_values = Vec::new();
    let mut consumer = BoxStatefulBiConsumer::new(|first: &i32, second: &i32| {
        received_values.push((*first, *second));
    });

    consumer.accept(&2, &3);
    consumer.accept(&5, &7);
    drop(consumer);

    assert_eq!(received_values, vec![(2, 3), (5, 7)]);
}
