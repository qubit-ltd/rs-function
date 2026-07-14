// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

// qubit-style: allow explicit-imports

// ============================================================================
// BoxTransformer Tests - Immutable, single ownership
// ============================================================================

#[cfg(test)]
mod box_transformer_tests {
    use qubit_function::{
        BoxTransformer,
        Transformer,
    };

    #[test]
    fn test_new_and_apply() {
        let double = BoxTransformer::new(|x: i32| x * 2);
        assert_eq!(double.apply(21), 42);
    }

    #[test]
    fn test_multiple_calls() {
        let double = BoxTransformer::new(|x: i32| x * 2);
        assert_eq!(double.apply(21), 42);
        assert_eq!(double.apply(42), 84);
        assert_eq!(double.apply(10), 20);
    }

    #[test]
    fn test_identity() {
        let identity = BoxTransformer::<i32, i32>::identity();
        assert_eq!(identity.apply(42), 42);
    }

    #[test]
    fn test_constant() {
        let constant = BoxTransformer::constant("hello");
        assert_eq!(constant.apply(123), "hello");
        assert_eq!(constant.apply(456), "hello");
    }

    #[test]
    fn test_with_string() {
        let len = BoxTransformer::new(|s: String| s.len());
        let text = "hello".to_string();
        assert_eq!(len.apply(text), 5);
        // Note: text is consumed by transform
    }

    #[test]
    fn test_captured_variable() {
        let multiplier = 3;
        let multiply = BoxTransformer::new(move |x: i32| x * multiplier);
        assert_eq!(multiply.apply(7), 21);
    }

    #[test]
    fn test_and_then() {
        let double = BoxTransformer::new(|x: i32| x * 2);
        let to_string = BoxTransformer::new(|x: i32| x.to_string());
        let composed = double.and_then(to_string);
        assert_eq!(composed.apply(21), "42");
    }

    #[test]
    fn test_compose() {
        let double = BoxTransformer::new(|x: i32| x * 2);
        let add_one = BoxTransformer::new(|x: i32| x + 1);
        let composed = add_one.and_then(double);
        assert_eq!(composed.apply(5), 12); // (5 + 1) * 2
    }

    #[test]
    fn test_display_with_name() {
        let transformer =
            BoxTransformer::new_with_name("double", |x: i32| x * 2);
        let display_str = format!("{}", transformer);
        assert_eq!(display_str, "BoxTransformer(double)");
    }

    #[test]
    fn test_display_without_name() {
        let transformer = BoxTransformer::new(|x: i32| x * 2);
        let display_str = format!("{}", transformer);
        assert_eq!(display_str, "BoxTransformer");
    }
}

// ============================================================================
// ArcTransformer Tests - Immutable, thread-safe
// ============================================================================

#[cfg(test)]
mod arc_transformer_tests {
    use qubit_function::{
        ArcTransformer,
        Transformer,
    };
    use std::thread;

    #[test]
    fn test_new_and_apply() {
        let double = ArcTransformer::new(|x: i32| x * 2);
        assert_eq!(double.apply(21), 42);
    }

    #[test]
    fn test_clone() {
        let double = ArcTransformer::new(|x: i32| x * 2);
        let cloned = double.clone();

        assert_eq!(double.apply(21), 42);
        assert_eq!(cloned.apply(21), 42);
    }

    #[test]
    fn test_thread_safe() {
        let double = ArcTransformer::new(|x: i32| x * 2);
        let cloned = double.clone();

        let handle = thread::spawn(move || cloned.apply(21));

        assert_eq!(handle.join().expect("thread should not panic"), 42);
        assert_eq!(double.apply(21), 42);
    }

    #[test]
    fn test_identity() {
        let identity = ArcTransformer::<i32, i32>::identity();
        assert_eq!(identity.apply(42), 42);
    }

    #[test]
    fn test_constant() {
        let constant = ArcTransformer::constant("hello");
        assert_eq!(constant.apply(123), "hello");
    }

    #[test]
    fn test_multiple_threads() {
        let square = ArcTransformer::new(|x: i32| x * x);

        let handles: Vec<_> = (0..4)
            .map(|i| {
                let sq = square.clone();
                thread::spawn(move || sq.apply(i))
            })
            .collect();

        let results: Vec<_> = handles
            .into_iter()
            .map(|h| h.join().expect("thread should not panic"))
            .collect();

        assert_eq!(results, vec![0, 1, 4, 9]);
    }

