// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Unit tests for StatefulSupplier types

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use qubit_function::ArcStatefulSupplier;
use qubit_function::BoxStatefulSupplier;
use qubit_function::RcStatefulSupplier;
use qubit_function::StatefulSupplier;

// ==========================================================================
// StatefulSupplier Trait Tests (for closures)
// ==========================================================================

#[cfg(test)]
mod test_stateful_supplier_trait {
    use super::BoxStatefulSupplier;
    use super::StatefulSupplier;

    #[test]
    fn test_closure_stateful() {
        let mut counter = 0;
        let mut boxed = BoxStatefulSupplier::new(move || {
            counter += 1;
            counter
        });
        assert_eq!(boxed.get(), 1);
        assert_eq!(boxed.get(), 2);
        assert_eq!(boxed.get(), 3);
    }

    #[test]
    fn test_closure_get() {
        // Test the get method in impl<T, F> StatefulSupplier<T> for F
        let mut closure = || 42;
        assert_eq!(closure.get(), 42);
        assert_eq!(closure.get(), 42);
    }

    #[test]
    fn test_closure_get_fn() {
        // Test an Fn closure that returns the captured value without mutation.
        let value = 42;
        let mut closure = move || value;
        assert_eq!(closure.get(), 42);
        assert_eq!(closure.get(), 42);
        assert_eq!(closure.get(), 42);
    }
}

// ==========================================================================
// BoxStatefulSupplier Tests
// ==========================================================================
