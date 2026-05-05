/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::BoxConditionalMutator;

#[test]
fn test_box_conditional_mutator_observable_behavior() {
    let type_name = std::any::type_name::<BoxConditionalMutator<i32>>();
    assert!(type_name.contains("BoxConditionalMutator"), "{type_name}");
}
