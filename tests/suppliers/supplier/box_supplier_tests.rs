// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::any::type_name;

use qubit_function::BoxSupplier;

#[test]
fn test_box_supplier_observable_behavior() {
    let supplier_type_name = type_name::<BoxSupplier<i32>>();
    assert!(supplier_type_name.contains("BoxSupplier"), "{supplier_type_name}");
}
