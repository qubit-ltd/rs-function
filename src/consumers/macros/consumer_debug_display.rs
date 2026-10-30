// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Consumer Debug Display Macro
//!
//! Generates standard `Debug` and `Display` implementations for consumer
//! wrappers. `Debug` shows the optional metadata name and a `"<function>"`
//! placeholder; `Display` shows the wrapper name and its diagnostic name.
//!
//! # Parameters
//!
//! * `$struct_name` - The struct name
//! * `$generic` - Generic parameter list (one or more type parameters)
//!
//! # Example
//!
//! ```rust
//! use qubit_function::BoxConsumer;
//!
//! let consumer = BoxConsumer::new_with_name("observe", |_: &i32| {});
//! assert_eq!(format!("{consumer}"), "BoxConsumer(observe)");
//! assert_eq!(
//!     format!("{consumer:?}"),
//!     "BoxConsumer { name: Some(\"observe\"), function: \"<function>\" }"
//! );
//! ```

/// Generates `Debug` and `Display` implementations for consumer wrappers.
///
/// Use this macro at module scope to generate complete implementations for
/// the specified struct.
///
/// # Parameters
///
/// * `$struct_name` - The struct name
/// * `$t` - Generic parameter list (one or more type parameters)
/// * `$u` - Generic parameter list (one or more type parameters)
///
/// # Example
///
/// ```rust
/// use qubit_function::BoxConsumer;
///
/// let consumer = BoxConsumer::new_with_name("observe", |_: &i32| {});
/// assert_eq!(format!("{consumer}"), "BoxConsumer(observe)");
/// assert_eq!(
///     format!("{consumer:?}"),
///     "BoxConsumer { name: Some(\"observe\"), function: \"<function>\" }"
/// );
/// ```

macro_rules! impl_consumer_debug_display {
    // Single generic parameter - Consumer types
    ($struct_name:ident < $t:ident >) => {
        impl<$t> std::fmt::Debug for $struct_name<$t> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($struct_name))
                    .field("name", &self.metadata.name())
                    .field("function", &"<function>")
                    .finish()
            }
        }

        impl<$t> std::fmt::Display for $struct_name<$t> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self.metadata.name() {
                    Some(name) => {
                        write!(f, "{}({})", stringify!($struct_name), name)
                    }
                    None => write!(f, "{}", stringify!($struct_name)),
                }
            }
        }
    };
    // Two generic parameters - BiConsumer types
    ($struct_name:ident < $t:ident, $u:ident >) => {
        impl<$t, $u> std::fmt::Debug for $struct_name<$t, $u> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($struct_name))
                    .field("name", &self.metadata.name())
                    .field("function", &"<function>")
                    .finish()
            }
        }

        impl<$t, $u> std::fmt::Display for $struct_name<$t, $u> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self.metadata.name() {
                    Some(name) => {
                        write!(f, "{}({})", stringify!($struct_name), name)
                    }
                    None => write!(f, "{}", stringify!($struct_name)),
                }
            }
        }
    };
}

pub(crate) use impl_consumer_debug_display;
