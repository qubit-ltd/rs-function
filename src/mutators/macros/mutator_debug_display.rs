// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Mutator Debug Display Macro
//!
//! Provides the macro that generates standard `Debug` and `Display`
//! implementations for mutator structs.

/// Generates Debug and Display trait implementations for Mutator structs
///
/// This macro should be used at the top level (outside of any impl block)
/// to generate Debug and Display trait implementations for Mutator structs.
/// The generated implementations read the display name from
/// `self.metadata.name()`. `Debug` also uses a `"<function>"` placeholder
/// instead of exposing the wrapped callback, while `Display` prints the type
/// name alone when no display name is set.
///
/// # Parameters
///
/// * `$struct_name` - The struct name
/// * `$generic` - Generic parameter list (one type parameter)
///
/// # Usage Location
///
/// This macro should be used at the top level, outside of any impl block,
/// typically in the same file as the struct definition.
///
/// # Example
///
/// Place the invocation at module scope, outside any `impl` block:
///
/// ```text
/// impl_mutator_debug_display!(BoxMutator<T>);
/// ```
macro_rules! impl_mutator_debug_display {
    // Single generic parameter
    ($struct_name:ident < $generic:ident >) => {
        impl<$generic> std::fmt::Debug for $struct_name<$generic> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($struct_name))
                    .field("name", &self.metadata.name())
                    .field("function", &"<function>")
                    .finish()
            }
        }

        impl<$generic> std::fmt::Display for $struct_name<$generic> {
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

pub(crate) use impl_mutator_debug_display;
