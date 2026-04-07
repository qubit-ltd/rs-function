/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! Comprehensive tests for BiMutatingFunctionOnce trait and its implementations

use qubit_function::{
    BiMutatingFunctionOnce,
    BoxBiMutatingFunctionOnce,
    FnBiMutatingFunctionOnceOps,
};

// ============================================================================
// Helper Functions and Data Structures
// ============================================================================

fn append_strings_once(x: &mut String, y: &mut String) -> usize {
    x.push_str("_modified");
    y.push_str("_changed");
    x.len() + y.len()
}

#[derive(Clone, Debug, PartialEq)]
struct TestStruct {
    value: i32,
}

impl TestStruct {
    fn new(value: i32) -> Self {
        Self { value }
    }

    fn modify(&mut self, other: &mut Self) -> i32 {
        self.value += other.value;
        other.value *= 2;
        self.value + other.value
    }
}

fn modify_structs_once(a: &mut TestStruct, b: &mut TestStruct) -> i32 {
    a.modify(b)
}

// ============================================================================
// BiMutatingFunctionOnce Trait Tests - Core Functionality
// ============================================================================

#[test]
fn test_bi_mutating_function_once_trait_apply() {
    // Test that BiMutatingFunctionOnce trait's apply method works correctly
    let swap_sum = |x: &mut i32, y: &mut i32| {
        std::mem::swap(&mut *x, &mut *y);
        *x + *y
    };

    let mut a = 20;
    let mut b = 22;
    assert_eq!(swap_sum.apply(&mut a, &mut b), 42);
    assert_eq!(a, 22);
    assert_eq!(b, 20);
}

#[test]
fn test_bi_mutating_function_once_trait_apply_with_complex_types() {
    let modify = |a: &mut TestStruct, b: &mut TestStruct| a.modify(b);

    let mut s1 = TestStruct::new(10);
    let mut s2 = TestStruct::new(5);
    let result = modify.apply(&mut s1, &mut s2);

    assert_eq!(result, 25); // (10+5) + (5*2) = 15 + 10 = 25
    assert_eq!(s1.value, 15);
    assert_eq!(s2.value, 10);
}

#[test]
fn test_bi_mutating_function_once_trait_into_box() {
    // Test conversion from closure to BoxBiMutatingFunctionOnce
    let swap_sum = |x: &mut i32, y: &mut i32| {
        std::mem::swap(&mut *x, &mut *y);
        *x + *y
    };
    let boxed = BiMutatingFunctionOnce::into_box(swap_sum);

    let mut a = 20;
    let mut b = 22;
    assert_eq!(boxed.apply(&mut a, &mut b), 42);
    assert_eq!(a, 22);
    assert_eq!(b, 20);
}

#[test]
fn test_bi_mutating_function_once_trait_into_fn() {
    // Test conversion to closure
    let swap_sum = |x: &mut i32, y: &mut i32| {
        std::mem::swap(&mut *x, &mut *y);
        *x + *y
    };
    let func = BiMutatingFunctionOnce::into_fn(swap_sum);

    let mut a = 20;
    let mut b = 22;
    assert_eq!(func(&mut a, &mut b), 42);
    assert_eq!(a, 22);
    assert_eq!(b, 20);
}

#[test]
fn test_bi_mutating_function_once_trait_to_box() {
    // Test non-consuming conversion to BoxBiMutatingFunctionOnce
    let swap_sum = |x: &mut i32, y: &mut i32| {
        std::mem::swap(&mut *x, &mut *y);
        *x + *y
    };
    let boxed = swap_sum.to_box();

    let mut a = 20;
    let mut b = 22;
    assert_eq!(boxed.apply(&mut a, &mut b), 42);
    assert_eq!(a, 22);
    assert_eq!(b, 20);

    // Original closure should still be usable (it's Clone)
    let mut c = 30;
    let mut d = 32;
    assert_eq!(swap_sum.apply(&mut c, &mut d), 62);
}

