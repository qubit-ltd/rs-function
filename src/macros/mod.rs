// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! # Macros Module
//!
//! Common macro definitions for the function library.

#[cfg(feature = "once")]
pub mod closure_once_trait;
pub mod closure_trait;
pub mod common_name_methods;
pub mod common_new_methods;

// Re-export macros for easier use
#[cfg(feature = "once")]
pub(crate) use closure_once_trait::impl_closure_once_trait;
pub(crate) use closure_trait::impl_closure_trait;
pub(crate) use common_name_methods::impl_common_name_methods;
pub(crate) use common_new_methods::impl_common_new_methods;
