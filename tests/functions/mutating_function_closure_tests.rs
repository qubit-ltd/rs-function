// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Unit tests for MutatingFunction types (stateless Fn(&mut T) -> R)

use qubit_function::MutatingFunction;

#[cfg(test)]
mod test_closure {
    use super::MutatingFunction;

    #[test]
    fn test_closure_implements_trait() {
        let closure = |x: &mut i32| {
            *x *= 2;
            *x
        };

        let mut value = 5;
        assert_eq!(closure.apply(&mut value), 10);
        assert_eq!(value, 10);
    }
}
