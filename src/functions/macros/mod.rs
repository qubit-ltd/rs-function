// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Function Macros Module
//!
//! Provides declarative macros to simplify Function implementations and
//! reduce code duplication.

// Module declarations
#[cfg(feature = "combinators")]
mod box_conditional_function;
mod box_function_methods;
#[cfg(feature = "combinators")]
mod conditional_function_clone;
#[cfg(feature = "combinators")]
mod conditional_function_debug_display;
#[cfg(feature = "combinators")]
mod fn_ops_trait;
mod function_clone;
mod function_common_methods;
mod function_constant_method;
mod function_debug_display;
mod function_identity_method;
#[cfg(feature = "combinators")]
mod shared_conditional_function;
mod shared_function_methods;

// Export all macros for use within the crate
#[cfg(feature = "combinators")]
pub(crate) use box_conditional_function::impl_box_conditional_function;
pub(crate) use box_function_methods::impl_box_function_methods;
#[cfg(feature = "combinators")]
pub(crate) use conditional_function_clone::impl_conditional_function_clone;
#[cfg(feature = "combinators")]
pub(crate) use conditional_function_debug_display::impl_conditional_function_debug_display;
#[cfg(feature = "combinators")]
pub(crate) use fn_ops_trait::impl_fn_ops_trait;
pub(crate) use function_clone::impl_function_clone;
pub(crate) use function_common_methods::impl_function_common_methods;
#[cfg(feature = "combinators")]
pub(crate) use function_common_methods::impl_function_new_callback;
pub(crate) use function_constant_method::impl_function_constant_method;
pub(crate) use function_debug_display::impl_function_debug_display;
pub(crate) use function_identity_method::impl_function_identity_method;
#[cfg(feature = "combinators")]
pub(crate) use shared_conditional_function::impl_shared_conditional_function;
pub(crate) use shared_function_methods::impl_shared_function_methods;
