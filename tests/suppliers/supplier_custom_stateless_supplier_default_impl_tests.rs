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
mod test_custom_stateless_supplier_default_impl {
    use super::Supplier;

    /// A simple custom type that implements Supplier with
    /// only the core `get` method, relying on default
    /// implementations for `into_box`, `into_rc`, and `into_arc`.
    struct CounterSupplier {
        /// The value to return each time `get` is called.
        value: i32,
    }

    impl CounterSupplier {
        /// Creates a new CounterSupplier with the given value.
        fn new(value: i32) -> Self {
            Self { value }
        }
    }

    impl Supplier<i32> for CounterSupplier {
        fn get(&self) -> i32 {
            self.value
        }

        // All into_xxx methods use default implementations
    }

    #[test]
    fn test_custom_supplier_get() {
        // Test that the custom supplier correctly implements the
        // core get method
        let supplier = CounterSupplier::new(42);
        assert_eq!(supplier.get(), 42);
        assert_eq!(supplier.get(), 42);
    }

    // Implement Clone for CounterSupplier to enable to_* methods
    impl Clone for CounterSupplier {
        fn clone(&self) -> Self {
            Self { value: self.value }
        }
    }
}
// ======================================================================
// Debug and Display Trait Tests
// ======================================================================
