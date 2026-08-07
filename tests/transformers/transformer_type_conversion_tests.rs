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
mod type_conversion_tests {
    use qubit_function::ArcTransformer;
    use qubit_function::RcTransformer;
    use qubit_function::Transformer;

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
