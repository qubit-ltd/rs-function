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
mod box_conditional_tests {
    use qubit_function::BoxTransformer;
    use qubit_function::Transformer;

    #[test]
    fn test_when_or_else_with_closure() {
        let double_fn = |x: i32| x * 2;
        let result = BoxTransformer::new(double_fn)
            .when(|x: &i32| *x > 0)
            .or_else(|x: i32| -x);

        assert_eq!(result.apply(5), 10);
        assert_eq!(result.apply(-5), 5);
        assert_eq!(result.apply(0), 0);
    }
}
