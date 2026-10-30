// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

// ============================================================================
// Historical filename: transformer_transformer_once_tests.rs.
// Tests regular Transformer behavior: inputs are passed by value, while the
// wrappers are borrowed and can be called repeatedly.
// ============================================================================

#[cfg(test)]
mod transformer_tests {
    use std::thread;

    use qubit_function::ArcTransformer;
    use qubit_function::BoxTransformer;
    use qubit_function::RcTransformer;
    use qubit_function::Transformer;

    // BoxTransformer repeated apply tests
    #[cfg(test)]
    mod box_transformer_tests {
        use super::BoxTransformer;
        use super::Transformer;

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
            let parse_and_double = BoxTransformer::new(|s: String| s.parse::<i32>().unwrap_or(0) * 2);
            let result = parse_and_double.apply("21".to_string());
            assert_eq!(result, 42);
        }

        #[test]
        fn test_box_transformer_repeated_apply() {
            let double = BoxTransformer::new(|x: i32| x * 2);
            assert_eq!(double.apply(10), 20);
            assert_eq!(double.apply(15), 30);
            assert_eq!(double.apply(21), 42);
        }
    }

    // RcTransformer repeated apply tests
    #[cfg(test)]
    mod rc_transformer_tests {
        use super::RcTransformer;
        use super::Transformer;

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
            let parse_and_double = RcTransformer::new(|s: String| s.parse::<i32>().unwrap_or(0) * 2);
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
        fn test_rc_transformer_repeated_apply() {
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

    // ArcTransformer repeated apply and thread tests
    #[cfg(test)]
    mod arc_transformer_tests {
        use super::ArcTransformer;
        use super::Transformer;
        use super::thread;

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
            let parse_and_double = ArcTransformer::new(|s: String| s.parse::<i32>().unwrap_or(0) * 2);
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
        fn test_arc_transformer_repeated_apply() {
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
            let clone = double.clone();
            let handle = thread::spawn(move || clone.apply(21));
            assert_eq!(double.apply(10), 20);
            assert_eq!(handle.join().expect("thread should not panic"), 42);
        }
    }

    // Cross-type Transformer tests
    #[cfg(test)]
    mod cross_type_transformer_tests {
        use super::ArcTransformer;
        use super::BoxTransformer;
        use super::RcTransformer;
        use super::Transformer;

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
        fn test_repeated_apply_across_clones() {
            let box_transformer = BoxTransformer::new(|x: i32| x * 2);
            let rc_transformer = RcTransformer::new(|x: i32| x * 2);
            let arc_transformer = ArcTransformer::new(|x: i32| x * 2);

            // Regular apply (multiple calls)
            assert_eq!(box_transformer.apply(10), 20);
            assert_eq!(rc_transformer.apply(10), 20);
            assert_eq!(arc_transformer.apply(10), 20);

            // Inputs are consumed by value; cloning the wrappers lets both
            // clones and originals be borrowed for repeated apply calls.
            let rc_clone = rc_transformer.clone();
            let arc_clone = arc_transformer.clone();

            // Each call consumes its input value, not the wrapper.
            assert_eq!(rc_clone.apply(21), 42);
            assert_eq!(arc_clone.apply(21), 42);

            // Original transformers still work
            assert_eq!(rc_transformer.apply(5), 10);
            assert_eq!(arc_transformer.apply(5), 10);
        }
    }
}
