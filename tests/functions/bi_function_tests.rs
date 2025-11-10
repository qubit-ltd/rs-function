/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! Comprehensive tests for BiFunction trait and its implementations

use prism3_function::{
    ArcBiFunction,
    BiFunction,
    BiFunctionOnce,
    BoxBiFunction,
    RcBiFunction,
};

// ============================================================================
// BiFunction Trait Tests - Core Functionality
// ============================================================================

#[test]
fn test_bi_function_trait_apply() {
    // Test that BiFunction trait's apply method works correctly
    let add = |x: &i32, y: &i32| *x + *y;
    assert_eq!(add.apply(&20, &22), 42);
    assert_eq!(add.apply(&0, &0), 0);
    assert_eq!(add.apply(&-10, &5), -5);
}

#[test]
fn test_bi_function_trait_into_box() {
    // Test conversion from closure to BoxBiFunction
    let add = |x: &i32, y: &i32| *x + *y;
    let boxed = BiFunction::into_box(add);
    assert_eq!(boxed.apply(&20, &22), 42);
}

#[test]
fn test_bi_function_trait_into_rc() {
    // Test conversion from closure to RcBiFunction
    let add = |x: &i32, y: &i32| *x + *y;
    let rc = add.into_rc();
    assert_eq!(rc.apply(&20, &22), 42);
}

#[test]
fn test_bi_function_trait_into_arc() {
    // Test conversion from closure to ArcBiFunction
    let add = |x: &i32, y: &i32| *x + *y;
    let arc = add.into_arc();
    assert_eq!(arc.apply(&20, &22), 42);
}

#[test]
fn test_bi_function_trait_into_fn() {
    // Test conversion to closure
    let add = |x: &i32, y: &i32| *x + *y;
    let func = BiFunction::into_fn(add);
    assert_eq!(func(&20, &22), 42);
}

#[test]
fn test_bi_function_trait_into_once() {
    // Test conversion to BiFunctionOnce
    let add = |x: &i32, y: &i32| *x + *y;
    let once = add.into_once();
    assert_eq!(once.apply(&20, &22), 42);
}

// ============================================================================
// BoxBiFunction Tests
// ============================================================================

#[test]
fn test_box_bi_function_new() {
    let add = BoxBiFunction::new(|x: &i32, y: &i32| *x + *y);
    assert_eq!(add.apply(&10, &15), 25);
}

#[test]
fn test_box_bi_function_constant() {
    let constant = BoxBiFunction::constant(42);
    assert_eq!(constant.apply(&1, &2), 42);
    assert_eq!(constant.apply(&100, &200), 42);
}

// BoxBiFunction doesn't implement Clone
// #[test]
// fn test_box_bi_function_clone() {
//     let original = BoxBiFunction::new(|x: &i32, y: &i32| *x * *y);
//     let cloned = original.clone();
//     assert_eq!(original.apply(&6, &7), 42);
//     assert_eq!(cloned.apply(&6, &7), 42);
// }

#[test]
fn test_box_bi_function_debug_display() {
    let func = BoxBiFunction::new(|x: &i32, y: &i32| *x + *y);
    let debug_str = format!("{:?}", func);
    assert!(debug_str.contains("BoxBiFunction"));
    let display_str = format!("{}", func);
    assert!(display_str.starts_with("BoxBiFunction"));
}

// ============================================================================
// RcBiFunction Tests
// ============================================================================

#[test]
fn test_rc_bi_function_new() {
    let multiply = RcBiFunction::new(|x: &i32, y: &i32| *x * *y);
    assert_eq!(multiply.apply(&6, &7), 42);
}

#[test]
fn test_rc_bi_function_constant() {
    let constant = RcBiFunction::constant(100);
    assert_eq!(constant.apply(&1, &2), 100);
}

#[test]
fn test_rc_bi_function_clone() {
    let original = RcBiFunction::new(|x: &i32, y: &i32| *x + *y);
    let cloned = original.clone();
    assert_eq!(original.apply(&10, &20), 30);
    assert_eq!(cloned.apply(&10, &20), 30);
}

#[test]
fn test_rc_bi_function_debug_display() {
    let func = RcBiFunction::new(|x: &i32, y: &i32| *x + *y);
    let debug_str = format!("{:?}", func);
    assert!(debug_str.contains("RcBiFunction"));
    let display_str = format!("{}", func);
    assert!(display_str.starts_with("RcBiFunction"));
}

