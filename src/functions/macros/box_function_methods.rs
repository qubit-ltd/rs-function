// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Box Function Methods Macro
//!
//! Generates `when` and `and_then` method implementations for boxed functions.
//!
//! Generates conditional execution when method and chaining and_then method
//! for Box-based functions that consume self (because Box cannot be cloned).
//!
//! This macro supports both single-parameter and two-parameter functions
//! through pattern matching on the struct signature.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - The struct name with its generic parameters
//!   - Single parameter: `BoxFunction<T, R>`
//!   - Two parameters: `BoxBiFunction<T, U, R>`
//! * `$conditional_type` - The conditional function type for when (e.g.,
//!   BoxConditionalFunction)
//! * `$chained_function_trait` - The name of the function trait that is chained
//!   after the execution of this function (e.g., Function, BiFunction)
//!
//! # Parameter Usage Comparison
//!
//! | Function Type | Struct Signature | `$conditional_type` | `$chained_function_trait` |
//! |---------------|-----------------|----------------|------------------|
//! | **Function** | `BoxFunction<T, R>` | BoxConditionalFunction | Function |
//! | **FunctionOnce** | `BoxFunctionOnce<T, R>` | BoxConditionalFunctionOnce | FunctionOnce |
//! | **StatefulFunction** | `BoxStatefulFunction<T, R>` | BoxConditionalStatefulFunction | StatefulFunction |
//! | **BiFunction** | `BoxBiFunction<T, U, R>` | BoxConditionalBiFunction | BiFunction |
//! | **BiFunctionOnce** | `BoxBiFunctionOnce<T, U, R>` | BoxConditionalBiFunctionOnce | BiFunctionOnce |
//! | **StatefulBiFunction** | `BoxStatefulBiFunction<T, U, R>` | BoxConditionalStatefulBiFunction | StatefulBiFunction |
//!
//! # Examples
//!
//! ```text
//! // `impl_box_function_methods!` is crate-private and expanded in `qubit_function`
//! // internal macro exports.
//!
//! // Single-parameter function
//! // impl_box_function_methods!(
//! //     BoxFunction<T, R>,
//! //     BoxConditionalFunction,
//! //     Function
//! // );
//!
//! // Two-parameter function
//! // impl_box_function_methods!(
//! //     BoxBiFunction<T, U, R>,
//! //     BoxConditionalBiFunction,
//! //     BiFunction
//! // );
//! ```

