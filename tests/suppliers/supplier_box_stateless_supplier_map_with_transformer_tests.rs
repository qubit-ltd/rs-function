// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Unit tests for Supplier types

use std::sync::Arc;
use std::thread;

use qubit_function::ArcSupplier;
use qubit_function::ArcTransformer;
use qubit_function::BoxSupplier;
use qubit_function::BoxTransformer;
use qubit_function::RcSupplier;
use qubit_function::RcTransformer;
use qubit_function::Supplier;

// ======================================================================
// Supplier Trait Tests (for closures)
// ======================================================================

#[cfg(test)]
mod test_box_stateless_supplier_map_with_transformer {
    use super::BoxSupplier;
    use super::BoxTransformer;
    use super::Supplier;

    // Helper function pointers
    fn double(x: i32) -> i32 {
        x * 2
    }

    fn to_string(x: i32) -> String {
        x.to_string()
    }

    #[test]
    fn test_map_with_closure() {
        // Test map accepts closure
        let supplier = BoxSupplier::new(|| 10);
        let mapped = supplier.map(|x| x * 2);
        assert_eq!(mapped.get(), 20);
    }

    #[test]
    fn test_map_with_function_pointer() {
        // Test map accepts function pointer
        let supplier = BoxSupplier::new(|| 10);
        let mapped = supplier.map(double);
        assert_eq!(mapped.get(), 20);
    }

    #[test]
    fn test_map_with_box_transformer() {
        // Test map accepts BoxTransformer object
        let supplier = BoxSupplier::new(|| 10);
        let transformer = BoxTransformer::new(|x| x * 3);
        let mapped = supplier.map(transformer);
        assert_eq!(mapped.get(), 30);
    }

    #[test]
    fn test_map_chain_with_different_types() {
        // Test chained calls, each map uses different type of transformer
        let supplier = BoxSupplier::new(|| 10);
        let step1 = supplier.map(|x| x * 2); // closure
        let step2 = step1.map(double); // function pointer
        let step3 = step2.map(BoxTransformer::new(|x| x + 5)); // BoxTransformer
        assert_eq!(step3.get(), 45); // (10 * 2) * 2 + 5 = 45
    }

    #[test]
    fn test_map_with_closure_capturing_variables() {
        // Test map uses closure capturing variables
        let multiplier = 3;
        let supplier = BoxSupplier::new(|| 10);
        let mapped = supplier.map(move |x| x * multiplier);
        assert_eq!(mapped.get(), 30);
    }

    #[test]
    fn test_map_with_type_conversion() {
        // Test map performs type conversion
        let supplier = BoxSupplier::new(|| 42);

        // Use closure to convert type
        let mapped1 = supplier.map(|x: i32| x.to_string());
        assert_eq!(mapped1.get(), "42");

        // Use BoxTransformer to convert type
        let supplier2 = BoxSupplier::new(|| 42);
        let transformer = BoxTransformer::new(to_string);
        let mapped2 = supplier2.map(transformer);
        assert_eq!(mapped2.get(), "42");
    }

    #[test]
    fn test_map_with_complex_transformer() {
        // Test map uses complex Transformer
        #[derive(Debug, PartialEq)]
        struct Data {
            value: i32,
        }

        let supplier = BoxSupplier::new(|| 10);
        let transformer = BoxTransformer::new(|x| Data { value: x * 2 });
        let mapped = supplier.map(transformer);
        assert_eq!(mapped.get(), Data { value: 20 });
    }
}

// ======================================================================
// Map with Transformer Tests - ArcSupplier
// ======================================================================
