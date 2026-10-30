// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::ArcStatefulSupplier;
use qubit_function::StatefulSupplier;

#[test]
fn test_arc_stateful_supplier_clones_share_mutable_callback_state() {
    let mut count = 0;
    let mut supplier = ArcStatefulSupplier::new(move || {
        count += 1;
        count
    });
    let mut cloned = supplier.clone();

    assert_eq!(supplier.get(), 1);
    assert_eq!(cloned.get(), 2);
}
