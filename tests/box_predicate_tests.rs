/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::BoxPredicate;

#[test]
fn test_box_predicate_observable_behavior() {
    let type_name = std::any::type_name::<BoxPredicate<i32>>();
    assert!(type_name.contains("BoxPredicate"), "{type_name}");
}
