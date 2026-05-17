/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::RcStatefulSupplier;

#[test]
fn test_rc_stateful_supplier_observable_behavior() {
    let type_name = std::any::type_name::<RcStatefulSupplier<i32>>();
    assert!(type_name.contains("RcStatefulSupplier"), "{type_name}");
}
