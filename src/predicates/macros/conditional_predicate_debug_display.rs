/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Conditional Predicate Debug Display Macro
//!
//! Generates Debug and Display trait implementations for Conditional Predicate structs
//!
//! Generates standard Debug and Display trait implementations for Conditional
//! Predicate structs that have `predicate` and `condition` fields but no `name` field.
//!
//! # Parameters
//!
//! * `$struct_name` - The struct name
//! * `$generic` - Generic parameter list (one or more type parameters)
//!
//! # Examples
//!
//! ```ignore
//! // For single type parameter
//! impl_conditional_predicate_debug_display!(BoxConditionalPredicate<T>);
//!
//! // For two type parameters
//! impl_conditional_predicate_debug_display!(BoxConditionalBiPredicate<T, U>);
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates Debug and Display trait implementations for Conditional Predicate structs
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates complete `impl Debug` and `impl Display` blocks for the
/// specified struct. Generates standard Debug and Display trait implementations
/// for Conditional Predicate structs that have `predicate` and `condition` fields
/// but no `name` field.
///
/// # Parameters
///
/// * `$struct_name` - The struct name
/// * `$generic` - Generic parameter list (one or more type parameters)
///
/// # Examples
///
/// ```ignore
/// // For single type parameter
/// impl_conditional_predicate_debug_display!(BoxConditionalPredicate<T>);
///
/// // For two type parameters
/// impl_conditional_predicate_debug_display!(BoxConditionalBiPredicate<T, U>);
/// ```
macro_rules! impl_conditional_predicate_debug_display {
    // Single generic parameter
    ($struct_name:ident < $generic:ident >) => {
        impl<$generic> std::fmt::Debug for $struct_name<$generic> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($struct_name))
                    .field("predicate", &self.predicate)
                    .field("condition", &self.condition)
                    .finish()
            }
        }

        impl<$generic> std::fmt::Display for $struct_name<$generic> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}({}, {})",
                    stringify!($struct_name),
                    self.predicate,
                    self.condition
                )
            }
        }
    };
    // Two generic parameters
    ($struct_name:ident < $generic1:ident, $generic2:ident >) => {
        impl<$generic1, $generic2> std::fmt::Debug for $struct_name<$generic1, $generic2> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($struct_name))
                    .field("predicate", &self.predicate)
                    .field("condition", &self.condition)
                    .finish()
            }
        }

        impl<$generic1, $generic2> std::fmt::Display for $struct_name<$generic1, $generic2> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}({}, {})",
                    stringify!($struct_name),
                    self.predicate,
                    self.condition
                )
            }
        }
    };
}

pub(crate) use impl_conditional_predicate_debug_display;
