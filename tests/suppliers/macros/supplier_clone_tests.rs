/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/

use qubit_function::{
    BoxSupplier,
    Supplier,
};

#[test]
fn test_supplier_clone_observable_behavior() {
    let supplier = BoxSupplier::new(|| 42);
    assert_eq!(supplier.get(), 42);
}
