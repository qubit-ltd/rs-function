// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Comprehensive tests for Function trait and its implementations

use qubit_function::{
    ArcFunction,
    ArcPredicate,
    BoxFunction,
    BoxPredicate,
    Function,
    FunctionOnce,
    Predicate,
    RcFunction,
    RcPredicate,
};

// ============================================================================
// Function Trait Tests - Core Functionality
// ============================================================================

#[test]
fn test_function_trait_apply() {
    // Test that Function trait's apply method works correctly
    let double = |x: &i32| x * 2;
    assert_eq!(double.apply(&21), 42);
    assert_eq!(double.apply(&0), 0);
    assert_eq!(double.apply(&-10), -20);
}

#[test]
fn test_box_function_new_allows_non_static_t() {
    fn run<'a>(value: &'a str) -> usize {
        let func: BoxFunction<&'a str, usize> =
            BoxFunction::new(|x: &&'a str| x.len());
        func.apply(&value)
    }

    let text = String::from("hello");
    assert_eq!(run(text.as_str()), 5);
}

#[test]
fn test_box_function_new_allows_non_static_r() {
    fn run<'a>(value: &'a str) -> &'a str {
        let func: BoxFunction<&'a str, &'a str> =
            BoxFunction::new(|x: &&'a str| *x);
        func.apply(&value)
    }

    let text = String::from("qubit");
    assert_eq!(run(text.as_str()), "qubit");
}

#[test]
fn test_rc_function_new_allows_non_static_t() {
    fn run<'a>(value: &'a str) -> usize {
        let func: RcFunction<&'a str, usize> =
            RcFunction::new(|x: &&'a str| x.len());
        func.apply(&value)
    }

    let text = String::from("hello");
    assert_eq!(run(text.as_str()), 5);
}

#[test]
fn test_arc_function_new_allows_non_static_t() {
    fn run<'a>(value: &'a str) -> usize {
        let func: ArcFunction<&'a str, usize> =
            ArcFunction::new(|x: &&'a str| x.len() + 1);
        func.apply(&value)
    }

    let text = String::from("hello");
    assert_eq!(run(text.as_str()), 6);
}

#[test]
fn test_rc_function_new_allows_non_static_r() {
    fn run<'a>(value: &'a str) -> &'a str {
        let func: RcFunction<&'a str, &'a str> =
            RcFunction::new(|x: &&'a str| *x);
        func.apply(&value)
    }

    let text = String::from("qubit");
    assert_eq!(run(text.as_str()), "qubit");
}

// ============================================================================
// BoxFunction Tests - Constructor and Basic Operations
// ============================================================================

#[test]
fn test_box_function_new() {
    // Test BoxFunction::new with simple closure
    let double = BoxFunction::new(|x: &i32| x * 2);
    assert_eq!(double.apply(&21), 42);
    assert_eq!(double.apply(&0), 0);
    assert_eq!(double.apply(&-5), -10);
}

#[test]
fn test_box_function_identity() {
    // Test BoxFunction::identity
    let identity = BoxFunction::<i32, i32>::identity();
    assert_eq!(identity.apply(&42), 42);
    assert_eq!(identity.apply(&0), 0);
    assert_eq!(identity.apply(&-100), -100);
}

#[test]
fn test_box_function_constant() {
    // Test BoxFunction::constant
    let constant = BoxFunction::constant("hello");
    assert_eq!(constant.apply(&123), "hello");
    assert_eq!(constant.apply(&456), "hello");
    assert_eq!(constant.apply(&0), "hello");
}

#[test]
fn test_box_function_apply() {
    // Test Function trait implementation for BoxFunction
    let add_one = BoxFunction::new(|x: &i32| x + 1);
    assert_eq!(add_one.apply(&41), 42);
    assert_eq!(add_one.apply(&0), 1);
    assert_eq!(add_one.apply(&-1), 0);
}

// ============================================================================
// BoxFunction Tests - Composition Methods
// ============================================================================

