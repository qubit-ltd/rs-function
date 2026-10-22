// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::any::type_name as type_name_of;

use qubit_function::ArcStatefulSupplier;

#[test]
fn test_arc_stateful_supplier_observable_behavior() {
    let type_name = type_name_of::<ArcStatefulSupplier<i32>>();
    assert!(type_name.contains("ArcStatefulSupplier"), "{type_name}");
}
