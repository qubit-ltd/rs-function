/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::BoxMutator;

#[test]
fn test_box_mutator_observable_behavior() {
    let type_name = std::any::type_name::<BoxMutator<i32>>();
    assert!(type_name.contains("BoxMutator"), "{type_name}");
}