#[test]
fn test_bi_mutating_function_once_trait_to_fn() {
    // Test non-consuming conversion to closure
    let swap_sum = |x: &mut i32, y: &mut i32| {
        std::mem::swap(&mut *x, &mut *y);
        *x + *y
    };
    let func = swap_sum.to_fn();

    let mut a = 20;
    let mut b = 22;
    assert_eq!(func(&mut a, &mut b), 42);
    assert_eq!(a, 22);
    assert_eq!(b, 20);

    // Original closure should still be usable
    let mut c = 30;
    let mut d = 32;
    assert_eq!(swap_sum.apply(&mut c, &mut d), 62);
}

// ============================================================================
// BoxBiMutatingFunctionOnce Tests
// ============================================================================

#[test]
fn test_box_bi_mutating_function_once_new() {
    let swap_sum = BoxBiMutatingFunctionOnce::new(|x: &mut i32, y: &mut i32| {
        std::mem::swap(&mut *x, &mut *y);
        *x + *y
    });
    let mut a = 10;
    let mut b = 15;
    assert_eq!(swap_sum.apply(&mut a, &mut b), 25);
    assert_eq!(a, 15);
    assert_eq!(b, 10);
}

#[test]
fn test_box_bi_mutating_function_once_new_with_name() {
    let swap_sum = BoxBiMutatingFunctionOnce::new_with_name(
        "swap_and_sum_once",
        |x: &mut i32, y: &mut i32| {
            std::mem::swap(&mut *x, &mut *y);
            *x + *y
        },
    );
    assert_eq!(swap_sum.name(), Some("swap_and_sum_once"));
    let mut a = 10;
    let mut b = 15;
    assert_eq!(swap_sum.apply(&mut a, &mut b), 25);
}

#[test]
fn test_box_bi_mutating_function_once_new_with_optional_name() {
    let swap_sum = BoxBiMutatingFunctionOnce::new_with_optional_name(
        |x: &mut i32, y: &mut i32| {
            std::mem::swap(&mut *x, &mut *y);
            *x + *y
        },
        Some("test_function_once".to_string()),
    );
    assert_eq!(swap_sum.name(), Some("test_function_once"));

    let no_name =
        BoxBiMutatingFunctionOnce::new_with_optional_name(|x: &mut i32, y: &mut i32| *x + *y, None);
    assert_eq!(no_name.name(), None);
}

#[test]
fn test_box_bi_mutating_function_once_name_and_set_name() {
    let mut swap_sum = BoxBiMutatingFunctionOnce::new(|x: &mut i32, y: &mut i32| {
        std::mem::swap(&mut *x, &mut *y);
        *x + *y
    });

    assert_eq!(swap_sum.name(), None);
    swap_sum.set_name("modified_name_once");
    assert_eq!(swap_sum.name(), Some("modified_name_once"));
    swap_sum.set_name("another_name_once");
    assert_eq!(swap_sum.name(), Some("another_name_once"));
}

#[test]
fn test_box_bi_mutating_function_once_constant() {
    let constant = BoxBiMutatingFunctionOnce::constant(42);
    let mut a = 1;
    let mut b = 2;
    assert_eq!(constant.apply(&mut a, &mut b), 42);
    assert_eq!(a, 1); // inputs unchanged
    assert_eq!(b, 2);
}

#[test]
fn test_box_bi_mutating_function_once_debug_display() {
    let swap_sum = BoxBiMutatingFunctionOnce::new(|x: &mut i32, y: &mut i32| {
        std::mem::swap(&mut *x, &mut *y);
        *x + *y
    });

    let debug_str = format!("{:?}", swap_sum);
    assert!(debug_str.contains("BoxBiMutatingFunctionOnce"));

    let display_str = format!("{}", swap_sum);
    assert!(display_str.contains("BoxBiMutatingFunctionOnce"));
}

