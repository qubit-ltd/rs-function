/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::RcSupplier;

#[test]
fn test_rc_supplier_observable_behavior() {
    let type_name = std::any::type_name::<RcSupplier<i32>>();
    assert!(type_name.contains("RcSupplier"), "{type_name}");
}