// ============================================================================
// ArcBiFunction Tests
// ============================================================================

#[test]
fn test_arc_bi_function_new() {
    let divide = ArcBiFunction::new(|x: &i32, y: &i32| *x / *y);
    assert_eq!(divide.apply(&42, &2), 21);
}

#[test]
fn test_arc_bi_function_constant() {
    let constant = ArcBiFunction::constant("hello".to_string());
    assert_eq!(constant.apply(&1, &2), "hello");
}

#[test]
fn test_arc_bi_function_clone() {
    let original = ArcBiFunction::new(|x: &i32, y: &i32| *x - *y);
    let cloned = original.clone();
    assert_eq!(original.apply(&50, &8), 42);
    assert_eq!(cloned.apply(&50, &8), 42);
}

#[test]
fn test_arc_bi_function_debug_display() {
    let func = ArcBiFunction::new(|x: &i32, y: &i32| *x + *y);
    let debug_str = format!("{:?}", func);
    assert!(debug_str.contains("ArcBiFunction"));
    let display_str = format!("{}", func);
    assert!(display_str.starts_with("ArcBiFunction"));
}

// ============================================================================
// Conversion Tests
// ============================================================================

#[test]
fn test_bi_function_conversions() {
    let add = |x: &i32, y: &i32| *x + *y;

    // Test to_box
    let boxed = BiFunction::to_box(&add);
    assert_eq!(boxed.apply(&10, &20), 30);

    // Test to_rc
    let rc = BiFunction::to_rc(&add);
    assert_eq!(rc.apply(&10, &20), 30);

    // Test to_arc
    let arc = BiFunction::to_arc(&add);
    assert_eq!(arc.apply(&10, &20), 30);

    // Test to_fn
    let func = BiFunction::to_fn(&add);
    assert_eq!(func(&10, &20), 30);

    // Test to_once
    let once = add.to_once();
    assert_eq!(once.apply(&10, &20), 30);
}

// ============================================================================
// BiFunction Composition Tests
// ============================================================================

#[test]
fn test_bi_function_and_then() {
    use prism3_function::FnBiFunctionOps;

    let add = |x: &i32, y: &i32| *x + *y;
    let double = |x: &i32| *x * 2;

    let composed = add.and_then(double);
    assert_eq!(composed.apply(&10, &15), 50); // (10 + 15) * 2 = 50
}

#[test]
fn test_bi_function_when_or_else() {
    use prism3_function::FnBiFunctionOps;

    let add = |x: &i32, y: &i32| *x + *y;
    let multiply = |x: &i32, y: &i32| *x * *y;

    let conditional = add
        .when(|x: &i32, y: &i32| *x > 0 && *y > 0)
        .or_else(multiply);

    assert_eq!(conditional.apply(&5, &3), 8); // add: 5 + 3 = 8
    assert_eq!(conditional.apply(&-5, &3), -15); // multiply: -5 * 3 = -15
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_bi_function_with_complex_types() {
    let concat = |s1: &&str, s2: &&str| format!("{} {}", *s1, *s2);
    let boxed = BoxBiFunction::new(concat);

    assert_eq!(boxed.apply(&"Hello", &"World"), "Hello World");
}

#[test]
fn test_bi_function_with_option_types() {
    let combine_options = |opt1: &Option<i32>, opt2: &Option<i32>| match (opt1, opt2) {
        (Some(a), Some(b)) => Some(a + b),
        _ => None,
    };

    let func = RcBiFunction::new(combine_options);

    assert_eq!(func.apply(&Some(10), &Some(20)), Some(30));
    assert_eq!(func.apply(&Some(10), &None), None);
    assert_eq!(func.apply(&None, &Some(20)), None);
}

#[test]
fn test_bi_function_with_result_types() {
    let safe_divide = |a: &i32, b: &i32| {
        if *b == 0 {
            Err("Division by zero")
        } else {
            Ok(*a / *b)
        }
    };

    let func = ArcBiFunction::new(safe_divide);

    assert_eq!(func.apply(&10, &2), Ok(5));
    assert_eq!(func.apply(&10, &0), Err("Division by zero"));
}
