// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Functions Module
//!
//! This module provides function-related functional programming abstractions
//! for transforming values from one type to another with reference semantics.

pub mod bi_function;
#[cfg(feature = "once")]
pub mod bi_function_once;
pub mod bi_mutating_function;
#[cfg(feature = "once")]
pub mod bi_mutating_function_once;
pub mod function;
#[cfg(feature = "once")]
pub mod function_once;
pub(crate) mod macros;
pub mod mutating_function;
#[cfg(feature = "once")]
pub mod mutating_function_once;
#[cfg(feature = "stateful")]
pub mod stateful_function;
#[cfg(feature = "stateful")]
pub mod stateful_mutating_function;

pub use bi_function::ArcBiFunction;
pub use bi_function::ArcBinaryFunction;
pub use bi_function::ArcConditionalBiFunction;
pub use bi_function::BiFunction;
pub use bi_function::BoxBiFunction;
pub use bi_function::BoxBinaryFunction;
pub use bi_function::BoxConditionalBiFunction;
#[cfg(feature = "rc")]
pub use bi_function::RcBiFunction;
#[cfg(feature = "rc")]
pub use bi_function::RcBinaryFunction;
#[cfg(feature = "rc")]
pub use bi_function::RcConditionalBiFunction;
#[cfg(feature = "once")]
pub use bi_function_once::BiFunctionOnce;
#[cfg(feature = "once")]
pub use bi_function_once::BoxBiFunctionOnce;
#[cfg(feature = "once")]
pub use bi_function_once::BoxConditionalBiFunctionOnce;
pub use bi_mutating_function::ArcBiMutatingFunction;
pub use bi_mutating_function::ArcBinaryMutatingFunction;
pub use bi_mutating_function::ArcConditionalBiMutatingFunction;
pub use bi_mutating_function::BiMutatingFunction;
pub use bi_mutating_function::BoxBiMutatingFunction;
pub use bi_mutating_function::BoxBinaryMutatingFunction;
pub use bi_mutating_function::BoxConditionalBiMutatingFunction;
#[cfg(feature = "rc")]
pub use bi_mutating_function::RcBiMutatingFunction;
#[cfg(feature = "rc")]
pub use bi_mutating_function::RcBinaryMutatingFunction;
#[cfg(feature = "rc")]
pub use bi_mutating_function::RcConditionalBiMutatingFunction;
#[cfg(feature = "once")]
pub use bi_mutating_function_once::BiMutatingFunctionOnce;
#[cfg(feature = "once")]
pub use bi_mutating_function_once::BoxBiMutatingFunctionOnce;
#[cfg(feature = "once")]
pub use bi_mutating_function_once::BoxConditionalBiMutatingFunctionOnce;
pub use function::ArcConditionalFunction;
pub use function::ArcFunction;
pub use function::BoxConditionalFunction;
pub use function::BoxFunction;
pub use function::Function;
#[cfg(feature = "rc")]
pub use function::RcConditionalFunction;
#[cfg(feature = "rc")]
pub use function::RcFunction;
#[cfg(feature = "once")]
pub use function_once::BoxConditionalFunctionOnce;
#[cfg(feature = "once")]
pub use function_once::BoxFunctionOnce;
#[cfg(feature = "once")]
pub use function_once::FunctionOnce;
pub use mutating_function::ArcConditionalMutatingFunction;
pub use mutating_function::ArcMutatingFunction;
pub use mutating_function::BoxConditionalMutatingFunction;
pub use mutating_function::BoxMutatingFunction;
pub use mutating_function::MutatingFunction;
#[cfg(feature = "rc")]
pub use mutating_function::RcConditionalMutatingFunction;
#[cfg(feature = "rc")]
pub use mutating_function::RcMutatingFunction;
#[cfg(feature = "once")]
pub use mutating_function_once::BoxConditionalMutatingFunctionOnce;
#[cfg(feature = "once")]
pub use mutating_function_once::BoxMutatingFunctionOnce;
#[cfg(feature = "once")]
pub use mutating_function_once::MutatingFunctionOnce;
#[cfg(feature = "stateful")]
pub use stateful_function::ArcConditionalStatefulFunction;
#[cfg(feature = "stateful")]
pub use stateful_function::ArcStatefulFunction;
#[cfg(feature = "stateful")]
pub use stateful_function::BoxConditionalStatefulFunction;
#[cfg(feature = "stateful")]
pub use stateful_function::BoxStatefulFunction;
#[cfg(all(feature = "stateful", feature = "rc"))]
pub use stateful_function::RcConditionalStatefulFunction;
#[cfg(all(feature = "stateful", feature = "rc"))]
pub use stateful_function::RcStatefulFunction;
#[cfg(feature = "stateful")]
pub use stateful_function::StatefulFunction;
#[cfg(feature = "stateful")]
pub use stateful_mutating_function::ArcConditionalStatefulMutatingFunction;
#[cfg(feature = "stateful")]
pub use stateful_mutating_function::ArcStatefulMutatingFunction;
#[cfg(feature = "stateful")]
pub use stateful_mutating_function::BoxConditionalStatefulMutatingFunction;
#[cfg(feature = "stateful")]
pub use stateful_mutating_function::BoxStatefulMutatingFunction;
#[cfg(all(feature = "stateful", feature = "rc"))]
pub use stateful_mutating_function::RcConditionalStatefulMutatingFunction;
#[cfg(all(feature = "stateful", feature = "rc"))]
pub use stateful_mutating_function::RcStatefulMutatingFunction;
#[cfg(feature = "stateful")]
pub use stateful_mutating_function::StatefulMutatingFunction;
