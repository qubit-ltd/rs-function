// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

// qubit-style: allow explicit-imports

use qubit_function::{
    ArcPredicate,
    ArcStatefulTransformer,
    BoxPredicate,
    BoxStatefulTransformer,
    FnStatefulTransformerOps,
    Predicate,
    RcPredicate,
    RcStatefulTransformer,
    StatefulTransformer,
};


// ============================================================================
// BoxStatefulTransformer Tests
// ============================================================================

#[test]
fn test_box_mapper_new() {
    let mut counter = 0;
    let mut mapper = BoxStatefulTransformer::new(move |x: i32| {
        counter += 1;
        x + counter
    });

    assert_eq!(mapper.apply(10), 11);
    assert_eq!(mapper.apply(10), 12);
    assert_eq!(mapper.apply(10), 13);
}

#[test]
fn test_box_mapper_identity() {
    let mut identity = BoxStatefulTransformer::<i32, i32>::identity();
    assert_eq!(identity.apply(42), 42);
    assert_eq!(identity.apply(100), 100);
}

#[test]
fn test_box_mapper_constant() {
    let mut constant = BoxStatefulTransformer::constant("hello");
    assert_eq!(constant.apply(1), "hello");
    assert_eq!(constant.apply(2), "hello");
    assert_eq!(constant.apply(3), "hello");
}

#[test]
fn test_box_mapper_and_then() {
    let mut counter1 = 0;
    let mapper1 = BoxStatefulTransformer::new(move |x: i32| {
        counter1 += 1;
        x + counter1
    });

    let mut counter2 = 0;
    let mapper2 = BoxStatefulTransformer::new(move |x: i32| {
        counter2 += 1;
        x * counter2
    });

    let mut composed = mapper1.and_then(mapper2);
    assert_eq!(composed.apply(10), 11); // (10 + 1) * 1
    assert_eq!(composed.apply(10), 24); // (10 + 2) * 2
    assert_eq!(composed.apply(10), 39); // (10 + 3) * 3
}

#[test]
fn test_box_mapper_and_then_with_closure() {
    let mut counter = 0;
    let mapper = BoxStatefulTransformer::new(move |x: i32| {
        counter += 1;
        x * counter
    });

    let mut composed = mapper.and_then(|x: i32| x + 1);
    assert_eq!(composed.apply(10), 11); // 10 * 1 + 1
    assert_eq!(composed.apply(10), 21); // 10 * 2 + 1
    assert_eq!(composed.apply(10), 31); // 10 * 3 + 1
}

#[test]
fn test_box_mapper_when_or_else() {
    let mut high_count = 0;
    let mut low_count = 0;

    let mut mapper = BoxStatefulTransformer::new(move |x: i32| {
        high_count += 1;
        format!("High[{}]: {}", high_count, x * 2)
    })
    .when(|x: &i32| *x >= 10)
    .or_else(move |x| {
        low_count += 1;
        format!("Low[{}]: {}", low_count, x + 1)
    });

    assert_eq!(mapper.apply(15), "High[1]: 30");
    assert_eq!(mapper.apply(5), "Low[1]: 6");
    assert_eq!(mapper.apply(20), "High[2]: 40");
    assert_eq!(mapper.apply(3), "Low[2]: 4");
}




// ============================================================================
// ArcStatefulTransformer Tests
// ============================================================================

#[test]
fn test_arc_mapper_new() {
    let mut counter = 0;
    let mut mapper = ArcStatefulTransformer::new(move |x: i32| {
        counter += 1;
        x + counter
    });

    assert_eq!(mapper.apply(10), 11);
    assert_eq!(mapper.apply(10), 12);
    assert_eq!(mapper.apply(10), 13);
}

#[test]
fn test_arc_mapper_identity() {
    let mut identity = ArcStatefulTransformer::<i32, i32>::identity();
    assert_eq!(identity.apply(42), 42);
    assert_eq!(identity.apply(100), 100);
}

