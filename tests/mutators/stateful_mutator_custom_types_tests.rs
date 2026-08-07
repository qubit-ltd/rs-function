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
mod test_custom_types {
    use super::BoxStatefulMutator;
    use super::StatefulMutator;

    #[derive(Debug, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[test]
    fn test_with_custom_struct() {
        let mut mutator = BoxStatefulMutator::new(|p: &mut Point| {
            p.x += 10;
            p.y += 10;
        });

        let mut point = Point { x: 5, y: 15 };
        mutator.apply(&mut point);
        assert_eq!(point, Point { x: 15, y: 25 });
    }

    #[test]
    fn test_chaining_with_custom_struct() {
        let mut processor = BoxStatefulMutator::new(|p: &mut Point| p.x *= 2)
            .and_then(|p: &mut Point| p.y *= 2)
            .and_then(|p: &mut Point| p.x += p.y);

        let mut point = Point { x: 3, y: 4 };
        processor.apply(&mut point);
        assert_eq!(point, Point { x: 14, y: 8 });
    }

    #[test]
    fn test_conditional_with_custom_struct() {
        let mut normalizer = BoxStatefulMutator::new(|p: &mut Point| {
            if p.x < 0 {
                p.x = 0;
            }
            if p.y < 0 {
                p.y = 0;
            }
        })
        .when(|p: &Point| p.x < 0 || p.y < 0);

        let mut point1 = Point { x: -5, y: 10 };
        normalizer.apply(&mut point1);
        assert_eq!(point1, Point { x: 0, y: 10 });

        let mut point2 = Point { x: 5, y: -10 };
        normalizer.apply(&mut point2);
        assert_eq!(point2, Point { x: 5, y: 0 });

        let mut point3 = Point { x: 5, y: 10 };
        normalizer.apply(&mut point3);
        assert_eq!(point3, Point { x: 5, y: 10 });
    }
}

// ============================================================================
// Edge Cases Tests
// ============================================================================
