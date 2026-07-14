// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
// qubit-style: allow explicit-imports
use qubit_function::{
    ArcBiTransformer,
    BiTransformer,
    BoxBiTransformer,
    RcBiTransformer,
};
use std::thread;

// ============================================================================
// BoxBiTransformer Tests - Immutable, single ownership
// ============================================================================

#[cfg(test)]
mod box_bi_transformer_tests {
    use super::{
        BiTransformer,
        BoxBiTransformer,
    };

    #[test]
    fn test_new_and_transform() {
        let add = BoxBiTransformer::new(|x: i32, y: i32| x + y);
        assert_eq!(add.apply(20, 22), 42);
    }

    #[test]
    fn test_multiple_calls() {
        let add = BoxBiTransformer::new(|x: i32, y: i32| x + y);
        assert_eq!(add.apply(20, 22), 42);
        assert_eq!(add.apply(10, 10), 20);
        assert_eq!(add.apply(5, 3), 8);
    }

    #[test]
    fn test_multiply() {
        let multiply = BoxBiTransformer::new(|x: i32, y: i32| x * y);
        assert_eq!(multiply.apply(6, 7), 42);
    }

    #[test]
    fn test_constant() {
        let constant = BoxBiTransformer::constant("hello");
        assert_eq!(constant.apply(123, 456), "hello");
        assert_eq!(constant.apply(789, 101), "hello");
    }

    #[test]
    fn test_with_string() {
        let concat = BoxBiTransformer::new(|s1: String, s2: String| {
            format!("{}{}", s1, s2)
        });
        assert_eq!(
            concat.apply("hello".to_string(), "world".to_string()),
            "helloworld"
        );
    }

    #[test]
    fn test_captured_variable() {
        let multiplier = 3;
        let weighted_sum = BoxBiTransformer::new(move |x: i32, y: i32| {
            x * multiplier + y * multiplier
        });
        assert_eq!(weighted_sum.apply(2, 3), 15); // (2 * 3) + (3 * 3) = 15
    }

    #[test]
    fn test_different_types() {
        let format = BoxBiTransformer::new(|name: String, age: i32| {
            format!("{} is {}", name, age)
        });
        assert_eq!(format.apply("Alice".to_string(), 30), "Alice is 30");
    }

    #[test]
    fn test_with_option() {
        let safe_divide =
            BoxBiTransformer::new(
                |x: i32, y: i32| if y == 0 { None } else { Some(x / y) },
            );
        assert_eq!(safe_divide.apply(42, 2), Some(21));
        assert_eq!(safe_divide.apply(42, 0), None);
    }

    #[test]
    fn test_display_with_name() {
        let transformer =
            BoxBiTransformer::new_with_name("add", |x: i32, y: i32| x + y);
        let display_str = format!("{}", transformer);
        assert_eq!(display_str, "BoxBiTransformer(add)");
    }

    #[test]
    fn test_display_without_name() {
        let transformer = BoxBiTransformer::new(|x: i32, y: i32| x + y);
        let display_str = format!("{}", transformer);
        assert_eq!(display_str, "BoxBiTransformer");
    }
}

// ============================================================================
// ArcBiTransformer Tests - Immutable, thread-safe
// ============================================================================

#[cfg(test)]
mod arc_bi_transformer_tests {
    use super::{
        ArcBiTransformer,
        BiTransformer,
        thread,
    };

    #[test]
    fn test_new_and_transform() {
        let add = ArcBiTransformer::new(|x: i32, y: i32| x + y);
        assert_eq!(add.apply(20, 22), 42);
    }

    #[test]
    fn test_clone() {
        let add = ArcBiTransformer::new(|x: i32, y: i32| x + y);
        let cloned = add.clone();

        assert_eq!(add.apply(20, 22), 42);
        assert_eq!(cloned.apply(20, 22), 42);
    }

