// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for BiTransformerOnce trait and implementations

use qubit_function::{
    BiTransformerOnce,
    BoxBiTransformerOnce,
};

// ============================================================================
// Tests for BiTransformerOnce trait
// ============================================================================

#[cfg(test)]
mod conditional_bi_transformer_once_display_debug_tests {
    use super::BoxBiTransformerOnce;

    #[test]
    fn test_box_conditional_bi_transformer_once_display() {
        let add = BoxBiTransformerOnce::new(|x: i32, y: i32| x + y);
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let display_str = format!("{}", conditional);
        assert!(display_str.contains("BoxConditionalBiTransformerOnce"));
    }

    #[test]
    fn test_box_conditional_bi_transformer_once_display_no_name() {
        let add = BoxBiTransformerOnce::new(|x: i32, y: i32| x + y);
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let display_str = format!("{}", conditional);
        assert_eq!(
            display_str,
            "BoxConditionalBiTransformerOnce(BoxBiTransformerOnce, BoxBiPredicate(unnamed))"
        );
    }

    #[test]
    fn test_box_conditional_bi_transformer_once_debug() {
        let add = BoxBiTransformerOnce::new(|x: i32, y: i32| x + y);
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("BoxConditionalBiTransformerOnce"));
    }
}