    #[test]
    fn test_and_then() {
        let double = ArcTransformer::new(|x: i32| x * 2);
        let to_string = ArcTransformer::new(|x: i32| x.to_string());
        let composed = double.and_then(to_string);

        // Original double transformer still usable
        assert_eq!(double.apply(21), 42);
        assert_eq!(composed.apply(21), "42");
    }

    #[test]
    fn test_compose() {
        let double = ArcTransformer::new(|x: i32| x * 2);
        let add_one = ArcTransformer::new(|x: i32| x + 1);
        let composed = add_one.and_then(double);

        assert_eq!(composed.apply(5), 12); // (5 + 1) * 2
    }
}

// ============================================================================
// RcTransformer Tests - Immutable, single-threaded
// ============================================================================

#[cfg(test)]
mod rc_transformer_tests {
    use qubit_function::{
        RcTransformer,
        Transformer,
    };

    #[test]
    fn test_new_and_apply() {
        let double = RcTransformer::new(|x: i32| x * 2);
        assert_eq!(double.apply(21), 42);
    }

    #[test]
    fn test_clone() {
        let double = RcTransformer::new(|x: i32| x * 2);
        let cloned = double.clone();

        assert_eq!(double.apply(21), 42);
        assert_eq!(cloned.apply(21), 42);
    }

    #[test]
    fn test_identity() {
        let identity = RcTransformer::<i32, i32>::identity();
        assert_eq!(identity.apply(42), 42);
    }

    #[test]
    fn test_constant() {
        let constant = RcTransformer::constant("hello");
        assert_eq!(constant.apply(123), "hello");
    }

    #[test]
    fn test_shared_usage() {
        let to_upper = RcTransformer::new(|s: String| s.to_uppercase());

        let func1 = to_upper.clone();
        let func2 = to_upper.clone();

        assert_eq!(to_upper.apply("hello".to_string()), "HELLO");
        assert_eq!(func1.apply("world".to_string()), "WORLD");
        assert_eq!(func2.apply("rust".to_string()), "RUST");
    }

    #[test]
    fn test_and_then() {
        let double = RcTransformer::new(|x: i32| x * 2);
        let to_string = RcTransformer::new(|x: i32| x.to_string());
        let composed = double.and_then(to_string);

        // Original double transformer still usable
        assert_eq!(double.apply(21), 42);
        assert_eq!(composed.apply(21), "42");
    }

    #[test]
    fn test_compose() {
        let double = RcTransformer::new(|x: i32| x * 2);
        let add_one = RcTransformer::new(|x: i32| x + 1);
        let composed = add_one.and_then(double);

        assert_eq!(composed.apply(5), 12); // (5 + 1) * 2
    }

    #[test]
    fn test_display_with_name() {
        let transformer =
            RcTransformer::new_with_name("double", |x: i32| x * 2);
        let display_str = format!("{}", transformer);
        assert_eq!(display_str, "RcTransformer(double)");
    }

    #[test]
    fn test_display_without_name() {
        let transformer = RcTransformer::new(|x: i32| x * 2);
        let display_str = format!("{}", transformer);
        assert_eq!(display_str, "RcTransformer");
    }
}

// ============================================================================
// Conditional Transformer Tests
// ============================================================================

#[cfg(test)]
mod box_conditional_tests {
    use qubit_function::{
        FnTransformerOps,
        Transformer,
    };

    #[test]
    fn test_when_or_else_with_closure() {
        let double_fn = |x: i32| x * 2;
        let result = FnTransformerOps::when(double_fn, |x: &i32| *x > 0)
            .or_else(|x: i32| -x);

        assert_eq!(result.apply(5), 10);
        assert_eq!(result.apply(-5), 5);
        assert_eq!(result.apply(0), 0);
    }
}

#[cfg(test)]
mod arc_conditional_tests {
    use qubit_function::{
        FnTransformerOps,
        Transformer,
    };