#[test]
fn test_box_bi_mutating_function_once_with_strings() {
    let append = BoxBiMutatingFunctionOnce::new(append_strings_once);
    let mut s1 = "hello".to_string();
    let mut s2 = "world".to_string();

    let result = append.apply(&mut s1, &mut s2);
    assert_eq!(result, 14 + 13); // "hello_modified".len() + "world_changed".len()
    assert_eq!(s1, "hello_modified");
    assert_eq!(s2, "world_changed");
}

#[test]
fn test_box_bi_mutating_function_once_with_structs() {
    let modify = BoxBiMutatingFunctionOnce::new(modify_structs_once);
    let mut s1 = TestStruct::new(10);
    let mut s2 = TestStruct::new(5);

    let result = modify.apply(&mut s1, &mut s2);
    assert_eq!(result, 25); // (10+5) + (5*2) = 15 + 10 = 25
    assert_eq!(s1.value, 15);
    assert_eq!(s2.value, 10);
}

#[test]
fn test_box_bi_mutating_function_once_one_time_use() {
    // Test that BoxBiMutatingFunctionOnce can only be used once
    let counter = std::rc::Rc::new(std::cell::RefCell::new(0));
    let counter_clone = std::rc::Rc::clone(&counter);

    let increment = BoxBiMutatingFunctionOnce::new(move |x: &mut i32, y: &mut i32| {
        *counter_clone.borrow_mut() += 1;
        *x += 1;
        *y += 1;
        *x + *y
    });

    let mut a = 10;
    let mut b = 20;
    assert_eq!(increment.apply(&mut a, &mut b), 32); // 11 + 21 = 32
    assert_eq!(*counter.borrow(), 1);
    assert_eq!(a, 11);
    assert_eq!(b, 21);
}

// ============================================================================
// Function Composition Tests - and_then
// ============================================================================

#[test]
fn test_fn_bi_mutating_function_once_ops_and_then() {
    let swap_and_sum = |x: &mut i32, y: &mut i32| {
        std::mem::swap(&mut *x, &mut *y);
        *x + *y
    };

    let double = |result: &i32| *result * 2;
    let composed = swap_and_sum.and_then(double);

    let mut a = 3;
    let mut b = 5;
    // swap_and_sum: a=5, b=3, result=5+3=8
    // double: 8*2=16
    assert_eq!(composed.apply(&mut a, &mut b), 16);
    assert_eq!(a, 5);
    assert_eq!(b, 3);
}

#[test]
fn test_fn_bi_mutating_function_once_ops_and_then_chain() {
    let add_and_modify = |x: &mut i32, y: &mut i32| {
        *x += 10;
        *y += 20;
        *x + *y
    };

    let to_string = |x: &i32| x.to_string();
    let add_prefix = |s: &mut String| {
        let result = format!("Result: {}", *s);
        *s = String::new();
        result
    };

    let composed = add_and_modify.and_then(to_string).and_then(add_prefix);

    let mut a = 5;
    let mut b = 3;
    // add_and_modify: a=15, b=23, result=15+23=38
    // to_string: "38"
    // add_prefix: "Result: 38"
    let result = composed.apply(&mut a, &mut b);
    assert_eq!(result, "Result: 38");
    assert_eq!(a, 15);
    assert_eq!(b, 23);
}

// ============================================================================
// Conditional Function Tests - when/or_else
// ============================================================================

#[test]
fn test_fn_bi_mutating_function_once_ops_when_or_else() {
    let swap_and_sum = |x: &mut i32, y: &mut i32| {
        std::mem::swap(&mut *x, &mut *y);
        *x + *y
    };

    let multiply = |x: &mut i32, y: &mut i32| {
        *x *= *y;
        *x
    };

    let conditional = swap_and_sum
        .when(|x: &i32, y: &i32| *x > 0 && *y > 0)
        .or_else(multiply);

    // Test when condition is true
    let mut a = 5;
    let mut b = 3;
    assert_eq!(conditional.apply(&mut a, &mut b), 8); // swap_and_sum: (3+5) = 8
    assert_eq!(a, 3); // swapped from 5
    assert_eq!(b, 5); // swapped from 3

    // Test when condition is false (negative numbers) - create separate conditional
    let conditional_false = swap_and_sum
        .when(|x: &i32, y: &i32| *x > 0 && *y > 0)
        .or_else(multiply);
    let mut c = -5;
    let mut d = 3;
    assert_eq!(conditional_false.apply(&mut c, &mut d), -15); // multiply: (-5 * 3) = -15
    assert_eq!(c, -15);
    assert_eq!(d, 3);
}

