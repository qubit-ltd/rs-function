// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Unit tests for StatefulMutatingFunction types (stateful FnMut(&mut T) ->
//! R)

use qubit_function::RcStatefulMutatingFunction;
use qubit_function::StatefulMutatingFunction;

// ============================================================================
// RcStatefulMutatingFunction Tests
// ============================================================================

#[cfg(test)]
mod test_rc_stateful_mutating_function {
    use super::RcStatefulMutatingFunction;
    use super::StatefulMutatingFunction;

    #[test]
    fn test_new_tracks_calls_and_mutates_each_input() {
        let mut counter = {
            let mut count = 0;
            RcStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x += 1;
                count
            })
        };
        let mut value = 5;
        assert_eq!(counter.apply(&mut value), 1, "first call should return count 1");
        assert_eq!(value, 6, "first call should increment the input");
        assert_eq!(counter.apply(&mut value), 2, "second call should return count 2");
        assert_eq!(value, 7, "second call should increment the input again");
    }

    #[test]
    fn test_cloned_function_mutates_inputs_and_tracks_calls() {
        let counter = {
            let mut count = 0;
            RcStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x *= 2;
                count
            })
        };
        let mut clone = counter.clone();

        let mut value1 = 5;
        assert_eq!(clone.apply(&mut value1), 1, "first cloned call should return count 1");
        assert_eq!(value1, 10, "cloned call should double the input");

        // The cloned function continues tracking calls.
        let mut value2 = 3;
        assert_eq!(
            clone.apply(&mut value2),
            2,
            "second cloned call should retain the call count"
        );
        assert_eq!(value2, 6, "second cloned call should double its input");
    }

    #[test]
    fn test_identity_returns_input_without_mutating_it() {
        let mut identity = RcStatefulMutatingFunction::<i32, i32>::identity();
        let mut value = 42;
        assert_eq!(identity.apply(&mut value), 42, "identity should return the input value");
        assert_eq!(value, 42, "identity should leave the input unchanged");
    }

    #[test]
    fn test_and_then_maps_call_count_to_string() {
        let func = {
            let mut count = 0;
            RcStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x *= 2;
                count
            })
        };
        let mut mapped = func.and_then::<String, _>(|count: &i32| format!("Call #{}", *count));

        let mut value = 5;
        assert_eq!(
            mapped.apply(&mut value),
            "Call #1",
            "and_then should map the call count"
        );
        assert_eq!(value, 10, "and_then should preserve the input mutation");
    }
}
