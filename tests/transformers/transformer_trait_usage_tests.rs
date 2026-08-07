// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

// ============================================================================
// BoxTransformer Tests - Immutable, single ownership
// ============================================================================

#[cfg(test)]
mod trait_usage_tests {
    use qubit_function::BoxTransformer;
    use qubit_function::Transformer;

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