#[test]
fn test_box_conditional_bi_mutating_function_once() {
    let swap_and_sum = BoxBiMutatingFunctionOnce::new(|x: &mut i32, y: &mut i32| {
        std::mem::swap(&mut *x, &mut *y);
        *x + *y
    });

    let multiply = BoxBiMutatingFunctionOnce::new(|x: &mut i32, y: &mut i32| {
        *x *= *y;
        *x
    });

    let conditional = swap_and_sum
        .when(|x: &i32, _y: &i32| *x > 0)
        .or_else(multiply);

    // Test when condition is true
    let mut a = 5;
    let mut b = 3;
    assert_eq!(conditional.apply(&mut a, &mut b), 8); // swap_and_sum executed

    // Test when condition is false - create a new conditional since BiMutatingFunctionOnce consumes self
    let conditional2 = BoxBiMutatingFunctionOnce::new(|x: &mut i32, y: &mut i32| {
        std::mem::swap(&mut *x, &mut *y);
        *x + *y
    })
    .when(|x: &i32, _y: &i32| *x > 0)
    .or_else(BoxBiMutatingFunctionOnce::new(
        |x: &mut i32, y: &mut i32| {
            *x *= *y;
            *x
        },
    ));
    let mut c = -5;
    let mut d = 3;
    assert_eq!(conditional2.apply(&mut c, &mut d), -15); // multiply executed
}

#[test]
fn test_conditional_bi_mutating_function_once_with_structs() {
    let modify = BoxBiMutatingFunctionOnce::new(modify_structs_once);
    let no_op = BoxBiMutatingFunctionOnce::new(|_a: &mut TestStruct, _b: &mut TestStruct| 0);

    let conditional = modify
        .when(|a: &TestStruct, b: &TestStruct| a.value > 0 && b.value > 0)
        .or_else(no_op);

    // Test when condition is true
    let mut s1 = TestStruct::new(10);
    let mut s2 = TestStruct::new(5);
    let result = conditional.apply(&mut s1, &mut s2);
    assert_eq!(result, 25); // modify executed: (10+5) + (5*2) = 15 + 10 = 25
    assert_eq!(s1.value, 15);
    assert_eq!(s2.value, 10);

    // Test when condition is false - create new conditional since BiMutatingFunctionOnce consumes self
    let conditional2 = BoxBiMutatingFunctionOnce::new(modify_structs_once)
        .when(|a: &TestStruct, b: &TestStruct| a.value > 0 && b.value > 0)
        .or_else(BoxBiMutatingFunctionOnce::new(
            |_a: &mut TestStruct, _b: &mut TestStruct| 0,
        ));
    let mut s3 = TestStruct::new(-10);
    let mut s4 = TestStruct::new(5);
    let result2 = conditional2.apply(&mut s3, &mut s4);
    assert_eq!(result2, 0); // no_op executed
    assert_eq!(s3.value, -10); // unchanged
    assert_eq!(s4.value, 5); // unchanged
}

// ============================================================================
// Edge Cases and Boundary Tests
// ============================================================================

#[test]
fn test_bi_mutating_function_once_with_zero_values() {
    let add = |x: &mut i32, y: &mut i32| {
        *x += *y;
        *x
    };

    let mut a = 0;
    let mut b = 0;
    assert_eq!(add.apply(&mut a, &mut b), 0);
    assert_eq!(a, 0);
    assert_eq!(b, 0);

    let mut c = 0;
    let mut d = 5;
    assert_eq!(add.apply(&mut c, &mut d), 5);
    assert_eq!(c, 5);
    assert_eq!(d, 5);
}