    #[test]
    fn test_when_or_else() {
        let double_fn = |x: i32| x * 2;
        let negate_fn = |x: i32| -x;
        let result = FnTransformerOps::when(double_fn, |x: &i32| *x > 0)
            .or_else(negate_fn);

        assert_eq!(result.apply(5), 10);
        assert_eq!(result.apply(-5), 5);
    }

    #[test]
    fn test_when_or_else_with_closure() {
        let double_fn = |x: i32| x * 2;
        let result = FnTransformerOps::when(double_fn, |x: &i32| *x > 0)
            .or_else(|x: i32| -x);

        assert_eq!(result.apply(5), 10);
        assert_eq!(result.apply(-5), 5);
        assert_eq!(result.apply(0), 0);
    }

    #[test]
    fn test_conditional_or_else() {
        let double_fn = |x: i32| x * 2;
        let result = FnTransformerOps::when(double_fn, |x: &i32| *x > 0)
            .or_else(|x: i32| -x);

        assert_eq!(result.apply(5), 10);
        assert_eq!(result.apply(-5), 5);
    }
}

#[cfg(test)]
mod rc_conditional_tests {
    use qubit_function::{
        FnTransformerOps,
        Transformer,
    };

    #[test]
    fn test_when_or_else() {
        let double_fn = |x: i32| x * 2;
        let negate_fn = |x: i32| -x;
        let result = FnTransformerOps::when(double_fn, |x: &i32| *x > 0)
            .or_else(negate_fn);

        assert_eq!(result.apply(5), 10);
        assert_eq!(result.apply(-5), 5);
    }

    #[test]
    fn test_when_or_else_with_closure() {
        let double_fn = |x: i32| x * 2;
        let result = FnTransformerOps::when(double_fn, |x: &i32| *x > 0)
            .or_else(|x: i32| -x);

        assert_eq!(result.apply(5), 10);
        assert_eq!(result.apply(-5), 5);
        assert_eq!(result.apply(0), 0);
    }

    #[test]
    fn test_conditional_or_else() {
        let double_fn = |x: i32| x * 2;
        let result = FnTransformerOps::when(double_fn, |x: &i32| *x > 0)
            .or_else(|x: i32| -x);

        assert_eq!(result.apply(5), 10);
        assert_eq!(result.apply(-5), 5);
    }
}

// ============================================================================
// Conversion Tests
// ============================================================================

// ============================================================================
// Non-consuming Conversion Tests (to_xxx methods)
// ============================================================================

#[cfg(test)]
mod to_conversion_tests {
    use qubit_function::ArcTransformer;

    // ArcTransformer to_xxx tests

    // RcTransformer to_xxx tests

    // Test to_xxx with composition

    // Test multiple conversions

    // Test with different types

    // Test thread safety with Arc - clone first to get owned values

    // Test that to_xxx creates independent copies

    // ========================================================================
    // Closure / function-pointer Transformer to_xxx Tests
    // ========================================================================

    #[test]
    fn test_display_with_name() {
        let transformer =
            ArcTransformer::new_with_name("double", |x: i32| x * 2);
        let display_str = format!("{}", transformer);
        assert_eq!(display_str, "ArcTransformer(double)");
    }

    #[test]
    fn test_display_without_name() {
        let transformer = ArcTransformer::new(|x: i32| x * 2);
        let display_str = format!("{}", transformer);
        assert_eq!(display_str, "ArcTransformer");
    }
}

// ============================================================================
// Trait Usage Tests
// ============================================================================

#[cfg(test)]
mod trait_usage_tests {
    use qubit_function::{
        BoxTransformer,
        Transformer,
    };

    #[test]
    fn test_transformer_trait() {
        fn apply_transformer<F: Transformer<i32, i32>>(f: &F, x: i32) -> i32 {
            f.apply(x)
        }

        let double = BoxTransformer::new(|x: i32| x * 2);
        assert_eq!(apply_transformer(&double, 21), 42);
    }

    #[test]
    fn test_closure_as_transformer() {
        fn apply_transformer<F: Transformer<i32, i32>>(f: &F, x: i32) -> i32 {
            f.apply(x)
        }

        let double = |x: i32| x * 2;
        assert_eq!(apply_transformer(&double, 21), 42);
    }

