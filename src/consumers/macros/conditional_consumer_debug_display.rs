// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Conditional Consumer Debug Display Macro
//!
//! Generates standard `Debug` and `Display` implementations for conditional
//! consumer wrappers. Their output uses the stored `consumer` and `predicate`;
//! each inner wrapper formats its own metadata and callback placeholder.
//!
//! # Parameters
//!
//! * `$struct_name` - The struct name
//! * `$t` - The first type parameter
//! * `$u` - The second type parameter, when the struct has two parameters
//!
//! # Example
//!
//! ```rust
//! use qubit_function::{BoxConsumer, Consumer};
//!
//! let conditional = BoxConsumer::new(|_: &i32| {}).when(|value: &i32| *value > 0);
//! assert_eq!(
//!     format!("{conditional}"),
//!     "BoxConditionalConsumer(BoxConsumer, BoxPredicate(unnamed))"
//! );
//! assert_eq!(
//!     format!("{conditional:?}"),
//!     "BoxConditionalConsumer { consumer: BoxConsumer { name: None, function: \"<function>\" }, predicate: BoxPredicate { name: None, function: \"<function>\" } }"
//! );
//! conditional.accept(&1);
//! ```

/// Generates `Debug` and `Display` implementations for conditional consumer
/// wrappers.
///
/// Use this macro at module scope to generate complete implementations from
/// the wrapper's stored `consumer` and `predicate` fields.
///
/// # Parameters
///
/// * `$struct_name` - The struct name
/// * `$t` - The first type parameter
/// * `$u` - The second type parameter, when the struct has two parameters
///
/// # Example
///
/// ```rust
/// use qubit_function::{BoxConsumer, Consumer};
///
/// let conditional = BoxConsumer::new(|_: &i32| {}).when(|value: &i32| *value > 0);
/// assert_eq!(
///     format!("{conditional}"),
///     "BoxConditionalConsumer(BoxConsumer, BoxPredicate(unnamed))"
/// );
/// assert_eq!(
///     format!("{conditional:?}"),
///     "BoxConditionalConsumer { consumer: BoxConsumer { name: None, function: \"<function>\" }, predicate: BoxPredicate { name: None, function: \"<function>\" } }"
/// );
/// conditional.accept(&1);
/// ```

macro_rules! impl_conditional_consumer_debug_display {
    // Single generic parameter - Consumer types
    ($struct_name:ident < $t:ident >) => {
        impl<$t> std::fmt::Debug for $struct_name<$t> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($struct_name))
                    .field("consumer", &self.consumer)
                    .field("predicate", &self.predicate)
                    .finish()
            }
        }

        impl<$t> std::fmt::Display for $struct_name<$t> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}({}, {})",
                    stringify!($struct_name),
                    self.consumer,
                    self.predicate
                )
            }
        }
    };

    // Two generic parameters - BiConsumer types
    ($struct_name:ident < $t:ident, $u:ident >) => {
        impl<$t, $u> std::fmt::Debug for $struct_name<$t, $u> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($struct_name))
                    .field("consumer", &self.consumer)
                    .field("predicate", &self.predicate)
                    .finish()
            }
        }

        impl<$t, $u> std::fmt::Display for $struct_name<$t, $u> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}({}, {})",
                    stringify!($struct_name),
                    self.consumer,
                    self.predicate
                )
            }
        }
    };
}

pub(crate) use impl_conditional_consumer_debug_display;
