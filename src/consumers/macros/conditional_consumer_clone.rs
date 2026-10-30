// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
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
//! # Expansion Shape
//!
//! The macro is an internal implementation detail. For a conditional consumer
//! with one type parameter, it generates an implementation equivalent to:
//!
//! ```text
//! impl<T> Clone for ConditionalConsumer<T> {
//!     fn clone(&self) -> Self {
//!         Self {
//!             consumer: self.consumer.clone(),
//!             predicate: self.predicate.clone(),
//!         }
//!     }
//! }
//! ```

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
/// # Expansion Shape
///
/// For a single type parameter, the generated implementation clones both
/// fields:
///
/// ```text
/// impl<T> Clone for ConditionalConsumer<T> {
///     fn clone(&self) -> Self {
///         Self {
///             consumer: self.consumer.clone(),
///             predicate: self.predicate.clone(),
///         }
///     }
/// }
/// ```
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