    #[test]
    fn test_with_different_types() {
        fn apply_transformer<T, R, F: Transformer<T, R>>(f: &F, x: T) -> R {
            f.apply(x)
        }

        let to_string = BoxTransformer::new(|x: i32| x.to_string());
        assert_eq!(apply_transformer(&to_string, 42), "42");
    }
}

// ============================================================================
// Complex Composition Tests
// ============================================================================

#[cfg(test)]
mod complex_composition_tests {
    use qubit_function::{
        ArcTransformer,
        BoxTransformer,
        RcTransformer,
        Transformer,
    };

    #[test]
    fn test_multiple_and_then() {
        let add_one = BoxTransformer::new(|x: i32| x + 1);
        let double = BoxTransformer::new(|x: i32| x * 2);
        let to_string = BoxTransformer::new(|x: i32| x.to_string());
        let composed = add_one.and_then(double).and_then(to_string);
        assert_eq!(composed.apply(5), "12"); // (5 + 1) * 2 = 12
    }

    #[test]
    fn test_multiple_and_then_with_box() {
        let add_one = BoxTransformer::new(|x: i32| x + 1);
        let double = BoxTransformer::new(|x: i32| x * 2);
        let square = BoxTransformer::new(|x: i32| x * x);
        let composed = add_one.and_then(double).and_then(square);
        assert_eq!(composed.apply(5), 144); // ((5 + 1) * 2)^2 = 144
    }

    #[test]
    fn test_arc_multiple_and_then() {
        let add_one = ArcTransformer::new(|x: i32| x + 1);
        let double = ArcTransformer::new(|x: i32| x * 2);
        let to_string = ArcTransformer::new(|x: i32| x.to_string());
        let composed =
            add_one.and_then(double.clone()).and_then(to_string.clone());
        assert_eq!(composed.apply(5), "12");
        // Original transformers still usable
        assert_eq!(add_one.apply(5), 6);
        assert_eq!(double.apply(5), 10);
    }

    #[test]
    fn test_rc_multiple_and_then() {
        let add_one = RcTransformer::new(|x: i32| x + 1);
        let double = RcTransformer::new(|x: i32| x * 2);
        let square = RcTransformer::new(|x: i32| x * x);
        let composed =
            add_one.and_then(double.clone()).and_then(square.clone());
        assert_eq!(composed.apply(5), 144); // (5 + 1) * 2 = 12, then 12 * 12 = 144
        // Original transformers still usable
        assert_eq!(add_one.apply(5), 6);
        assert_eq!(double.apply(5), 10);
        assert_eq!(square.apply(5), 25);
    }
}

// ============================================================================
// Edge Cases Tests
// ============================================================================

#[cfg(test)]
mod edge_cases_tests {
    use qubit_function::{
        ArcTransformer,
        BoxTransformer,
        Transformer,
    };

    #[test]
    fn test_identity_composition() {
        let double = BoxTransformer::new(|x: i32| x * 2);
        let identity = BoxTransformer::<i32, i32>::identity();
        let composed = double.and_then(identity);
        assert_eq!(composed.apply(21), 42);
    }

    #[test]
    fn test_constant_with_different_types() {
        let constant = BoxTransformer::constant("hello");
        assert_eq!(constant.apply(123), "hello");
        assert_eq!(constant.apply(456), "hello");
        assert_eq!(constant.apply(789), "hello");
    }

    #[test]
    fn test_with_option() {
        let parse = BoxTransformer::new(|s: String| s.parse::<i32>().ok());
        assert_eq!(parse.apply("42".to_string()), Some(42));
        assert_eq!(parse.apply("abc".to_string()), None);
    }

    #[test]
    fn test_with_result() {
        let parse = BoxTransformer::new(|s: String| s.parse::<i32>());
        assert!(parse.apply("42".to_string()).is_ok());
        assert!(parse.apply("abc".to_string()).is_err());
    }

