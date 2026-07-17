// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Unit tests for Mutator types (stateless Fn(&mut T))

use qubit_function::{
    ArcMutator,
    BoxMutator,
    Mutator,
    MutatorOnce,
    RcMutator,
};

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
mod test_unified_interface {
    use super::{
        ArcMutator,
        BoxMutator,
        Mutator,
        RcMutator,
    };

    fn apply_mutator<C: Mutator<i32>>(mutator: &C, value: i32) -> i32 {
        let mut val = value;
        mutator.apply(&mut val);
        val
    }

    #[test]
    fn test_with_box_mutator() {
        let mutator = BoxMutator::new(|x: &mut i32| *x *= 2);
        assert_eq!(apply_mutator(&mutator, 5), 10);
    }

    #[test]
    fn test_with_arc_mutator() {
        let mutator = ArcMutator::new(|x: &mut i32| *x *= 2);
        assert_eq!(apply_mutator(&mutator, 5), 10);
    }

    #[test]
    fn test_with_rc_mutator() {
        let mutator = RcMutator::new(|x: &mut i32| *x *= 2);
        assert_eq!(apply_mutator(&mutator, 5), 10);
    }

    #[test]
    fn test_with_closure() {
        let closure = |x: &mut i32| *x *= 2;
        assert_eq!(apply_mutator(&closure, 5), 10);
    }
}

// ============================================================================
// Conditional Mutator and_then Tests
// ============================================================================
