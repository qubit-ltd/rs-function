// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxSupplier;
use qubit_function::Supplier;

#[test]
fn test_box_supplier_produces_value_on_each_call() {
    let supplier = BoxSupplier::new(|| 42);

    assert_eq!(supplier.get(), 42);
    assert_eq!(supplier.get(), 42);
}
