// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Debug and display tests for conditional mutator types.

#[cfg(test)]
mod test_conditional_mutator_debug_display {
    use qubit_function::ArcMutator;
    use qubit_function::BoxMutator;
    use qubit_function::RcMutator;

    #[test]
    fn test_box_conditional_mutator_debug() {
        let mutator = BoxMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("BoxConditionalMutator"));
        assert!(debug_str.contains("BoxMutator"));
        assert!(debug_str.contains("BoxPredicate"));
    }

    #[test]
    fn test_box_conditional_mutator_display() {
        let mutator = BoxMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let display_str = format!("{}", conditional);
        assert!(display_str.contains("BoxConditionalMutator"));
    }

    #[test]
    fn test_rc_conditional_mutator_debug() {
        let mutator = RcMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("RcConditionalMutator"));
        assert!(debug_str.contains("RcMutator"));
        assert!(debug_str.contains("RcPredicate"));
    }

    #[test]
    fn test_rc_conditional_mutator_display() {
        let mutator = RcMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let display_str = format!("{}", conditional);
        assert!(display_str.contains("RcConditionalMutator"));
    }

    #[test]
    fn test_arc_conditional_mutator_debug() {
        let mutator = ArcMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("ArcConditionalMutator"));
        assert!(debug_str.contains("ArcMutator"));
        assert!(debug_str.contains("ArcPredicate"));
    }

    #[test]
    fn test_arc_conditional_mutator_display() {
        let mutator = ArcMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let display_str = format!("{}", conditional);
        assert!(display_str.contains("ArcConditionalMutator"));
    }
}
