// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Tests for closure implementations of the `Mutator` trait.

#[cfg(test)]
mod wrapper_composition_tests {
    use qubit_function::Mutator;

    #[test]
    fn test_closure_mutate() {
        let closure = |x: &mut i32| *x *= 2;
        let mut value = 5;
        closure.apply(&mut value);
        assert_eq!(value, 10);
    }
}