    #[test]
    fn test_thread_safe() {
        let add = ArcBiTransformer::new(|x: i32, y: i32| x + y);
        let cloned = add.clone();

        let handle = thread::spawn(move || cloned.apply(20, 22));

        assert_eq!(handle.join().expect("thread should not panic"), 42);
        assert_eq!(add.apply(20, 22), 42);
    }

    #[test]
    fn test_constant() {
        let constant = ArcBiTransformer::constant("hello");
        assert_eq!(constant.apply(123, 456), "hello");
    }

    #[test]
    fn test_multiple_threads() {
        let multiply = ArcBiTransformer::new(|x: i32, y: i32| x * y);

        let handles: Vec<_> = (0..4)
            .map(|i| {
                let m = multiply.clone();
                thread::spawn(move || m.apply(i, i + 1))
            })
            .collect();

        let results: Vec<_> = handles
            .into_iter()
            .map(|h| h.join().expect("thread should not panic"))
            .collect();

        assert_eq!(results, vec![0, 2, 6, 12]); // 0*1, 1*2, 2*3, 3*4
    }

    #[test]
    fn test_with_different_types() {
        let format = ArcBiTransformer::new(|name: String, age: i32| {
            format!("{} is {}", name, age)
        });
        assert_eq!(format.apply("Alice".to_string(), 30), "Alice is 30");
    }

    #[test]
    fn test_display_with_name() {
        let transformer =
            ArcBiTransformer::new_with_name("multiply", |x: i32, y: i32| x * y);
        let display_str = format!("{}", transformer);
        assert_eq!(display_str, "ArcBiTransformer(multiply)");
    }

    #[test]
    fn test_display_without_name() {
        let transformer = ArcBiTransformer::new(|x: i32, y: i32| x * y);
        let display_str = format!("{}", transformer);
        assert_eq!(display_str, "ArcBiTransformer");
    }
}

// ============================================================================
// RcBiTransformer Tests - Immutable, single-threaded
// ============================================================================

#[cfg(test)]
mod rc_bi_transformer_tests {
    use super::{
        BiTransformer,
        RcBiTransformer,
    };

    #[test]
    fn test_new_and_transform() {
        let add = RcBiTransformer::new(|x: i32, y: i32| x + y);
        assert_eq!(add.apply(20, 22), 42);
    }

    #[test]
    fn test_clone() {
        let add = RcBiTransformer::new(|x: i32, y: i32| x + y);
        let cloned = add.clone();

        assert_eq!(add.apply(20, 22), 42);
        assert_eq!(cloned.apply(20, 22), 42);
    }

    #[test]
    fn test_constant() {
        let constant = RcBiTransformer::constant("hello");
        assert_eq!(constant.apply(123, 456), "hello");
    }

    #[test]
    fn test_shared_usage() {
        let concat = RcBiTransformer::new(|s1: String, s2: String| {
            format!("{}{}", s1, s2)
        });

        let func1 = concat.clone();
        let func2 = concat.clone();

        assert_eq!(
            concat.apply("hello".to_string(), "world".to_string()),
            "helloworld"
        );
        assert_eq!(func1.apply("foo".to_string(), "bar".to_string()), "foobar");
        assert_eq!(
            func2.apply("rust".to_string(), "lang".to_string()),
            "rustlang"
        );
    }

    #[test]
    fn test_with_different_types() {
        let format = RcBiTransformer::new(|name: String, age: i32| {
            format!("{} is {}", name, age)
        });
        assert_eq!(format.apply("Alice".to_string(), 30), "Alice is 30");
    }

    #[test]
    fn test_display_with_name() {
        let transformer = RcBiTransformer::new_with_name(
            "concat",
            |s1: String, s2: String| format!("{}{}", s1, s2),
        );
        let display_str = format!("{}", transformer);
        assert_eq!(display_str, "RcBiTransformer(concat)");
    }

