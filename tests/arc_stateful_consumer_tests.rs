/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::ArcStatefulConsumer;

#[test]
fn test_arc_stateful_consumer_observable_behavior() {
    let type_name = std::any::type_name::<ArcStatefulConsumer<i32>>();
    assert!(type_name.contains("ArcStatefulConsumer"), "{type_name}");
}
