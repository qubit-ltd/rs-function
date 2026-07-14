// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

// qubit-style: allow explicit-imports
//! Unit tests for StatefulMutator types

use qubit_function::{
    ArcStatefulMutator,
    BoxStatefulMutator,
    FnMutStatefulMutatorOps,
    MutatorOnce,
    RcStatefulMutator,
    StatefulMutator,
};

// ============================================================================
// BoxStatefulMutator Tests
// ============================================================================

#[cfg(test)]
mod test_box_mutator {
    use super::{
        BoxStatefulMutator,
        StatefulMutator,
    };

    #[test]
    fn test_new() {
        let mut mutator = BoxStatefulMutator::new(|x: &mut i32| *x += 1);
        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }

    #[test]
    fn test_with_different_types() {
        // String
        let mut string_mutator =
            BoxStatefulMutator::new(|s: &mut String| s.push('!'));
        let mut text = String::from("hello");
        string_mutator.apply(&mut text);
        assert_eq!(text, "hello!");

        // Vec
        let mut vec_mutator =
            BoxStatefulMutator::new(|v: &mut Vec<i32>| v.push(42));
        let mut numbers = vec![1, 2, 3];
        vec_mutator.apply(&mut numbers);
        assert_eq!(numbers, vec![1, 2, 3, 42]);

        // bool
        let mut bool_mutator = BoxStatefulMutator::new(|b: &mut bool| *b = !*b);
        let mut flag = true;
        bool_mutator.apply(&mut flag);
        assert!(!flag);
    }

    #[test]
    fn test_and_then() {
        let mut mutator = BoxStatefulMutator::new(|x: &mut i32| *x *= 2)
            .and_then(|x: &mut i32| *x += 10);

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 20); // (5 * 2) + 10
    }

    #[test]
    fn test_and_then_multiple_chains() {
        let mut mutator = BoxStatefulMutator::new(|x: &mut i32| *x += 1)
            .and_then(|x: &mut i32| *x *= 2)
            .and_then(|x: &mut i32| *x -= 5);

        let mut value = 10;
        mutator.apply(&mut value);
        assert_eq!(value, 17); // ((10 + 1) * 2) - 5
    }

    #[test]
    fn test_and_then_with_box_consumer() {
        let c1 = BoxStatefulMutator::new(|x: &mut i32| *x *= 2);
        let c2 = BoxStatefulMutator::new(|x: &mut i32| *x += 10);
        let mut combined = c1.and_then(c2);

        let mut value = 5;
        combined.apply(&mut value);
        assert_eq!(value, 20);
    }

    #[test]
    fn test_noop() {
        let mut noop = BoxStatefulMutator::<i32>::noop();
        let mut value = 42;
        noop.apply(&mut value);
        assert_eq!(value, 42);
    }

    #[test]
    fn test_noop_with_different_types() {
        // Test with String
        let mut noop = BoxStatefulMutator::<String>::noop();
        let mut text = String::from("hello");
        noop.apply(&mut text);
        assert_eq!(text, "hello");

        // Test with Vec
        let mut noop = BoxStatefulMutator::<Vec<i32>>::noop();
        let mut numbers = vec![1, 2, 3];
        noop.apply(&mut numbers);
        assert_eq!(numbers, vec![1, 2, 3]);
    }

    #[test]
    fn test_noop_chaining() {
        let mut chained = BoxStatefulMutator::<i32>::noop()
            .and_then(|x: &mut i32| *x *= 2)
            .and_then(BoxStatefulMutator::<i32>::noop());

        let mut value = 5;
        chained.apply(&mut value);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_if_then_true() {
        let mut mutator = BoxStatefulMutator::new(|x: &mut i32| *x += 10)
            .when(|x: &i32| *x > 0);

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 15);
    }

    #[test]
    fn test_if_then_false() {
        let mut mutator = BoxStatefulMutator::new(|x: &mut i32| *x += 10)
            .when(|x: &i32| *x > 0);

        let mut value = -5;
        mutator.apply(&mut value);
        assert_eq!(value, -5); // unchanged
    }

    #[test]
    fn test_if_then_else() {
        let mut mutator = BoxStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0)
            .or_else(|x: &mut i32| *x = -*x);

        let mut positive = 10;
        mutator.apply(&mut positive);
        assert_eq!(positive, 20);

        let mut negative = -10;
        mutator.apply(&mut negative);
        assert_eq!(negative, 10);
    }

    #[test]
    fn test_new_with_name() {
        let mut mutator = BoxStatefulMutator::new_with_name(
            "box_stateful_test",
            |x: &mut i32| *x += 1,
        );
        assert_eq!(mutator.name(), Some("box_stateful_test"));

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }

    #[test]
    fn test_new_with_optional_name_some() {
        let mut mutator = BoxStatefulMutator::new_with_optional_name(
            |x: &mut i32| *x += 1,
            Some("box_optional".to_string()),
        );
        assert_eq!(mutator.name(), Some("box_optional"));

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }

    #[test]
    fn test_new_with_optional_name_none() {
        let mut mutator = BoxStatefulMutator::new_with_optional_name(
            |x: &mut i32| *x += 1,
            None,
        );
        assert_eq!(mutator.name(), None);

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }

    #[test]
    fn test_name_and_set_name() {
        let mut mutator = BoxStatefulMutator::new(|x: &mut i32| *x += 1);
        assert_eq!(mutator.name(), None);

        mutator.set_name("box_set_name");
        assert_eq!(mutator.name(), Some("box_set_name"));

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }

    // Note: BoxStatefulMutator cannot be safely converted to ArcStatefulMutator
    // because the inner function may not be Send. This test has been
    // removed.
}

