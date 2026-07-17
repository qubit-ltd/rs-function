// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Unit tests for Supplier types

use qubit_function::{
    ArcSupplier,
    ArcTransformer,
    BoxSupplier,
    BoxTransformer,
    RcSupplier,
    RcTransformer,
    Supplier,
};
use std::sync::Arc;
use std::thread;

// ======================================================================
// Supplier Trait Tests (for closures)
// ======================================================================

#[cfg(test)]
mod test_supplier_debug_display {
    use super::{
        ArcSupplier,
        BoxSupplier,
        RcSupplier,
    };

    // ============================================================
    // BoxSupplier Debug and Display Tests
    // ============================================================

    mod test_box_supplier_debug_display {
        use super::BoxSupplier;

        #[test]
        fn test_debug_without_name() {
            // Test Debug formatting for BoxSupplier without name
            let supplier = BoxSupplier::new(|| 42);
            let debug_str = format!("{:?}", supplier);
            assert!(debug_str.contains("BoxSupplier"));
            assert!(debug_str.contains("name: None"));
            assert!(debug_str.contains("function: \"<function>\""));
        }

        #[test]
        fn test_debug_with_name() {
            // Test Debug formatting for BoxSupplier with name
            let supplier = BoxSupplier::new_with_name("test_supplier", || 42);
            let debug_str = format!("{:?}", supplier);
            assert!(debug_str.contains("BoxSupplier"));
            assert!(debug_str.contains("name: Some(\"test_supplier\")"));
            assert!(debug_str.contains("function: \"<function>\""));
        }

        #[test]
        fn test_display_without_name() {
            // Test Display formatting for BoxSupplier without name
            let supplier = BoxSupplier::new(|| 42);
            let display_str = format!("{}", supplier);
            assert_eq!(display_str, "BoxSupplier");
        }

        #[test]
        fn test_display_with_name() {
            // Test Display formatting for BoxSupplier with name
            let supplier = BoxSupplier::new_with_name("test_supplier", || 42);
            let display_str = format!("{}", supplier);
            assert_eq!(display_str, "BoxSupplier(test_supplier)");
        }
    }

    // ============================================================
    // ArcSupplier Debug and Display Tests
    // ============================================================

    mod test_arc_supplier_debug_display {
        use super::ArcSupplier;

        #[test]
        fn test_debug_without_name() {
            // Test Debug formatting for ArcSupplier without name
            let supplier = ArcSupplier::new(|| 42);
            let debug_str = format!("{:?}", supplier);
            assert!(debug_str.contains("ArcSupplier"));
            assert!(debug_str.contains("name: None"));
            assert!(debug_str.contains("function: \"<function>\""));
        }

        #[test]
        fn test_debug_with_name() {
            // Test Debug formatting for ArcSupplier with name
            let supplier = ArcSupplier::new_with_name("test_supplier", || 42);
            let debug_str = format!("{:?}", supplier);
            assert!(debug_str.contains("ArcSupplier"));
            assert!(debug_str.contains("name: Some(\"test_supplier\")"));
            assert!(debug_str.contains("function: \"<function>\""));
        }

        #[test]
        fn test_display_without_name() {
            // Test Display formatting for ArcSupplier without name
            let supplier = ArcSupplier::new(|| 42);
            let display_str = format!("{}", supplier);
            assert_eq!(display_str, "ArcSupplier");
        }

        #[test]
        fn test_display_with_name() {
            // Test Display formatting for ArcSupplier with name
            let supplier = ArcSupplier::new_with_name("test_supplier", || 42);
            let display_str = format!("{}", supplier);
            assert_eq!(display_str, "ArcSupplier(test_supplier)");
        }
    }

    // ============================================================
    // RcSupplier Debug and Display Tests
    // ============================================================

    mod test_rc_supplier_debug_display {
        use super::RcSupplier;

        #[test]
        fn test_debug_without_name() {
            // Test Debug formatting for RcSupplier without name
            let supplier = RcSupplier::new(|| 42);
            let debug_str = format!("{:?}", supplier);
            assert!(debug_str.contains("RcSupplier"));
            assert!(debug_str.contains("name: None"));
            assert!(debug_str.contains("function: \"<function>\""));
        }

        #[test]
        fn test_debug_with_name() {
            // Test Debug formatting for RcSupplier with name
            let supplier = RcSupplier::new_with_name("test_supplier", || 42);
            let debug_str = format!("{:?}", supplier);
            assert!(debug_str.contains("RcSupplier"));
            assert!(debug_str.contains("name: Some(\"test_supplier\")"));
            assert!(debug_str.contains("function: \"<function>\""));
        }

        #[test]
        fn test_display_without_name() {
            // Test Display formatting for RcSupplier without name
            let supplier = RcSupplier::new(|| 42);
            let display_str = format!("{}", supplier);
            assert_eq!(display_str, "RcSupplier");
        }

        #[test]
        fn test_display_with_name() {
            // Test Display formatting for RcSupplier with name
            let supplier = RcSupplier::new_with_name("test_supplier", || 42);
            let display_str = format!("{}", supplier);
            assert_eq!(display_str, "RcSupplier(test_supplier)");
        }
    }
}

// ============================================================================
// Supplier Trait Default Methods Tests - into_once, to_once
// ============================================================================
