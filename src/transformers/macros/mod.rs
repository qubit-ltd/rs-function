/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Transformer Macros Module
//!
//! Provides declarative macros to simplify Transformer implementations and
//! reduce code duplication.
//!

// Module declarations
mod box_conditional_transformer;
mod box_transformer_methods;
mod conditional_transformer_clone;
mod conditional_transformer_debug_display;
mod shared_conditional_transformer;
mod shared_transformer_methods;
mod transformer_clone;
mod transformer_common_methods;
mod transformer_constant_method;
mod transformer_debug_display;

// Export all macros for use within the crate
pub(crate) use box_conditional_transformer::impl_box_conditional_transformer;
pub(crate) use box_transformer_methods::impl_box_transformer_methods;
pub(crate) use conditional_transformer_clone::impl_conditional_transformer_clone;
pub(crate) use conditional_transformer_debug_display::impl_conditional_transformer_debug_display;
pub(crate) use shared_conditional_transformer::impl_shared_conditional_transformer;
pub(crate) use shared_transformer_methods::impl_shared_transformer_methods;
pub(crate) use transformer_clone::impl_transformer_clone;
pub(crate) use transformer_common_methods::impl_transformer_common_methods;
pub(crate) use transformer_constant_method::impl_transformer_constant_method;
pub(crate) use transformer_debug_display::impl_transformer_debug_display;