#[test]
fn test_box_function_and_then() {
    // Test BoxFunction::and_then composition
    let double = BoxFunction::new(|x: &i32| x * 2);
    let to_string = BoxFunction::new(|x: &i32| x.to_string());
    let composed = double.and_then(to_string);
    assert_eq!(composed.apply(&21), "42");
    assert_eq!(composed.apply(&0), "0");
    assert_eq!(composed.apply(&-5), "-10");
}

#[test]
fn test_box_function_and_then_with_closure() {
    // Test and_then with closure
    let double = BoxFunction::new(|x: &i32| x * 2);
    let composed = double.and_then(|x: &i32| x + 10);
    assert_eq!(composed.apply(&16), 42);
}

// ============================================================================
// BoxFunction Tests - Conditional Execution
// ============================================================================

#[test]
fn test_box_function_when_or_else() {
    // Test conditional execution with when/or_else
    let double = BoxFunction::new(|x: &i32| x * 2);
    let identity = BoxFunction::<i32, i32>::identity();
    let conditional = double.when(|x: &i32| *x > 0).or_else(identity);

    assert_eq!(conditional.apply(&5), 10); // when branch
    assert_eq!(conditional.apply(&-5), -5); // or_else branch
    assert_eq!(conditional.apply(&0), 0); // or_else branch
}

#[test]
fn test_box_function_when_with_closure() {
    // Test when with closure predicate and or_else
    let double = BoxFunction::new(|x: &i32| x * 2);
    let conditional = double.when(|x: &i32| *x >= 10).or_else(|x: &i32| *x);

    assert_eq!(conditional.apply(&15), 30); // when branch
    assert_eq!(conditional.apply(&5), 5); // or_else branch
}

#[test]
fn test_box_function_when_with_predicate() {
    // Test when with BoxPredicate
    let double = BoxFunction::new(|x: &i32| x * 2);
    let is_positive = BoxPredicate::new(|x: &i32| *x > 0);
    let conditional = double.when(is_positive).or_else(|x: &i32| -(*x));

    assert_eq!(conditional.apply(&5), 10); // when branch
    assert_eq!(conditional.apply(&-5), 5); // or_else branch
}

// ============================================================================
// BoxFunction Tests - Type Conversions (Function trait)
// ============================================================================

// ============================================================================
// BoxFunction Tests - FunctionOnce Implementation
// ============================================================================

#[test]
fn test_box_function_once_impl_apply() {
    // Test FunctionOnce::apply for BoxFunction
    let double = BoxFunction::new(|x: &i32| x * 2);
    let result = double.apply(&21);
    assert_eq!(result, 42);
}

// ============================================================================
// ArcFunction Tests - Constructor and Basic Operations
// ============================================================================

#[test]
fn test_arc_function_new() {
    // Test ArcFunction::new with simple closure
    let double = ArcFunction::new(|x: &i32| x * 2);
    assert_eq!(double.apply(&21), 42);
    assert_eq!(double.apply(&0), 0);
    assert_eq!(double.apply(&-5), -10);
}

#[test]
fn test_arc_function_identity() {
    // Test ArcFunction::identity
    let identity = ArcFunction::<i32, i32>::identity();
    assert_eq!(identity.apply(&42), 42);
    assert_eq!(identity.apply(&0), 0);
    assert_eq!(identity.apply(&-100), -100);
}

#[test]
fn test_arc_function_constant() {
    // Test ArcFunction::constant
    let constant = ArcFunction::constant("hello");
    assert_eq!(constant.apply(&123), "hello");
    assert_eq!(constant.apply(&456), "hello");
}

#[test]
fn test_arc_function_once_impl_apply() {
    // Test Function trait implementation for ArcFunction
    let add_one = ArcFunction::new(|x: &i32| x + 1);
    assert_eq!(add_one.apply(&41), 42);
    assert_eq!(add_one.apply(&0), 1);
}

#[test]
fn test_arc_function_clone() {
    // Test ArcFunction::clone
    let double = ArcFunction::new(|x: &i32| x * 2);
    let cloned = double.clone();
    assert_eq!(double.apply(&21), 42);
    assert_eq!(cloned.apply(&21), 42);
}