// ============================================================================
// ArcStatefulMutator Tests
// ============================================================================

#[cfg(test)]
mod test_arc_mutator {
    use super::{
        ArcStatefulMutator,
        StatefulMutator,
    };
    use std::thread;

    #[test]
    fn test_new() {
        let mutator = ArcStatefulMutator::new(|x: &mut i32| *x += 1);
        let mut value = 5;
        let mut c = mutator;
        c.apply(&mut value);
        assert_eq!(value, 6);
    }

    #[test]
    fn test_clone() {
        let mutator = ArcStatefulMutator::new(|x: &mut i32| *x *= 2);
        let clone1 = mutator.clone();
        let clone2 = mutator.clone();

        let mut value1 = 5;
        let mut c1 = clone1;
        c1.apply(&mut value1);
        assert_eq!(value1, 10);

        let mut value2 = 3;
        let mut c2 = clone2;
        c2.apply(&mut value2);
        assert_eq!(value2, 6);
    }

    #[test]
    fn test_and_then() {
        let first = ArcStatefulMutator::new(|x: &mut i32| *x *= 2);
        let second = ArcStatefulMutator::new(|x: &mut i32| *x += 10);

        let chained = first.and_then(second);

        let mut value = 5;
        let mut c = chained;
        c.apply(&mut value);
        assert_eq!(value, 20); // (5 * 2) + 10

        // first and second are still usable
        let mut value2 = 3;
        let mut f = first;
        f.apply(&mut value2);
        assert_eq!(value2, 6);
    }

    #[test]
    fn test_thread_safety() {
        let mutator = ArcStatefulMutator::new(|x: &mut i32| *x *= 2);
        let clone = mutator.clone();

        let handle = thread::spawn(move || {
            let mut value = 5;
            let mut c = clone;
            c.apply(&mut value);
            value
        });

        let mut value = 3;
        let mut c = mutator;
        c.apply(&mut value);
        assert_eq!(value, 6);

        assert_eq!(handle.join().expect("thread should not panic"), 10);
    }

    #[test]
    fn test_noop() {
        let noop = ArcStatefulMutator::<i32>::noop();
        let mut value = 42;
        let mut m = noop;
        m.apply(&mut value);
        assert_eq!(value, 42);
    }

    #[test]
    fn test_noop_clone() {
        let noop = ArcStatefulMutator::<i32>::noop();
        let clone1 = noop.clone();
        let clone2 = noop.clone();

        let mut value1 = 42;
        let mut m1 = clone1;
        m1.apply(&mut value1);
        assert_eq!(value1, 42);

        let mut value2 = 100;
        let mut m2 = clone2;
        m2.apply(&mut value2);
        assert_eq!(value2, 100);
    }

    #[test]
    fn test_noop_chaining() {
        let noop = ArcStatefulMutator::<i32>::noop();
        let double = ArcStatefulMutator::new(|x: &mut i32| *x *= 2);

        let chained = noop.and_then(double);

        let mut value = 5;
        let mut c = chained;
        c.apply(&mut value);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_new_with_name() {
        let mut mutator = ArcStatefulMutator::new_with_name(
            "arc_stateful_test",
            |x: &mut i32| *x += 1,
        );
        assert_eq!(mutator.name(), Some("arc_stateful_test"));

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }

    #[test]
    fn test_new_with_optional_name_some() {
        let mut mutator = ArcStatefulMutator::new_with_optional_name(
            |x: &mut i32| *x += 1,
            Some("arc_stateful_optional".to_string()),
        );
        assert_eq!(mutator.name(), Some("arc_stateful_optional"));

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }

    #[test]
    fn test_new_with_optional_name_none() {
        let mut mutator = ArcStatefulMutator::new_with_optional_name(
            |x: &mut i32| *x += 1,
            None,
        );
        assert_eq!(mutator.name(), None);

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }

    #[test]
    fn test_name_and_set_name() {
        let mut mutator = ArcStatefulMutator::new(|x: &mut i32| *x += 1);
        assert_eq!(mutator.name(), None);

        mutator.set_name("arc_stateful_set_name");
        assert_eq!(mutator.name(), Some("arc_stateful_set_name"));

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }
}

// ============================================================================
// RcStatefulMutator Tests
// ============================================================================

#[cfg(test)]
mod test_rc_mutator {
    use super::{
        RcStatefulMutator,
        StatefulMutator,
    };

    #[test]
    fn test_new() {
        let mutator = RcStatefulMutator::new(|x: &mut i32| *x += 1);
        let mut value = 5;
        let mut c = mutator;
        c.apply(&mut value);
        assert_eq!(value, 6);
    }

    #[test]
    fn test_clone() {
        let mutator = RcStatefulMutator::new(|x: &mut i32| *x *= 2);
        let clone1 = mutator.clone();
        let clone2 = mutator.clone();

        let mut value1 = 5;
        let mut c1 = clone1;
        c1.apply(&mut value1);
        assert_eq!(value1, 10);

        let mut value2 = 3;
        let mut c2 = clone2;
        c2.apply(&mut value2);
        assert_eq!(value2, 6);
    }

