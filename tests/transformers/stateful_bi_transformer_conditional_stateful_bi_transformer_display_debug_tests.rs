// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::ArcStatefulBiTransformer;
use qubit_function::ArcStatefulBinaryOperator;
use qubit_function::ArcStatefulTransformer;
use qubit_function::BoxBiPredicate;
use qubit_function::BoxStatefulBiTransformer;
use qubit_function::BoxStatefulBinaryOperator;
use qubit_function::BoxStatefulTransformer;
use qubit_function::RcStatefulBiTransformer;
use qubit_function::RcStatefulBinaryOperator;
use qubit_function::RcStatefulTransformer;
use qubit_function::StatefulBiTransformer;
use qubit_function::StatefulBinaryOperator;

#[cfg(test)]
mod conditional_stateful_bi_transformer_display_debug_tests {
    use super::ArcStatefulBiTransformer;
    use super::BoxStatefulBiTransformer;
    use super::RcStatefulBiTransformer;

    #[test]
    fn test_box_conditional_stateful_bi_transformer_display() {
        let mut counter = 0;
        let add = BoxStatefulBiTransformer::new(move |x: i32, y: i32| {
            counter += 1;
            x + y + counter
        });
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let display_str = format!("{}", conditional);
        assert!(display_str.contains("BoxConditionalStatefulBiTransformer"));
    }

    #[test]
    fn test_box_conditional_stateful_bi_transformer_display_no_name() {
        let mut counter = 0;
        let add = BoxStatefulBiTransformer::new(move |x: i32, y: i32| {
            counter += 1;
            x + y + counter
        });
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let display_str = format!("{}", conditional);
        assert_eq!(
            display_str,
            "BoxConditionalStatefulBiTransformer(BoxStatefulBiTransformer, BoxBiPredicate(unnamed))"
        );
    }

    #[test]
    fn test_box_conditional_stateful_bi_transformer_debug() {
        let mut counter = 0;
        let add = BoxStatefulBiTransformer::new(move |x: i32, y: i32| {
            counter += 1;
            x + y + counter
        });
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("BoxConditionalStatefulBiTransformer"));
    }

    #[test]
    fn test_rc_conditional_stateful_bi_transformer_display() {
        let mut counter = 0;
        let add = RcStatefulBiTransformer::new(move |x: i32, y: i32| {
            counter += 1;
            x + y + counter
        });
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let display_str = format!("{}", conditional);
        assert!(display_str.contains("RcConditionalStatefulBiTransformer"));
    }

    #[test]
    fn test_rc_conditional_stateful_bi_transformer_display_no_name() {
        let mut counter = 0;
        let add = RcStatefulBiTransformer::new(move |x: i32, y: i32| {
            counter += 1;
            x + y + counter
        });
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let display_str = format!("{}", conditional);
        assert_eq!(
            display_str,
            "RcConditionalStatefulBiTransformer(RcStatefulBiTransformer, RcBiPredicate(unnamed))"
        );
    }

    #[test]
    fn test_rc_conditional_stateful_bi_transformer_debug() {
        let mut counter = 0;
        let add = RcStatefulBiTransformer::new(move |x: i32, y: i32| {
            counter += 1;
            x + y + counter
        });
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("RcConditionalStatefulBiTransformer"));
    }

    #[test]
    fn test_arc_conditional_stateful_bi_transformer_display() {
        let mut counter = 0;
        let add = ArcStatefulBiTransformer::new(move |x: i32, y: i32| {
            counter += 1;
            x + y + counter
        });
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let display_str = format!("{}", conditional);
        assert!(display_str.contains("ArcConditionalStatefulBiTransformer"));
    }

    #[test]
    fn test_arc_conditional_stateful_bi_transformer_display_no_name() {
        let mut counter = 0;
        let add = ArcStatefulBiTransformer::new(move |x: i32, y: i32| {
            counter += 1;
            x + y + counter
        });
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let display_str = format!("{}", conditional);
        assert_eq!(
            display_str,
            "ArcConditionalStatefulBiTransformer(ArcStatefulBiTransformer, ArcBiPredicate(unnamed))"
        );
    }

    #[test]
    fn test_arc_conditional_stateful_bi_transformer_debug() {
        let mut counter = 0;
        let add = ArcStatefulBiTransformer::new(move |x: i32, y: i32| {
            counter += 1;
            x + y + counter
        });
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("ArcConditionalStatefulBiTransformer"));
    }
}
// ============================================================================
// Basic StatefulBiTransformer Display Tests
// ============================================================================
