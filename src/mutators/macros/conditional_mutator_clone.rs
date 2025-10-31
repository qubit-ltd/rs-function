/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
/*! # Conditional Mutator Clone Macro
//!
//! Generates Clone trait implementation for Conditional Mutator types
//!
//! Generates Clone implementation for Conditional Mutator structs that have
//! `mutator` and `predicate` fields. Both fields are cloned using their
//! respective Clone implementations.
//!
//! # Parameters
//!
//! * `$struct_name` - The struct name
//! * `$generic` - Generic parameter list (one type parameter)
//!
//! # Examples
//!
//! ```ignore
//! // For single type parameter
//! impl_conditional_mutator_clone!(ArcConditionalMutator<T>);
//! impl_conditional_mutator_clone!(RcConditionalMutator<T>);
//! ```
//!
//! # Author
//!
//! Haixing Hu
*/

/// Generates Clone trait implementation for Conditional Mutator types
///
/// Generates Clone implementation for Conditional Mutator structs that have
/// `mutator` and `predicate` fields. Both fields are cloned using their
/// respective Clone implementations.
///
/// # Parameters
///
/// * `$struct_name` - The struct name
/// * `$generic` - Generic parameter list (one type parameter)
///
/// # Examples
///
/// ```ignore
/// // For single type parameter
/// impl_conditional_mutator_clone!(ArcConditionalMutator<T>);
/// impl_conditional_mutator_clone!(RcConditionalMutator<T>);
/// ```
macro_rules! impl_conditional_mutator_clone {
    // Single generic parameter
    ($struct_name:ident < $generic:ident >) => {
        impl<$generic> Clone for $struct_name<$generic> {
            fn clone(&self) -> Self {
                Self {
                    mutator: self.mutator.clone(),
                    predicate: self.predicate.clone(),
                }
            }
        }
    };
}

pub(crate) use impl_conditional_mutator_clone;