    #[test]
    fn test_and_then() {
        let first = RcStatefulMutator::new(|x: &mut i32| *x *= 2);
        let second = RcStatefulMutator::new(|x: &mut i32| *x += 10);

        let chained = first.and_then(second.clone());

        let mut value = 5;
        let mut c = chained;
        c.apply(&mut value);
        assert_eq!(value, 20); // (5 * 2) + 10

        // first and second are still usable
        let mut value2 = 3;
        let mut f = first;
        f.apply(&mut value2);
        assert_eq!(value2, 6);
    }

    #[test]
    fn test_noop() {
        let noop = RcStatefulMutator::<i32>::noop();
        let mut value = 42;
        let mut m = noop;
        m.apply(&mut value);
        assert_eq!(value, 42);
    }

    #[test]
    fn test_noop_clone() {
        let noop = RcStatefulMutator::<i32>::noop();
        let clone1 = noop.clone();
        let clone2 = noop.clone();

        let mut value1 = 42;
        let mut m1 = clone1;
        m1.apply(&mut value1);
        assert_eq!(value1, 42);

        let mut value2 = 100;
        let mut m2 = clone2;
        m2.apply(&mut value2);
        assert_eq!(value2, 100);
    }

    #[test]
    fn test_noop_chaining() {
        let noop = RcStatefulMutator::<i32>::noop();
        let double = RcStatefulMutator::new(|x: &mut i32| *x *= 2);

        let chained = noop.and_then(double.clone());

        let mut value = 5;
        let mut c = chained;
        c.apply(&mut value);
        assert_eq!(value, 10);
    }

    // Note: RcStatefulMutator cannot be converted to ArcStatefulMutator because
    // Rc is not Send. This test has been removed.

    #[test]
    fn test_new_with_name() {
        let mut mutator = RcStatefulMutator::new_with_name(
            "rc_stateful_test",
            |x: &mut i32| *x += 1,
        );
        assert_eq!(mutator.name(), Some("rc_stateful_test"));

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }

    #[test]
    fn test_new_with_optional_name_some() {
        let mut mutator = RcStatefulMutator::new_with_optional_name(
            |x: &mut i32| *x += 1,
            Some("rc_stateful_optional".to_string()),
        );
        assert_eq!(mutator.name(), Some("rc_stateful_optional"));

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }

    #[test]
    fn test_new_with_optional_name_none() {
        let mut mutator = RcStatefulMutator::new_with_optional_name(
            |x: &mut i32| *x += 1,
            None,
        );
        assert_eq!(mutator.name(), None);

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }

    #[test]
    fn test_name_and_set_name() {
        let mut mutator = RcStatefulMutator::new(|x: &mut i32| *x += 1);
        assert_eq!(mutator.name(), None);

        mutator.set_name("rc_stateful_set_name");
        assert_eq!(mutator.name(), Some("rc_stateful_set_name"));

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }
}

// ============================================================================
// Closure Extension Methods Tests
// ============================================================================

#[cfg(test)]
mod test_fn_mutator_ops {
    use super::{
        FnMutStatefulMutatorOps,
        MutatorOnce,
        StatefulMutator,
    };

    #[test]
    fn test_closure_accept() {
        let closure = |x: &mut i32| *x *= 2;
        let mut value = 5;
        closure.apply(&mut value);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_closure_and_then() {
        let mut chained =
            (|x: &mut i32| *x *= 2).and_then(|x: &mut i32| *x += 10);

        let mut value = 5;
        chained.apply(&mut value);
        assert_eq!(value, 20); // (5 * 2) + 10
    }
}

// ============================================================================
// Unified Interface Tests
// ============================================================================

#[cfg(test)]
mod test_unified_interface {
    use super::{
        ArcStatefulMutator,
        BoxStatefulMutator,
        RcStatefulMutator,
        StatefulMutator,
    };

    fn apply_mutator<C: StatefulMutator<i32>>(
        mutator: &mut C,
        value: i32,
    ) -> i32 {
        let mut val = value;
        mutator.apply(&mut val);
        val
    }

    #[test]
    fn test_with_box_consumer() {
        let mut mutator = BoxStatefulMutator::new(|x: &mut i32| *x *= 2);
        assert_eq!(apply_mutator(&mut mutator, 5), 10);
    }

    #[test]
    fn test_with_arc_consumer() {
        let mut mutator = ArcStatefulMutator::new(|x: &mut i32| *x *= 2);
        assert_eq!(apply_mutator(&mut mutator, 5), 10);
    }

    #[test]
    fn test_with_rc_consumer() {
        let mut mutator = RcStatefulMutator::new(|x: &mut i32| *x *= 2);
        assert_eq!(apply_mutator(&mut mutator, 5), 10);
    }

    #[test]
    fn test_with_closure() {
        let mut closure = |x: &mut i32| *x *= 2;
        assert_eq!(apply_mutator(&mut closure, 5), 10);
    }
}

// ============================================================================
// Complex Scenarios Tests
// ============================================================================

#[cfg(test)]
mod test_complex_scenarios {
    use super::{
        ArcStatefulMutator,
        BoxStatefulMutator,
        RcStatefulMutator,
        StatefulMutator,
    };

    #[test]
    fn test_data_processing_pipeline() {
        let mut pipeline = BoxStatefulMutator::new(|x: &mut i32| {
            *x = (*x).clamp(0, 100);
        })
        .and_then(|x: &mut i32| *x /= 10)
        .and_then(|x: &mut i32| *x = *x * *x);

        let mut value1 = -50;
        pipeline.apply(&mut value1);
        assert_eq!(value1, 0);

        let mut value2 = 200;
        pipeline.apply(&mut value2);
        assert_eq!(value2, 100);

        let mut value3 = 30;
        pipeline.apply(&mut value3);
        assert_eq!(value3, 9);
    }