    #[test]
    fn test_display_without_name() {
        let transformer = RcBiTransformer::new(|s1: String, s2: String| {
            format!("{}{}", s1, s2)
        });
        let display_str = format!("{}", transformer);
        assert_eq!(display_str, "RcBiTransformer");
    }
}

// ============================================================================
// Conditional BiTransformer Tests
// ============================================================================

#[cfg(test)]
mod box_conditional_tests {
    use super::{
        BiTransformer,
        BoxBiTransformer,
    };
    use qubit_function::BoxBiPredicate;

    #[test]
    fn test_when_or_else() {
        let add = BoxBiTransformer::new(|x: i32, y: i32| x + y);
        let both_positive =
            BoxBiPredicate::new(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let multiply = BoxBiTransformer::new(|x: i32, y: i32| x * y);
        let result = add.when(both_positive).or_else(multiply);

        assert_eq!(result.apply(5, 3), 8); // both positive, add
        assert_eq!(result.apply(-5, 3), -15); // not both positive, multiply
    }

    #[test]
    fn test_when_or_else_with_closure() {
        let add = BoxBiTransformer::new(|x: i32, y: i32| x + y);
        let result = add
            .when(|x: &i32, y: &i32| *x > 0 && *y > 0)
            .or_else(|x: i32, y: i32| x * y);

        assert_eq!(result.apply(5, 3), 8);
        assert_eq!(result.apply(-5, 3), -15);
        assert_eq!(result.apply(0, 5), 0);
    }
}

#[cfg(test)]
mod arc_conditional_tests {
    use super::{
        ArcBiTransformer,
        BiTransformer,
    };
    use qubit_function::ArcBiPredicate;

    #[test]
    fn test_when_or_else() {
        let add = ArcBiTransformer::new(|x: i32, y: i32| x + y);
        let both_positive =
            ArcBiPredicate::new(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let multiply = ArcBiTransformer::new(|x: i32, y: i32| x * y);
        let result = add.when(both_positive).or_else(multiply);

        assert_eq!(result.apply(5, 3), 8);
        assert_eq!(result.apply(-5, 3), -15);
    }

    #[test]
    fn test_when_or_else_with_closure() {
        let add = ArcBiTransformer::new(|x: i32, y: i32| x + y);
        let result = add
            .when(|x: &i32, y: &i32| *x > 0 && *y > 0)
            .or_else(|x: i32, y: i32| x * y);

        assert_eq!(result.apply(5, 3), 8);
        assert_eq!(result.apply(-5, 3), -15);
        assert_eq!(result.apply(0, 5), 0);
    }

    #[test]
    fn test_conditional_clone() {
        let add = ArcBiTransformer::new(|x: i32, y: i32| x + y);
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let cloned = conditional.clone();

        let result1 = conditional.or_else(|x: i32, y: i32| x * y);
        let result2 = cloned.or_else(|x: i32, y: i32| x * y);

        assert_eq!(result1.apply(5, 3), 8);
        assert_eq!(result2.apply(5, 3), 8);
        assert_eq!(result1.apply(-5, 3), -15);
        assert_eq!(result2.apply(-5, 3), -15);
    }
}

#[cfg(test)]
mod rc_conditional_tests {
    use super::{
        BiTransformer,
        RcBiTransformer,
    };
    use qubit_function::RcBiPredicate;