#[test]
fn test_arc_mapper_constant() {
    let mut constant = ArcStatefulTransformer::constant("hello");
    assert_eq!(constant.apply(1), "hello");
    assert_eq!(constant.apply(2), "hello");
    assert_eq!(constant.apply(3), "hello");
}

#[test]
fn test_arc_mapper_clone() {
    let mut counter = 0;
    let mapper = ArcStatefulTransformer::new(move |x: i32| {
        counter += 1;
        x + counter
    });

    let mut mapper1 = mapper.clone();
    let mut mapper2 = mapper.clone();

    assert_eq!(mapper1.apply(10), 11);
    assert_eq!(mapper2.apply(10), 12);
    assert_eq!(mapper1.apply(10), 13);
}

#[test]
fn test_arc_mapper_and_then() {
    let mut counter1 = 0;
    let mapper1 = ArcStatefulTransformer::new(move |x: i32| {
        counter1 += 1;
        x + counter1
    });

    let mut counter2 = 0;
    let mapper2 = ArcStatefulTransformer::new(move |x: i32| {
        counter2 += 1;
        x * counter2
    });

    let mut composed = mapper1.and_then(mapper2);
    assert_eq!(composed.apply(10), 11); // (10 + 1) * 1
    assert_eq!(composed.apply(10), 24); // (10 + 2) * 2
    assert_eq!(composed.apply(10), 39); // (10 + 3) * 3
}

#[test]
fn test_arc_mapper_and_then_with_closure() {
    let mut counter = 0;
    let mapper = ArcStatefulTransformer::new(move |x: i32| {
        counter += 1;
        x * counter
    });

    let mut composed = mapper.and_then(|x: i32| x + 1);
    assert_eq!(composed.apply(10), 11); // 10 * 1 + 1
    assert_eq!(composed.apply(10), 21); // 10 * 2 + 1
    assert_eq!(composed.apply(10), 31); // 10 * 3 + 1
}

#[test]
fn test_arc_mapper_when_or_else() {
    let mut high_count = 0;
    let mut low_count = 0;

    let mut mapper = ArcStatefulTransformer::new(move |x: i32| {
        high_count += 1;
        format!("High[{}]: {}", high_count, x * 2)
    })
    .when(|x: &i32| *x >= 10)
    .or_else(move |x| {
        low_count += 1;
        format!("Low[{}]: {}", low_count, x + 1)
    });

    assert_eq!(mapper.apply(15), "High[1]: 30");
    assert_eq!(mapper.apply(5), "Low[1]: 6");
    assert_eq!(mapper.apply(20), "High[2]: 40");
    assert_eq!(mapper.apply(3), "Low[2]: 4");
}












// ============================================================================
// RcStatefulTransformer Tests
// ============================================================================

#[test]
fn test_rc_mapper_new() {
    let mut counter = 0;
    let mut mapper = RcStatefulTransformer::new(move |x: i32| {
        counter += 1;
        x + counter
    });

    assert_eq!(mapper.apply(10), 11);
    assert_eq!(mapper.apply(10), 12);
    assert_eq!(mapper.apply(10), 13);
}

#[test]
fn test_rc_mapper_identity() {
    let mut identity = RcStatefulTransformer::<i32, i32>::identity();
    assert_eq!(identity.apply(42), 42);
    assert_eq!(identity.apply(100), 100);
}

#[test]
fn test_rc_mapper_constant() {
    let mut constant = RcStatefulTransformer::constant("hello");
    assert_eq!(constant.apply(1), "hello");
    assert_eq!(constant.apply(2), "hello");
    assert_eq!(constant.apply(3), "hello");
}