#[test]
fn test_bi_mutating_function_once_with_negative_values() {
    let multiply = |x: &mut i32, y: &mut i32| {
        *x *= *y;
        *x
    };

    let mut a = -5;
    let mut b = 3;
    assert_eq!(multiply.apply(&mut a, &mut b), -15);
    assert_eq!(a, -15);
    assert_eq!(b, 3);

    let mut c = -2;
    let mut d = -4;
    assert_eq!(multiply.apply(&mut c, &mut d), 8);
    assert_eq!(c, 8);
    assert_eq!(d, -4);
}

#[test]
fn test_bi_mutating_function_once_with_large_values() {
    let add = |x: &mut i64, y: &mut i64| {
        *x += *y;
        *x
    };

    let mut a = i64::MAX - 10;
    let mut b = 5;
    assert_eq!(add.apply(&mut a, &mut b), i64::MAX - 5);
    assert_eq!(a, i64::MAX - 5);
    assert_eq!(b, 5);
}

#[test]
fn test_bi_mutating_function_once_with_empty_strings() {
    let concat = |x: &mut String, y: &mut String| {
        x.push_str(y);
        x.len()
    };

    let mut s1 = String::new();
    let mut s2 = String::new();
    assert_eq!(concat.apply(&mut s1, &mut s2), 0);
    assert_eq!(s1, "");
    assert_eq!(s2, "");

    let mut s3 = "hello".to_string();
    let mut s4 = String::new();
    assert_eq!(concat.apply(&mut s3, &mut s4), 5);
    assert_eq!(s3, "hello");
    assert_eq!(s4, "");
}

#[test]
fn test_bi_mutating_function_once_with_unicode_strings() {
    let append = |x: &mut String, y: &mut String| {
        x.push('🌟');
        y.push('⭐');
        x.len() + y.len()
    };

    let mut s1 = "Hello".to_string();
    let mut s2 = "World".to_string();
    let result = append.apply(&mut s1, &mut s2);
    assert_eq!(s1, "Hello🌟");
    assert_eq!(s2, "World⭐");
    assert_eq!(result, 9 + 8); // "Hello🌟".len() + "World⭐".len()
}

#[test]
fn test_bi_mutating_function_once_identity_operations() {
    // Test functions that don't modify inputs
    let sum = |x: &mut i32, y: &mut i32| *x + *y;

    let mut a = 10;
    let mut b = 20;
    assert_eq!(sum.apply(&mut a, &mut b), 30);
    assert_eq!(a, 10); // unchanged
    assert_eq!(b, 20); // unchanged
}

#[test]
fn test_bi_mutating_function_once_chained_modifications() {
    let complex_op = |x: &mut i32, y: &mut i32| {
        *x = *x * 2 + *y;
        *y = *y * 3 - *x;
        *x + *y
    };

    let mut a = 3;
    let mut b = 5;
    let result = complex_op.apply(&mut a, &mut b);
    // a = 3*2 + 5 = 11
    // y = 5*3 - 11 = 15 - 11 = 4
    // result = 11 + 4 = 15
    assert_eq!(result, 15);
    assert_eq!(a, 11);
    assert_eq!(b, 4);
}

// ============================================================================
// Error and Panic Tests
// ============================================================================

#[test]
#[should_panic]
fn test_bi_mutating_function_once_panic_in_closure() {
    let panic_func = |x: &mut i32, y: &mut i32| {
        if *x < 0 {
            panic!("Negative value not allowed");
        }
        *x + *y
    };

    let mut a = -5;
    let mut b = 10;
    let _ = panic_func.apply(&mut a, &mut b);
}