    #[test]
    fn test_when_or_else() {
        let add = RcBiTransformer::new(|x: i32, y: i32| x + y);
        let both_positive =
            RcBiPredicate::new(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let multiply = RcBiTransformer::new(|x: i32, y: i32| x * y);
        let result = add.when(both_positive).or_else(multiply);

        assert_eq!(result.apply(5, 3), 8);
        assert_eq!(result.apply(-5, 3), -15);
    }

    #[test]
    fn test_when_or_else_with_closure() {
        let add = RcBiTransformer::new(|x: i32, y: i32| x + y);
        let result = add
            .when(|x: &i32, y: &i32| *x > 0 && *y > 0)
            .or_else(|x: i32, y: i32| x * y);

        assert_eq!(result.apply(5, 3), 8);
        assert_eq!(result.apply(-5, 3), -15);
        assert_eq!(result.apply(0, 5), 0);
    }

    #[test]
    fn test_conditional_clone() {
        let add = RcBiTransformer::new(|x: i32, y: i32| x + y);
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let cloned = conditional.clone();

        let result1 = conditional.or_else(|x: i32, y: i32| x * y);
        let result2 = cloned.or_else(|x: i32, y: i32| x * y);

        assert_eq!(result1.apply(5, 3), 8);
        assert_eq!(result2.apply(5, 3), 8);
        assert_eq!(result1.apply(-5, 3), -15);
        assert_eq!(result2.apply(-5, 3), -15);
    }
}

// ============================================================================
// Conversion Tests
// ============================================================================

// ============================================================================
// Trait Usage Tests
// ============================================================================

#[cfg(test)]
mod trait_usage_tests {
    use super::{
        BiTransformer,
        BoxBiTransformer,
    };

    #[test]
    fn test_bi_transformer_trait() {
        fn apply_bi_transformer<F: BiTransformer<i32, i32, i32>>(
            f: &F,
            x: i32,
            y: i32,
        ) -> i32 {
            f.apply(x, y)
        }

        let add = BoxBiTransformer::new(|x: i32, y: i32| x + y);
        assert_eq!(apply_bi_transformer(&add, 20, 22), 42);
    }

    #[test]
    fn test_closure_as_bi_transformer() {
        fn apply_bi_transformer<F: BiTransformer<i32, i32, i32>>(
            f: &F,
            x: i32,
            y: i32,
        ) -> i32 {
            f.apply(x, y)
        }

        let add = |x: i32, y: i32| x + y;
        assert_eq!(apply_bi_transformer(&add, 20, 22), 42);
    }

    #[test]
    fn test_with_different_types() {
        fn apply_bi_transformer<T, U, R, F: BiTransformer<T, U, R>>(
            f: &F,
            x: T,
            y: U,
        ) -> R {
            f.apply(x, y)
        }

        let format = BoxBiTransformer::new(|name: String, age: i32| {
            format!("{} is {}", name, age)
        });
        assert_eq!(
            apply_bi_transformer(&format, "Alice".to_string(), 30),
            "Alice is 30"
        );
    }
}

// ============================================================================
// Edge Cases Tests
// ============================================================================

#[cfg(test)]
mod edge_cases_tests {
    use super::{
        ArcBiTransformer,
        BiTransformer,
        BoxBiTransformer,
    };

    #[test]
    fn test_constant_with_different_types() {
        let constant = BoxBiTransformer::constant("hello");
        assert_eq!(constant.apply(123, 456), "hello");
        assert_eq!(constant.apply(789, 101), "hello");
    }

    #[test]
    fn test_with_option() {
        let safe_divide =
            BoxBiTransformer::new(
                |x: i32, y: i32| if y == 0 { None } else { Some(x / y) },
            );
        assert_eq!(safe_divide.apply(42, 2), Some(21));
        assert_eq!(safe_divide.apply(42, 0), None);
    }

    #[test]
    fn test_with_result() {
        let safe_divide =
            BoxBiTransformer::new(|x: i32, y: i32| -> Result<i32, String> {
                if y == 0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(x / y)
                }
            });
        assert_eq!(safe_divide.apply(42, 2), Ok(21));
        assert!(safe_divide.apply(42, 0).is_err());
    }

    #[test]
    fn test_with_vec() {
        let combine = BoxBiTransformer::new(|v1: Vec<i32>, v2: Vec<i32>| {
            let mut result = v1;
            result.extend(v2);
            result
        });
        assert_eq!(
            combine.apply(vec![1, 2, 3], vec![4, 5, 6]),
            vec![1, 2, 3, 4, 5, 6]
        );
    }