#[test]
fn test_rc_mapper_clone() {
    let mut counter = 0;
    let mapper = RcStatefulTransformer::new(move |x: i32| {
        counter += 1;
        x + counter
    });

    let mut mapper1 = mapper.clone();
    let mut mapper2 = mapper.clone();

    assert_eq!(mapper1.apply(10), 11);
    assert_eq!(mapper2.apply(10), 12);
    assert_eq!(mapper1.apply(10), 13);
}

#[test]
fn test_rc_mapper_and_then() {
    let mut counter1 = 0;
    let mapper1 = RcStatefulTransformer::new(move |x: i32| {
        counter1 += 1;
        x + counter1
    });

    let mut counter2 = 0;
    let mapper2 = RcStatefulTransformer::new(move |x: i32| {
        counter2 += 1;
        x * counter2
    });

    let mut composed = mapper1.and_then(mapper2);
    assert_eq!(composed.apply(10), 11); // (10 + 1) * 1
    assert_eq!(composed.apply(10), 24); // (10 + 2) * 2
    assert_eq!(composed.apply(10), 39); // (10 + 3) * 3
}

#[test]
fn test_rc_mapper_and_then_with_closure() {
    let mut counter = 0;
    let mapper = RcStatefulTransformer::new(move |x: i32| {
        counter += 1;
        x * counter
    });

    let mut composed = mapper.and_then(|x: i32| x + 1);
    assert_eq!(composed.apply(10), 11); // 10 * 1 + 1
    assert_eq!(composed.apply(10), 21); // 10 * 2 + 1
    assert_eq!(composed.apply(10), 31); // 10 * 3 + 1
}

#[test]
fn test_rc_mapper_when_or_else() {
    let mut high_count = 0;
    let mut low_count = 0;

    let mut mapper = RcStatefulTransformer::new(move |x: i32| {
        high_count += 1;
        format!("High[{}]: {}", high_count, x * 2)
    })
    .when(|x: &i32| *x >= 10)
    .or_else(move |x| {
        low_count += 1;
        format!("Low[{}]: {}", low_count, x + 1)
    });

    assert_eq!(mapper.apply(15), "High[1]: 30");
    assert_eq!(mapper.apply(5), "Low[1]: 6");
    assert_eq!(mapper.apply(20), "High[2]: 40");
    assert_eq!(mapper.apply(3), "Low[2]: 4");
}





// ============================================================================
// Closure StatefulTransformer Tests
// ============================================================================

#[test]
fn test_closure_as_mapper() {
    let mut counter = 0;
    let mut mapper = |x: i32| {
        counter += 1;
        x + counter
    };

    assert_eq!(mapper.apply(10), 11);
}




// ============================================================================
// FnStatefulTransformerOps Tests
// ============================================================================

#[test]
fn test_fn_mapper_ops_and_then() {
    let mut counter1 = 0;
    let mapper1 = move |x: i32| {
        counter1 += 1;
        x + counter1
    };

    let mut counter2 = 0;
    let mapper2 = move |x: i32| {
        counter2 += 1;
        x * counter2
    };

    let mut composed = FnStatefulTransformerOps::and_then(mapper1, mapper2);
    assert_eq!(composed.apply(10), 11); // (10 + 1) * 1
    assert_eq!(composed.apply(10), 24); // (10 + 2) * 2
}

#[test]
fn test_fn_mapper_ops_when() {
    let mut mapper =
        FnStatefulTransformerOps::when(|x: i32| x * 2, |x: &i32| *x > 0)
            .or_else(|x: i32| -x);

    assert_eq!(mapper.apply(5), 10);
    assert_eq!(mapper.apply(-5), 5);
}

// ============================================================================
// Conditional StatefulTransformer Tests
// ============================================================================

#[test]
fn test_box_conditional_mapper_with_predicate() {
    let predicate = BoxPredicate::new(|x: &i32| *x >= 10);

    let mut mapper = BoxStatefulTransformer::new(|x: i32| x * 2)
        .when(predicate)
        .or_else(|x| x + 1);

    assert_eq!(mapper.apply(15), 30);
    assert_eq!(mapper.apply(5), 6);
}

