// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Implements formatting macros used by predicate types.

/// Implements `Debug` and `Display` for a predicate type with metadata.
///
/// Invoke this macro at module scope for a predicate type whose `metadata`
/// field provides a `name()` method. The debug output includes the type name,
/// metadata name, and a placeholder for the wrapped function. The display
/// output includes the type name and metadata name, or `unnamed` when no name
/// is set.
///
/// # Parameters
///
/// * `$struct_name` is the predicate type name.
/// * `$t` and optional `$u` are its one or two generic type parameters.
macro_rules! impl_predicate_debug_display {
    // Single generic parameter
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
                    None => {
                        write!(f, "{}({})", stringify!($struct_name), "unnamed")
                    }
                }
            }
        }
    };
    // Two generic parameters
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
                    None => {
                        write!(f, "{}({})", stringify!($struct_name), "unnamed")
                    }
                }
            }
        }
    };
}

pub(crate) use impl_predicate_debug_display;