#[test]
fn test_bi_mutating_function_once_with_option_modification() {
    let modify_option = |x: &mut Option<i32>, y: &mut Option<i32>| {
        if let (Some(val1), Some(val2)) = (*x, *y) {
            *x = Some(val1 + val2);
            *y = Some(val1 * val2);
            val1 + val2
        } else {
            0
        }
    };

    let mut a = Some(10);
    let mut b = Some(5);
    let result = modify_option.apply(&mut a, &mut b);
    assert_eq!(result, 15);
    assert_eq!(a, Some(15));
    assert_eq!(b, Some(50));

    let mut c = None;
    let mut d = Some(5);
    let result2 = modify_option.apply(&mut c, &mut d);
    assert_eq!(result2, 0);
    assert_eq!(c, None);
    assert_eq!(d, Some(5));
}

// ============================================================================
// One-Time Use Semantics Tests
// ============================================================================

#[test]
fn test_box_bi_mutating_function_once_consumption() {
    // Test that BoxBiMutatingFunctionOnce is consumed after use
    let create_func = || {
        BoxBiMutatingFunctionOnce::new(|x: &mut i32, y: &mut i32| {
            *x += 1;
            *y += 1;
            *x + *y
        })
    };

    let func = create_func();
    let mut a = 10;
    let mut b = 20;

    // Use the function once
    let result = func.apply(&mut a, &mut b);
    assert_eq!(result, 32); // (11) + (21) = 32
    assert_eq!(a, 11);
    assert_eq!(b, 21);

    // Create another function and use it
    let func2 = create_func();
    let mut c = 30;
    let mut d = 40;
    let result2 = func2.apply(&mut c, &mut d);
    assert_eq!(result2, 72); // (31) + (41) = 72
    assert_eq!(c, 31);
    assert_eq!(d, 41);
}

#[test]
fn test_bi_mutating_function_once_with_moving_data() {
    // Test with data that gets moved into the function
    let data = vec![1, 2, 3];
    let func = |x: &mut Vec<i32>, y: &mut Vec<i32>| {
        x.extend_from_slice(&data);
        y.push(42);
        x.len() + y.len()
    };

    let mut v1 = vec![10];
    let mut v2 = vec![20];
    let result = func.apply(&mut v1, &mut v2);

    assert_eq!(result, 6); // [10,1,2,3].len() + [20,42].len() = 4 + 2 = 6
    assert_eq!(v1, vec![10, 1, 2, 3]);
    assert_eq!(v2, vec![20, 42]);
}

// ============================================================================
// Complex Composition Scenarios
// ============================================================================

#[test]
fn test_complex_conditional_chains() {
    // Create a complex conditional chain
    let add = |x: &mut i32, y: &mut i32| {
        *x += *y;
        *x
    };

    let subtract = |x: &mut i32, y: &mut i32| {
        *x -= *y;
        *x
    };

    let multiply = |x: &mut i32, y: &mut i32| {
        *x *= *y;
        *x
    };

    // Complex conditional: if x > y then add, else if x < 0 then multiply, else subtract
    let complex = add
        .when(|x: &i32, y: &i32| *x > *y)
        .or_else(multiply.when(|x: &i32, _y: &i32| *x < 0).or_else(subtract));

    // Test case 1: x > y (add)
    let mut a = 10;
    let mut b = 5;
    assert_eq!(complex.apply(&mut a, &mut b), 15); // 10 + 5 = 15

    // Test case 2: x < 0 (multiply) - create new since BiMutatingFunctionOnce consumes self
    let complex2 = add
        .when(|x: &i32, y: &i32| *x > *y)
        .or_else(multiply.when(|x: &i32, _y: &i32| *x < 0).or_else(subtract));
    let mut c = -3;
    let mut d = 4;
    assert_eq!(complex2.apply(&mut c, &mut d), -12); // -3 * 4 = -12

    // Test case 3: neither condition (subtract) - create new since BiMutatingFunctionOnce consumes self
    let complex3 = add
        .when(|x: &i32, y: &i32| *x > *y)
        .or_else(multiply.when(|x: &i32, _y: &i32| *x < 0).or_else(subtract));
    let mut e = 3;
    let mut f = 5;
    assert_eq!(complex3.apply(&mut e, &mut f), -2); // 3 - 5 = -2
}

