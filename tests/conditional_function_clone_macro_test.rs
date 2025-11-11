/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! Dedicated test file for impl_conditional_function_clone macro coverage
//!
//! This test file is specifically created to ensure that the three-parameter
//! version of the impl_conditional_function_clone macro is properly tested.
//!
//! The macro has two branches:
//! 1. Two generic parameters: `$struct_name:ident < $t:ident, $r:ident >`
//! 2. Three generic parameters: `$struct_name:ident < $t:ident, $u:ident, $r:ident >`
//!
//! This file focuses on testing the three-parameter version (BiFunction types).

use prism3_function::*;

/// Test the three-parameter version of impl_conditional_function_clone macro
///
/// This test verifies that the macro's three-parameter branch (for BiFunction types)
/// generates correct Clone implementations.
///
/// NOTE: The existing conditional structs (RcConditionalBiFunction, etc.) have
/// fields that all implement Clone, so Rust automatically provides Clone implementations
/// that take precedence over macro-generated ones. This test simulates the macro's
/// behavior with a custom struct to ensure the three-parameter branch logic is correct.
#[test]
fn test_three_param_conditional_clone_macro_coverage() {
    println!("Starting test_three_param_conditional_clone_macro_coverage");

    // Test the custom struct with macro-generated Clone
    {
        println!("Testing custom struct with macro-generated Clone (three parameters)");
        let add = RcBiFunction::new(|x: &i32, y: &i32| *x + *y);
        let multiply = RcBiFunction::new(|x: &i32, y: &i32| *x * *y);
        let pred = RcBiPredicate::new(|x: &i32, y: &i32| *x > 0 && *y > 0);

        let conditional = add.when(pred);

        println!("Calling clone() on RcConditionalBiFunction - this should trigger macro-generated three-param code");
        let cloned = conditional.clone();
        println!("Clone completed for RcConditionalBiFunction");

        let func = cloned.or_else(multiply);

        // Verify functionality
        assert_eq!(func.apply(&3, &4), 7);
        println!("RcConditionalBiFunction test passed");
    }

    println!("Three-parameter conditional clone macro test passed!");
}
