/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Transformer Common Methods Macro
//!
//! Generates common Transformer methods (new, new_with_name, name,
//! set_name, identity)
//!
//! Generates constructor methods, name management methods and identity
//! constructor for Transformer structs. This macro should be called inside
//! an impl block.
//!
//! The macro automatically detects the number of generic parameters and
//! generates the appropriate implementations for single-parameter or
//! two-parameter transformers.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Struct name with generic parameters
//! * `$fn_trait_with_bounds` - Closure trait with complete bounds
//!   (e.g., `Fn(&T) -> U + 'static`)
//! * `$wrapper_expr` - Wrapper expression (uses `f` for the closure)
//!
//! # Usage
//!
//! ```ignore
//! // Single generic parameter - Transformer
//! impl_transformer_common_methods!(
//!     BoxTransformer<T, U>,
//!     (Fn(&T) -> U + 'static),
//!     |f| Box::new(f)
//! );
//!
//! // Single generic parameter - StatefulTransformer
//! impl_transformer_common_methods!(
//!     ArcStatefulTransformer<T, U>,
//!     (FnMut(&T) -> U + Send + 'static),
//!     |f| Arc::new(Mutex::new(f))
//! );
//!
//! // Two generic parameters - BiTransformer
//! impl_transformer_common_methods!(
//!     BoxBiTransformer<T, U, V>,
//!     (Fn(&T, &U) -> V + 'static),
//!     |f| Box::new(f)
//! );
//! ```
//!
//! # Generated Methods
//!
//! * `new()` - Creates a new transformer
//! * `new_with_name()` - Creates a named transformer
//! * `name()` - Gets the name of the transformer
//! * `set_name()` - Sets the name of the transformer
//! * `identity()` - Creates a transformer that returns the input unchanged
//!
//! # Author
//!
//! Haixing Hu

/// Generates common Transformer methods (new, new_with_name, name,
/// set_name, identity)
///
/// This macro should be used inside an existing impl block for the target
/// struct. It generates individual methods but does not create a complete
/// impl block itself. Generates constructor methods, name management methods
/// and identity constructor for Transformer structs.
///
/// The macro automatically detects the number of generic parameters and
/// generates the appropriate implementations for single-parameter or
/// two-parameter transformers.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - Struct name with generic parameters
/// * `$fn_trait_with_bounds` - Closure trait with complete bounds
///   (e.g., `Fn(&T) -> U + 'static`)
/// * `$wrapper_expr` - Wrapper expression (uses `f` for the closure)
///
/// # Usage
///
/// ```ignore
/// // Single generic parameter - Transformer
/// impl_transformer_common_methods!(
///     BoxTransformer<T, U>,
///     (Fn(&T) -> U + 'static),
///     |f| Box::new(f)
/// );
///
/// // Single generic parameter - StatefulTransformer
/// impl_transformer_common_methods!(
///     ArcStatefulTransformer<T, U>,
///     (FnMut(&T) -> U + Send + 'static),
///     |f| Arc::new(Mutex::new(f))
/// );
///
/// // Two generic parameters - BiTransformer
/// impl_transformer_common_methods!(
///     BoxBiTransformer<T, U, V>,
///     (Fn(&T, &U) -> V + 'static),
///     |f| Box::new(f)
//! );
//! ```
///
/// # Generated Methods
///
/// * `new()` - Creates a new transformer
/// * `new_with_name()` - Creates a named transformer
/// * `name()` - Gets the name of the transformer
/// * `set_name()` - Sets the name of the transformer
/// * `identity()` - Creates a transformer that returns the input unchanged
macro_rules! impl_transformer_common_methods {
    // Internal rule: generates new and new_with_name methods
    // Parameters:
    //   $fn_trait_with_bounds - Function trait bounds
    //   $f - Closure parameter name
    //   $wrapper_expr - Wrapper expression
    //   $type_desc - Type description for docs (e.g., "transformer" or "bi-transformer")
    (@new_methods
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr,
        $type_desc:literal
    ) => {
        #[doc = concat!("Creates a new ", $type_desc, ".")]
        ///
        #[doc = concat!("Wraps the provided closure in the appropriate smart pointer type for this ", $type_desc, " implementation.")]
        ///
        /// # Type Parameters
        ///
        /// * `F` - The closure type
        ///
        /// # Parameters
        ///
        /// * `f` - The closure to wrap
        ///
        /// # Returns
        ///
        #[doc = concat!("Returns a new ", $type_desc, " instance wrapping the closure.")]
        pub fn new<F>($f: F) -> Self
        where
            F: $($fn_trait_with_bounds)+,
        {
            Self {
                function: $wrapper_expr,
                name: None,
            }
        }

        #[doc = concat!("Creates a new named ", $type_desc, ".")]
        ///
        /// Wraps the provided closure and assigns it a name, which is
        /// useful for debugging and logging purposes.
        ///
        /// # Type Parameters
        ///
        /// * `F` - The closure type
        ///
        /// # Parameters
        ///
        #[doc = concat!("* `name` - The name for this ", $type_desc)]
        /// * `f` - The closure to wrap
        ///
        /// # Returns
        ///
        #[doc = concat!("Returns a new named ", $type_desc, " instance wrapping the closure.")]
        pub fn new_with_name<F>(name: &str, $f: F) -> Self
        where
            F: $($fn_trait_with_bounds)+,
        {
            Self {
                function: $wrapper_expr,
                name: Some(name.to_string()),
            }
        }
    };

    // Internal rule: generates name and set_name methods
    (@name_methods $type_desc:literal) => {
        #[doc = concat!("Gets the name of this ", $type_desc, ".")]
        ///
        /// # Returns
        ///
        /// Returns `Some(&str)` if a name was set, `None` otherwise.
        pub fn name(&self) -> Option<&str> {
            self.name.as_deref()
        }

        #[doc = concat!("Sets the name of this ", $type_desc, ".")]
        ///
        /// # Parameters
        ///
        #[doc = concat!("* `name` - The name to set for this ", $type_desc)]
        pub fn set_name(&mut self, name: &str) {
            self.name = Some(name.to_string());
        }
    };

    // Single generic parameter - Transformer types
    (
        $struct_name:ident < $t:ident, $u:ident >,
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr
    ) => {
        impl_transformer_common_methods!(@new_methods
            ($($fn_trait_with_bounds)+),
            |$f| $wrapper_expr,
            "transformer"
        );

        impl_transformer_common_methods!(@name_methods "transformer");

        /// Creates an identity transformer.
        ///
        /// Creates a transformer that returns the input value unchanged. Useful for
        /// default values or placeholder implementations.
        ///
        /// # Returns
        ///
        /// Returns a new transformer instance that returns the input unchanged.
        pub fn identity() -> Self {
            Self::new(|t: &$t| t.clone())
        }
    };

    // Two generic parameters - BiTransformer types
    (
        $struct_name:ident < $t:ident, $u:ident, $v:ident >,
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr
    ) => {
        impl_transformer_common_methods!(@new_methods
            ($($fn_trait_with_bounds)+),
            |$f| $wrapper_expr,
            "bi-transformer"
        );

        impl_transformer_common_methods!(@name_methods "bi-transformer");

        /// Creates an identity bi-transformer.
        ///
        /// Creates a bi-transformer that returns the first input value unchanged. Useful for
        /// default values or placeholder implementations.
        ///
        /// # Returns
        ///
        /// Returns a new bi-transformer instance that returns the first input unchanged.
        pub fn identity() -> Self {
            Self::new(|t: &$t, _: &$u| t.clone())
        }
    };
}

pub(crate) use impl_transformer_common_methods;
