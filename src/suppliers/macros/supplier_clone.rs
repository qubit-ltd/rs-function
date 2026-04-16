/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Supplier Clone Macro
//!
//! Generates Clone trait implementation for basic Supplier types
//!
//! Generates Clone implementation for Supplier structs that have `function`
//! and `name` fields. The function field is cloned using its inherent `clone`
//! method, which performs a shallow clone for smart pointers like `Arc` or `Rc`.
//!
//! # Parameters
//!
//! * `$struct_name` - The struct name
//! * `$generic` - Generic parameter list (one type parameter)
//!
//! # Examples
//!
//! ```rust
//! use qubit_function::{ArcStatefulSupplier, ArcSupplier, RcStatefulSupplier, RcSupplier, Supplier};
//!
//! let arc = ArcSupplier::new(|| 1);
//! let arc_clone = arc.clone();
//! assert_eq!(arc_clone.get(), 1);
//!
//! let rc = RcSupplier::new(|| "ok".to_string());
//! let rc_clone = rc.clone();
//! assert_eq!(rc_clone.get(), "ok".to_string());
//!
//! let stateful_arc = ArcStatefulSupplier::new(|| 1);
//! let _stateful_arc_clone = stateful_arc.clone();
//!
//! let stateful_rc = RcStatefulSupplier::new(|| 1);
//! let _stateful_rc_clone = stateful_rc.clone();
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates Clone trait implementation for basic Supplier types
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete `impl Clone for $struct_name` block. It generates
/// Clone implementation for Supplier structs that have `function` and `name`
/// fields. The function field is cloned using its inherent `clone` method,
/// which performs a shallow clone for smart pointers like `Arc` or `Rc`.
///
/// # Parameters
///
/// * `$struct_name` - The struct name
/// * `$t` - Generic parameter list (one type parameter)
///
/// # Examples
///
/// ```rust
/// use qubit_function::{ArcStatefulSupplier, ArcSupplier, RcStatefulSupplier, RcSupplier, Supplier};
///
/// let arc = ArcSupplier::new(|| 1);
/// let arc_clone = arc.clone();
/// assert_eq!(arc_clone.get(), 1);
///
/// let rc = RcSupplier::new(|| "ok".to_string());
/// let rc_clone = rc.clone();
/// assert_eq!(rc_clone.get(), "ok".to_string());
///
/// let stateful_arc = ArcStatefulSupplier::new(|| 1);
/// let _stateful_arc_clone = stateful_arc.clone();
///
/// let stateful_rc = RcStatefulSupplier::new(|| 1);
/// let _stateful_rc_clone = stateful_rc.clone();
/// ```
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_supplier_clone {
    // Single generic parameter
    ($struct_name:ident < $t:ident >) => {
        impl<$t> Clone for $struct_name<$t> {
            fn clone(&self) -> Self {
                Self {
                    function: self.function.clone(),
                    name: self.name.clone(),
                }
            }
        }
    };
}

pub(crate) use impl_supplier_clone;
