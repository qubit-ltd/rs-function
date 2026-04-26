/*******************************************************************************
 *
 *    Copyright (c) 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! Tests for public re-exports from the crate root and module roots.

fn assert_type_is_exported<T>(expected_name: &str) {
    let type_name = std::any::type_name::<T>();
    assert!(
        type_name.contains(expected_name),
        "expected `{type_name}` to contain `{expected_name}`"
    );
}

#[test]
fn test_root_exports_conditional_consumer_types() {
    assert_type_is_exported::<qubit_function::BoxConditionalConsumer<i32>>(
        "BoxConditionalConsumer",
    );
    assert_type_is_exported::<qubit_function::RcConditionalConsumer<i32>>("RcConditionalConsumer");
    assert_type_is_exported::<qubit_function::ArcConditionalConsumer<i32>>(
        "ArcConditionalConsumer",
    );
    assert_type_is_exported::<qubit_function::BoxConditionalConsumerOnce<i32>>(
        "BoxConditionalConsumerOnce",
    );
    assert_type_is_exported::<qubit_function::BoxConditionalBiConsumer<i32, i64>>(
        "BoxConditionalBiConsumer",
    );
    assert_type_is_exported::<qubit_function::RcConditionalBiConsumer<i32, i64>>(
        "RcConditionalBiConsumer",
    );
    assert_type_is_exported::<qubit_function::ArcConditionalBiConsumer<i32, i64>>(
        "ArcConditionalBiConsumer",
    );
    assert_type_is_exported::<qubit_function::BoxConditionalBiConsumerOnce<i32, i64>>(
        "BoxConditionalBiConsumerOnce",
    );
    assert_type_is_exported::<qubit_function::BoxConditionalStatefulConsumer<i32>>(
        "BoxConditionalStatefulConsumer",
    );
    assert_type_is_exported::<qubit_function::RcConditionalStatefulConsumer<i32>>(
        "RcConditionalStatefulConsumer",
    );
    assert_type_is_exported::<qubit_function::ArcConditionalStatefulConsumer<i32>>(
        "ArcConditionalStatefulConsumer",
    );
    assert_type_is_exported::<qubit_function::BoxConditionalStatefulBiConsumer<i32, i64>>(
        "BoxConditionalStatefulBiConsumer",
    );
    assert_type_is_exported::<qubit_function::RcConditionalStatefulBiConsumer<i32, i64>>(
        "RcConditionalStatefulBiConsumer",
    );
    assert_type_is_exported::<qubit_function::ArcConditionalStatefulBiConsumer<i32, i64>>(
        "ArcConditionalStatefulBiConsumer",
    );
}

#[test]
fn test_root_exports_conditional_function_types() {
    assert_type_is_exported::<qubit_function::BoxConditionalFunctionOnce<i32, i64>>(
        "BoxConditionalFunctionOnce",
    );
    assert_type_is_exported::<qubit_function::BoxConditionalBiFunctionOnce<i32, i64, String>>(
        "BoxConditionalBiFunctionOnce",
    );
    assert_type_is_exported::<qubit_function::BoxConditionalMutatingFunction<i32, i64>>(
        "BoxConditionalMutatingFunction",
    );
    assert_type_is_exported::<qubit_function::RcConditionalMutatingFunction<i32, i64>>(
        "RcConditionalMutatingFunction",
    );
    assert_type_is_exported::<qubit_function::ArcConditionalMutatingFunction<i32, i64>>(
        "ArcConditionalMutatingFunction",
    );
    assert_type_is_exported::<qubit_function::BoxConditionalMutatingFunctionOnce<i32, i64>>(
        "BoxConditionalMutatingFunctionOnce",
    );
    assert_type_is_exported::<qubit_function::BoxConditionalStatefulMutatingFunction<i32, i64>>(
        "BoxConditionalStatefulMutatingFunction",
    );
    assert_type_is_exported::<qubit_function::RcConditionalStatefulMutatingFunction<i32, i64>>(
        "RcConditionalStatefulMutatingFunction",
    );
    assert_type_is_exported::<qubit_function::ArcConditionalStatefulMutatingFunction<i32, i64>>(
        "ArcConditionalStatefulMutatingFunction",
    );
}

#[test]
fn test_root_exports_conditional_transformer_types() {
    assert_type_is_exported::<qubit_function::BoxConditionalBiTransformer<i32, i64, String>>(
        "BoxConditionalBiTransformer",
    );
    assert_type_is_exported::<qubit_function::RcConditionalBiTransformer<i32, i64, String>>(
        "RcConditionalBiTransformer",
    );
    assert_type_is_exported::<qubit_function::ArcConditionalBiTransformer<i32, i64, String>>(
        "ArcConditionalBiTransformer",
    );
    assert_type_is_exported::<qubit_function::BoxConditionalBiTransformerOnce<i32, i64, String>>(
        "BoxConditionalBiTransformerOnce",
    );
}

#[test]
fn test_module_roots_export_conditional_types() {
    assert_type_is_exported::<qubit_function::consumers::BoxConditionalConsumer<i32>>(
        "BoxConditionalConsumer",
    );
    assert_type_is_exported::<qubit_function::consumers::ArcConditionalStatefulBiConsumer<i32, i64>>(
        "ArcConditionalStatefulBiConsumer",
    );
    assert_type_is_exported::<qubit_function::functions::BoxConditionalFunctionOnce<i32, i64>>(
        "BoxConditionalFunctionOnce",
    );
    assert_type_is_exported::<
        qubit_function::functions::ArcConditionalStatefulMutatingFunction<i32, i64>,
    >("ArcConditionalStatefulMutatingFunction");
    assert_type_is_exported::<
        qubit_function::transformers::BoxConditionalBiTransformerOnce<i32, i64, String>,
    >("BoxConditionalBiTransformerOnce");
    assert_type_is_exported::<
        qubit_function::transformers::RcConditionalBiTransformer<i32, i64, String>,
    >("RcConditionalBiTransformer");
}
