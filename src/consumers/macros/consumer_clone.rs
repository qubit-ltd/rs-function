// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Consumer Clone Macro
//!
//! Generates Clone trait implementation for basic Consumer types
//!
//! Generates Clone implementation for Consumer structs that have `function`
//! and `metadata` fields. Both fields are cloned using their `Clone`
//! implementations. For smart pointers like `Arc` and `Rc`, cloning the
//! function field creates another shared reference to the callback.
//!
//! # Parameters
//!
//! * `$struct_name` - The struct name
//! * `$generic` - Generic parameter list (one or two type parameters)
//!
//! # Example
//!
//! Cloning an `ArcConsumer` shares its callback while each clone keeps its own
//! diagnostic name.
//!
//! ```rust
//! use std::sync::Arc;
//! use std::sync::atomic::AtomicUsize;
//! use std::sync::atomic::Ordering;
//!
//! use qubit_function::ArcConsumer;
//! use qubit_function::Consumer;
//!
//! let total = Arc::new(AtomicUsize::new(0));
//! let captured = Arc::clone(&total);
//! let original = ArcConsumer::new_with_name("original", move |value: &usize| {
//!     captured.fetch_add(*value, Ordering::SeqCst);
//! });
//! let mut cloned = original.clone();
//! cloned.set_name("cloned");
//! original.accept(&2);
//! cloned.accept(&3);
//! assert_eq!(total.load(Ordering::SeqCst), 5);
//! assert_eq!(original.name(), Some("original"));
//! assert_eq!(cloned.name(), Some("cloned"));
//! ```

/// Generates Clone trait implementation for basic Consumer types
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete `impl Clone for $struct_name` block. It generates
/// Clone implementation for Consumer structs that have `function` and
/// `metadata` fields. Both fields are cloned using their `Clone`
/// implementations. For smart pointers like `Arc` and `Rc`, cloning the
/// function field creates another shared reference to the callback.
///
/// # Parameters
///
/// * `$struct_name` - The struct name
/// * `$t` - The first type parameter identifier
/// * `$u` - The second type parameter identifier for bi-consumer types
///
/// # Internal invocation syntax
///
/// This macro is invoked outside the target type's `impl` block. The following
/// syntax illustrates its internal use; it is not a public macro invocation.
///
/// ```text
/// impl_consumer_clone!(ArcConsumer<T>);
/// impl_consumer_clone!(ArcBiConsumer<T, U>);
/// ```
macro_rules! impl_consumer_clone {
    // Single generic parameter - Consumer types
    ($struct_name:ident < $t:ident >) => {
        impl<$t> Clone for $struct_name<$t> {
            fn clone(&self) -> Self {
                Self {
                    function: self.function.clone(),
                    metadata: self.metadata.clone(),
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
                    metadata: self.metadata.clone(),
                }
            }
        }
    };
}

pub(crate) use impl_consumer_clone;
