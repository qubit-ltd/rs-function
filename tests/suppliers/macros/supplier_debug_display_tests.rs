// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxSupplier;

#[test]
fn test_box_supplier_debug_and_display_formatting() {
    let unnamed_supplier = BoxSupplier::new(|| 42);
    assert_eq!(
        format!("{unnamed_supplier:?}"),
        "BoxSupplier { name: None, function: \"<function>\" }"
    );
    assert_eq!(format!("{unnamed_supplier}"), "BoxSupplier");

    let named_supplier = BoxSupplier::new_with_name("numbers", || 42);
    assert_eq!(
        format!("{named_supplier:?}"),
        "BoxSupplier { name: Some(\"numbers\"), function: \"<function>\" }"
    );
    assert_eq!(format!("{named_supplier}"), "BoxSupplier(numbers)");
}
