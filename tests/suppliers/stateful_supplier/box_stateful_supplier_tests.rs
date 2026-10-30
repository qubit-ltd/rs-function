// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxStatefulSupplier;
use qubit_function::StatefulSupplier;

#[test]
fn test_box_stateful_supplier_retains_mutable_callback_state() {
    let mut count = 0;
    let mut supplier = BoxStatefulSupplier::new(move || {
        count += 1;
        count
    });

    assert_eq!(supplier.get(), 1);
    assert_eq!(supplier.get(), 2);
}