    #[test]
    fn test_string_processing() {
        let mut processor = BoxStatefulMutator::new(|s: &mut String| {
            s.retain(|c| !c.is_whitespace())
        })
        .and_then(|s: &mut String| *s = s.to_lowercase())
        .and_then(|s: &mut String| s.push_str("!!!"));

        let mut text = String::from("Hello World");
        processor.apply(&mut text);
        assert_eq!(text, "helloworld!!!");
    }

    #[test]
    fn test_conditional_processing() {
        let cond1 = BoxStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0);
        let cond2 = BoxStatefulMutator::new(|x: &mut i32| *x = 100)
            .when(|x: &i32| *x > 100);
        let mut processor = cond1.and_then(cond2);

        let mut small = 5;
        processor.apply(&mut small);
        assert_eq!(small, 10);

        let mut large = 60;
        processor.apply(&mut large);
        assert_eq!(large, 100);
    }

    #[test]
    fn test_mixed_operations() {
        let cond = BoxStatefulMutator::new(|x: &mut i32| *x -= 20)
            .when(|x: &i32| *x > 50);
        let mut processor = BoxStatefulMutator::new(|x: &mut i32| *x += 10)
            .and_then(|x: &mut i32| *x *= 2)
            .and_then(cond);

        let mut value1 = 5;
        processor.apply(&mut value1);
        assert_eq!(value1, 30); // (5 + 10) * 2 = 30

        let mut value2 = 20;
        processor.apply(&mut value2);
        assert_eq!(value2, 40); // (20 + 10) * 2 = 60, 60 > 50 so 60 - 20 = 40
    }

    #[test]
    fn test_arc_mutator_reuse() {
        let double = ArcStatefulMutator::new(|x: &mut i32| *x *= 2);
        let add_ten = ArcStatefulMutator::new(|x: &mut i32| *x += 10);

        let pipeline1 = double.and_then(add_ten.clone());
        let pipeline2 = add_ten.and_then(double.clone());

        let mut value1 = 5;
        let mut p1 = pipeline1;
        p1.apply(&mut value1);
        assert_eq!(value1, 20); // (5 * 2) + 10

        let mut value2 = 5;
        let mut p2 = pipeline2;
        p2.apply(&mut value2);
        assert_eq!(value2, 30); // (5 + 10) * 2
    }

    #[test]
    fn test_rc_mutator_reuse() {
        let double = RcStatefulMutator::new(|x: &mut i32| *x *= 2);
        let add_ten = RcStatefulMutator::new(|x: &mut i32| *x += 10);

        let pipeline1 = double.and_then(add_ten.clone());
        let pipeline2 = add_ten.and_then(double.clone());

        let mut value1 = 5;
        let mut p1 = pipeline1;
        p1.apply(&mut value1);
        assert_eq!(value1, 20); // (5 * 2) + 10

        let mut value2 = 5;
        let mut p2 = pipeline2;
        p2.apply(&mut value2);
        assert_eq!(value2, 30); // (5 + 10) * 2
    }
}

// ============================================================================
// Custom Types Tests
// ============================================================================

#[cfg(test)]
mod test_custom_types {
    use super::{
        BoxStatefulMutator,
        StatefulMutator,
    };

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

#[cfg(test)]
mod test_edge_cases {
    use super::{
        BoxStatefulMutator,
        StatefulMutator,
    };

    #[test]
    fn test_with_zero() {
        let mut mutator = BoxStatefulMutator::new(|x: &mut i32| *x += 1);
        let mut value = 0;
        mutator.apply(&mut value);
        assert_eq!(value, 1);
    }

    #[test]
    fn test_with_negative() {
        let mut mutator = BoxStatefulMutator::new(|x: &mut i32| *x = x.abs());
        let mut value = -42;
        mutator.apply(&mut value);
        assert_eq!(value, 42);
    }

    #[test]
    fn test_with_max_value() {
        let mut mutator =
            BoxStatefulMutator::new(|x: &mut i32| *x = x.saturating_add(1));
        let mut value = i32::MAX;
        mutator.apply(&mut value);
        assert_eq!(value, i32::MAX);
    }

    #[test]
    fn test_with_min_value() {
        let mut mutator =
            BoxStatefulMutator::new(|x: &mut i32| *x = x.saturating_sub(1));
        let mut value = i32::MIN;
        mutator.apply(&mut value);
        assert_eq!(value, i32::MIN);
    }

    #[test]
    fn test_with_empty_string() {
        let mut mutator =
            BoxStatefulMutator::new(|s: &mut String| s.push_str("added"));
        let mut text = String::new();
        mutator.apply(&mut text);
        assert_eq!(text, "added");
    }

    #[test]
    fn test_with_empty_vec() {
        let mut mutator = BoxStatefulMutator::new(|v: &mut Vec<i32>| v.push(1));
        let mut numbers = Vec::new();
        mutator.apply(&mut numbers);
        assert_eq!(numbers, vec![1]);
    }

