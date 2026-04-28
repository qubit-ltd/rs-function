/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Consumer Clone Macro
//!
//! Generates Clone trait implementation for basic Consumer types
//!
//! Generates Clone implementation for Consumer structs that have `function`
//! and `name` fields. The function field is cloned using its inherent `clone`
//! method, which performs a shallow clone for smart pointers like `Arc` or `Rc`.
//!
//! # Parameters
//!
//! * `$struct_name` - The struct name
//! * `$generic` - Generic parameter list (one or two type parameters)
//!
//! # Examples
//!
//! ```rust
//! // For single type parameter
//! use qubit_function::{ArcConsumer, RcConsumer};
//! use qubit_function::consumers::{ArcBiConsumer, RcBiConsumer};
//! macro_rules! impl_consumer_clone {
//!     ($struct_name:ident<$t:ident>) => {
//!         let _ = core::marker::PhantomData::<$t>;
//!         let _ = stringify!($struct_name);
//!     };
//!     ($struct_name:ident<$t:ident, $u:ident>) => {
//!         let _ = core::marker::PhantomData::<($t, $u)>;
//!         let _ = stringify!($struct_name);
//!     };
//! }
//! impl_consumer_clone!(ArcConsumer<i32>);
//!
//! // For single type parameter with Rc
//! impl_consumer_clone!(RcConsumer<i32>);
//!
//! // For two type parameters
//! impl_consumer_clone!(ArcBiConsumer<i32, i32>);
//!
//! // For two type parameters with Rc
//! impl_consumer_clone!(RcBiConsumer<i32, i32>);
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates Clone trait implementation for basic Consumer types
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete `impl Clone for $struct_name` block. It generates
/// Clone implementation for Consumer structs that have `function` and `name`
/// fields. The function field is cloned using its inherent `clone` method,
/// which performs a shallow clone for smart pointers like `Arc` or `Rc`.
///
/// # Parameters
///
/// * `$struct_name` - The struct name
/// * `$t` - Generic parameter list (one or two type parameters)
/// * `$u` - Generic parameter list (one or two type parameters)
///
/// # Examples
///
/// ```rust
/// // For single type parameter with Arc
/// use qubit_function::{ArcConsumer, RcConsumer};
/// use qubit_function::consumers::{ArcBiConsumer, RcBiConsumer};
/// macro_rules! impl_consumer_clone {
///     ($struct_name:ident<$t:ident>) => {
///         let _ = core::marker::PhantomData::<$t>;
///         let _ = stringify!($struct_name);
///     };
///     ($struct_name:ident<$t:ident, $u:ident>) => {
///         let _ = core::marker::PhantomData::<($t, $u)>;
///         let _ = stringify!($struct_name);
///     };
/// }
/// impl_consumer_clone!(ArcConsumer<i32>);
///
/// // For single type parameter with Rc
/// impl_consumer_clone!(RcConsumer<i32>);
///
/// // For two type parameters with Arc
/// impl_consumer_clone!(ArcBiConsumer<i32, i32>);
///
/// // For two type parameters with Rc
/// impl_consumer_clone!(RcBiConsumer<i32, i32>);
/// ```
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_consumer_clone {
    // Single generic parameter - Consumer types
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
    // Two generic parameters - BiConsumer types
    ($struct_name:ident < $t:ident, $u:ident >) => {
        impl<$t, $u> Clone for $struct_name<$t, $u> {
            fn clone(&self) -> Self {
                Self {
                    function: self.function.clone(),
                    name: self.name.clone(),
                }
            }
        }
    };
}

pub(crate) use impl_consumer_clone;