#[test]
fn test_arc_conditional_mapper_clone() {
    let conditional =
        ArcStatefulTransformer::new(|x: i32| x * 2).when(|x: &i32| *x > 0);

    // Clone the ArcConditionalStatefulTransformer before calling or_else
    let conditional_clone = conditional.clone();

    let mut mapper1 = conditional.or_else(|x: i32| -x);
    let mut mapper2 = conditional_clone.or_else(|x: i32| x + 100);

    // Both cloned conditional mappers work correctly
    assert_eq!(mapper1.apply(5), 10); // Condition satisfied: 5 * 2
    assert_eq!(mapper1.apply(-5), 5); // Condition not satisfied: -(-5)
    assert_eq!(mapper2.apply(5), 10); // Condition satisfied: 5 * 2
    assert_eq!(mapper2.apply(-5), 95); // Condition not satisfied: -5 + 100
}

#[test]
fn test_rc_conditional_mapper_clone() {
    let conditional =
        RcStatefulTransformer::new(|x: i32| x * 2).when(|x: &i32| *x > 0);

    // Clone the RcConditionalStatefulTransformer before calling or_else
    let conditional_clone = conditional.clone();

    let mut mapper1 = conditional.or_else(|x: i32| -x);
    let mut mapper2 = conditional_clone.or_else(|x: i32| x + 100);

    // Both cloned conditional mappers work correctly
    assert_eq!(mapper1.apply(5), 10); // Condition satisfied: 5 * 2
    assert_eq!(mapper1.apply(-5), 5); // Condition not satisfied: -(-5)
    assert_eq!(mapper2.apply(5), 10); // Condition satisfied: 5 * 2
    assert_eq!(mapper2.apply(-5), 95); // Condition not satisfied: -5 + 100
}

// ============================================================================
// Complex Composition Tests
// ============================================================================

#[test]
fn test_complex_pipeline() {
    let mut counter1 = 0;
    let step1 = BoxStatefulTransformer::new(move |x: i32| {
        counter1 += 1;
        format!("Step1[{}]: {}", counter1, x)
    });

    let mut counter2 = 0;
    let step2 = BoxStatefulTransformer::new(move |s: String| {
        counter2 += 1;
        format!("{} -> Step2[{}]", s, counter2)
    });

    let mut counter3 = 0;
    let step3 = BoxStatefulTransformer::new(move |s: String| {
        counter3 += 1;
        format!("{} -> Step3[{}]", s, counter3)
    });

    let mut pipeline = step1.and_then(step2).and_then(step3);

    assert_eq!(pipeline.apply(10), "Step1[1]: 10 -> Step2[1] -> Step3[1]");
    assert_eq!(pipeline.apply(20), "Step1[2]: 20 -> Step2[2] -> Step3[2]");
}

#[test]
fn test_nested_conditional() {
    let mut valid_count = 0;
    let mut invalid_count = 0;
    let mut error_count = 0;

    let mut mapper = BoxStatefulTransformer::new(move |x: i32| {
        valid_count += 1;
        format!("Valid[{}]: {}", valid_count, x * 2)
    })
    .when(|x: &i32| *x > 0)
    .or_else(move |x: i32| {
        let mut sub_mapper = BoxStatefulTransformer::new(move |x: i32| {
            invalid_count += 1;
            format!("Invalid[{}]: {}", invalid_count, x + 100)
        })
        .when(move |x: &i32| *x < 0)
        .or_else(move |x: i32| {
            error_count += 1;
            format!("Error[{}]: {}", error_count, x)
        });
        sub_mapper.apply(x)
    });

    assert_eq!(mapper.apply(5), "Valid[1]: 10");
    assert_eq!(mapper.apply(-5), "Invalid[1]: 95");
    assert_eq!(mapper.apply(0), "Error[1]: 0");
    assert_eq!(mapper.apply(10), "Valid[2]: 20");
}