/// Generates `when` and `and_then` method implementations for boxed functions.
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete impl block with methods for the specified struct.
/// Generates conditional execution when method and chaining and_then method
/// for Box-based functions that consume self (because Box cannot be cloned).
///
/// This macro supports both single-parameter and two-parameter functions
/// through pattern matching on the struct signature.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - The struct name with its generic parameters
///   - Single parameter: `BoxFunction<T, R>`
///   - Two parameters: `BoxBiFunction<T, U, R>`
/// * `$conditional_type` - The conditional function type for when (e.g.,
///   BoxConditionalFunction)
/// * `$chained_function_trait` - The name of the function trait that is chained
///   after the execution of this function (e.g., Function, BiFunction)
///
/// # Parameter Usage Comparison
///
/// | Function Type | Struct Signature | `$conditional_type` | `$chained_function_trait` |
/// |---------------|-----------------|----------------|------------------|
/// | **Function** | `BoxFunction<T, R>` | BoxConditionalFunction | Function |
/// | **FunctionOnce** | `BoxFunctionOnce<T, R>` | BoxConditionalFunctionOnce | FunctionOnce |
/// | **StatefulFunction** | `BoxStatefulFunction<T, R>` | BoxConditionalStatefulFunction | StatefulFunction |
/// | **BiFunction** | `BoxBiFunction<T, U, R>` | BoxConditionalBiFunction | BiFunction |
/// | **BiFunctionOnce** | `BoxBiFunctionOnce<T, U, R>` | BoxConditionalBiFunctionOnce | BiFunctionOnce |
/// | **StatefulBiFunction** | `BoxStatefulBiFunction<T, U, R>` | BoxConditionalStatefulBiFunction | StatefulBiFunction |
///
/// # Examples
///
/// ```text
/// // Single-parameter function
/// // impl_box_function_methods!(
/// //     BoxFunction<T, R>,
/// //     BoxConditionalFunction,
/// //     Function
/// // );
///
/// // Two-parameter function
/// // impl_box_function_methods!(
/// //     BoxBiFunction<T, U, R>,
/// //     BoxConditionalBiFunction,
/// //     BiFunction
/// // );
/// ```
macro_rules! impl_box_function_methods {
    (@doc_when BoxFunction) => { impl_box_function_methods!(@doc_when_function BoxFunction, "Function") };
    (@doc_when BoxStatefulFunction) => { impl_box_function_methods!(@doc_when_stateful BoxStatefulFunction, "StatefulFunction") };
    (@doc_when BoxMutatingFunction) => { impl_box_function_methods!(@doc_when_mutating BoxMutatingFunction, "MutatingFunction") };
    (@doc_when BoxStatefulMutatingFunction) => { impl_box_function_methods!(@doc_when_stateful_mutating BoxStatefulMutatingFunction, "StatefulMutatingFunction") };
    (@doc_when BoxFunctionOnce) => { impl_box_function_methods!(@doc_when_once BoxFunctionOnce, "FunctionOnce") };
    (@doc_when BoxMutatingFunctionOnce) => { impl_box_function_methods!(@doc_when_mutating_once BoxMutatingFunctionOnce, "MutatingFunctionOnce") };
    (@doc_when BoxBiFunction) => { impl_box_function_methods!(@doc_when_bi BoxBiFunction, "BiFunction") };
    (@doc_when BoxBiFunctionOnce) => { impl_box_function_methods!(@doc_when_bi_once BoxBiFunctionOnce, "BiFunctionOnce") };
    (@doc_when BoxBiMutatingFunction) => { impl_box_function_methods!(@doc_when_bi_mutating BoxBiMutatingFunction, "BiMutatingFunction") };
    (@doc_when BoxBiMutatingFunctionOnce) => { impl_box_function_methods!(@doc_when_bi_mutating_once BoxBiMutatingFunctionOnce, "BiMutatingFunctionOnce") };

    (@doc_when_function $wrapper:ident, $source_trait:literal) => { concat!(
        "```rust\nuse qubit_function::{", stringify!($wrapper), ", ", $source_trait, "};\n",
        "let positive = ", stringify!($wrapper), "::new(|value: &i32| *value + 1);\n",
        "let positive = positive.when(|value: &i32| *value > 0);\nlet mut positive = positive.or_else(|_: &i32| 0);\n",
        "assert_eq!(positive.apply(&5), 6);\n",
        "let negative = ", stringify!($wrapper), "::new(|value: &i32| *value + 1);\n",
        "let negative = negative.when(|value: &i32| *value > 0);\nlet mut negative = negative.or_else(|_: &i32| 0);\n",
        "assert_eq!(negative.apply(&-5), 0);\n```"
    ) };
    (@doc_when_stateful $wrapper:ident, $source_trait:literal) => { concat!(
        "```rust\nuse qubit_function::{", stringify!($wrapper), ", ", $source_trait, "};\n",
        "let mut positive_calls = 0;\nlet positive = ", stringify!($wrapper), "::new(move |value: &i32| { positive_calls += 1; *value + positive_calls });\n",
        "let positive = positive.when(|value: &i32| *value > 0);\n",
        "assert_eq!(positive.or_else(|_: &i32| 0).apply(&5), 6);\n",
        "let mut negative_calls = 0;\nlet negative = ", stringify!($wrapper), "::new(move |value: &i32| { negative_calls += 1; *value + negative_calls });\n",
        "let negative = negative.when(|value: &i32| *value > 0);\n",
        "assert_eq!(negative.or_else(|_: &i32| 0).apply(&-5), 0);\n```"
    ) };
    (@doc_when_mutating $wrapper:ident, $source_trait:literal) => { concat!(
        "```rust\nuse qubit_function::{", stringify!($wrapper), ", ", $source_trait, "};\n",
        "let positive = ", stringify!($wrapper), "::new(|value: &mut i32| { *value += 1; *value });\n",
        "let positive = positive.when(|value: &i32| *value > 0);\nlet mut input = 5;\n",
        "let result = positive.or_else(|_: &mut i32| 0).apply(&mut input);\nassert_eq!((input, result), (6, 6));\n",
        "let negative = ", stringify!($wrapper), "::new(|value: &mut i32| { *value += 1; *value });\n",
        "let negative = negative.when(|value: &i32| *value > 0);\nlet mut input = -5;\n",
        "let result = negative.or_else(|_: &mut i32| 0).apply(&mut input);\nassert_eq!((input, result), (-5, 0));\n```"
    ) };
    (@doc_when_stateful_mutating $wrapper:ident, $source_trait:literal) => { concat!(
        "```rust\nuse qubit_function::{", stringify!($wrapper), ", ", $source_trait, "};\n",
        "let mut positive_calls = 0;\nlet positive = ", stringify!($wrapper), "::new(move |value: &mut i32| { positive_calls += 1; *value += positive_calls; *value });\n",
        "let positive = positive.when(|value: &i32| *value > 0);\nlet mut input = 5;\n",
        "let mut positive = positive.or_else(|_: &mut i32| 0);\nlet result = positive.apply(&mut input);\nassert_eq!((input, result), (6, 6));\n",
        "let mut negative_calls = 0;\nlet negative = ", stringify!($wrapper), "::new(move |value: &mut i32| { negative_calls += 1; *value += negative_calls; *value });\n",
        "let negative = negative.when(|value: &i32| *value > 0);\nlet mut input = -5;\n",
        "let mut negative = negative.or_else(|_: &mut i32| 0);\nlet result = negative.apply(&mut input);\nassert_eq!((input, result), (-5, 0));\n```"
    ) };
    (@doc_when_once $wrapper:ident, $source_trait:literal) => { concat!(
        "```rust\nuse std::sync::Mutex;\nuse qubit_function::{", stringify!($wrapper), ", ", $source_trait, "};\n",
        "let resource = Mutex::new(6);\nlet positive = ", stringify!($wrapper), "::new(move |value: &i32| *value + resource.into_inner().unwrap());\n",
        "let positive = positive.when(|value: &i32| *value > 0);\nassert_eq!(positive.or_else(|_: &i32| 0).apply(&5), 11);\n",
        "let resource = Mutex::new(6);\nlet negative = ", stringify!($wrapper), "::new(move |value: &i32| *value + resource.into_inner().unwrap());\n",
        "let negative = negative.when(|value: &i32| *value > 0);\nassert_eq!(negative.or_else(|_: &i32| 0).apply(&-5), 0);\n```"
    ) };
    (@doc_when_mutating_once $wrapper:ident, $source_trait:literal) => { concat!(
        "```rust\nuse std::sync::Mutex;\nuse qubit_function::{", stringify!($wrapper), ", ", $source_trait, "};\n",
        "let resource = Mutex::new(6);\nlet positive = ", stringify!($wrapper), "::new(move |value: &mut i32| { *value += resource.into_inner().unwrap(); *value });\n",
        "let positive = positive.when(|value: &i32| *value > 0);\nlet mut input = 5;\nlet result = positive.or_else(|_: &mut i32| 0).apply(&mut input);\nassert_eq!((input, result), (11, 11));\n",
        "let resource = Mutex::new(6);\nlet negative = ", stringify!($wrapper), "::new(move |value: &mut i32| { *value += resource.into_inner().unwrap(); *value });\n",
        "let negative = negative.when(|value: &i32| *value > 0);\nlet mut input = -5;\nlet result = negative.or_else(|_: &mut i32| 0).apply(&mut input);\nassert_eq!((input, result), (-5, 0));\n```"
    ) };
    (@doc_when_bi $wrapper:ident, $source_trait:literal) => { concat!(
        "```rust\nuse qubit_function::{", stringify!($wrapper), ", ", $source_trait, "};\n",
        "let positive = ", stringify!($wrapper), "::new(|first: &i32, second: &i32| first + second);\nlet positive = positive.when(|first: &i32, second: &i32| *first > 0 && *second > 0);\n",
        "assert_eq!(positive.or_else(|_: &i32, _: &i32| 0).apply(&2, &3), 5);\n",
        "let negative = ", stringify!($wrapper), "::new(|first: &i32, second: &i32| first + second);\nlet negative = negative.when(|first: &i32, second: &i32| *first > 0 && *second > 0);\n",
        "assert_eq!(negative.or_else(|_: &i32, _: &i32| 0).apply(&-2, &3), 0);\n```"
    ) };
    (@doc_when_bi_once $wrapper:ident, $source_trait:literal) => { concat!(
        "```rust\nuse std::sync::Mutex;\nuse qubit_function::{", stringify!($wrapper), ", ", $source_trait, "};\n",
        "let resource = Mutex::new(6);\nlet positive = ", stringify!($wrapper), "::new(move |first: &i32, second: &i32| first + second + resource.into_inner().unwrap());\nlet positive = positive.when(|first: &i32, second: &i32| *first > 0 && *second > 0);\n",
        "assert_eq!(positive.or_else(|_: &i32, _: &i32| 0).apply(&2, &3), 11);\n",
        "let resource = Mutex::new(6);\nlet negative = ", stringify!($wrapper), "::new(move |first: &i32, second: &i32| first + second + resource.into_inner().unwrap());\nlet negative = negative.when(|first: &i32, second: &i32| *first > 0 && *second > 0);\n",
        "assert_eq!(negative.or_else(|_: &i32, _: &i32| 0).apply(&-2, &3), 0);\n```"
    ) };
    (@doc_when_bi_mutating $wrapper:ident, $source_trait:literal) => { concat!(
        "```rust\nuse qubit_function::{", stringify!($wrapper), ", ", $source_trait, "};\n",
        "let positive = ", stringify!($wrapper), "::new(|first: &mut i32, second: &mut i32| { *first += 1; *second += 2; *first + *second });\nlet positive = positive.when(|first: &i32, second: &i32| *first > 0 && *second > 0);\n",
        "let (mut first, mut second) = (2, 3);\nlet result = positive.or_else(|_: &mut i32, _: &mut i32| 0).apply(&mut first, &mut second);\nassert_eq!((first, second, result), (3, 5, 8));\n",
        "let negative = ", stringify!($wrapper), "::new(|first: &mut i32, second: &mut i32| { *first += 1; *second += 2; *first + *second });\nlet negative = negative.when(|first: &i32, second: &i32| *first > 0 && *second > 0);\n",
        "let (mut first, mut second) = (-2, 3);\nlet result = negative.or_else(|_: &mut i32, _: &mut i32| 0).apply(&mut first, &mut second);\nassert_eq!((first, second, result), (-2, 3, 0));\n```"
    ) };
    (@doc_when_bi_mutating_once $wrapper:ident, $source_trait:literal) => { concat!(
        "```rust\nuse std::sync::Mutex;\nuse qubit_function::{", stringify!($wrapper), ", ", $source_trait, "};\n",
        "let resource = Mutex::new(6);\nlet positive = ", stringify!($wrapper), "::new(move |first: &mut i32, second: &mut i32| { *first += resource.into_inner().unwrap(); *second += 2; *first + *second });\nlet positive = positive.when(|first: &i32, second: &i32| *first > 0 && *second > 0);\n",
        "let (mut first, mut second) = (2, 3);\nlet result = positive.or_else(|_: &mut i32, _: &mut i32| 0).apply(&mut first, &mut second);\nassert_eq!((first, second, result), (8, 5, 13));\n",
        "let resource = Mutex::new(6);\nlet negative = ", stringify!($wrapper), "::new(move |first: &mut i32, second: &mut i32| { *first += resource.into_inner().unwrap(); *second += 2; *first + *second });\nlet negative = negative.when(|first: &i32, second: &i32| *first > 0 && *second > 0);\n",
        "let (mut first, mut second) = (-2, 3);\nlet result = negative.or_else(|_: &mut i32, _: &mut i32| 0).apply(&mut first, &mut second);\nassert_eq!((first, second, result), (-2, 3, 0));\n```"
    ) };

    (@doc_then BoxFunction, $chained_trait:ident) => { impl_box_function_methods!(@doc_then_function BoxFunction, "Function", $chained_trait) };
    (@doc_then BoxStatefulFunction, $chained_trait:ident) => { impl_box_function_methods!(@doc_then_stateful BoxStatefulFunction, "StatefulFunction", $chained_trait) };
    (@doc_then BoxMutatingFunction, $chained_trait:ident) => { impl_box_function_methods!(@doc_then_mutating BoxMutatingFunction, "MutatingFunction", $chained_trait) };
    (@doc_then BoxStatefulMutatingFunction, $chained_trait:ident) => { impl_box_function_methods!(@doc_then_stateful_mutating BoxStatefulMutatingFunction, "StatefulMutatingFunction", $chained_trait) };
    (@doc_then BoxFunctionOnce, $chained_trait:ident) => { impl_box_function_methods!(@doc_then_once BoxFunctionOnce, "FunctionOnce", $chained_trait) };
    (@doc_then BoxMutatingFunctionOnce, $chained_trait:ident) => { impl_box_function_methods!(@doc_then_mutating_once BoxMutatingFunctionOnce, "MutatingFunctionOnce", $chained_trait) };
    (@doc_then BoxBiFunction, $chained_trait:ident) => { impl_box_function_methods!(@doc_then_bi BoxBiFunction, "BiFunction", $chained_trait) };
    (@doc_then BoxBiFunctionOnce, $chained_trait:ident) => { impl_box_function_methods!(@doc_then_bi_once BoxBiFunctionOnce, "BiFunctionOnce", $chained_trait) };
    (@doc_then BoxBiMutatingFunction, $chained_trait:ident) => { impl_box_function_methods!(@doc_then_bi_mutating BoxBiMutatingFunction, "BiMutatingFunction", $chained_trait) };
    (@doc_then BoxBiMutatingFunctionOnce, $chained_trait:ident) => { impl_box_function_methods!(@doc_then_bi_mutating_once BoxBiMutatingFunctionOnce, "BiMutatingFunctionOnce", $chained_trait) };

    (@doc_then_function $wrapper:ident, $source_trait:literal, $chained_trait:ident) => { concat!(
        "```rust\nuse qubit_function::{", stringify!($wrapper), ", ", $source_trait, "};\n",
        "// Source trait: ", $source_trait, "; chained callback trait: ", stringify!($chained_trait), ".\n",
        "let source = ", stringify!($wrapper), "::new(|value: &i32| *value + 1);\n",
        "let chained = source.and_then(|value: &i32| *value * 2);\nassert_eq!(chained.apply(&5), 12);\n```"
    ) };
    (@doc_then_stateful $wrapper:ident, $source_trait:literal, $chained_trait:ident) => { concat!(
        "```rust\nuse qubit_function::{", stringify!($wrapper), ", ", $source_trait, "};\n",
        "// Source trait: ", $source_trait, "; chained callback trait: ", stringify!($chained_trait), ".\n",
        "let mut source_calls = 0;\nlet source = ", stringify!($wrapper), "::new(move |value: &i32| { source_calls += 1; *value + source_calls });\n",
        "let mut mapper_calls = 0;\nlet mut chained = source.and_then(move |value: &i32| { mapper_calls += 1; *value * 2 + mapper_calls });\n",
        "assert_eq!(chained.apply(&5), 13);\nassert_eq!(chained.apply(&5), 16);\n```"
    ) };
    (@doc_then_mutating $wrapper:ident, $source_trait:literal, $chained_trait:ident) => { concat!(
        "```rust\nuse qubit_function::{", stringify!($wrapper), ", ", $source_trait, "};\n",
        "// Source trait: ", $source_trait, "; chained callback trait: ", stringify!($chained_trait), ".\n",
        "let source = ", stringify!($wrapper), "::new(|value: &mut i32| { *value += 1; *value });\n",
        "let mut chained = source.and_then(|value: &i32| *value * 2);\nlet mut input = 5;\n",
        "assert_eq!(chained.apply(&mut input), 12);\nassert_eq!(input, 6);\n```"
    ) };
    (@doc_then_stateful_mutating $wrapper:ident, $source_trait:literal, $chained_trait:ident) => { concat!(
        "```rust\nuse qubit_function::{", stringify!($wrapper), ", ", $source_trait, "};\n",
        "// Source trait: ", $source_trait, "; chained callback trait: ", stringify!($chained_trait), ".\n",
        "let mut calls = 0;\nlet source = ", stringify!($wrapper), "::new(move |value: &mut i32| { calls += 1; *value += calls; *value });\n",
        "let mut chained = source.and_then(|value: &i32| *value * 2);\nlet mut input = 5;\n",
        "assert_eq!(chained.apply(&mut input), 12);\nassert_eq!(input, 6);\n```"
    ) };
    (@doc_then_once $wrapper:ident, $source_trait:literal, $chained_trait:ident) => { concat!(
        "```rust\nuse std::sync::Mutex;\nuse qubit_function::{", stringify!($wrapper), ", ", $source_trait, "};\n",
        "// Source trait: ", $source_trait, "; chained callback trait: ", stringify!($chained_trait), ".\n",
        "let source_resource = Mutex::new(7);\nlet source = ", stringify!($wrapper), "::new(move |value: &i32| *value + source_resource.into_inner().unwrap());\n",
        "let mapper_resource = Mutex::new(3);\nlet chained = source.and_then(move |value: &i32| *value + mapper_resource.into_inner().unwrap());\n",
        "assert_eq!(chained.apply(&5), 15);\n```"
    ) };
    (@doc_then_mutating_once $wrapper:ident, $source_trait:literal, $chained_trait:ident) => { concat!(
        "```rust\nuse std::sync::Mutex;\nuse qubit_function::{", stringify!($wrapper), ", ", $source_trait, "};\n",
        "// Source trait: ", $source_trait, "; chained callback trait: ", stringify!($chained_trait), ".\n",
        "let source_resource = Mutex::new(7);\nlet source = ", stringify!($wrapper), "::new(move |value: &mut i32| { *value += source_resource.into_inner().unwrap(); *value });\n",
        "let mapper_resource = Mutex::new(3);\nlet chained = source.and_then(move |value: &i32| *value + mapper_resource.into_inner().unwrap());\n",
        "let mut input = 5;\nassert_eq!(chained.apply(&mut input), 15);\nassert_eq!(input, 12);\n```"
    ) };
    (@doc_then_bi $wrapper:ident, $source_trait:literal, $chained_trait:ident) => { concat!(
        "```rust\nuse qubit_function::{", stringify!($wrapper), ", ", $source_trait, "};\n",
        "// Source trait: ", $source_trait, "; chained callback trait: ", stringify!($chained_trait), ".\n",
        "let source = ", stringify!($wrapper), "::new(|first: &i32, second: &i32| first + second);\n",
        "let chained = source.and_then(|value: &i32| *value * 2);\nassert_eq!(chained.apply(&2, &3), 10);\n```"
    ) };
    (@doc_then_bi_once $wrapper:ident, $source_trait:literal, $chained_trait:ident) => { concat!(
        "```rust\nuse std::sync::Mutex;\nuse qubit_function::{", stringify!($wrapper), ", ", $source_trait, "};\n",
        "// Source trait: ", $source_trait, "; chained callback trait: ", stringify!($chained_trait), ".\n",
        "let source_resource = Mutex::new(7);\nlet source = ", stringify!($wrapper), "::new(move |first: &i32, second: &i32| first + second + source_resource.into_inner().unwrap());\n",
        "let mapper_resource = Mutex::new(3);\nlet chained = source.and_then(move |value: &i32| *value + mapper_resource.into_inner().unwrap());\n",
        "assert_eq!(chained.apply(&2, &3), 15);\n```"
    ) };
    (@doc_then_bi_mutating $wrapper:ident, $source_trait:literal, $chained_trait:ident) => { concat!(
        "```rust\nuse qubit_function::{", stringify!($wrapper), ", ", $source_trait, "};\n",
        "// Source trait: ", $source_trait, "; chained callback trait: ", stringify!($chained_trait), ".\n",
        "let source = ", stringify!($wrapper), "::new(|first: &mut i32, second: &mut i32| { *first += 1; *second += 2; *first + *second });\n",
        "let chained = source.and_then(|value: &mut i32| { *value *= 2; *value });\nlet (mut first, mut second) = (2, 3);\n",
        "assert_eq!(chained.apply(&mut first, &mut second), 16);\nassert_eq!((first, second), (3, 5));\n```"
    ) };
    (@doc_then_bi_mutating_once $wrapper:ident, $source_trait:literal, $chained_trait:ident) => { concat!(
        "```rust\nuse std::sync::Mutex;\nuse qubit_function::{", stringify!($wrapper), ", ", $source_trait, "};\n",
        "// Source trait: ", $source_trait, "; chained callback trait: ", stringify!($chained_trait), ".\n",
        "let source_resource = Mutex::new(7);\nlet source = ", stringify!($wrapper), "::new(move |first: &mut i32, second: &mut i32| { *first += source_resource.into_inner().unwrap(); *second += 2; *first + *second });\n",
        "let mapper_resource = Mutex::new(3);\nlet chained = source.and_then(move |value: &mut i32| { *value += mapper_resource.into_inner().unwrap(); *value });\n",
        "let (mut first, mut second) = (2, 3);\nassert_eq!(chained.apply(&mut first, &mut second), 17);\nassert_eq!((first, second), (9, 5));\n```"
    ) };
    (@let_before BoxStatefulFunction, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_before BoxStatefulMutatingFunction, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_before $struct_name:ident, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_after StatefulFunction, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_after StatefulMutatingFunction, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_after $function_trait:ident, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@apply_after Function, $after:ident, $value:expr) => {{
        let value = $value;
        $after.apply(&value)
    }};

    (@apply_after FunctionOnce, $after:ident, $value:expr) => {{
        let value = $value;
        $after.apply(&value)
    }};

    (@apply_after StatefulFunction, $after:ident, $value:expr) => {{
        let value = $value;
        $after.apply(&value)
    }};

    (@apply_after MutatingFunction, $after:ident, $value:expr) => {{
        let mut value = $value;
        $after.apply(&mut value)
    }};

    (@apply_after MutatingFunctionOnce, $after:ident, $value:expr) => {{
        let mut value = $value;
        $after.apply(&mut value)
    }};

    (@apply_after StatefulMutatingFunction, $after:ident, $value:expr) => {{
        let mut value = $value;
        $after.apply(&mut value)
    }};

    // Two generic parameters - Function
    (
        $struct_name:ident < $t:ident, $r:ident >,
        $conditional_type:ident,
        $chained_function_trait:ident
    ) => {
        /// Creates a conditional function that executes based on predicate
        /// result.
        ///
        /// # Type Parameters
        ///
        /// * `P` - The predicate type that determines whether to execute the
        ///   function.
        ///
        /// # Parameters
        ///
        /// * `predicate` - The predicate to determine whether to execute
        ///   the function operation
        ///
        /// # Returns
        ///
        /// Returns a conditional function that only executes when the
        /// predicate returns `true`.
        ///
        /// # Examples
        ///
        #[doc = impl_box_function_methods!(@doc_when $struct_name)]
        #[inline]
        pub fn when<P>(self, predicate: P) -> $conditional_type<$t, $r>
        where
            $t: 'static,
            $r: 'static,
            P: Predicate<$t> + 'static,
        {
            $conditional_type {
                function: self,
                predicate: $crate::BoxPredicate::new(predicate),
            }
        }

        /// Chains execution with another function, executing the current
        /// function first, then the subsequent function.
        ///
        /// # Type Parameters
        ///
        /// * `S` - The result type produced by the subsequent function.
        /// * `F` - The function to execute after the current function.
        ///
        /// # Parameters
        ///
        /// * `after` - The subsequent function to execute after the current
        ///   function completes
        ///
        /// # Returns
        ///
        /// Returns a new function that executes the current function and
        /// the subsequent function in sequence.
        ///
        /// # Examples
        ///
        #[doc = impl_box_function_methods!(@doc_then $struct_name, $chained_function_trait)]
        #[inline]
        pub fn and_then<S, F>(self, after: F) -> $struct_name<$t, S>
        where
            $t: 'static,
            $r: 'static,
            S: 'static,
            F: $chained_function_trait<$r, S> + 'static,
        {
            impl_box_function_methods!(@let_before $struct_name, before, self.function);
            impl_box_function_methods!(@let_after $chained_function_trait, after, after);
            $crate::functions::macros::impl_function_new_callback!($chained_function_trait, $struct_name, $t, |t| {
                impl_box_function_methods!(@apply_after $chained_function_trait, after, before(t))
            })
        }
    };

    // Three generic parameters - BiFunction
    (
        $struct_name:ident < $t:ident, $u:ident, $r:ident >,
        $conditional_type:ident,
        $chained_function_trait:ident
    ) => {
        /// Creates a conditional two-parameter function that executes based
        /// on bi-predicate result.
        ///
        /// # Type Parameters
        ///
        /// * `P` - The bi-predicate type that determines whether to execute
        ///   the function.
        ///
        /// # Parameters
        ///
        /// * `predicate` - The bi-predicate to determine whether to execute
        ///   the function operation
        ///
        /// # Returns
        ///
        /// Returns a conditional two-parameter function that only executes
        /// when the predicate returns `true`.
        ///
        /// # Examples
        ///
        #[doc = impl_box_function_methods!(@doc_when $struct_name)]
        #[inline]
        pub fn when<P>(self, predicate: P) -> $conditional_type<$t, $u, $r>
        where
            $t: 'static,
            $u: 'static,
            $r: 'static,
            P: BiPredicate<$t, $u> + 'static,
        {
            $conditional_type {
                function: self,
                predicate: $crate::BoxBiPredicate::new(predicate),
            }
        }

        /// Chains execution with another two-parameter function, executing
        /// the current function first, then the subsequent function.
        ///
        /// # Type Parameters
        ///
        /// * `S` - The result type produced by the subsequent function.
        /// * `F` - The function to execute after the current function.
        ///
        /// # Parameters
        ///
        /// * `after` - The subsequent one-parameter function to execute after
        ///   the current function completes
        ///
        /// # Returns
        ///
        /// Returns a new two-parameter function that executes the current
        /// function and the subsequent function in sequence.
        ///
        /// # Examples
        ///
        #[doc = impl_box_function_methods!(@doc_then $struct_name, $chained_function_trait)]
        #[inline]
        pub fn and_then<S, F>(self, after: F) -> $struct_name<$t, $u, S>
        where
            $t: 'static,
            $u: 'static,
            $r: 'static,
            S: 'static,
            F: $chained_function_trait<$r, S> + 'static,
        {
            impl_box_function_methods!(@let_before $struct_name, before, self.function);
            impl_box_function_methods!(@let_after $chained_function_trait, after, after);
            $crate::functions::macros::impl_function_new_callback!($chained_function_trait, $struct_name, $t, $u, |t, u| {
                impl_box_function_methods!(@apply_after $chained_function_trait, after, before(t, u))
            })
        }
    };
}

pub(crate) use impl_box_function_methods;