    #[test]
    fn test_with_vec() {
        let split = BoxTransformer::new(|s: String| {
            s.split(',').map(|s| s.to_string()).collect::<Vec<_>>()
        });
        assert_eq!(
            split.apply("a,b,c".to_string()),
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
    }

    #[test]
    fn test_arc_with_large_data() {
        let process = ArcTransformer::new(|v: Vec<i32>| v.iter().sum::<i32>());
        let data = (1..=100).collect::<Vec<_>>();
        assert_eq!(process.apply(data), 5050);
    }
}
// ============================================================================
// Specialized into_fn Implementation Tests
// ============================================================================

// ============================================================================
// Type Conversion Tests
// ============================================================================

#[cfg(test)]
mod type_conversion_tests {
    use qubit_function::{
        ArcTransformer,
        RcTransformer,
        Transformer,
    };

    #[test]
    fn test_arc_constant_with_clone() {
        let constant = ArcTransformer::constant(42);
        assert_eq!(constant.apply(1), 42);
        assert_eq!(constant.apply(2), 42);
        assert_eq!(constant.apply(3), 42);
    }

    #[test]
    fn test_rc_constant_with_clone() {
        let constant = RcTransformer::constant("test");
        assert_eq!(constant.apply(1), "test");
        assert_eq!(constant.apply(2), "test");
        assert_eq!(constant.apply(3), "test");
    }
}
// ============================================================================
// TransformerOnce Tests for BoxTransformer, RcTransformer, ArcTransformer
// ============================================================================

#[cfg(test)]
mod transformer_once_tests {
    use qubit_function::{
        ArcTransformer,
        BoxTransformer,
        RcTransformer,
        Transformer,
    };
    use std::sync::Arc;
    use std::thread;

    // BoxTransformer TransformerOnce Tests
    #[cfg(test)]
    mod box_transformer_once_tests {
        use super::{
            BoxTransformer,
            Transformer,
        };

        #[test]
        fn test_box_transformer_apply() {
            let double = BoxTransformer::new(|x: i32| x * 2);
            let result = double.apply(21);
            assert_eq!(result, 42);
        }

        #[test]
        fn test_box_transformer_string_transformation() {
            let uppercase = BoxTransformer::new(|s: String| s.to_uppercase());
            let result = uppercase.apply("hello".to_string());
            assert_eq!(result, "HELLO");
        }

        #[test]
        fn test_box_transformer_complex_transformation() {
            let parse_and_double = BoxTransformer::new(|s: String| {
                s.parse::<i32>().unwrap_or(0) * 2
            });
            let result = parse_and_double.apply("21".to_string());
            assert_eq!(result, 42);
        }

        #[test]
        fn test_box_transformer_regular_and_once() {
            let double = BoxTransformer::new(|x: i32| x * 2);

            // Regular apply can be called multiple times
            assert_eq!(double.apply(10), 20);
            assert_eq!(double.apply(15), 30);

            // But apply consumes the transformer
            let double = BoxTransformer::new(|x: i32| x * 2);
            let result = double.apply(21);
            assert_eq!(result, 42);
        }
    }

    // RcTransformer TransformerOnce Tests
    #[cfg(test)]
    mod rc_transformer_once_tests {
        use super::{
            RcTransformer,
            Transformer,
        };

        #[test]
        fn test_rc_transformer_apply() {
            let double = RcTransformer::new(|x: i32| x * 2);
            let result = double.apply(21);
            assert_eq!(result, 42);
        }

        #[test]
        fn test_rc_transformer_string_transformation() {
            let uppercase = RcTransformer::new(|s: String| s.to_uppercase());
            let result = uppercase.apply("hello".to_string());
            assert_eq!(result, "HELLO");
        }

        #[test]
        fn test_rc_transformer_complex_transformation() {
            let parse_and_double = RcTransformer::new(|s: String| {
                s.parse::<i32>().unwrap_or(0) * 2
            });
            let result = parse_and_double.apply("21".to_string());
            assert_eq!(result, 42);
        }

        #[test]
        fn test_rc_transformer_clone_before_apply() {
            let double = RcTransformer::new(|x: i32| x * 2);
            let double_clone = double.clone();

            // Both should work
            assert_eq!(double.apply(21), 42);
            assert_eq!(double_clone.apply(21), 42);
        }

        #[test]
        fn test_rc_transformer_regular_and_once() {
            let double = RcTransformer::new(|x: i32| x * 2);

            // Regular apply can be called multiple times
            assert_eq!(double.apply(10), 20);
            assert_eq!(double.apply(15), 30);

            // Clone before using apply
            let double_clone = double.clone();
            let result = double_clone.apply(21);
            assert_eq!(result, 42);

            // Original is still usable
            assert_eq!(double.apply(5), 10);
        }
    }

