/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Function Debug Display Macro
//!
//! Generates Debug and Display trait implementations for Function structs
//!
//! Generates standard Debug and Display trait implementations for Function
//! structs that have a `name: Option<String>` field.
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
//! impl_function_debug_display!(BoxFunction<T, R>);
//!
//! // For three type parameters
//! impl_function_debug_display!(BoxBiFunction<T, U, R>);
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates Debug and Display trait implementations for Function structs
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates complete `impl Debug` and `impl Display` blocks for the
/// specified struct. Generates standard Debug and Display trait implementations
/// for Function structs that have a `name: Option<String>` field.
///
/// # Parameters
///
/// * `$struct_name` - The struct name
/// * `$generic` - Generic parameter list (two or three type parameters)
///
/// # Examples
///
/// ```ignore
/// // For two type parameters
/// impl_function_debug_display!(BoxFunction<T, R>);
///
/// // For three type parameters
/// impl_function_debug_display!(BoxBiFunction<T, U, R>);
/// ```
macro_rules! impl_function_debug_display {
    // Two generic parameters - Function types
    ($struct_name:ident < $generic1:ident, $generic2:ident >) => {
        impl<$generic1, $generic2> std::fmt::Debug for $struct_name<$generic1, $generic2> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($struct_name))
                    .field("name", &self.name)
                    .field("function", &"<function>")
                    .finish()
            }
        }

        impl<$generic1, $generic2> std::fmt::Display for $struct_name<$generic1, $generic2> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match &self.name {
                    Some(name) => write!(f, "{}({})", stringify!($struct_name), name),
                    None => write!(f, "{}", stringify!($struct_name)),
                }
            }
        }
    };
    // Three generic parameters - BiFunction types
    ($struct_name:ident < $generic1:ident, $generic2:ident, $generic3:ident >) => {
        impl<$generic1, $generic2, $generic3> std::fmt::Debug for $struct_name<$generic1, $generic2, $generic3> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($struct_name))
                    .field("name", &self.name)
                    .field("function", &"<function>")
                    .finish()
            }
        }

        impl<$generic1, $generic2, $generic3> std::fmt::Display for $struct_name<$generic1, $generic2, $generic3> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match &self.name {
                    Some(name) => write!(f, "{}({})", stringify!($struct_name), name),
                    None => write!(f, "{}", stringify!($struct_name)),
                }
            }
        }
    };
}

pub(crate) use impl_function_debug_display;