    #[test]
    fn test_arc_with_large_data() {
        let sum_vecs = ArcBiTransformer::new(|v1: Vec<i32>, v2: Vec<i32>| {
            v1.iter().sum::<i32>() + v2.iter().sum::<i32>()
        });
        let data1 = (1..=50).collect::<Vec<_>>();
        let data2 = (51..=100).collect::<Vec<_>>();
        assert_eq!(sum_vecs.apply(data1, data2), 5050);
    }

    #[test]
    fn test_with_tuples() {
        let swap = BoxBiTransformer::new(|x: i32, y: i32| (y, x));
        assert_eq!(swap.apply(1, 2), (2, 1));
    }

    #[test]
    fn test_string_operations() {
        let join = BoxBiTransformer::new(|s1: String, s2: String| {
            format!("{} {}", s1, s2)
        });
        assert_eq!(
            join.apply("Hello".to_string(), "World".to_string()),
            "Hello World"
        );
    }
}

// ============================================================================
// Type Conversion Tests - Testing into_box, into_rc, into_arc methods
// ============================================================================

// ============================================================================
// Closure BiTransformer Tests - Testing blanket implementation for closures
// ============================================================================

#[cfg(test)]
mod closure_bi_transformer_tests {
    use super::BiTransformer;

    #[test]
    fn test_closure_transform() {
        let add = |x: i32, y: i32| x + y;
        assert_eq!(add.apply(10, 20), 30);
    }

    #[test]
    fn test_closure_transform_with_string() {
        let concat = |s1: String, s2: String| format!("{}{}", s1, s2);
        assert_eq!(
            concat.apply("Hello".to_string(), "World".to_string()),
            "HelloWorld"
        );
    }

    #[test]
    fn test_function_pointer_transform() {
        fn multiply(x: i32, y: i32) -> i32 {
            x * y
        }
        assert_eq!(multiply.apply(6, 7), 42);
    }

    #[test]
    fn test_closure_with_captured_variable() {
        let multiplier = 3;
        let multiply_by = move |x: i32, y: i32| (x + y) * multiplier;
        assert_eq!(multiply_by.apply(5, 5), 30);
    }
}

// ============================================================================
// Custom BiTransformer Tests - Testing default into_xxx() implementations
// ============================================================================

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

    /// Custom Send + Sync BiTransformer implementation
    struct ThreadSafeBiTransformer {
        multiplier: i32,
    }

    impl ThreadSafeBiTransformer {
        fn new(multiplier: i32) -> Self {
            Self { multiplier }
        }
    }

    impl BiTransformer<i32, i32, i32> for ThreadSafeBiTransformer {
        fn apply(&self, first: i32, second: i32) -> i32 {
            (first + second) * self.multiplier
        }
    }

    // Manual implementation of Send and Sync
    unsafe impl Send for ThreadSafeBiTransformer {}
    unsafe impl Sync for ThreadSafeBiTransformer {}

    /// Test custom BiTransformer with different types combination
    struct StringCombiner {
        separator: String,
    }

    impl StringCombiner {
        fn new(separator: &str) -> Self {
            Self {
                separator: separator.to_string(),
            }
        }
    }

    impl BiTransformer<String, String, String> for StringCombiner {
        fn apply(&self, first: String, second: String) -> String {
            format!("{}{}{}", first, self.separator, second)
        }
    }

    /// Test custom BiTransformer's default to_xxx() implementations
    /// These are default implementations provided by the BiTransformer trait,
    /// requiring the type to implement Clone
    #[derive(Clone)]
    struct CloneableCustomBiTransformer {
        multiplier: i32,
    }

    impl CloneableCustomBiTransformer {
        fn new(multiplier: i32) -> Self {
            Self { multiplier }
        }
    }

    impl BiTransformer<i32, i32, i32> for CloneableCustomBiTransformer {
        fn apply(&self, first: i32, second: i32) -> i32 {
            (first + second) * self.multiplier
        }
    }

