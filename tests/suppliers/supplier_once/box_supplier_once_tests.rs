// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxSupplierOnce;
use qubit_function::SupplierOnce;

#[test]
fn test_box_supplier_once_moves_non_clone_capture_into_result() {
    struct Payload(&'static str);

    let payload = Payload("owned");
    let supplier = BoxSupplierOnce::new(move || payload);

    assert_eq!(supplier.get().0, "owned");
}