    #[test]
    fn test_unicode() {
        let mut mutator =
            BoxStatefulMutator::new(|s: &mut String| *s = s.to_uppercase());
        let mut text = String::from("héllo world");
        mutator.apply(&mut text);
        assert_eq!(text, "HÉLLO WORLD");
    }
}

// ============================================================================
// Custom StatefulMutator with Default into_xxx() Implementation Tests
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

#[cfg(test)]
mod test_conditional_execution {
    use super::{
        ArcStatefulMutator,
        BoxStatefulMutator,
        RcStatefulMutator,
        StatefulMutator,
    };
    use qubit_function::predicates::{
        ArcPredicate,
        BoxPredicate,
        RcPredicate,
    };

    // Helper function pointer for testing
    fn is_positive(x: &i32) -> bool {
        *x > 0
    }

    fn negate(x: &mut i32) {
        *x = -*x;
    }

    // ========================================================================
    // BoxStatefulMutator::when() tests
    // ========================================================================

    #[test]
    fn test_box_when_with_closure() {
        let mut mutator = BoxStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0);

        let mut positive = 5;
        mutator.apply(&mut positive);
        assert_eq!(positive, 10);

        let mut negative = -5;
        mutator.apply(&mut negative);
        assert_eq!(negative, -5);
    }

    #[test]
    fn test_box_when_with_function_pointer() {
        let mut mutator = BoxStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(is_positive as fn(&i32) -> bool);

        let mut positive = 5;
        mutator.apply(&mut positive);
        assert_eq!(positive, 10);

        let mut negative = -5;
        mutator.apply(&mut negative);
        assert_eq!(negative, -5);
    }

    #[test]
    fn test_box_when_with_box_predicate() {
        let pred = BoxPredicate::new(|x: &i32| *x > 0);
        let mut mutator =
            BoxStatefulMutator::new(|x: &mut i32| *x *= 2).when(pred);

        let mut positive = 5;
        mutator.apply(&mut positive);
        assert_eq!(positive, 10);

        let mut negative = -5;
        mutator.apply(&mut negative);
        assert_eq!(negative, -5);
    }

    #[test]
    fn test_box_when_with_rc_predicate() {
        let pred = RcPredicate::new(|x: &i32| *x > 0);
        let mut mutator =
            BoxStatefulMutator::new(|x: &mut i32| *x *= 2).when(pred);

        let mut positive = 5;
        mutator.apply(&mut positive);
        assert_eq!(positive, 10);

        let mut negative = -5;
        mutator.apply(&mut negative);
        assert_eq!(negative, -5);
    }

    #[test]
    fn test_box_when_with_arc_predicate() {
        let pred = ArcPredicate::new(|x: &i32| *x > 0);
        let mut mutator =
            BoxStatefulMutator::new(|x: &mut i32| *x *= 2).when(pred);

        let mut positive = 5;
        mutator.apply(&mut positive);
        assert_eq!(positive, 10);

        let mut negative = -5;
        mutator.apply(&mut negative);
        assert_eq!(negative, -5);
    }

    // ========================================================================
    // BoxConditionalStatefulMutator::or_else() tests
    // ========================================================================

    #[test]
    fn test_box_or_else_with_closure() {
        let mut mutator = BoxStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0)
            .or_else(|x: &mut i32| *x -= 1);

        let mut positive = 5;
        mutator.apply(&mut positive);
        assert_eq!(positive, 10);

        let mut negative = -5;
        mutator.apply(&mut negative);
        assert_eq!(negative, -6);
    }

    #[test]
    fn test_box_or_else_with_function_pointer() {
        let mut mutator = BoxStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0)
            .or_else(negate as fn(&mut i32));

        let mut positive = 5;
        mutator.apply(&mut positive);
        assert_eq!(positive, 10);