// ============================================================================
// ArcFunction Tests - Composition Methods
// ============================================================================

#[test]
fn test_arc_function_and_then() {
    // Test ArcFunction::and_then composition
    let double = ArcFunction::new(|x: &i32| x * 2);
    let to_string = ArcFunction::new(|x: &i32| x.to_string());
    let composed = double.and_then(to_string);
    assert_eq!(composed.apply(&21), "42");
    // Original still usable
    assert_eq!(double.apply(&10), 20);
}

#[test]
fn test_arc_function_and_then_with_clone() {
    // Test and_then preserving original with clone
    let double = ArcFunction::new(|x: &i32| x * 2);
    let to_string = ArcFunction::new(|x: &i32| x.to_string());
    let composed = double.and_then(to_string.clone());
    assert_eq!(composed.apply(&21), "42");
    assert_eq!(to_string.apply(&5), "5");
}

// ============================================================================
// ArcFunction Tests - Conditional Execution
// ============================================================================

#[test]
fn test_arc_function_when_or_else() {
    // Test conditional execution with when/or_else
    let double = ArcFunction::new(|x: &i32| x * 2);
    let identity = ArcFunction::<i32, i32>::identity();
    let conditional = double.when(|x: &i32| *x > 0).or_else(identity);

    let conditional_clone = conditional.clone();
    assert_eq!(conditional.apply(&5), 10);
    assert_eq!(conditional_clone.apply(&-5), -5);
}

#[test]
fn test_arc_function_when_with_predicate() {
    // Test when with ArcPredicate
    let double = ArcFunction::new(|x: &i32| x * 2);
    let is_positive = ArcPredicate::new(|x: &i32| *x > 0);
    let conditional = double
        .when(is_positive.clone())
        .or_else(ArcFunction::identity());

    assert_eq!(conditional.apply(&5), 10);
    assert!(is_positive.test(&3));
}

// ============================================================================
// ArcFunction Tests - Type Conversions
// ============================================================================

// ============================================================================
// ArcFunction Tests - FunctionOnce Implementation
// ============================================================================

// ============================================================================
// ArcFunction Tests - Thread Safety
// ============================================================================

#[test]
fn test_arc_function_send_sync() {
    // Test that ArcFunction is Send + Sync
    let double = ArcFunction::new(|x: &i32| x * 2);
    let double_clone = double.clone();

    let handle = std::thread::spawn(move || double_clone.apply(&21));

    assert_eq!(handle.join().expect("thread should not panic"), 42);
    assert_eq!(double.apply(&10), 20);
}

// ============================================================================
// RcFunction Tests - Constructor and Basic Operations
// ============================================================================

#[test]
fn test_rc_function_new() {
    // Test RcFunction::new with simple closure
    let double = RcFunction::new(|x: &i32| x * 2);
    assert_eq!(double.apply(&21), 42);
    assert_eq!(double.apply(&0), 0);
    assert_eq!(double.apply(&-5), -10);
}

#[test]
fn test_rc_function_identity() {
    // Test RcFunction::identity
    let identity = RcFunction::<i32, i32>::identity();
    assert_eq!(identity.apply(&42), 42);
    assert_eq!(identity.apply(&0), 0);
    assert_eq!(identity.apply(&-100), -100);
}

#[test]
fn test_rc_function_constant() {
    // Test RcFunction::constant
    let constant = RcFunction::constant("hello");
    assert_eq!(constant.apply(&123), "hello");
    assert_eq!(constant.apply(&456), "hello");
}

#[test]
fn test_rc_function_once_impl_apply() {
    // Test Function trait implementation for RcFunction
    let add_one = RcFunction::new(|x: &i32| x + 1);
    assert_eq!(add_one.apply(&41), 42);
    assert_eq!(add_one.apply(&0), 1);
}

#[test]
fn test_rc_function_clone() {
    // Test RcFunction::clone
    let double = RcFunction::new(|x: &i32| x * 2);
    let cloned = double.clone();
    assert_eq!(double.apply(&21), 42);
    assert_eq!(cloned.apply(&21), 42);
}

