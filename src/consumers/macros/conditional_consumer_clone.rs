/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Conditional Consumer Clone Macro
//!
//! Generates Clone trait implementation for Conditional Consumer types
//!
//! Generates Clone implementation for Conditional Consumer structs that have
//! `consumer` and `predicate` fields. Both fields are cloned using their
//! respective Clone implementations.
//!
//! # Parameters
//!
//! * `$struct_name` - The struct name
//! * `$generic` - Generic parameter list (one or two type parameters)
//!
//! # Examples
//!
//! ```rust
//! use qubit_function::consumers::{ArcConditionalBiConsumer, RcConditionalBiConsumer};
//! use qubit_function::consumers::{ArcConditionalConsumer, RcConditionalConsumer};
//! macro_rules! impl_conditional_consumer_clone {
//!     ($struct_name:ident<$t:ident>) => {
//!         let _ = std::marker::PhantomData::<$t>;
//!         let _ = std::any::TypeId::of::<$struct_name<$t>>();
//!     };
//!     ($struct_name:ident<$t:ident, $u:ident>) => {
//!         let _ = std::marker::PhantomData::<($t, $u)>;
//!         let _ = std::any::TypeId::of::<$struct_name<$t, $u>>();
//!     };
//! }
//! // For single type parameter
//! impl_conditional_consumer_clone!(ArcConditionalConsumer<i32>);
//! impl_conditional_consumer_clone!(RcConditionalConsumer<i32>);
//!
//! // For two type parameters
//! impl_conditional_consumer_clone!(ArcConditionalBiConsumer<i32, i32>);
//! impl_conditional_consumer_clone!(RcConditionalBiConsumer<i32, i32>);
//! ```
//!

/// Generates Clone trait implementation for Conditional Consumer types
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete `impl Clone for $struct_name` block. Generates
/// Clone implementation for Conditional Consumer structs that have `consumer`
/// and `predicate` fields. Both fields are cloned using their respective
/// Clone implementations.
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
/// use qubit_function::consumers::{ArcConditionalBiConsumer, RcConditionalBiConsumer};
/// use qubit_function::consumers::{ArcConditionalConsumer, RcConditionalConsumer};
/// use std::marker::PhantomData;
/// macro_rules! impl_conditional_consumer_clone {
///     ($struct_name:ident<$t:ident>) => {
///         let _ = PhantomData::<$t>;
///     };
///     ($struct_name:ident<$t:ident, $u:ident>) => {
///         let _ = PhantomData::<($t, $u)>;
///     };
/// }
/// // For single type parameter
/// impl_conditional_consumer_clone!(ArcConditionalConsumer<i32>);
/// impl_conditional_consumer_clone!(RcConditionalConsumer<i32>);
///
/// // For two type parameters
/// impl_conditional_consumer_clone!(ArcConditionalBiConsumer<i32, i32>);
/// impl_conditional_consumer_clone!(RcConditionalBiConsumer<i32, i32>);
/// ```
///
///
macro_rules! impl_conditional_consumer_clone {
    // Single generic parameter - Consumer types
    ($struct_name:ident < $t:ident >) => {
        impl<$t> Clone for $struct_name<$t> {
            fn clone(&self) -> Self {
                Self {
                    consumer: self.consumer.clone(),
                    predicate: self.predicate.clone(),
                }
            }
        }
    };
    // Two generic parameters - BiConsumer types
    ($struct_name:ident < $t:ident, $u:ident >) => {
        impl<$t, $u> Clone for $struct_name<$t, $u> {
            fn clone(&self) -> Self {
                Self {
                    consumer: self.consumer.clone(),
                    predicate: self.predicate.clone(),
                }
            }
        }
    };
}

pub(crate) use impl_conditional_consumer_clone;