    // ArcTransformer TransformerOnce Tests
    #[cfg(test)]
    mod arc_transformer_once_tests {
        use super::{
            Arc,
            ArcTransformer,
            Transformer,
            thread,
        };

        #[test]
        fn test_arc_transformer_apply() {
            let double = ArcTransformer::new(|x: i32| x * 2);
            let result = double.apply(21);
            assert_eq!(result, 42);
        }

        #[test]
        fn test_arc_transformer_string_transformation() {
            let uppercase = ArcTransformer::new(|s: String| s.to_uppercase());
            let result = uppercase.apply("hello".to_string());
            assert_eq!(result, "HELLO");
        }

        #[test]
        fn test_arc_transformer_complex_transformation() {
            let parse_and_double = ArcTransformer::new(|s: String| {
                s.parse::<i32>().unwrap_or(0) * 2
            });
            let result = parse_and_double.apply("21".to_string());
            assert_eq!(result, 42);
        }

        #[test]
        fn test_arc_transformer_clone_before_apply() {
            let double = ArcTransformer::new(|x: i32| x * 2);
            let double_clone = double.clone();

            // Both should work
            assert_eq!(double.apply(21), 42);
            assert_eq!(double_clone.apply(21), 42);
        }

        #[test]
        fn test_arc_transformer_regular_and_once() {
            let double = ArcTransformer::new(|x: i32| x * 2);

            // Regular apply can be called multiple times
            assert_eq!(double.apply(10), 20);
            assert_eq!(double.apply(15), 30);

            // Clone before using apply
            let double_clone = double.clone();
            let result = double_clone.apply(21);
            assert_eq!(result, 42);

            // Original is still usable
            assert_eq!(double.apply(5), 10);
        }

        #[test]
        fn test_arc_transformer_thread_safety() {
            let double = ArcTransformer::new(|x: i32| x * 2);
            let double_arc = Arc::new(double);
            let _double_clone = Arc::clone(&double_arc);

            let handle = thread::spawn(move || {
                // Create a new transformer in the thread to demonstrate thread
                // safety
                let new_double = ArcTransformer::new(|x: i32| x * 2);
                new_double.apply(21)
            });

            let result = handle.join().expect("thread should not panic");
            assert_eq!(result, 42);
        }
    }

    // Cross-type TransformerOnce Tests
    #[cfg(test)]
    mod cross_type_transformer_once_tests {
        use super::{
            ArcTransformer,
            BoxTransformer,
            RcTransformer,
            Transformer,
        };

        #[test]
        fn test_all_types_apply() {
            let box_double = BoxTransformer::new(|x: i32| x * 2);
            let rc_double = RcTransformer::new(|x: i32| x * 2);
            let arc_double = ArcTransformer::new(|x: i32| x * 2);

            assert_eq!(box_double.apply(21), 42);
            assert_eq!(rc_double.apply(21), 42);
            assert_eq!(arc_double.apply(21), 42);
        }

        #[test]
        fn test_mixed_regular_and_once_usage() {
            // Test that regular apply and apply work together
            let box_transformer = BoxTransformer::new(|x: i32| x * 2);
            let rc_transformer = RcTransformer::new(|x: i32| x * 2);
            let arc_transformer = ArcTransformer::new(|x: i32| x * 2);

            // Regular apply (multiple calls)
            assert_eq!(box_transformer.apply(10), 20);
            assert_eq!(rc_transformer.apply(10), 20);
            assert_eq!(arc_transformer.apply(10), 20);

            // Clone for apply
            let rc_clone = rc_transformer.clone();
            let arc_clone = arc_transformer.clone();

            // Apply once (consuming)
            assert_eq!(rc_clone.apply(21), 42);
            assert_eq!(arc_clone.apply(21), 42);

            // Original transformers still work
            assert_eq!(rc_transformer.apply(5), 10);
            assert_eq!(arc_transformer.apply(5), 10);
        }
    }
}

// ============================================================================
// Transformer Trait Default Methods Tests - into_once, to_once
// ============================================================================
