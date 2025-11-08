/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Macros Module
//!
//! Common macro definitions for the function library.
//!
//! # Author
//!
//! Haixing Hu

pub mod box_into_conversions;
pub mod common_new_methods;
pub mod common_name_methods;

// Re-export macros for easier use
pub(crate) use box_into_conversions::impl_box_into_conversions;
pub(crate) use common_new_methods::impl_common_new_methods;
pub(crate) use common_name_methods::impl_common_name_methods;
