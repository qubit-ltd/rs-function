/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Conditional Transformer Debug Display Macro
//!
//! Generates Debug and Display trait implementations for Conditional Transformer structs
//!
//! Generates standard Debug and Display trait implementations for Conditional
//! Transformer structs that have `transformer` and `predicate` fields but no `name` field.
//!
//! # Parameters
//!
//! * `$struct_name` - The struct name
//! * `$generic` - Generic parameter list (two or three type parameters)
//!
//! # Examples
//!
//! ```ignore
//! // For two type parameters
//! impl_conditional_transformer_debug_display!(BoxConditionalTransformer<T, U>);
//!
//! // For three type parameters
//! impl_conditional_transformer_debug_display!(BoxConditionalBiTransformer<T, U, V>);
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates Debug and Display trait implementations for Conditional Transformer structs
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates complete `impl Debug` and `impl Display` blocks for the
/// specified struct. Generates standard Debug and Display trait implementations
/// for Conditional Transformer structs that have `transformer` and `predicate` fields
/// but no `name` field.
///
/// # Parameters
///
/// * `$struct_name` - The struct name
/// * `$t` - Generic parameter list (two or three type parameters)
/// * `$u` - Generic parameter list (two or three type parameters)
/// * `$r` - Generic parameter list (two or three type parameters)
///
/// # Examples
///
/// ```ignore
/// // For two type parameters
/// impl_conditional_transformer_debug_display!(BoxConditionalTransformer<T, U>);
///
/// // For three type parameters
/// impl_conditional_transformer_debug_display!(BoxConditionalBiTransformer<T, U, V>);
/// ```
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_conditional_transformer_debug_display {
    // Two generic parameters
    ($struct_name:ident < $t:ident, $r:ident >) => {
        impl<$t, $r> std::fmt::Debug for $struct_name<$t, $r> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($struct_name))
                    .field("transformer", &self.transformer)
                    .field("predicate", &self.predicate)
                    .finish()
            }
        }

        impl<$t, $r> std::fmt::Display for $struct_name<$t, $r> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}({}, {})",
                    stringify!($struct_name),
                    self.transformer,
                    self.predicate
                )
            }
        }
    };
    // Three generic parameters
    ($struct_name:ident < $t:ident, $u:ident, $r:ident >) => {
        impl<$t, $u, $r> std::fmt::Debug for $struct_name<$t, $u, $r> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($struct_name))
                    .field("transformer", &self.transformer)
                    .field("predicate", &self.predicate)
                    .finish()
            }
        }

        impl<$t, $u, $r> std::fmt::Display for $struct_name<$t, $u, $r> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}({}, {})",
                    stringify!($struct_name),
                    self.transformer,
                    self.predicate
                )
            }
        }
    };
}

pub(crate) use impl_conditional_transformer_debug_display;