// ============================================================================
// State Modification Tests
// ============================================================================

#[test]
fn test_stateful_counting() {
    let mut count = 0;
    let mut mapper = BoxStatefulTransformer::new(move |x: i32| {
        count += 1;
        (x, count)
    });

    assert_eq!(mapper.apply(100), (100, 1));
    assert_eq!(mapper.apply(200), (200, 2));
    assert_eq!(mapper.apply(300), (300, 3));
}

#[test]
fn test_stateful_accumulation() {
    let mut sum = 0;
    let mut mapper = BoxStatefulTransformer::new(move |x: i32| {
        sum += x;
        sum
    });

    assert_eq!(mapper.apply(10), 10);
    assert_eq!(mapper.apply(20), 30);
    assert_eq!(mapper.apply(30), 60);
}

// ============================================================================
// Type Conversion Tests
// ============================================================================

#[test]
fn test_different_types() {
    let mut counter = 0;
    let mut mapper = BoxStatefulTransformer::new(move |x: i32| {
        counter += 1;
        format!("Item #{}: {}", counter, x)
    });

    assert_eq!(mapper.apply(100), "Item #1: 100");
    assert_eq!(mapper.apply(200), "Item #2: 200");
}

#[test]
fn test_string_to_length() {
    let mut total_length = 0;
    let mut mapper = BoxStatefulTransformer::new(move |s: String| {
        total_length += s.len();
        total_length
    });

    assert_eq!(mapper.apply("hello".to_string()), 5);
    assert_eq!(mapper.apply("world".to_string()), 10);
    assert_eq!(mapper.apply("!".to_string()), 11);
}

// ============================================================================
// Predicate Integration Tests
// ============================================================================

#[test]
fn test_with_arc_predicate() {
    let predicate = ArcPredicate::new(|x: &i32| *x > 0);

    let mut mapper = ArcStatefulTransformer::new(|x: i32| x * 2)
        .when(predicate.clone())
        .or_else(|x: i32| -x);

    assert_eq!(mapper.apply(5), 10);
    assert_eq!(mapper.apply(-5), 5);

    // Predicate still usable
    assert!(predicate.test(&10));
    assert!(!predicate.test(&-10));
}

#[test]
fn test_with_rc_predicate() {
    let predicate = RcPredicate::new(|x: &i32| *x > 0);

    let mut mapper = RcStatefulTransformer::new(|x: i32| x * 2)
        .when(predicate.clone())
        .or_else(|x: i32| -x);

    assert_eq!(mapper.apply(5), 10);
    assert_eq!(mapper.apply(-5), 5);

    // Predicate still usable
    assert!(predicate.test(&10));
    assert!(!predicate.test(&-10));
}

// ============================================================================
// Custom StatefulTransformer Default Implementation Tests
// ============================================================================

/// Custom StatefulTransformer struct for testing default into_xxx() methods
#[derive(Clone)]
struct CustomStatefulTransformer {
    multiplier: i32,
}

impl StatefulTransformer<i32, i32> for CustomStatefulTransformer {
    fn apply(&mut self, input: i32) -> i32 {
        self.multiplier += 1;
        input * self.multiplier
    }
}

/// Custom thread-safe StatefulTransformer struct
#[derive(Clone)]
struct CustomSendStatefulTransformer {
    multiplier: i32,
}

impl StatefulTransformer<i32, i32> for CustomSendStatefulTransformer {
    fn apply(&mut self, input: i32) -> i32 {
        self.multiplier += 1;
        input * self.multiplier
    }
}

// Implement Send for CustomSendStatefulTransformer to allow conversion to
// ArcStatefulTransformer
unsafe impl Send for CustomSendStatefulTransformer {}
unsafe impl Sync for CustomSendStatefulTransformer {}








/// Test custom StatefulTransformer with string types
#[derive(Clone)]
struct StringLengthStatefulTransformer {
    total_length: usize,
}

