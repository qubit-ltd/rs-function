// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use qubit_function::BiTransformer;

#[cfg(test)]
mod custom_bi_transformer_tests {
    use super::BiTransformer;

    /// Custom BiTransformer implementation for testing default into_xxx()
    /// methods
    struct CustomBiTransformer {
        multiplier: i32,
    }

    impl CustomBiTransformer {
        fn new(multiplier: i32) -> Self {
            Self { multiplier }
        }
    }

    impl BiTransformer<i32, i32, i32> for CustomBiTransformer {
        fn apply(&self, first: i32, second: i32) -> i32 {
            (first + second) * self.multiplier
        }
    }

    #[test]
    fn test_custom_bi_transformer_apply() {
        let transformer = CustomBiTransformer::new(3);
        assert_eq!(transformer.apply(5, 10), 45); // (5 + 10) * 3 = 45
    }
}
