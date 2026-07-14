// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Mutator Macros Module
//!
//! Provides declarative macros to simplify Mutator implementations and
//! reduce code duplication.

// Module declarations
#[cfg(feature = "combinators")]
mod box_conditional_mutator;
mod box_mutator_methods;
#[cfg(feature = "combinators")]
mod conditional_mutator_clone;
#[cfg(feature = "combinators")]
mod conditional_mutator_debug_display;
mod mutator_clone;
mod mutator_common_methods;
mod mutator_debug_display;
#[cfg(feature = "combinators")]
mod shared_conditional_mutator;
mod shared_mutator_methods;

// Export all macros for use within the crate
#[cfg(feature = "combinators")]
pub(crate) use box_conditional_mutator::impl_box_conditional_mutator;
pub(crate) use box_mutator_methods::impl_box_mutator_methods;
#[cfg(feature = "combinators")]
pub(crate) use conditional_mutator_clone::impl_conditional_mutator_clone;
#[cfg(feature = "combinators")]
pub(crate) use conditional_mutator_debug_display::impl_conditional_mutator_debug_display;
pub(crate) use mutator_clone::impl_mutator_clone;
pub(crate) use mutator_common_methods::{
    impl_mutator_common_methods,
    impl_mutator_new_methods,
};
pub(crate) use mutator_debug_display::impl_mutator_debug_display;
#[cfg(feature = "combinators")]
pub(crate) use shared_conditional_mutator::impl_shared_conditional_mutator;
pub(crate) use shared_mutator_methods::impl_shared_mutator_methods;
