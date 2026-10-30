// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Demonstrates the usage of StatefulTransformer types
//!
//! This example shows how to use BoxStatefulTransformer, RcStatefulTransformer,
//! and ArcStatefulTransformer for stateful value transformation.

use qubit_function::ArcStatefulTransformer;
use qubit_function::BoxStatefulTransformer;
use qubit_function::BoxTransformerOnce;
use qubit_function::RcStatefulTransformer;
use qubit_function::StatefulTransformer;
use qubit_function::TransformerOnce;

fn main() {
    println!("=== StatefulTransformer Demo ===\n");

    // 1. Basic BoxStatefulTransformer with state
    println!("1. BoxStatefulTransformer with stateful counter:");
    let mut counter = 0;
    let mut mapper = BoxStatefulTransformer::new(move |x: i32| {
        counter += 1;
        format!("Item #{}: {}", counter, x)
    });

    println!("  {}", mapper.apply(100)); // Item #1: 100
    println!("  {}", mapper.apply(200)); // Item #2: 200
    println!("  {}", mapper.apply(300)); // Item #3: 300

    // 2. Composing mappers with and_then
    println!("\n2. Composing mappers with and_then:");
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
    println!("  First call:  {}", composed.apply(10)); // (10 + 1) * 1 = 11
    println!("  Second call: {}", composed.apply(10)); // (10 + 2) * 2 = 24
    println!("  Third call:  {}", composed.apply(10)); // (10 + 3) * 3 = 39

    // 3. Conditional mapping with when/or_else
    println!("\n3. Conditional mapping:");
    let mut high_count = 0;
    let mut low_count = 0;

    let mut conditional = BoxStatefulTransformer::new(move |x: i32| {
        high_count += 1;
        format!("High[{}]: {} * 2 = {}", high_count, x, x * 2)
    })
    .when(|x: &i32| *x >= 10)
    .or_else(move |x| {
        low_count += 1;
        format!("Low[{}]: {} + 1 = {}", low_count, x, x + 1)
    });

    println!("  {}", conditional.apply(15)); // High[1]: 15 * 2 = 30
    println!("  {}", conditional.apply(5)); // Low[1]: 5 + 1 = 6
    println!("  {}", conditional.apply(20)); // High[2]: 20 * 2 = 40

    // 4. RcStatefulTransformer for cloneable mappers
    println!("\n4. RcStatefulTransformer (cloneable, single-threaded):");
    let mut counter = 0;
    let mapper = RcStatefulTransformer::new(move |x: i32| {
        counter += 1;
        x + counter
    });

    let mut mapper1 = mapper.clone();
    let mut mapper2 = mapper.clone();

    println!("  mapper1: {}", mapper1.apply(10)); // 11
    println!("  mapper2: {}", mapper2.apply(10)); // 12
    println!("  mapper1: {}", mapper1.apply(10)); // 13

    // 5. ArcStatefulTransformer for thread-safe mappers
    println!("\n5. ArcStatefulTransformer (thread-safe):");
    let mut counter = 0;
    let mapper = ArcStatefulTransformer::new(move |x: i32| {
        counter += 1;
        format!("Result[{}]: {}", counter, x * 2)
    });

    let mut mapper_clone = mapper.clone();
    println!("  Original: {}", mapper_clone.apply(5)); // Result[1]: 10
    println!("  Clone:    {}", mapper_clone.apply(7)); // Result[2]: 14

    // 6. Explicitly wrapping a closure before composition
    println!("\n6. Explicit closure wrapping:");
    let mut count = 0;
    let mut mapper = BoxStatefulTransformer::new(move |x: i32| {
        count += 1;
        x + count
    })
    .and_then(|x| x * 2);

    println!("  {}", mapper.apply(10)); // (10 + 1) * 2 = 22
    println!("  {}", mapper.apply(10)); // (10 + 2) * 2 = 24

    // 7. Building a complex pipeline
    println!("\n7. Complex processing pipeline:");
    let mut step1_count = 0;
    let step1 = BoxStatefulTransformer::new(move |x: i32| {
        step1_count += 1;
        format!("Step1[{}]: {}", step1_count, x)
    });

    let mut step2_count = 0;
    let step2 = BoxStatefulTransformer::new(move |s: String| {
        step2_count += 1;
        format!("{} -> Step2[{}]", s, step2_count)
    });

    let mut step3_count = 0;
    let step3 = BoxStatefulTransformer::new(move |s: String| {
        step3_count += 1;
        format!("{} -> Step3[{}]", s, step3_count)
    });

    let mut pipeline = step1.and_then(step2).and_then(step3);

    println!("  {}", pipeline.apply(100));
    println!("  {}", pipeline.apply(200));

    // 8. Stateful transformers can be reused across calls
    println!("\n8. Reusing stateful transformers:");

    let mut counter = 0;
    let mut box_mapper = BoxStatefulTransformer::new(move |x: i32| {
        counter += 1;
        x * counter
    });
    let first = box_mapper.apply(10);
    let second = box_mapper.apply(10);
    assert_eq!((first, second), (10, 20));
    println!("  BoxStatefulTransformer calls: {first}, {second}");

    // Clones share the same state, even when calls alternate between them.
    let mut counter = 0;
    let mut rc_mapper = RcStatefulTransformer::new(move |x: i32| {
        counter += 1;
        x + counter
    });
    let mut rc_clone = rc_mapper.clone();
    let first = rc_mapper.apply(10);
    let second = rc_clone.apply(10);
    let third = rc_mapper.apply(10);
    assert_eq!((first, second, third), (11, 12, 13));
    println!("  RcStatefulTransformer calls: {first}, {second}, {third}");

    // Arc clones also observe one continuously advancing state.
    let mut counter = 0;
    let mut arc_mapper = ArcStatefulTransformer::new(move |x: i32| {
        counter += 1;
        x * counter
    });
    let mut arc_clone = arc_mapper.clone();
    let first = arc_mapper.apply(10);
    let second = arc_clone.apply(10);
    let third = arc_mapper.apply(10);
    assert_eq!((first, second, third), (10, 20, 30));
    println!("  ArcStatefulTransformer calls: {first}, {second}, {third}");

    // 9. Explicitly adapt one stateful call to a consuming wrapper.
    println!("\n9. Wrapping one stateful call in BoxTransformerOnce:");

    let mut counter = 0;
    let mapper = BoxStatefulTransformer::new(move |x: i32| {
        counter += 1;
        x * counter
    });
    let mut mapper = mapper;
    // This adapter implements TransformerOnce on the outer wrapper, which is
    // consumed by this call. It does not add TransformerOnce to the inner
    // stateful mapper.
    let once_mapper = BoxTransformerOnce::new(move |value| mapper.apply(value));
    let result = TransformerOnce::apply(once_mapper, 5);
    assert_eq!(result, 5);
    println!("  BoxStatefulTransformer->BoxTransformerOnce: {result}");

    // Cloning an RcStatefulTransformer still shares its state with the original.
    let mut counter = 0;
    let rc_mapper = RcStatefulTransformer::new(move |x: i32| {
        counter += 1;
        x * counter
    });
    let mut rc_clone = rc_mapper.clone();
    let once_mapper = BoxTransformerOnce::new(move |value| rc_clone.apply(value));
    println!("  RcStatefulTransformer->BoxTransformerOnce: {}", once_mapper.apply(5)); // 5 * 1 = 5
    println!("  Original RcStatefulTransformer still works: {}", {
        let mut rc_original_for_call = rc_mapper.clone();
        rc_original_for_call.apply(5)
    }); // 5 * 2 = 10

    println!("\n=== Demo Complete ===");
}