        let mut negative = -5;
        mutator.apply(&mut negative);
        assert_eq!(negative, 5);
    }

    #[test]
    fn test_box_or_else_with_box_mutator() {
        let else_mutator = BoxStatefulMutator::new(|x: &mut i32| *x = 0);
        let mut mutator = BoxStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0)
            .or_else(else_mutator);

        let mut positive = 5;
        mutator.apply(&mut positive);
        assert_eq!(positive, 10);

        let mut negative = -5;
        mutator.apply(&mut negative);
        assert_eq!(negative, 0);
    }

    #[test]
    fn test_box_or_else_with_rc_mutator() {
        let else_mutator = RcStatefulMutator::new(|x: &mut i32| *x = 100);
        let mut mutator = BoxStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0)
            .or_else(else_mutator);

        let mut positive = 5;
        mutator.apply(&mut positive);
        assert_eq!(positive, 10);

        let mut negative = -5;
        mutator.apply(&mut negative);
        assert_eq!(negative, 100);
    }

    #[test]
    fn test_box_or_else_with_arc_mutator() {
        let else_mutator = ArcStatefulMutator::new(|x: &mut i32| *x = 200);
        let mut mutator = BoxStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0)
            .or_else(else_mutator);

        let mut positive = 5;
        mutator.apply(&mut positive);
        assert_eq!(positive, 10);

        let mut negative = -5;
        mutator.apply(&mut negative);
        assert_eq!(negative, 200);
    }

    // ========================================================================
    // BoxConditionalStatefulMutator::and_then() tests
    // ========================================================================

    #[test]
    fn test_box_conditional_and_then_with_closure() {
        let cond1 = BoxStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0);
        let mut chained = cond1.and_then(|x: &mut i32| *x += 10);

        let mut positive = 5;
        chained.apply(&mut positive);
        assert_eq!(positive, 20); // 5 * 2 + 10

        let mut negative = -5;
        chained.apply(&mut negative);
        assert_eq!(negative, 5); // -5 + 10 (not doubled)
    }

    #[test]
    fn test_box_conditional_and_then_with_box_mutator() {
        let cond1 = BoxStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0);
        let next = BoxStatefulMutator::new(|x: &mut i32| *x += 100);
        let mut chained = cond1.and_then(next);

        let mut positive = 10;
        chained.apply(&mut positive);
        assert_eq!(positive, 120); // 10 * 2 + 100

        let mut negative = -10;
        chained.apply(&mut negative);
        assert_eq!(negative, 90); // -10 + 100 (not doubled)
    }

    #[test]
    fn test_box_conditional_and_then_conditional() {
        let cond1 = BoxStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0);
        let cond2 = BoxStatefulMutator::new(|x: &mut i32| *x = 100)
            .when(|x: &i32| *x > 100);
        let mut chained = cond1.and_then(cond2);

        let mut small = 5;
        chained.apply(&mut small);
        assert_eq!(small, 10); // 5 * 2 = 10 (< 100, not capped)

        let mut large = 60;
        chained.apply(&mut large);
        assert_eq!(large, 100); // 60 * 2 = 120 (> 100, capped)
    }

    // ========================================================================
    // RcConditionalStatefulMutator::and_then() tests
    // ========================================================================

    #[test]
    fn test_rc_conditional_and_then_with_closure() {
        let conditional = RcStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0);
        let mut chained = conditional.and_then(|x: &mut i32| *x += 10);

        let mut positive = 5;
        chained.apply(&mut positive);
        assert_eq!(positive, 20); // 5 * 2 + 10

        let mut negative = -5;
        chained.apply(&mut negative);
        assert_eq!(negative, 5); // -5 + 10 (condition not met)
    }

    #[test]
    fn test_rc_conditional_and_then_with_rc_mutator() {
        let conditional = RcStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0);
        let next = RcStatefulMutator::new(|x: &mut i32| *x += 100);
        let mut chained = conditional.and_then(next);

        let mut positive = 10;
        chained.apply(&mut positive);
        assert_eq!(positive, 120); // 10 * 2 + 100

        let mut negative = -10;
        chained.apply(&mut negative);
        assert_eq!(negative, 90); // -10 + 100 (condition not met)
    }

    // ========================================================================
    // ArcConditionalStatefulMutator::and_then() tests
    // ========================================================================

    #[test]
    fn test_arc_conditional_and_then_with_closure() {
        let conditional = ArcStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0);
        let mut chained = conditional.and_then(|x: &mut i32| *x += 10);

        let mut positive = 5;
        chained.apply(&mut positive);
        assert_eq!(positive, 20); // 5 * 2 + 10

        let mut negative = -5;
        chained.apply(&mut negative);
        assert_eq!(negative, 5); // -5 + 10 (condition not met)
    }

    #[test]
    fn test_arc_conditional_and_then_with_arc_mutator() {
        let conditional = ArcStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0);
        let next = ArcStatefulMutator::new(|x: &mut i32| *x += 100);
        let mut chained = conditional.and_then(next);

        let mut positive = 10;
        chained.apply(&mut positive);
        assert_eq!(positive, 120); // 10 * 2 + 100

        let mut negative = -10;
        chained.apply(&mut negative);
        assert_eq!(negative, 90); // -10 + 100 (condition not met)
    }

    // ========================================================================
    // RcStatefulMutator::when() tests
    // ========================================================================

    #[test]
    fn test_rc_when_with_closure() {
        let conditional = RcStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0);
        let mut m = conditional.clone();

        let mut positive = 5;
        m.apply(&mut positive);
        assert_eq!(positive, 10);

        let mut negative = -5;
        m.apply(&mut negative);
        assert_eq!(negative, -5);
    }

    #[test]
    fn test_rc_when_with_function_pointer() {
        let conditional = RcStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(is_positive as fn(&i32) -> bool);
        let mut m = conditional.clone();

        let mut positive = 5;
        m.apply(&mut positive);
        assert_eq!(positive, 10);

        let mut negative = -5;
        m.apply(&mut negative);
        assert_eq!(negative, -5);
    }

    #[test]
    fn test_rc_when_with_rc_predicate() {
        let pred = RcPredicate::new(|x: &i32| *x > 0);
        let conditional =
            RcStatefulMutator::new(|x: &mut i32| *x *= 2).when(pred);
        let mut m = conditional.clone();

        let mut positive = 5;
        m.apply(&mut positive);
        assert_eq!(positive, 10);

        let mut negative = -5;
        m.apply(&mut negative);
        assert_eq!(negative, -5);
    }

    #[test]
    fn test_rc_when_with_box_predicate() {
        let pred = BoxPredicate::new(|x: &i32| *x > 0);
        let conditional =
            RcStatefulMutator::new(|x: &mut i32| *x *= 2).when(pred);
        let mut m = conditional.clone();

        let mut positive = 5;
        m.apply(&mut positive);
        assert_eq!(positive, 10);

        let mut negative = -5;
        m.apply(&mut negative);
        assert_eq!(negative, -5);
    }

    // ========================================================================
    // RcConditionalStatefulMutator::or_else() tests
    // ========================================================================

    #[test]
    fn test_rc_or_else_with_closure() {
        let mut mutator = RcStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0)
            .or_else(|x: &mut i32| *x -= 1);

        let mut positive = 5;
        mutator.apply(&mut positive);
        assert_eq!(positive, 10);

        let mut negative = -5;
        mutator.apply(&mut negative);
        assert_eq!(negative, -6);
    }

    #[test]
    fn test_rc_or_else_with_function_pointer() {
        let mut mutator = RcStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0)
            .or_else(negate as fn(&mut i32));

        let mut positive = 5;
        mutator.apply(&mut positive);
        assert_eq!(positive, 10);

        let mut negative = -5;
        mutator.apply(&mut negative);
        assert_eq!(negative, 5);
    }

    #[test]
    fn test_rc_or_else_with_rc_mutator() {
        let else_mutator = RcStatefulMutator::new(|x: &mut i32| *x = 100);
        let mut mutator = RcStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0)
            .or_else(else_mutator);

        let mut positive = 5;
        mutator.apply(&mut positive);
        assert_eq!(positive, 10);

        let mut negative = -5;
        mutator.apply(&mut negative);
        assert_eq!(negative, 100);
    }

    #[test]
    fn test_rc_or_else_with_box_mutator() {
        let else_mutator = BoxStatefulMutator::new(|x: &mut i32| *x = 200);
        let mut mutator = RcStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0)
            .or_else(else_mutator);

        let mut positive = 5;
        mutator.apply(&mut positive);
        assert_eq!(positive, 10);

        let mut negative = -5;
        mutator.apply(&mut negative);
        assert_eq!(negative, 200);
    }

    // ========================================================================
    // RcConditionalStatefulMutator::clone() tests
    // ========================================================================

    #[test]
    fn test_rc_conditional_clone() {
        let conditional = RcStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0);
        let mut clone1 = conditional.clone();
        let mut clone2 = conditional.clone();

        let mut value1 = 5;
        clone1.apply(&mut value1);
        assert_eq!(value1, 10);

        let mut value2 = -5;
        clone2.apply(&mut value2);
        assert_eq!(value2, -5);
    }

    // ========================================================================
    // ArcStatefulMutator::when() tests
    // ========================================================================

    #[test]
    fn test_arc_when_with_closure() {
        let conditional = ArcStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0);
        let mut m = conditional.clone();

        let mut positive = 5;
        m.apply(&mut positive);
        assert_eq!(positive, 10);

        let mut negative = -5;
        m.apply(&mut negative);
        assert_eq!(negative, -5);
    }

    #[test]
    fn test_arc_when_with_function_pointer() {
        let conditional = ArcStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(is_positive as fn(&i32) -> bool);
        let mut m = conditional.clone();

        let mut positive = 5;
        m.apply(&mut positive);
        assert_eq!(positive, 10);

        let mut negative = -5;
        m.apply(&mut negative);
        assert_eq!(negative, -5);
    }

    #[test]
    fn test_arc_when_with_arc_predicate() {
        let pred = ArcPredicate::new(|x: &i32| *x > 0);
        let conditional =
            ArcStatefulMutator::new(|x: &mut i32| *x *= 2).when(pred);
        let mut m = conditional.clone();

        let mut positive = 5;
        m.apply(&mut positive);
        assert_eq!(positive, 10);

        let mut negative = -5;
        m.apply(&mut negative);
        assert_eq!(negative, -5);
    }

    // ========================================================================
    // ArcConditionalStatefulMutator::or_else() tests
    // ========================================================================

    #[test]
    fn test_arc_or_else_with_closure() {
        let mut mutator = ArcStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0)
            .or_else(|x: &mut i32| *x -= 1);

        let mut positive = 5;
        mutator.apply(&mut positive);
        assert_eq!(positive, 10);

        let mut negative = -5;
        mutator.apply(&mut negative);
        assert_eq!(negative, -6);
    }

    #[test]
    fn test_arc_or_else_with_function_pointer() {
        let mut mutator = ArcStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0)
            .or_else(negate as fn(&mut i32));

        let mut positive = 5;
        mutator.apply(&mut positive);
        assert_eq!(positive, 10);

        let mut negative = -5;
        mutator.apply(&mut negative);
        assert_eq!(negative, 5);
    }

    #[test]
    fn test_arc_or_else_with_arc_mutator() {
        let else_mutator = ArcStatefulMutator::new(|x: &mut i32| *x = 100);
        let mut mutator = ArcStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0)
            .or_else(else_mutator);

        let mut positive = 5;
        mutator.apply(&mut positive);
        assert_eq!(positive, 10);

        let mut negative = -5;
        mutator.apply(&mut negative);
        assert_eq!(negative, 100);
    }

    // Note: BoxStatefulMutator is not Send, so it cannot be used with
    // ArcStatefulMutator::or_else()

    // ========================================================================
    // ArcConditionalStatefulMutator::clone() tests
    // ========================================================================

    #[test]
    fn test_arc_conditional_clone() {
        let conditional = ArcStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0);
        let mut clone1 = conditional.clone();
        let mut clone2 = conditional.clone();

        let mut value1 = 5;
        clone1.apply(&mut value1);
        assert_eq!(value1, 10);

        let mut value2 = -5;
        clone2.apply(&mut value2);
        assert_eq!(value2, -5);
    }

    // ========================================================================
    // Thread safety tests for ArcConditionalStatefulMutator
    // ========================================================================

    #[test]
    fn test_arc_conditional_thread_safety() {
        use std::thread;

        let conditional = ArcStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0);
        let clone = conditional.clone();

        let handle = thread::spawn(move || {
            let mut value = 5;
            let mut m = clone;
            m.apply(&mut value);
            value
        });

        let mut value = -5;
        let mut m = conditional;
        m.apply(&mut value);
        assert_eq!(value, -5);

        assert_eq!(handle.join().expect("thread should not panic"), 10);
    }

    #[test]
    fn test_arc_or_else_thread_safety() {
        use std::thread;

        let mutator = ArcStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0)
            .or_else(|x: &mut i32| *x = 0);

        let clone = mutator.clone();

        let handle = thread::spawn(move || {
            let mut value = -5;
            let mut m = clone;
            m.apply(&mut value);
            value
        });

        let mut value = 5;
        let mut m = mutator;
        m.apply(&mut value);
        assert_eq!(value, 10);

        assert_eq!(handle.join().expect("thread should not panic"), 0);
    }

    // ========================================================================
    // Type conversion tests for ConditionalStatefulMutator
    // ========================================================================

    // ========================================================================
    // into_fn tests for ConditionalStatefulMutator
    // ========================================================================

    // ========================================================================
    // to_xxx tests for RcConditionalStatefulMutator
    // ========================================================================

    // ========================================================================
    // to_xxx tests for ArcConditionalStatefulMutator
    // ========================================================================

    // ========================================================================
    // Complex conditional composition tests
    // ========================================================================

    #[test]
    fn test_nested_conditionals() {
        // When x > 0: multiply by 2, then if result > 10: cap at 10
        let mut mutator = BoxStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0)
            .and_then(
                BoxStatefulMutator::new(|x: &mut i32| *x = 10)
                    .when(|x: &i32| *x > 10),
            );

        let mut small = 3;
        mutator.apply(&mut small);
        assert_eq!(small, 6); // 3 * 2 = 6 (not capped)

        let mut medium = 5;
        mutator.apply(&mut medium);
        assert_eq!(medium, 10); // 5 * 2 = 10 (not capped)

        let mut large = 8;
        mutator.apply(&mut large);
        assert_eq!(large, 10); // 8 * 2 = 16 -> capped to 10

        let mut negative = -5;
        mutator.apply(&mut negative);
        assert_eq!(negative, -5); // Not doubled (condition failed)
    }

    #[test]
    fn test_or_else_chaining() {
        // If positive: double, else: triple
        let mut mutator = BoxStatefulMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0)
            .or_else(|x: &mut i32| *x *= 3);

        let mut positive = 5;
        mutator.apply(&mut positive);
        assert_eq!(positive, 10);

        let mut negative = -5;
        mutator.apply(&mut negative);
        assert_eq!(negative, -15);

        let mut zero = 0;
        mutator.apply(&mut zero);
        assert_eq!(zero, 0); // 0 * 3
    }

    #[test]
    fn test_combined_predicate_types() {
        use qubit_function::predicates::FnPredicateOps;

        // Combine predicates: x > 0 AND x < 100
        let pred = (|x: &i32| *x > 0).and(|x: &i32| *x < 100);
        let mut mutator =
            BoxStatefulMutator::new(|x: &mut i32| *x *= 2).when(pred);

        let mut in_range = 50;
        mutator.apply(&mut in_range);
        assert_eq!(in_range, 100); // Doubled

        let mut too_small = -10;
        mutator.apply(&mut too_small);
        assert_eq!(too_small, -10); // Not doubled

        let mut too_large = 150;
        mutator.apply(&mut too_large);
        assert_eq!(too_large, 150); // Not doubled
    }
}

