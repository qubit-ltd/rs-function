// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Unit tests for Mutator types (stateless Fn(&mut T))

use qubit_function::ArcMutator;
use qubit_function::BoxMutator;
use qubit_function::Mutator;
use qubit_function::MutatorOnce;
use qubit_function::RcMutator;

// ============================================================================
// Mutator Default Implementation Tests
// ============================================================================

/// Test struct that implements Mutator to test default methods
struct TestMutator {
    multiplier: i32,
}

impl TestMutator {
    fn new(multiplier: i32) -> Self {
        TestMutator { multiplier }
    }
}

impl Mutator<i32> for TestMutator {
    fn apply(&self, input: &mut i32) {
        *input *= self.multiplier;
    }
}

impl Clone for TestMutator {
    fn clone(&self) -> Self {
        TestMutator {
            multiplier: self.multiplier,
        }
    }
}

// ============================================================================
// BoxMutator Tests
// ============================================================================

#[cfg(test)]
mod wrapper_composition_tests {
    use super::Mutator;
    use super::MutatorOnce;

    #[test]
    fn test_closure_mutate() {
        let closure = |x: &mut i32| *x *= 2;
        let mut value = 5;
        closure.apply(&mut value);
        assert_eq!(value, 10);
    }
}

// ============================================================================
// Unified Interface Tests
// ============================================================================