// ============================================================================
// RcFunction Tests - Composition Methods
// ============================================================================

#[test]
fn test_rc_function_and_then() {
    // Test RcFunction::and_then composition
    let double = RcFunction::new(|x: &i32| x * 2);
    let to_string = RcFunction::new(|x: &i32| x.to_string());
    let composed = double.and_then(to_string);
    assert_eq!(composed.apply(&21), "42");
    // Original still usable
    assert_eq!(double.apply(&10), 20);
}

#[test]
fn test_rc_function_and_then_with_clone() {
    // Test and_then preserving original with clone
    let double = RcFunction::new(|x: &i32| x * 2);
    let to_string = RcFunction::new(|x: &i32| x.to_string());
    let composed = double.and_then(to_string.clone());
    assert_eq!(composed.apply(&21), "42");
    assert_eq!(to_string.apply(&5), "5");
}

// ============================================================================
// RcFunction Tests - Conditional Execution
// ============================================================================

#[test]
fn test_rc_function_when_or_else() {
    // Test conditional execution with when/or_else
    let double = RcFunction::new(|x: &i32| x * 2);
    let identity = RcFunction::<i32, i32>::identity();
    let conditional = double.when(|x: &i32| *x > 0).or_else(identity);

    let conditional_clone = conditional.clone();
    assert_eq!(conditional.apply(&5), 10);
    assert_eq!(conditional_clone.apply(&-5), -5);
}

#[test]
fn test_rc_function_when_with_predicate() {
    // Test when with RcPredicate
    let double = RcFunction::new(|x: &i32| x * 2);
    let is_positive = RcPredicate::new(|x: &i32| *x > 0);
    let conditional =
        double
            .when(is_positive.clone())
            .or_else(RcFunction::<i32, i32>::identity());

    assert_eq!(conditional.apply(&5), 10);
    assert!(is_positive.test(&3));
}

// ============================================================================
// RcFunction Tests - Type Conversions
// ============================================================================

// ============================================================================
// RcFunction Tests - FunctionOnce Implementation
// ============================================================================

// ============================================================================
// Edge Cases and Boundary Tests
// ============================================================================

#[test]
fn test_function_with_zero() {
    // Test functions with zero input
    let double = BoxFunction::new(|x: &i32| x * 2);
    assert_eq!(double.apply(&0), 0);
}

#[test]
fn test_function_with_negative() {
    // Test functions with negative input
    let double = BoxFunction::new(|x: &i32| x * 2);
    assert_eq!(double.apply(&-42), -84);
}

#[test]
fn test_function_with_max_value() {
    // Test functions with maximum value
    let identity = BoxFunction::<i32, i32>::identity();
    assert_eq!(identity.apply(&i32::MAX), i32::MAX);
}

#[test]
fn test_function_with_min_value() {
    // Test functions with minimum value
    let identity = BoxFunction::<i32, i32>::identity();
    assert_eq!(identity.apply(&i32::MIN), i32::MIN);
}

#[test]
fn test_function_chain_multiple() {
    // Test chaining multiple functions
    let add_one = BoxFunction::new(|x: &i32| x + 1);
    let double = BoxFunction::new(|x: &i32| x * 2);
    let add_ten = BoxFunction::new(|x: &i32| x + 10);

    let composed = add_one.and_then(double).and_then(add_ten);
    assert_eq!(composed.apply(&5), 22); // ((5 + 1) * 2) + 10
}

#[test]
fn test_function_with_string() {
    // Test functions with String type
    let to_upper = BoxFunction::new(|s: &String| s.to_uppercase());
    let input = String::from("hello");
    assert_eq!(to_upper.apply(&input), "HELLO");
}

#[test]
fn test_function_with_vec() {
    // Test functions with Vec type
    let get_len = BoxFunction::new(|v: &Vec<i32>| v.len());
    let vec = vec![1, 2, 3, 4, 5];
    assert_eq!(get_len.apply(&vec), 5);
}

