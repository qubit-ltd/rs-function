// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Unit tests for StatefulSupplier types

use qubit_function::BoxStatefulSupplier;
use qubit_function::StatefulSupplier;

// ==========================================================================
// StatefulSupplier Trait Tests (for closures)
// ==========================================================================

#[cfg(test)]
mod test_box_stateful_supplier_once {
    use super::BoxStatefulSupplier;
    use super::StatefulSupplier;

    mod test_get {
        use super::BoxStatefulSupplier;
        use super::StatefulSupplier;

        #[test]
        fn test_stateful_supplier_can_be_reused() {
            let mut value = 0;
            let mut supplier = BoxStatefulSupplier::new(move || {
                value += 1;
                value
            });
            assert_eq!(supplier.get(), 1);
            assert_eq!(supplier.get(), 2);
        }

        #[test]
        fn test_with_string() {
            let mut supplier = BoxStatefulSupplier::new(|| String::from("hello"));
            let value = supplier.get();
            assert_eq!(value, "hello");
        }

        #[test]
        fn test_with_vec() {
            let mut supplier = BoxStatefulSupplier::new(|| vec![1, 2, 3]);
            let value = supplier.get();
            assert_eq!(value, vec![1, 2, 3]);
        }

        #[test]
        fn test_moves_captured_value() {
            let data = String::from("captured");
            let mut supplier = BoxStatefulSupplier::new(move || data.clone());
            let value = supplier.get();
            assert_eq!(value, "captured");
        }

        #[test]
        fn test_with_stateful_closure() {
            let mut counter = 0;
            let mut supplier = BoxStatefulSupplier::new(move || {
                counter += 1;
                counter
            });
            let value = supplier.get();
            assert_eq!(value, 1);
        }
    }
}

// ==========================================================================
// SupplierOnce Implementation Tests for ArcStatefulSupplier
// ==========================================================================
