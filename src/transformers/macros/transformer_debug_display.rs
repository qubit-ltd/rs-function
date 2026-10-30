// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
/// Generates Debug and Display trait implementations for Transformer structs
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates complete `impl Debug` and `impl Display` blocks for the
/// specified struct. Generates standard Debug and Display trait implementations
/// for Transformer structs that have a `name: Option<String>` field.
///
/// # Parameters
///
/// * `$struct_name` - The struct name
/// * `$t` and `$r` - Type parameters for a transformer with two generic
///   parameters
/// * `$t`, `$u`, and `$r` - Type parameters for a transformer with three
///   generic parameters
///
/// # Usage
///
/// The following invocations show the supported generic arities:
///
/// ```text
/// // For two type parameters
/// impl_transformer_debug_display!(BoxTransformer<T, R>);
///
/// // For three type parameters
/// impl_transformer_debug_display!(BoxBiTransformer<T, U, R>);
/// ```
macro_rules! impl_transformer_debug_display {
    // Two generic parameters
    ($struct_name:ident < $t:ident, $r:ident >) => {
        impl<$t, $r> std::fmt::Debug for $struct_name<$t, $r> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($struct_name))
                    .field("name", &self.metadata.name())
                    .field("function", &"<function>")
                    .finish()
            }
        }

        impl<$t, $r> std::fmt::Display for $struct_name<$t, $r> {
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

    // Three generic parameters
    ($struct_name:ident < $t:ident, $u:ident, $r:ident >) => {
        impl<$t, $u, $r> std::fmt::Debug for $struct_name<$t, $u, $r> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($struct_name))
                    .field("name", &self.metadata.name())
                    .field("function", &"<function>")
                    .finish()
            }
        }

        impl<$t, $u, $r> std::fmt::Display for $struct_name<$t, $u, $r> {
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

pub(crate) use impl_transformer_debug_display;
