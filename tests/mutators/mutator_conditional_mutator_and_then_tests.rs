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
mod test_conditional_mutator_and_then {
    use super::{
        ArcMutator,
        BoxMutator,
        Mutator,
        RcMutator,
    };

    #[test]
    fn test_box_conditional_mutator_and_then_with_closure() {
        let mutator = BoxMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);
        let chained = conditional.and_then(|x: &mut i32| *x += 10);

        let mut positive = 5;
        chained.apply(&mut positive);
        assert_eq!(positive, 20); // 5 * 2 + 10

        let mut negative = -5;
        chained.apply(&mut negative);
        assert_eq!(negative, 5); // -5 + 10 (condition not met)
    }

    #[test]
    fn test_box_conditional_mutator_and_then_with_box_mutator() {
        let mutator1 = BoxMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator1.when(|x: &i32| *x > 0);
        let mutator2 = BoxMutator::new(|x: &mut i32| *x += 100);
        let chained = conditional.and_then(mutator2);

        let mut positive = 10;
        chained.apply(&mut positive);
        assert_eq!(positive, 120); // 10 * 2 + 100

        let mut negative = -10;
        chained.apply(&mut negative);
        assert_eq!(negative, 90); // -10 + 100 (condition not met)
    }

    #[test]
    fn test_rc_conditional_mutator_and_then_with_closure() {
        let mutator = RcMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);
        let chained = conditional.and_then(|x: &mut i32| *x += 10);

        let mut positive = 5;
        chained.apply(&mut positive);
        assert_eq!(positive, 20); // 5 * 2 + 10

        let mut negative = -5;
        chained.apply(&mut negative);
        assert_eq!(negative, 5); // -5 + 10 (condition not met)
    }

    #[test]
    fn test_rc_conditional_mutator_and_then_with_rc_mutator() {
        let mutator1 = RcMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator1.when(|x: &i32| *x > 0);
        let mutator2 = RcMutator::new(|x: &mut i32| *x += 100);
        let chained = conditional.and_then(mutator2);

        let mut positive = 10;
        chained.apply(&mut positive);
        assert_eq!(positive, 120); // 10 * 2 + 100

        let mut negative = -10;
        chained.apply(&mut negative);
        assert_eq!(negative, 90); // -10 + 100 (condition not met)
    }

    #[test]
    fn test_arc_conditional_mutator_and_then_with_closure() {
        let mutator = ArcMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);
        let chained = conditional.and_then(|x: &mut i32| *x += 10);

        let mut positive = 5;
        chained.apply(&mut positive);
        assert_eq!(positive, 20); // 5 * 2 + 10

        let mut negative = -5;
        chained.apply(&mut negative);
        assert_eq!(negative, 5); // -5 + 10 (condition not met)
    }

    #[test]
    fn test_arc_conditional_mutator_and_then_with_arc_mutator() {
        let mutator1 = ArcMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator1.when(|x: &i32| *x > 0);
        let mutator2 = ArcMutator::new(|x: &mut i32| *x += 100);
        let chained = conditional.and_then(mutator2);

        let mut positive = 10;
        chained.apply(&mut positive);
        assert_eq!(positive, 120); // 10 * 2 + 100

        let mut negative = -10;
        chained.apply(&mut negative);
        assert_eq!(negative, 90); // -10 + 100 (condition not met)
    }
}

// ============================================================================
// Conditional Mutator Debug/Display Tests
// ============================================================================
