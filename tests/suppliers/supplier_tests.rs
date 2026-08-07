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
mod test_stateless_supplier_trait {
    use super::BoxSupplier;
    use super::Supplier;

    #[test]
    fn test_closure_stateless() {
        // Test stateless closure (always returns same value)
        let boxed = BoxSupplier::new(|| 42);
        assert_eq!(boxed.get(), 42);
        assert_eq!(boxed.get(), 42);
        assert_eq!(boxed.get(), 42);
    }

    #[test]
    fn test_closure_get() {
        // Test the get method in impl<T, F> Supplier<T>
        // for F
        let closure = || 42;
        assert_eq!(closure.get(), 42);
        assert_eq!(closure.get(), 42);
    }

    #[test]
    fn test_closure_get_stateless() {
        // Test stateless closure (doesn't modify captured
        // variables)
        let value = 100;
        let closure = move || value * 2;
        assert_eq!(closure.get(), 200);
        assert_eq!(closure.get(), 200);
        assert_eq!(closure.get(), 200);
    }
}

// ======================================================================
// BoxSupplier Tests
// ======================================================================
