/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Supplier Macros Module
//!
//! Provides declarative macros to simplify Supplier implementations and
//! reduce code duplication.
//!
//! # Author
//!
//! Haixing Hu

// Module declarations
mod box_conditional_supplier;
mod box_supplier_methods;
mod conditional_supplier_clone;
mod conditional_supplier_conversions;
mod conditional_supplier_debug_display;
mod shared_conditional_supplier;
mod shared_supplier_methods;
mod supplier_clone;
mod supplier_common_methods;
mod supplier_debug_display;

// Export all macros for use within the crate
pub(crate) use box_conditional_supplier::impl_box_conditional_supplier;
pub(crate) use box_supplier_methods::impl_box_supplier_methods;
pub(crate) use conditional_supplier_clone::impl_conditional_supplier_clone;
pub(crate) use conditional_supplier_conversions::impl_conditional_supplier_conversions;
pub(crate) use conditional_supplier_debug_display::impl_conditional_supplier_debug_display;
pub(crate) use shared_conditional_supplier::impl_shared_conditional_supplier;
pub(crate) use shared_supplier_methods::impl_shared_supplier_methods;
pub(crate) use supplier_clone::impl_supplier_clone;
pub(crate) use supplier_common_methods::impl_supplier_common_methods;
pub(crate) use supplier_debug_display::impl_supplier_debug_display;