#[test]
fn test_function_with_option() {
    // Test functions with Option type
    let unwrap_or_zero = BoxFunction::new(|opt: &Option<i32>| opt.unwrap_or(0));
    assert_eq!(unwrap_or_zero.apply(&Some(42)), 42);
    assert_eq!(unwrap_or_zero.apply(&None), 0);
}

#[test]
fn test_conditional_function_edge_cases() {
    // Test conditional function with boundary values
    let double = BoxFunction::new(|x: &i32| x * 2);
    let negate = BoxFunction::new(|x: &i32| -(*x));
    let conditional = double.when(|x: &i32| *x >= 0).or_else(negate);

    assert_eq!(conditional.apply(&0), 0); // Boundary: zero
    assert_eq!(conditional.apply(&1), 2); // Positive
    assert_eq!(conditional.apply(&-1), 1); // Negative
}

// ============================================================================
// FnFunctionOps Extension Trait Tests
// ============================================================================

#[test]
fn test_fn_function_ops_and_then() {
    // Test FnFunctionOps::and_then for closures
    use qubit_function::FnFunctionOps;

    let double = |x: &i32| x * 2;
    let to_string = |x: &i32| x.to_string();
    let composed = double.and_then(to_string);
    assert_eq!(composed.apply(&21), "42");
}

#[test]
fn test_fn_function_ops_when() {
    // Test FnFunctionOps::when for closures
    use qubit_function::FnFunctionOps;

    let double = |x: &i32| x * 2;
    let conditional = double.when(|x: &i32| *x > 0).or_else(|x: &i32| -(*x));
    assert_eq!(conditional.apply(&5), 10);
    assert_eq!(conditional.apply(&-5), 5);
}

// ============================================================================
// Function Trait Default Implementation Tests
// ============================================================================

#[cfg(test)]
mod function_default_impl_tests {
    use qubit_function::Function;

    /// Custom struct that only implements the core apply method of Function
    /// trait All into_xxx() and to_xxx() methods use default implementation
    struct CustomFunction {
        multiplier: i32,
    }

    impl Function<i32, i32> for CustomFunction {
        fn apply(&self, input: &i32) -> i32 {
            input * self.multiplier
        }
        // Does not override any into_xxx() or to_xxx() methods, testing default
        // implementations
    }

    /// Cloneable custom function for testing to_xxx() methods
    #[derive(Clone)]
    struct CloneableCustomFunction {
        multiplier: i32,
    }

    impl Function<i32, i32> for CloneableCustomFunction {
        fn apply(&self, input: &i32) -> i32 {
            input * self.multiplier
        }
        // Does not override any into_xxx() or to_xxx() methods, testing default
        // implementations
    }
}

// ============================================================================
// ArcConditionalFunction Clone Tests
// ============================================================================

#[test]
fn test_arc_conditional_function_clone() {
    let double = ArcFunction::new(|x: &i32| x * 2);
    let conditional = double.when(|x: &i32| *x > 0);

    // Clone conditional function
    let conditional_clone = conditional.clone();

    // Both cloned conditional functions work properly
    let result1 = conditional.or_else(|x: &i32| -(*x));
    let result2 = conditional_clone.or_else(|x: &i32| x + 100);

    assert_eq!(result1.apply(&5), 10); // Condition met: 5 * 2
    assert_eq!(result1.apply(&-5), 5); // Condition not met: -(-5)
    assert_eq!(result2.apply(&5), 10); // Condition met: 5 * 2
    assert_eq!(result2.apply(&-5), 95); // Condition not met: -5 + 100
}

#[test]
fn test_arc_conditional_function_clone_multiple() {
    let triple = ArcFunction::new(|x: &i32| x * 3);
    let conditional = triple.when(|x: &i32| *x % 2 == 0);

    // Create multiple clones
    let clone1 = conditional.clone();
    let clone2 = conditional.clone();
    let clone3 = conditional.clone();

    let result1 = conditional.or_else(|x: &i32| *x);
    let result2 = clone1.or_else(|x: &i32| *x);
    let result3 = clone2.or_else(|x: &i32| *x);
    let result4 = clone3.or_else(|x: &i32| *x);

    // All clones work properly
    assert_eq!(result1.apply(&4), 12); // Even number: 4 * 3
    assert_eq!(result2.apply(&4), 12);
    assert_eq!(result3.apply(&4), 12);
    assert_eq!(result4.apply(&4), 12);

    assert_eq!(result1.apply(&5), 5); // Odd number: 5
    assert_eq!(result2.apply(&5), 5);
    assert_eq!(result3.apply(&5), 5);
    assert_eq!(result4.apply(&5), 5);
}

