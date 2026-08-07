// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use std::thread;

use qubit_function::ArcBiTransformer;
use qubit_function::BiTransformer;
use qubit_function::BoxBiTransformer;
use qubit_function::RcBiTransformer;

// ============================================================================
// BoxBiTransformer Tests - Immutable, single ownership
// ============================================================================

#[cfg(test)]
mod conditional_transformer_display_debug_tests {
    use super::ArcBiTransformer;
    use super::BoxBiTransformer;
    use super::RcBiTransformer;

    #[test]
    fn test_box_conditional_bi_transformer_display() {
        let add = BoxBiTransformer::new(|x: i32, y: i32| x + y);
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let display_str = format!("{}", conditional);
        assert!(display_str.contains("BoxConditionalBiTransformer"));
    }

    #[test]
    fn test_box_conditional_bi_transformer_display_no_name() {
        let add = BoxBiTransformer::new(|x: i32, y: i32| x + y);
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let display_str = format!("{}", conditional);
        assert_eq!(
            display_str,
            "BoxConditionalBiTransformer(BoxBiTransformer, BoxBiPredicate(unnamed))"
        );
    }

    #[test]
    fn test_box_conditional_bi_transformer_debug() {
        let add = BoxBiTransformer::new(|x: i32, y: i32| x + y);
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("BoxConditionalBiTransformer"));
    }

    #[test]
    fn test_rc_conditional_bi_transformer_display() {
        let add = RcBiTransformer::new(|x: i32, y: i32| x + y);
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let display_str = format!("{}", conditional);
        assert!(display_str.contains("RcConditionalBiTransformer"));
    }

    #[test]
    fn test_rc_conditional_bi_transformer_display_no_name() {
        let add = RcBiTransformer::new(|x: i32, y: i32| x + y);
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let display_str = format!("{}", conditional);
        assert_eq!(
            display_str,
            "RcConditionalBiTransformer(RcBiTransformer, RcBiPredicate(unnamed))"
        );
    }

    #[test]
    fn test_rc_conditional_bi_transformer_debug() {
        let add = RcBiTransformer::new(|x: i32, y: i32| x + y);
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("RcConditionalBiTransformer"));
    }

    #[test]
    fn test_arc_conditional_bi_transformer_display() {
        let add = ArcBiTransformer::new(|x: i32, y: i32| x + y);
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let display_str = format!("{}", conditional);
        assert!(display_str.contains("ArcConditionalBiTransformer"));
    }

    #[test]
    fn test_arc_conditional_bi_transformer_display_no_name() {
        let add = ArcBiTransformer::new(|x: i32, y: i32| x + y);
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let display_str = format!("{}", conditional);
        assert_eq!(
            display_str,
            "ArcConditionalBiTransformer(ArcBiTransformer, ArcBiPredicate(unnamed))"
        );
    }

    #[test]
    fn test_arc_conditional_bi_transformer_debug() {
        let add = ArcBiTransformer::new(|x: i32, y: i32| x + y);
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("ArcConditionalBiTransformer"));
    }
}