impl StatefulTransformer<String, String> for StringLengthStatefulTransformer {
    fn apply(&mut self, input: String) -> String {
        self.total_length += input.len();
        format!("[{}] {}", self.total_length, input)
    }
}



/// Test custom StatefulTransformer with complex state
struct StatefulStatefulTransformer {
    count: i32,
    sum: i32,
    history: Vec<i32>,
}

impl StatefulTransformer<i32, (i32, i32, usize)>
    for StatefulStatefulTransformer
{
    fn apply(&mut self, input: i32) -> (i32, i32, usize) {
        self.count += 1;
        self.sum += input;
        self.history.push(input);
        (self.count, self.sum, self.history.len())
    }
}



// ============================================================================
// into_fn Tests
// ============================================================================



















// ============================================================================
// Closure to_xxx Non-Consuming Conversion Tests
// ============================================================================












// ============================================================================
// TransformerOnce Implementation Tests
// ============================================================================

/// Test BoxStatefulTransformer implements TransformerOnce trait
#[test]
fn test_box_mapper_apply() {
    let mut counter = 0;
    let mut mapper = BoxStatefulTransformer::new(move |x: i32| {
        counter += 1;
        x + counter
    });

    // BoxStatefulTransformer can be consumed as TransformerOnce
    assert_eq!(StatefulTransformer::apply(&mut mapper, 10), 11); // 10 + 1
}

// ============================================================================
// Conditional StatefulTransformer Display/Debug Tests
// ============================================================================

#[cfg(test)]
mod conditional_stateful_transformer_display_debug_tests {
    use super::{
        ArcStatefulTransformer,
        BoxStatefulTransformer,
        RcStatefulTransformer,
    };

    #[test]
    fn test_box_conditional_stateful_transformer_display() {
        let mut counter = 0;
        let add = BoxStatefulTransformer::new(move |x: i32| {
            counter += 1;
            x + counter
        });
        let conditional = add.when(|x: &i32| *x > 0);
        let display_str = format!("{}", conditional);
        assert!(display_str.contains("BoxConditionalStatefulTransformer"));
    }

    #[test]
    fn test_box_conditional_stateful_transformer_display_no_name() {
        let mut counter = 0;
        let add = BoxStatefulTransformer::new(move |x: i32| {
            counter += 1;
            x + counter
        });
        let conditional = add.when(|x: &i32| *x > 0);
        let display_str = format!("{}", conditional);
        assert_eq!(
            display_str,
            "BoxConditionalStatefulTransformer(BoxStatefulTransformer, BoxPredicate(unnamed))"
        );
    }

