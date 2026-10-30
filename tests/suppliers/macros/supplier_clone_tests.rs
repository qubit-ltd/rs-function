// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

#[cfg(feature = "rc")]
use qubit_function::RcSupplier;
#[cfg(feature = "rc")]
use qubit_function::Supplier;

#[cfg(feature = "rc")]
#[test]
fn test_supplier_clone_preserves_observable_behavior() {
    let supplier = RcSupplier::new(|| 42);
    let cloned_supplier = supplier.clone();

    assert_eq!(cloned_supplier.get(), 42);
}