    /// Test custom Send + Sync BiTransformer's default to_arc() implementation
    #[derive(Clone)]
    struct ThreadSafeCloneableBiTransformer {
        multiplier: i32,
    }

    impl ThreadSafeCloneableBiTransformer {
        fn new(multiplier: i32) -> Self {
            Self { multiplier }
        }
    }

    impl BiTransformer<i32, i32, i32> for ThreadSafeCloneableBiTransformer {
        fn apply(&self, first: i32, second: i32) -> i32 {
            (first + second) * self.multiplier
        }
    }

    // Manual implementation of Send and Sync
    unsafe impl Send for ThreadSafeCloneableBiTransformer {}
    unsafe impl Sync for ThreadSafeCloneableBiTransformer {}

    /// Test custom string type's default to_xxx() methods
    #[derive(Clone)]
    struct CloneableStringCombiner {
        separator: String,
    }

    impl CloneableStringCombiner {
        fn new(separator: &str) -> Self {
            Self {
                separator: separator.to_string(),
            }
        }
    }

    impl BiTransformer<String, String, String> for CloneableStringCombiner {
        fn apply(&self, first: String, second: String) -> String {
            format!("{}{}{}", first, self.separator, second)
        }
    }
}

// ============================================================================
// BiTransformer Default Methods - to_xxx() Non-consuming Conversions
// ============================================================================

// ============================================================================
// Closure BiTransformer to_xxx() Methods Tests
// ============================================================================

// ============================================================================
// Complete to_xxx() Test Coverage for All Types
// ============================================================================

// ============================================================================
// Consuming into_xxx() and Non-consuming to_xxx() Comparison Tests
// ============================================================================

// ============================================================================
// BiTransformer Default Trait Methods - into_xxx() with Various Inputs
// ============================================================================

// ============================================================================
// Type Conversion Chain Tests
// ============================================================================

// ============================================================================
// String and Complex Types Conversion Tests
// ============================================================================

// ============================================================================
// Send+Sync Verification Tests for Arc Conversions
// ============================================================================

// ============================================================================
// BoxBiTransformer BiTransformerOnce Tests
// ============================================================================

#[cfg(test)]
mod box_bi_transformer_once_tests {
    use super::{
        BiTransformer,
        BoxBiTransformer,
    };

    #[test]
    fn test_apply() {
        let add = BoxBiTransformer::new(|x: i32, y: i32| x + y);
        assert_eq!(add.apply(20, 22), 42);
    }

    #[test]
    fn test_multiply_once() {
        let multiply = BoxBiTransformer::new(|x: i32, y: i32| x * y);
        assert_eq!(multiply.apply(6, 7), 42);
    }

    #[test]
    fn test_string_concatenation_once() {
        let concat = BoxBiTransformer::new(|x: String, y: String| {
            format!("{} {}", x, y)
        });
        let result = concat.apply("Hello".to_string(), "World".to_string());
        assert_eq!(result, "Hello World");
    }
}

// ============================================================================
// RcBiTransformer BiTransformerOnce Tests
// ============================================================================

#[cfg(test)]
mod rc_bi_transformer_once_tests {
    use super::{
        BiTransformer,
        RcBiTransformer,
    };

    #[test]
    fn test_apply() {
        let add = RcBiTransformer::new(|x: i32, y: i32| x + y);
        assert_eq!(add.apply(20, 22), 42);
    }

    #[test]
    fn test_multiply_once() {
        let multiply = RcBiTransformer::new(|x: i32, y: i32| x * y);
        assert_eq!(multiply.apply(6, 7), 42);
    }

    #[test]
    fn test_string_concatenation_once() {
        let concat =
            RcBiTransformer::new(|x: String, y: String| format!("{} {}", x, y));
        let result = concat.apply("Hello".to_string(), "World".to_string());
        assert_eq!(result, "Hello World");
    }
}

// ============================================================================
// ArcBiTransformer BiTransformerOnce Tests
// ============================================================================

#[cfg(test)]
mod arc_bi_transformer_once_tests {
    use super::{
        ArcBiTransformer,
        BiTransformer,
    };
    use std::thread;

