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
mod test_conditional_mutator_debug_display {
    use super::ArcMutator;
    use super::BoxMutator;
    use super::RcMutator;

    #[test]
    fn test_box_conditional_mutator_debug() {
        let mutator = BoxMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("BoxConditionalMutator"));
        assert!(debug_str.contains("BoxMutator"));
        assert!(debug_str.contains("BoxPredicate"));
    }

    #[test]
    fn test_box_conditional_mutator_display() {
        let mutator = BoxMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let display_str = format!("{}", conditional);
        assert!(display_str.contains("BoxConditionalMutator"));
    }

    #[test]
    fn test_rc_conditional_mutator_debug() {
        let mutator = RcMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("RcConditionalMutator"));
        assert!(debug_str.contains("RcMutator"));
        assert!(debug_str.contains("RcPredicate"));
    }

    #[test]
    fn test_rc_conditional_mutator_display() {
        let mutator = RcMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let display_str = format!("{}", conditional);
        assert!(display_str.contains("RcConditionalMutator"));
    }

    #[test]
    fn test_arc_conditional_mutator_debug() {
        let mutator = ArcMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("ArcConditionalMutator"));
        assert!(debug_str.contains("ArcMutator"));
        assert!(debug_str.contains("ArcPredicate"));
    }

    #[test]
    fn test_arc_conditional_mutator_display() {
        let mutator = ArcMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let display_str = format!("{}", conditional);
        assert!(display_str.contains("ArcConditionalMutator"));
    }
}

// ============================================================================
// Name Preservation Tests for into_xxx and to_xxx Methods
// ============================================================================
