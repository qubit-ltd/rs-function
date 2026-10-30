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
use qubit_function::RcMutator;

// ============================================================================
// Mutator Unified Interface Tests
// ============================================================================

#[cfg(test)]
mod test_unified_interface {
    use super::ArcMutator;
    use super::BoxMutator;
    use super::Mutator;
    use super::RcMutator;

    fn apply_mutator<C: Mutator<i32>>(mutator: &C, value: i32) -> i32 {
        let mut val = value;
        mutator.apply(&mut val);
        val
    }

    #[test]
    fn test_box_mutator_applies_mutation() {
        let mutator = BoxMutator::new(|x: &mut i32| *x *= 2);
        assert_eq!(apply_mutator(&mutator, 5), 10);
    }

    #[test]
    fn test_arc_mutator_applies_mutation() {
        let mutator = ArcMutator::new(|x: &mut i32| *x *= 2);
        assert_eq!(apply_mutator(&mutator, 5), 10);
    }

    #[test]
    fn test_rc_mutator_applies_mutation() {
        let mutator = RcMutator::new(|x: &mut i32| *x *= 2);
        assert_eq!(apply_mutator(&mutator, 5), 10);
    }

    #[test]
    fn test_closure_mutator_applies_mutation() {
        let closure = |x: &mut i32| *x *= 2;
        assert_eq!(apply_mutator(&closure, 5), 10);
    }
}