    #[test]
    fn test_apply() {
        let add = ArcBiTransformer::new(|x: i32, y: i32| x + y);
        assert_eq!(add.apply(20, 22), 42);
    }

    #[test]
    fn test_multiply_once() {
        let multiply = ArcBiTransformer::new(|x: i32, y: i32| x * y);
        assert_eq!(multiply.apply(6, 7), 42);
    }

    #[test]
    fn test_string_concatenation_once() {
        let concat = ArcBiTransformer::new(|x: String, y: String| {
            format!("{} {}", x, y)
        });
        let result = concat.apply("Hello".to_string(), "World".to_string());
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_thread_safety_apply() {
        let add = ArcBiTransformer::new(|x: i32, y: i32| x + y);
        let handle = thread::spawn(move || add.apply(10, 20));
        assert_eq!(handle.join().expect("thread should not panic"), 30);
    }
}

// ============================================================================
// Conditional Transformer Display/Debug Tests
// ============================================================================

#[cfg(test)]
mod conditional_transformer_display_debug_tests {
    use super::{
        ArcBiTransformer,
        BoxBiTransformer,
        RcBiTransformer,
    };

    #[test]
    fn test_box_conditional_bi_transformer_display() {
        let add = BoxBiTransformer::new(|x: i32, y: i32| x + y);
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let display_str = format!("{}", conditional);
        assert!(display_str.contains("BoxConditionalBiTransformer"));
    }

    #[test]
    fn test_box_conditional_bi_transformer_display_no_name() {
        let add = BoxBiTransformer::new(|x: i32, y: i32| x + y);
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let display_str = format!("{}", conditional);
        assert_eq!(
            display_str,
            "BoxConditionalBiTransformer(BoxBiTransformer, BoxBiPredicate(unnamed))"
        );
    }

    #[test]
    fn test_box_conditional_bi_transformer_debug() {
        let add = BoxBiTransformer::new(|x: i32, y: i32| x + y);
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("BoxConditionalBiTransformer"));
    }

    #[test]
    fn test_rc_conditional_bi_transformer_display() {
        let add = RcBiTransformer::new(|x: i32, y: i32| x + y);
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let display_str = format!("{}", conditional);
        assert!(display_str.contains("RcConditionalBiTransformer"));
    }

    #[test]
    fn test_rc_conditional_bi_transformer_display_no_name() {
        let add = RcBiTransformer::new(|x: i32, y: i32| x + y);
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let display_str = format!("{}", conditional);
        assert_eq!(
            display_str,
            "RcConditionalBiTransformer(RcBiTransformer, RcBiPredicate(unnamed))"
        );
    }

    #[test]
    fn test_rc_conditional_bi_transformer_debug() {
        let add = RcBiTransformer::new(|x: i32, y: i32| x + y);
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("RcConditionalBiTransformer"));
    }

    #[test]
    fn test_arc_conditional_bi_transformer_display() {
        let add = ArcBiTransformer::new(|x: i32, y: i32| x + y);
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let display_str = format!("{}", conditional);
        assert!(display_str.contains("ArcConditionalBiTransformer"));
    }

    #[test]
    fn test_arc_conditional_bi_transformer_display_no_name() {
        let add = ArcBiTransformer::new(|x: i32, y: i32| x + y);
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let display_str = format!("{}", conditional);
        assert_eq!(
            display_str,
            "ArcConditionalBiTransformer(ArcBiTransformer, ArcBiPredicate(unnamed))"
        );
    }

    #[test]
    fn test_arc_conditional_bi_transformer_debug() {
        let add = ArcBiTransformer::new(|x: i32, y: i32| x + y);
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("ArcConditionalBiTransformer"));
    }
}
