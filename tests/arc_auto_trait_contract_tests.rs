// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::ArcBiConsumer;
use qubit_function::ArcBiFunction;
use qubit_function::ArcBiMutatingFunction;
use qubit_function::ArcBiPredicate;
use qubit_function::ArcBiTransformer;
use qubit_function::ArcBinaryFunction;
use qubit_function::ArcBinaryMutatingFunction;
use qubit_function::ArcBinaryOperator;
#[cfg(feature = "stateful")]
use qubit_function::ArcCallable;
#[cfg(feature = "stateful")]
use qubit_function::ArcCallableWith;
use qubit_function::ArcComparator;
use qubit_function::ArcConditionalBiConsumer;
use qubit_function::ArcConditionalBiFunction;
use qubit_function::ArcConditionalBiMutatingFunction;
use qubit_function::ArcConditionalBiTransformer;
use qubit_function::ArcConditionalConsumer;
use qubit_function::ArcConditionalFunction;
use qubit_function::ArcConditionalMutatingFunction;
use qubit_function::ArcConditionalMutator;
#[cfg(feature = "stateful")]
use qubit_function::ArcConditionalStatefulBiConsumer;
#[cfg(feature = "stateful")]
use qubit_function::ArcConditionalStatefulBiTransformer;
#[cfg(feature = "stateful")]
use qubit_function::ArcConditionalStatefulConsumer;
#[cfg(feature = "stateful")]
use qubit_function::ArcConditionalStatefulFunction;
#[cfg(feature = "stateful")]
use qubit_function::ArcConditionalStatefulMutatingFunction;
#[cfg(feature = "stateful")]
use qubit_function::ArcConditionalStatefulMutator;
#[cfg(feature = "stateful")]
use qubit_function::ArcConditionalStatefulTransformer;
use qubit_function::ArcConditionalTransformer;
use qubit_function::ArcConsumer;
use qubit_function::ArcFunction;
use qubit_function::ArcMutatingFunction;
use qubit_function::ArcMutator;
use qubit_function::ArcPredicate;
#[cfg(feature = "stateful")]
use qubit_function::ArcRunnable;
#[cfg(feature = "stateful")]
use qubit_function::ArcRunnableWith;
#[cfg(feature = "stateful")]
use qubit_function::ArcStatefulBiConsumer;
#[cfg(feature = "stateful")]
use qubit_function::ArcStatefulBiPredicate;
#[cfg(feature = "stateful")]
use qubit_function::ArcStatefulBiTransformer;
#[cfg(feature = "stateful")]
use qubit_function::ArcStatefulBinaryOperator;
#[cfg(feature = "stateful")]
use qubit_function::ArcStatefulConsumer;
#[cfg(feature = "stateful")]
use qubit_function::ArcStatefulFunction;
#[cfg(feature = "stateful")]
use qubit_function::ArcStatefulMutatingFunction;
#[cfg(feature = "stateful")]
use qubit_function::ArcStatefulMutator;
#[cfg(feature = "stateful")]
use qubit_function::ArcStatefulPredicate;
#[cfg(feature = "stateful")]
use qubit_function::ArcStatefulSupplier;
#[cfg(feature = "stateful")]
use qubit_function::ArcStatefulTester;
#[cfg(feature = "stateful")]
use qubit_function::ArcStatefulTransformer;
use qubit_function::ArcSupplier;
use qubit_function::ArcTester;
use qubit_function::ArcTransformer;
use qubit_function::ArcUnaryOperator;

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
