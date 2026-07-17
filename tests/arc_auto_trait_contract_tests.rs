// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::{
    ArcBiConsumer,
    ArcBiFunction,
    ArcBiMutatingFunction,
    ArcBiPredicate,
    ArcBiTransformer,
    ArcBinaryFunction,
    ArcBinaryMutatingFunction,
    ArcBinaryOperator,
    ArcComparator,
    ArcConditionalBiConsumer,
    ArcConditionalBiFunction,
    ArcConditionalBiMutatingFunction,
    ArcConditionalBiTransformer,
    ArcConditionalConsumer,
    ArcConditionalFunction,
    ArcConditionalMutatingFunction,
    ArcConditionalMutator,
    ArcConditionalTransformer,
    ArcConsumer,
    ArcFunction,
    ArcMutatingFunction,
    ArcMutator,
    ArcPredicate,
    ArcSupplier,
    ArcTester,
    ArcTransformer,
    ArcUnaryOperator,
};

#[cfg(feature = "stateful")]
use qubit_function::{
    ArcCallable,
    ArcCallableWith,
    ArcConditionalStatefulBiConsumer,
    ArcConditionalStatefulBiTransformer,
    ArcConditionalStatefulConsumer,
    ArcConditionalStatefulFunction,
    ArcConditionalStatefulMutatingFunction,
    ArcConditionalStatefulMutator,
    ArcConditionalStatefulTransformer,
    ArcRunnable,
    ArcRunnableWith,
    ArcStatefulBiConsumer,
    ArcStatefulBiPredicate,
    ArcStatefulBiTransformer,
    ArcStatefulBinaryOperator,
    ArcStatefulConsumer,
    ArcStatefulFunction,
    ArcStatefulMutatingFunction,
    ArcStatefulMutator,
    ArcStatefulPredicate,
    ArcStatefulSupplier,
    ArcStatefulTester,
    ArcStatefulTransformer,
};

/// Proves at compile time that `T` is transferable and shareable.
fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn test_all_arc_wrapper_types_are_send_and_sync() {
    assert_send_sync::<ArcComparator<i32>>();
    assert_send_sync::<ArcBiConsumer<i32, i64>>();
    assert_send_sync::<ArcConditionalBiConsumer<i32, i64>>();
    assert_send_sync::<ArcConsumer<i32>>();
    assert_send_sync::<ArcConditionalConsumer<i32>>();

    assert_send_sync::<ArcBiFunction<i32, i64, String>>();
    assert_send_sync::<ArcBinaryFunction<i32, String>>();
    assert_send_sync::<ArcConditionalBiFunction<i32, i64, String>>();
    assert_send_sync::<ArcBiMutatingFunction<i32, i64, String>>();
    assert_send_sync::<ArcBinaryMutatingFunction<i32, String>>();
    assert_send_sync::<ArcConditionalBiMutatingFunction<i32, i64, String>>();
    assert_send_sync::<ArcFunction<i32, String>>();
    assert_send_sync::<ArcConditionalFunction<i32, String>>();
    assert_send_sync::<ArcMutatingFunction<i32, String>>();
    assert_send_sync::<ArcConditionalMutatingFunction<i32, String>>();

    assert_send_sync::<ArcMutator<i32>>();
    assert_send_sync::<ArcConditionalMutator<i32>>();
    assert_send_sync::<ArcBiPredicate<i32, i64>>();
    assert_send_sync::<ArcPredicate<i32>>();
    assert_send_sync::<ArcSupplier<String>>();
    assert_send_sync::<ArcTester>();

    assert_send_sync::<ArcBiTransformer<i32, i64, String>>();
    assert_send_sync::<ArcBinaryOperator<i32>>();
    assert_send_sync::<ArcConditionalBiTransformer<i32, i64, String>>();
    assert_send_sync::<ArcTransformer<i32, String>>();
    assert_send_sync::<ArcUnaryOperator<i32>>();
    assert_send_sync::<ArcConditionalTransformer<i32, String>>();
}

#[cfg(feature = "stateful")]
#[test]
fn test_all_feature_gated_arc_wrapper_types_are_send_and_sync() {
    assert_send_sync::<ArcStatefulBiConsumer<i32, i64>>();
    assert_send_sync::<ArcConditionalStatefulBiConsumer<i32, i64>>();
    assert_send_sync::<ArcStatefulConsumer<i32>>();
    assert_send_sync::<ArcConditionalStatefulConsumer<i32>>();
    assert_send_sync::<ArcStatefulFunction<i32, String>>();
    assert_send_sync::<ArcConditionalStatefulFunction<i32, String>>();
    assert_send_sync::<ArcStatefulMutatingFunction<i32, String>>();
    assert_send_sync::<ArcConditionalStatefulMutatingFunction<i32, String>>();
    assert_send_sync::<ArcStatefulMutator<i32>>();
    assert_send_sync::<ArcConditionalStatefulMutator<i32>>();
    assert_send_sync::<ArcStatefulBiPredicate<i32, i64>>();
    assert_send_sync::<ArcStatefulPredicate<i32>>();
    assert_send_sync::<ArcStatefulSupplier<String>>();
    assert_send_sync::<ArcStatefulTester>();
    assert_send_sync::<ArcStatefulBiTransformer<i32, i64, String>>();
    assert_send_sync::<ArcStatefulBinaryOperator<i32>>();
    assert_send_sync::<ArcConditionalStatefulBiTransformer<i32, i64, String>>();
    assert_send_sync::<ArcStatefulTransformer<i32, String>>();
    assert_send_sync::<ArcConditionalStatefulTransformer<i32, String>>();
    assert_send_sync::<ArcCallable<i32, String>>();
    assert_send_sync::<ArcCallableWith<i32, i64, String>>();
    assert_send_sync::<ArcRunnable<String>>();
    assert_send_sync::<ArcRunnableWith<i32, String>>();
}