// ============================================================================
// ConditionalFunction Debug and Display Tests
// ============================================================================

#[test]
fn test_box_conditional_function_debug_display() {
    // Test Debug and Display for BoxConditionalFunction without name
    let double = BoxFunction::new(|x: &i32| x * 2);
    let conditional = double.when(|x: &i32| *x > 0);

    let debug_str = format!("{:?}", conditional);
    assert!(debug_str.contains("BoxConditionalFunction"));
    assert!(debug_str.contains("name"));
    assert!(debug_str.contains("function"));
    assert!(debug_str.contains("predicate"));

    let display_str = format!("{}", conditional);
    assert!(display_str.starts_with("BoxConditionalFunction("));
    assert!(display_str.contains("BoxFunction"));
    assert!(display_str.contains("BoxPredicate"));
    assert!(display_str.ends_with(")"));

    // Test Debug and Display for BoxConditionalFunction with name
    let triple = BoxFunction::new_with_name("triple_func", |x: &i32| x * 3);
    let named_conditional = triple.when(|x: &i32| *x % 2 == 0);

    let named_debug_str = format!("{:?}", named_conditional);
    assert!(named_debug_str.contains("BoxConditionalFunction"));
    assert!(named_debug_str.contains("name"));
    assert!(named_debug_str.contains("function"));
    assert!(named_debug_str.contains("predicate"));

    let named_display_str = format!("{}", named_conditional);
    assert!(named_display_str.starts_with("BoxConditionalFunction("));
    assert!(named_display_str.contains("BoxFunction(triple_func)"));
    assert!(named_display_str.contains("BoxPredicate"));
    assert!(named_display_str.ends_with(")"));
}

#[test]
fn test_rc_conditional_function_debug_display() {
    // Test Debug and Display for RcConditionalFunction without name
    let double = RcFunction::new(|x: &i32| x * 2);
    let conditional = double.when(|x: &i32| *x > 0);

    let debug_str = format!("{:?}", conditional);
    assert!(debug_str.contains("RcConditionalFunction"));
    assert!(debug_str.contains("name"));
    assert!(debug_str.contains("function"));
    assert!(debug_str.contains("predicate"));

    let display_str = format!("{}", conditional);
    assert!(display_str.starts_with("RcConditionalFunction("));
    assert!(display_str.contains("RcFunction"));
    assert!(display_str.contains("RcPredicate"));
    assert!(display_str.ends_with(")"));

    // Test Debug and Display for RcConditionalFunction with name
    let triple = RcFunction::new_with_name("rc_triple_func", |x: &i32| x * 3);
    let named_conditional = triple.when(|x: &i32| *x % 2 == 0);

    let named_debug_str = format!("{:?}", named_conditional);
    assert!(named_debug_str.contains("RcConditionalFunction"));
    assert!(named_debug_str.contains("name"));
    assert!(named_debug_str.contains("function"));
    assert!(named_debug_str.contains("predicate"));

    let named_display_str = format!("{}", named_conditional);
    assert!(named_display_str.starts_with("RcConditionalFunction("));
    assert!(named_display_str.contains("RcFunction(rc_triple_func)"));
    assert!(named_display_str.contains("RcPredicate"));
    assert!(named_display_str.ends_with(")"));
}

