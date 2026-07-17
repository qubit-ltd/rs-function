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
mod test_map_transformer_integration {
    use super::{
        ArcSupplier,
        ArcTransformer,
        BoxSupplier,
        Supplier,
    };

    #[test]
    fn test_mixed_transformer_types_in_pipeline() {
        // Test mixing different types of transformers in pipeline
        let supplier = BoxSupplier::new(|| 5);

        let pipeline = supplier
            .map(|x| x * 2) // closure
            .map(|x: i32| -> i32 { x + 3 }) // closure with explicit type annotation
            .map(|x: i32| x.to_string()); // type conversion closure

        assert_eq!(pipeline.get(), "13");
    }

    #[test]
    fn test_transformer_with_complex_logic() {
        // Test transformer with complex logic
        #[derive(Debug, PartialEq)]
        struct Result {
            doubled: i32,
            squared: i32,
        }

        let supplier = ArcSupplier::new(|| 5);
        let transformer = ArcTransformer::new(|x| Result {
            doubled: x * 2,
            squared: x * x,
        });

        let mapped = supplier.map(transformer);
        assert_eq!(
            mapped.get(),
            Result {
                doubled: 10,
                squared: 25
            }
        );
    }

    #[test]
    fn test_function_pointer_with_generic_supplier() {
        // Test function pointer with generic supplier
        fn process(x: i32) -> String {
            format!("Value: {}", x * 2)
        }

        let supplier = ArcSupplier::new(|| 21);
        let mapped = supplier.map(process);
        assert_eq!(mapped.get(), "Value: 42");
    }

    #[test]
    fn test_transformer_reusability() {
        // Test reusability of Transformer
        let transformer = ArcTransformer::new(|x: i32| x * 10);

        let supplier1 = ArcSupplier::new(|| 1);
        let supplier2 = ArcSupplier::new(|| 2);
        let supplier3 = ArcSupplier::new(|| 3);

        let mapped1 = supplier1.map(transformer.clone());
        let mapped2 = supplier2.map(transformer.clone());
        let mapped3 = supplier3.map(transformer);

        assert_eq!(mapped1.get(), 10);
        assert_eq!(mapped2.get(), 20);
        assert_eq!(mapped3.get(), 30);
    }
}

// ======================================================================
// Default Implementation Tests for Custom Types
// ======================================================================
