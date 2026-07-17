// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Unit tests for StatefulMutator types

use qubit_function::{
    ArcStatefulMutator,
    BoxStatefulMutator,
    MutatorOnce,
    RcStatefulMutator,
    StatefulMutator,
};

// ============================================================================
// BoxStatefulMutator Tests
// ============================================================================

#[cfg(test)]
mod test_custom_mutator_default_impl {
    use super::{
        MutatorOnce,
        StatefulMutator,
    };

    /// Custom mutator for testing default implementations
    ///
    /// This mutator demonstrates using the default trait method implementations
    /// for `into_box()`, `into_rc()`, `into_arc()`, and `into_fn()`.
    #[derive(Clone)]
    struct DoubleStatefulMutator {
        multiplier: i32,
    }

    impl DoubleStatefulMutator {
        fn new(multiplier: i32) -> Self {
            Self { multiplier }
        }
    }

    impl StatefulMutator<i32> for DoubleStatefulMutator {
        fn apply(&mut self, value: &mut i32) {
            *value *= self.multiplier;
        }

        // Note: All into_xxx() methods use the default implementations from the
        // trait We don't need to implement them here
    }

    #[test]
    fn test_custom_mutator_basic() {
        let mut mutator = DoubleStatefulMutator::new(3);
        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 15);
    }

    /// Custom mutator with state to test stateful operations
    struct CountingStatefulMutator {
        count: i32,
    }

    impl CountingStatefulMutator {
        fn new() -> Self {
            Self { count: 0 }
        }
    }

    impl StatefulMutator<i32> for CountingStatefulMutator {
        fn apply(&mut self, value: &mut i32) {
            self.count += 1;
            *value += self.count;
        }
    }

    #[test]
    fn test_stateful_mutator() {
        let mut mutator = CountingStatefulMutator::new();

        let mut value1 = 10;
        mutator.apply(&mut value1);
        assert_eq!(value1, 11); // 10 + 1

        let mut value2 = 10;
        mutator.apply(&mut value2);
        assert_eq!(value2, 12); // 10 + 2

        let mut value3 = 10;
        mutator.apply(&mut value3);
        assert_eq!(value3, 13); // 10 + 3
    }

    /// Custom mutator with complex type
    #[derive(Debug, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    struct OffsetStatefulMutator {
        dx: i32,
        dy: i32,
    }

    impl OffsetStatefulMutator {
        fn new(dx: i32, dy: i32) -> Self {
            Self { dx, dy }
        }
    }

    impl StatefulMutator<Point> for OffsetStatefulMutator {
        fn apply(&mut self, point: &mut Point) {
            point.x += self.dx;
            point.y += self.dy;
        }
    }

    #[test]
    fn test_custom_mutator_with_complex_type() {
        let mut mutator = OffsetStatefulMutator::new(10, 20);
        let mut point = Point { x: 5, y: 15 };

        mutator.apply(&mut point);
        assert_eq!(point, Point { x: 15, y: 35 });
    }

    /// Generic custom mutator
    struct GenericStatefulMutator<F>
    where
        F: FnMut(&mut i32),
    {
        func: F,
    }

    impl<F> GenericStatefulMutator<F>
    where
        F: FnMut(&mut i32),
    {
        fn new(func: F) -> Self {
            Self { func }
        }
    }

    impl<F> StatefulMutator<i32> for GenericStatefulMutator<F>
    where
        F: FnMut(&mut i32),
    {
        fn apply(&mut self, value: &mut i32) {
            (self.func)(value);
        }
    }

    #[test]
    fn test_generic_custom_mutator() {
        let mut mutator = GenericStatefulMutator::new(|x: &mut i32| *x *= 3);
        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 15);
    }
}

// ============================================================================
// into_fn Tests
// ============================================================================

// ============================================================================
// Conditional Execution Tests (when/or_else with various parameter types)
// ============================================================================
