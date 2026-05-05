/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::ArcStatefulBiConsumer;

#[test]
fn test_arc_stateful_bi_consumer_observable_behavior() {
    let type_name = std::any::type_name::<ArcStatefulBiConsumer<i32, i32>>();
    assert!(type_name.contains("ArcStatefulBiConsumer"), "{type_name}");
}
