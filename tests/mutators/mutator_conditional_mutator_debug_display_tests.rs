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
        assert_eq!(
            debug_str,
            concat!(
                "BoxConditionalMutator { mutator: BoxMutator { name: None, function: \"<function>\" }, ",
                "predicate: BoxPredicate { name: None, function: \"<function>\" } }"
            ),
            "Box conditional mutator Debug output changed"
        );
    }

    #[test]
    fn test_box_conditional_mutator_display() {
        let mutator = BoxMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let display_str = format!("{}", conditional);
        assert_eq!(
            display_str, "BoxConditionalMutator(BoxMutator, BoxPredicate(unnamed))",
            "Box conditional mutator Display output changed"
        );
    }

    #[test]
    fn test_rc_conditional_mutator_debug() {
        let mutator = RcMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let debug_str = format!("{:?}", conditional);
        assert_eq!(
            debug_str,
            concat!(
                "RcConditionalMutator { mutator: RcMutator { name: None, function: \"<function>\" }, ",
                "predicate: RcPredicate { name: None, function: \"<function>\" } }"
            ),
            "Rc conditional mutator Debug output changed"
        );
    }

    #[test]
    fn test_rc_conditional_mutator_display() {
        let mutator = RcMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let display_str = format!("{}", conditional);
        assert_eq!(
            display_str, "RcConditionalMutator(RcMutator, RcPredicate(unnamed))",
            "Rc conditional mutator Display output changed"
        );
    }

    #[test]
    fn test_arc_conditional_mutator_debug() {
        let mutator = ArcMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let debug_str = format!("{:?}", conditional);
        assert_eq!(
            debug_str,
            concat!(
                "ArcConditionalMutator { mutator: ArcMutator { name: None, function: \"<function>\" }, ",
                "predicate: ArcPredicate { name: None, function: \"<function>\" } }"
            ),
            "Arc conditional mutator Debug output changed"
        );
    }

    #[test]
    fn test_arc_conditional_mutator_display() {
        let mutator = ArcMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let display_str = format!("{}", conditional);
        assert_eq!(
            display_str, "ArcConditionalMutator(ArcMutator, ArcPredicate(unnamed))",
            "Arc conditional mutator Display output changed"
        );
    }
}
