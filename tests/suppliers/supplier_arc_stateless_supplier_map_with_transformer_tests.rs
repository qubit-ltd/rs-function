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
mod test_arc_stateless_supplier_map_with_transformer {
    use super::ArcSupplier;
    use super::ArcTransformer;
    use super::Supplier;
    use super::thread;

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
        let supplier = ArcSupplier::new(|| 10);
        let mapped = supplier.map(|x| x * 2);
        assert_eq!(mapped.get(), 20);
    }

    #[test]
    fn test_map_with_function_pointer() {
        // Test map accepts function pointer
        let supplier = ArcSupplier::new(|| 10);
        let mapped = supplier.map(double);
        assert_eq!(mapped.get(), 20);
    }

    #[test]
    fn test_map_with_arc_transformer() {
        // Test map accepts ArcTransformer object
        let supplier = ArcSupplier::new(|| 10);
        let transformer = ArcTransformer::new(|x| x * 3);
        let mapped = supplier.map(transformer);
        assert_eq!(mapped.get(), 30);
    }

    #[test]
    fn test_map_chain_with_different_types() {
        // Test chained calls, each map uses different type of transformer
        let supplier = ArcSupplier::new(|| 10);
        let step1 = supplier.map(|x| x * 2); // closure
        let step2 = step1.map(double); // function pointer
        let step3 = step2.map(ArcTransformer::new(|x| x + 5)); // ArcTransformer
        assert_eq!(step3.get(), 45); // (10 * 2) * 2 + 5 = 45
    }

    #[test]
    fn test_map_with_closure_capturing_variables() {
        // Test map uses closure capturing variables
        let multiplier = 3;
        let supplier = ArcSupplier::new(|| 10);
        let mapped = supplier.map(move |x| x * multiplier);
        assert_eq!(mapped.get(), 30);
    }

    #[test]
    fn test_map_preserves_original_with_transformer() {
        // Test original supplier still usable after using transformer
        let supplier = ArcSupplier::new(|| 10);
        let transformer = ArcTransformer::new(|x| x * 2);
        let mapped = supplier.map(transformer);

        // Original supplier still usable
        assert_eq!(supplier.get(), 10);
        assert_eq!(mapped.get(), 20);
    }

    #[test]
    fn test_map_thread_safety_with_transformer() {
        // Test map with transformer in multi-threaded environment
        let supplier = ArcSupplier::new(|| 10);
        let transformer = ArcTransformer::new(|x| x * 2);
        let mapped = supplier.map(transformer);

        let handles: Vec<_> = (0..10)
            .map(|_| {
                let m = mapped.clone();
                thread::spawn(move || m.get())
            })
            .collect();

        for h in handles {
            assert_eq!(h.join().expect("thread should not panic"), 20);
        }
    }

    #[test]
    fn test_map_with_type_conversion() {
        // Test map performs type conversion
        let supplier = ArcSupplier::new(|| 42);

        // Use closure to convert type
        let mapped1 = supplier.map(|x: i32| x.to_string());
        assert_eq!(mapped1.get(), "42");

        // Use ArcTransformer to convert type
        let transformer = ArcTransformer::new(to_string);
        let mapped2 = supplier.map(transformer);
        assert_eq!(mapped2.get(), "42");
    }

    #[test]
    fn test_map_with_shared_transformer() {
        // Test multiple suppliers sharing the same transformer
        let supplier1 = ArcSupplier::new(|| 10);
        let supplier2 = ArcSupplier::new(|| 20);

        let transformer = ArcTransformer::new(|x| x * 2);
        let mapped1 = supplier1.map(transformer.clone());
        let mapped2 = supplier2.map(transformer);

        assert_eq!(mapped1.get(), 20);
        assert_eq!(mapped2.get(), 40);
    }
}

// ======================================================================
// Map with Transformer Tests - RcSupplier
// ======================================================================
