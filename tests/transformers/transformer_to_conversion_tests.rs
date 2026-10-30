// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

// ============================================================================
// ArcTransformer Display Tests
// ============================================================================

#[cfg(test)]
mod to_conversion_tests {
    use qubit_function::ArcTransformer;

    // ========================================================================
    // ArcTransformer Display Tests
    // ========================================================================

    #[test]
    fn test_display_with_name() {
        let transformer = ArcTransformer::new_with_name("double", |x: i32| x * 2);
        let display_str = format!("{}", transformer);
        assert_eq!(display_str, "ArcTransformer(double)");
    }

    #[test]
    fn test_display_without_name() {
        let transformer = ArcTransformer::new(|x: i32| x * 2);
        let display_str = format!("{}", transformer);
        assert_eq!(display_str, "ArcTransformer");
    }
}
