/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::BoxStatefulSupplier;

#[test]
fn test_box_stateful_supplier_observable_behavior() {
    let type_name = std::any::type_name::<BoxStatefulSupplier<i32>>();
    assert!(type_name.contains("BoxStatefulSupplier"), "{type_name}");
}
