// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Unit tests for StatefulMutator types

use qubit_function::{
    ArcStatefulMutator,
    BoxStatefulMutator,
    MutatorOnce,
    RcStatefulMutator,
    StatefulMutator,
};

// ============================================================================
// BoxStatefulMutator Tests
// ============================================================================

#[cfg(test)]
mod test_conditional_stateful_mutator_debug_display {
    use super::{
        ArcStatefulMutator,
        BoxStatefulMutator,
        RcStatefulMutator,
    };

    #[test]
    fn test_box_conditional_stateful_mutator_debug() {
        let mutator = BoxStatefulMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("BoxConditionalStatefulMutator"));
        assert!(debug_str.contains("BoxStatefulMutator"));
        assert!(debug_str.contains("BoxPredicate"));
    }

    #[test]
    fn test_box_conditional_stateful_mutator_display() {
        let mutator = BoxStatefulMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let display_str = format!("{}", conditional);
        assert!(display_str.contains("BoxConditionalStatefulMutator"));
    }

    #[test]
    fn test_rc_conditional_stateful_mutator_debug() {
        let mutator = RcStatefulMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("RcConditionalStatefulMutator"));
        assert!(debug_str.contains("RcStatefulMutator"));
        assert!(debug_str.contains("RcPredicate"));
    }

    #[test]
    fn test_rc_conditional_stateful_mutator_display() {
        let mutator = RcStatefulMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let display_str = format!("{}", conditional);
        assert!(display_str.contains("RcConditionalStatefulMutator"));
    }

    #[test]
    fn test_arc_conditional_stateful_mutator_debug() {
        let mutator = ArcStatefulMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("ArcConditionalStatefulMutator"));
        assert!(debug_str.contains("ArcStatefulMutator"));
        assert!(debug_str.contains("ArcPredicate"));
    }

    #[test]
    fn test_arc_conditional_stateful_mutator_display() {
        let mutator = ArcStatefulMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let display_str = format!("{}", conditional);
        assert!(display_str.contains("ArcConditionalStatefulMutator"));
    }
}
