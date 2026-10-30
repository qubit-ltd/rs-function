// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::ArcSupplier;
use qubit_function::Supplier;

#[test]
fn test_arc_supplier_clones_share_callback_state() {
    use std::sync::Arc;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;

    let count = Arc::new(AtomicUsize::new(0));
    let captured_count = Arc::clone(&count);
    let supplier = ArcSupplier::new(move || captured_count.fetch_add(1, Ordering::SeqCst) + 1);
    let cloned = supplier.clone();

    assert_eq!(supplier.get(), 1);
    assert_eq!(cloned.get(), 2);
}
