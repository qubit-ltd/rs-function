// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::RcSupplier;
use qubit_function::Supplier;

#[test]
fn test_rc_supplier_clones_share_callback_state() {
    use std::cell::Cell;
    use std::rc::Rc;

    let count = Rc::new(Cell::new(0));
    let captured_count = Rc::clone(&count);
    let supplier = RcSupplier::new(move || {
        captured_count.set(captured_count.get() + 1);
        captured_count.get()
    });
    let cloned = supplier.clone();

    assert_eq!(supplier.get(), 1);
    assert_eq!(cloned.get(), 2);
}