// ============================================================================
// Conditional Stateful Mutator Debug/Display Tests
// ============================================================================

#[cfg(test)]
mod test_conditional_stateful_mutator_debug_display {
    use super::{
        ArcStatefulMutator,
        BoxStatefulMutator,
        RcStatefulMutator,
    };

    #[test]
    fn test_box_conditional_stateful_mutator_debug() {
        let mutator = BoxStatefulMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("BoxConditionalStatefulMutator"));
        assert!(debug_str.contains("BoxStatefulMutator"));
        assert!(debug_str.contains("BoxPredicate"));
    }

    #[test]
    fn test_box_conditional_stateful_mutator_display() {
        let mutator = BoxStatefulMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let display_str = format!("{}", conditional);
        assert!(display_str.contains("BoxConditionalStatefulMutator"));
    }

    #[test]
    fn test_rc_conditional_stateful_mutator_debug() {
        let mutator = RcStatefulMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("RcConditionalStatefulMutator"));
        assert!(debug_str.contains("RcStatefulMutator"));
        assert!(debug_str.contains("RcPredicate"));
    }

    #[test]
    fn test_rc_conditional_stateful_mutator_display() {
        let mutator = RcStatefulMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let display_str = format!("{}", conditional);
        assert!(display_str.contains("RcConditionalStatefulMutator"));
    }

    #[test]
    fn test_arc_conditional_stateful_mutator_debug() {
        let mutator = ArcStatefulMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("ArcConditionalStatefulMutator"));
        assert!(debug_str.contains("ArcStatefulMutator"));
        assert!(debug_str.contains("ArcPredicate"));
    }

    #[test]
    fn test_arc_conditional_stateful_mutator_display() {
        let mutator = ArcStatefulMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let display_str = format!("{}", conditional);
        assert!(display_str.contains("ArcConditionalStatefulMutator"));
    }
}