#[test]
fn test_nested_composition() {
    let base = |x: &mut i32, y: &mut i32| {
        *x += 1;
        *y += 1;
        *x + *y
    };

    // Create nested composition
    let inner_composed = base.and_then(|sum: &i32| *sum * 2);
    let outer_composed = inner_composed.and_then(|doubled: &mut i32| {
        let result = format!("Result: {}", *doubled);
        *doubled = 0;
        result
    });

    let mut a = 10;
    let mut b = 20;
    // base: a=11, b=21, sum=32
    // inner: 32*2=64
    // outer: "Result: 64"
    let result = outer_composed.apply(&mut a, &mut b);
    assert_eq!(result, "Result: 64");
    assert_eq!(a, 11);
    assert_eq!(b, 21);
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_mixed_function_types() {
    // Mix BiMutatingFunctionOnce with regular Function
    let mutating = |x: &mut i32, y: &mut i32| {
        *x += 10;
        *y += 20;
        *x + *y
    };

    let double = |x: &i32| *x * 2;

    // Chain mutating -> regular function
    let composed = mutating.and_then(double);

    let mut a = 5;
    let mut b = 3;
    // mutating: a=15, b=23, sum=38
    // double: 38 * 2 = 76
    assert_eq!(composed.apply(&mut a, &mut b), 76);
    assert_eq!(a, 15);
    assert_eq!(b, 23);
}

// ============================================================================
// Custom BiMutatingFunctionOnce Implementation Tests - Test Trait Default Methods
// ============================================================================

#[test]
fn test_custom_bi_mutating_function_once_default_methods() {
    // Test BiMutatingFunctionOnce trait default methods on custom implementation
    #[derive(Debug)]
    struct CustomBiMutatingFunctionOnce {
        multiplier: i32,
    }

    impl BiMutatingFunctionOnce<i32, i32, i32> for CustomBiMutatingFunctionOnce {
        fn apply(self, first: &mut i32, second: &mut i32) -> i32 {
            *first *= self.multiplier;
            *second += self.multiplier;
            *first + *second
        }
    }

    let custom_func = CustomBiMutatingFunctionOnce { multiplier: 3 };

    let mut a = 2;
    let mut b = 4;

    // Test default into_box method
    let boxed = custom_func.into_box();
    let result = boxed.apply(&mut a, &mut b);
    assert_eq!(result, 13); // (2*3) + (4+3) = 6 + 7 = 13
    assert_eq!(a, 6);
    assert_eq!(b, 7);
}

#[test]
fn test_cloneable_bi_mutating_function_once_default_methods() {
    // Test BiMutatingFunctionOnce trait default methods on cloneable implementation
    #[derive(Clone, Debug)]
    struct CloneableBiMutatingFunctionOnce {
        multiplier: i32,
    }

    impl BiMutatingFunctionOnce<i32, i32, i32> for CloneableBiMutatingFunctionOnce {
        fn apply(self, first: &mut i32, second: &mut i32) -> i32 {
            *first *= self.multiplier;
            *second += self.multiplier;
            *first + *second
        }
    }

    let custom_func = CloneableBiMutatingFunctionOnce { multiplier: 2 };

    let mut a = 3;
    let mut b = 5;

    // Test default to_box method (requires Clone)
    let boxed = custom_func.to_box();
    let result = boxed.apply(&mut a, &mut b);
    assert_eq!(result, 13); // (3*2) + (5+2) = 6 + 7 = 13
    assert_eq!(a, 6);
    assert_eq!(b, 7);

    // Test default to_fn method (requires Clone)
    let func = custom_func.to_fn();
    let mut c = 1;
    let mut d = 2;
    let result2 = func(&mut c, &mut d);
    assert_eq!(result2, 6); // (1*2) + (2+2) = 2 + 4 = 6
    assert_eq!(c, 2);
    assert_eq!(d, 4);
}
