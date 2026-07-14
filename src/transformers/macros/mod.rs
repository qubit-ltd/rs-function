// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Transformer Macros Module
//!
//! Provides declarative macros to simplify Transformer implementations and
//! reduce code duplication.

// Module declarations
#[cfg(feature = "combinators")]
mod box_conditional_transformer;
mod box_transformer_methods;
#[cfg(feature = "combinators")]
mod conditional_transformer_clone;
#[cfg(feature = "combinators")]
mod conditional_transformer_debug_display;
#[cfg(feature = "combinators")]
mod fn_ops_trait;
#[cfg(feature = "combinators")]
mod shared_conditional_transformer;
mod shared_transformer_methods;
mod transformer_clone;
mod transformer_common_methods;
mod transformer_constant_method;
mod transformer_debug_display;

// Export all macros for use within the crate
#[cfg(feature = "combinators")]
pub(crate) use box_conditional_transformer::impl_box_conditional_transformer;
pub(crate) use box_transformer_methods::impl_box_transformer_methods;
#[cfg(feature = "combinators")]
pub(crate) use conditional_transformer_clone::impl_conditional_transformer_clone;
#[cfg(feature = "combinators")]
pub(crate) use conditional_transformer_debug_display::impl_conditional_transformer_debug_display;
#[cfg(feature = "combinators")]
pub(crate) use fn_ops_trait::impl_transformer_fn_ops_trait;
#[cfg(feature = "combinators")]
pub(crate) use shared_conditional_transformer::impl_shared_conditional_transformer;
pub(crate) use shared_transformer_methods::impl_shared_transformer_methods;
pub(crate) use transformer_clone::impl_transformer_clone;
pub(crate) use transformer_common_methods::{
    impl_transformer_common_methods,
    impl_transformer_new_methods,
};
pub(crate) use transformer_constant_method::impl_transformer_constant_method;
pub(crate) use transformer_debug_display::impl_transformer_debug_display;