    #[test]
    fn test_box_conditional_stateful_transformer_debug() {
        let mut counter = 0;
        let add = BoxStatefulTransformer::new(move |x: i32| {
            counter += 1;
            x + counter
        });
        let conditional = add.when(|x: &i32| *x > 0);
        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("BoxConditionalStatefulTransformer"));
    }

    #[test]
    fn test_rc_conditional_stateful_transformer_display() {
        let mut counter = 0;
        let add = RcStatefulTransformer::new(move |x: i32| {
            counter += 1;
            x + counter
        });
        let conditional = add.when(|x: &i32| *x > 0);
        let display_str = format!("{}", conditional);
        assert!(display_str.contains("RcConditionalStatefulTransformer"));
    }

    #[test]
    fn test_rc_conditional_stateful_transformer_display_no_name() {
        let mut counter = 0;
        let add = RcStatefulTransformer::new(move |x: i32| {
            counter += 1;
            x + counter
        });
        let conditional = add.when(|x: &i32| *x > 0);
        let display_str = format!("{}", conditional);
        assert_eq!(
            display_str,
            "RcConditionalStatefulTransformer(RcStatefulTransformer, RcPredicate(unnamed))"
        );
    }

    #[test]
    fn test_rc_conditional_stateful_transformer_debug() {
        let mut counter = 0;
        let add = RcStatefulTransformer::new(move |x: i32| {
            counter += 1;
            x + counter
        });
        let conditional = add.when(|x: &i32| *x > 0);
        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("RcConditionalStatefulTransformer"));
    }

    #[test]
    fn test_arc_conditional_stateful_transformer_display() {
        let mut counter = 0;
        let add = ArcStatefulTransformer::new(move |x: i32| {
            counter += 1;
            x + counter
        });
        let conditional = add.when(|x: &i32| *x > 0);
        let display_str = format!("{}", conditional);
        assert!(display_str.contains("ArcConditionalStatefulTransformer"));
    }

    #[test]
    fn test_arc_conditional_stateful_transformer_display_no_name() {
        let mut counter = 0;
        let add = ArcStatefulTransformer::new(move |x: i32| {
            counter += 1;
            x + counter
        });
        let conditional = add.when(|x: &i32| *x > 0);
        let display_str = format!("{}", conditional);
        assert_eq!(
            display_str,
            "ArcConditionalStatefulTransformer(ArcStatefulTransformer, ArcPredicate(unnamed))"
        );
    }

    #[test]
    fn test_arc_conditional_stateful_transformer_debug() {
        let mut counter = 0;
        let add = ArcStatefulTransformer::new(move |x: i32| {
            counter += 1;
            x + counter
        });
        let conditional = add.when(|x: &i32| *x > 0);
        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("ArcConditionalStatefulTransformer"));
    }
}

// ============================================================================
// Basic StatefulTransformer Display Tests
// ============================================================================

#[test]
fn test_box_stateful_transformer_display_with_name() {
    let mut counter = 0;
    let transformer =
        BoxStatefulTransformer::new_with_name("counter", move |x: i32| {
            counter += 1;
            x + counter
        });
    let display_str = format!("{}", transformer);
    assert_eq!(display_str, "BoxStatefulTransformer(counter)");
}

#[test]
fn test_box_stateful_transformer_display_without_name() {
    let mut counter = 0;
    let transformer = BoxStatefulTransformer::new(move |x: i32| {
        counter += 1;
        x + counter
    });
    let display_str = format!("{}", transformer);
    assert_eq!(display_str, "BoxStatefulTransformer");
}

#[test]
fn test_rc_stateful_transformer_display_with_name() {
    let mut counter = 0;
    let transformer =
        RcStatefulTransformer::new_with_name("counter", move |x: i32| {
            counter += 1;
            x + counter
        });
    let display_str = format!("{}", transformer);
    assert_eq!(display_str, "RcStatefulTransformer(counter)");
}

#[test]
fn test_rc_stateful_transformer_display_without_name() {
    let mut counter = 0;
    let transformer = RcStatefulTransformer::new(move |x: i32| {
        counter += 1;
        x + counter
    });
    let display_str = format!("{}", transformer);
    assert_eq!(display_str, "RcStatefulTransformer");
}

#[test]
fn test_arc_stateful_transformer_display_with_name() {
    let mut counter = 0;
    let transformer =
        ArcStatefulTransformer::new_with_name("counter", move |x: i32| {
            counter += 1;
            x + counter
        });
    let display_str = format!("{}", transformer);
    assert_eq!(display_str, "ArcStatefulTransformer(counter)");
}

#[test]
fn test_arc_stateful_transformer_display_without_name() {
    let mut counter = 0;
    let transformer = ArcStatefulTransformer::new(move |x: i32| {
        counter += 1;
        x + counter
    });
    let display_str = format!("{}", transformer);
    assert_eq!(display_str, "ArcStatefulTransformer");
}

// ============================================================================
// StatefulTransformer Trait Default Methods Tests - into_once, to_once
// ============================================================================

#[cfg(test)]
mod test_stateful_transformer_trait_default_methods {





}