#[test]
fn test_arc_conditional_function_debug_display() {
    // Test Debug and Display for ArcConditionalFunction without name
    let double = ArcFunction::new(|x: &i32| x * 2);
    let conditional = double.when(|x: &i32| *x > 0);

    let debug_str = format!("{:?}", conditional);
    assert!(debug_str.contains("ArcConditionalFunction"));
    assert!(debug_str.contains("name"));
    assert!(debug_str.contains("function"));
    assert!(debug_str.contains("predicate"));

    let display_str = format!("{}", conditional);
    assert!(display_str.starts_with("ArcConditionalFunction("));
    assert!(display_str.contains("ArcFunction"));
    assert!(display_str.contains("ArcPredicate"));
    assert!(display_str.ends_with(")"));

    // Test Debug and Display for ArcConditionalFunction with name
    let triple = ArcFunction::new_with_name("arc_triple_func", |x: &i32| x * 3);
    let named_conditional = triple.when(|x: &i32| *x % 2 == 0);

    let named_debug_str = format!("{:?}", named_conditional);
    assert!(named_debug_str.contains("ArcConditionalFunction"));
    assert!(named_debug_str.contains("name"));
    assert!(named_debug_str.contains("function"));
    assert!(named_debug_str.contains("predicate"));

    let named_display_str = format!("{}", named_conditional);
    assert!(named_display_str.starts_with("ArcConditionalFunction("));
    assert!(named_display_str.contains("ArcFunction(arc_triple_func)"));
    assert!(named_display_str.contains("ArcPredicate"));
    assert!(named_display_str.ends_with(")"));
}

// ============================================================================
// RcConditionalFunction Clone Tests
// ============================================================================

#[test]
fn test_rc_conditional_function_clone() {
    let double = RcFunction::new(|x: &i32| x * 2);
    let conditional = double.when(|x: &i32| *x > 0);

    // Clone conditional function
    let conditional_clone = conditional.clone();

    // Both cloned conditional functions work properly
    let result1 = conditional.or_else(|x: &i32| -(*x));
    let result2 = conditional_clone.or_else(|x: &i32| x + 100);

    assert_eq!(result1.apply(&5), 10); // Condition met: 5 * 2
    assert_eq!(result1.apply(&-5), 5); // Condition not met: -(-5)
    assert_eq!(result2.apply(&5), 10); // Condition met: 5 * 2
    assert_eq!(result2.apply(&-5), 95); // Condition not met: -5 + 100
}

#[test]
fn test_rc_conditional_function_clone_multiple() {
    let triple = RcFunction::new(|x: &i32| x * 3);
    let conditional = triple.when(|x: &i32| *x % 2 == 0);

    // Create multiple clones
    let clone1 = conditional.clone();
    let clone2 = conditional.clone();
    let clone3 = conditional.clone();

    let result1 = conditional.or_else(|x: &i32| *x);
    let result2 = clone1.or_else(|x: &i32| *x);
    let result3 = clone2.or_else(|x: &i32| *x);
    let result4 = clone3.or_else(|x: &i32| *x);

    // All clones work properly
    assert_eq!(result1.apply(&4), 12); // Even number: 4 * 3
    assert_eq!(result2.apply(&4), 12);
    assert_eq!(result3.apply(&4), 12);
    assert_eq!(result4.apply(&4), 12);

    assert_eq!(result1.apply(&5), 5); // Odd number: 5
    assert_eq!(result2.apply(&5), 5);
    assert_eq!(result3.apply(&5), 5);
    assert_eq!(result4.apply(&5), 5);
}

// ============================================================================
// Name Preservation Tests for into_xxx and to_xxx Methods
// ============================================================================

#[test]
fn test_box_function_clear_name() {
    let mut function = BoxFunction::new_with_name("named_fn", |x: &i32| x * 2);
    assert_eq!(function.name(), Some("named_fn"));

    function.clear_name();
    assert_eq!(function.name(), None);
    assert_eq!(function.apply(&21), 42);
}

#[test]
fn test_box_function_set_name_same_value_keeps_storage() {
    let mut function =
        BoxFunction::new_with_name("stable_name", |x: &i32| x * 2);
    let ptr_before = function
        .name()
        .expect("name should be initialized")
        .as_ptr();

    function.set_name("stable_name");
    let ptr_after = function
        .name()
        .expect("name should remain initialized")
        .as_ptr();

    assert_eq!(function.name(), Some("stable_name"));
    assert_eq!(ptr_before, ptr_after);
}
