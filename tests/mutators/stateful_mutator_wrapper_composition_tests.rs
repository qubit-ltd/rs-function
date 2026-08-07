// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Unit tests for StatefulMutator types

use qubit_function::ArcStatefulMutator;
use qubit_function::BoxStatefulMutator;
use qubit_function::MutatorOnce;
use qubit_function::RcStatefulMutator;
use qubit_function::StatefulMutator;

// ============================================================================
// BoxStatefulMutator Tests
// ============================================================================

#[cfg(test)]
mod wrapper_composition_tests {
    use super::MutatorOnce;
    use super::StatefulMutator;

    #[test]
    fn test_closure_accept() {
        let closure = |x: &mut i32| *x *= 2;
        let mut value = 5;
        closure.apply(&mut value);
        assert_eq!(value, 10);
    }
}

// ============================================================================
// Unified Interface Tests
// ============================================================================
