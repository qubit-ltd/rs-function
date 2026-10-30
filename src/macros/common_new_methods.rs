// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! # Common New Methods Macro
//!
//! Generates common constructor methods for function-like structs.

/// Implements common constructor methods for function-like structs.
///
/// This macro generates `new`, `new_with_name`, and `new_with_optional_name`
/// methods for structs that wrap function pointers or closures. It provides
/// a standardized way to create instances with or without names for debugging
/// and logging purposes.
///
/// # Parameters
///
/// * `$($fn_trait_with_bounds)+` - Function trait bounds (e.g., Fn(i32) -> i32)
/// * `$f:ident` - Identifier for the function parameter
/// * `$wrapper_expr:expr` - Expression to wrap the function (e.g., Arc::new(f))
/// * `$type_desc:literal` - Description of the type (e.g., "consumer")
///
/// # Generated Methods
///
/// * `new<F>(f: F) -> Self` - Creates a new instance without a name
/// * `new_with_name<F>(name: &str, f: F) -> Self` - Creates a named instance
/// * `new_with_optional_name<F>(f: F, name: Option<String>) -> Self` - Creates
///   an instance with an optional name
macro_rules! impl_common_new_public_methods {
    (
        semantic_mut ($($semantic_bounds:tt)+),
        |$source:ident| $adapter_expr:expr,
        |$f:ident| $wrapper_expr:expr,
        $type_desc:literal
    ) => {
        #[doc = concat!("Creates a new ", $type_desc, ".")]
        ///
        /// # Type Parameters
        ///
        /// * `F` - The semantic callback implementation accepted by this
        ///   wrapper.
        ///
        /// # Type Inference
        ///
        /// For callbacks that borrow arguments, explicitly annotate closure
        /// parameter types when inference is insufficient, even when the
        /// wrapper's type arguments are specified. Constructors accept
        /// semantic callback traits, which may provide less closure parameter
        /// inference than direct `Fn` bounds.
        ///
        /// # Parameters
        ///
        #[doc = concat!("* `", stringify!($source), "` - The semantic callback implementation to wrap.")]
        ///
        /// # Returns
        ///
        #[doc = concat!("A new ", $type_desc, " wrapper.")]
        #[inline]
        pub fn new<F>(mut $source: F) -> Self
        where
            F: $($semantic_bounds)+,
        {
            let $f = $adapter_expr;
            Self {
                function: $wrapper_expr,
                metadata: $crate::internal::CallbackMetadata::unnamed(),
            }
        }

        #[doc = concat!("Creates a new named ", $type_desc, ".")]
        ///
        /// # Type Parameters
        ///
        /// * `F` - The semantic callback implementation accepted by this
        ///   wrapper.
        ///
        /// # Type Inference
        ///
        /// For callbacks that borrow arguments, explicitly annotate closure
        /// parameter types when inference is insufficient, even when the
        /// wrapper's type arguments are specified. Constructors accept
        /// semantic callback traits, which may provide less closure parameter
        /// inference than direct `Fn` bounds.
        ///
        /// # Parameters
        ///
        /// * `name` - The diagnostic name assigned to the wrapper.
        #[doc = concat!("* `", stringify!($source), "` - The semantic callback implementation to wrap.")]
        ///
        /// # Returns
        ///
        #[doc = concat!("A new named ", $type_desc, " wrapper.")]
        #[inline]
        pub fn new_with_name<F>(name: &str, mut $source: F) -> Self
        where
            F: $($semantic_bounds)+,
        {
            let $f = $adapter_expr;
            Self {
                function: $wrapper_expr,
                metadata: $crate::internal::CallbackMetadata::named(name),
            }
        }

        #[doc = concat!("Creates a new ", $type_desc, " with an optional name.")]
        ///
        /// # Type Parameters
        ///
        /// * `F` - The semantic callback implementation accepted by this
        ///   wrapper.
        ///
        /// # Type Inference
        ///
        /// For callbacks that borrow arguments, explicitly annotate closure
        /// parameter types when inference is insufficient, even when the
        /// wrapper's type arguments are specified. Constructors accept
        /// semantic callback traits, which may provide less closure parameter
        /// inference than direct `Fn` bounds.
        ///
        /// # Parameters
        ///
        #[doc = concat!("* `", stringify!($source), "` - The semantic callback implementation to wrap.")]
        /// * `name` - The optional diagnostic name assigned to the wrapper.
        ///
        /// # Returns
        ///
        #[doc = concat!("A new ", $type_desc, " wrapper.")]
        #[inline]
        pub fn new_with_optional_name<F>(mut $source: F, name: Option<String>) -> Self
        where
            F: $($semantic_bounds)+,
        {
            let $f = $adapter_expr;
            Self {
                function: $wrapper_expr,
                metadata: $crate::internal::CallbackMetadata::from_optional_name(name),
            }
        }

    };

    (
        semantic ($($semantic_bounds:tt)+),
        |$source:ident| $adapter_expr:expr,
        |$f:ident| $wrapper_expr:expr,
        $type_desc:literal
    ) => {
        #[doc = concat!("Creates a new ", $type_desc, ".")]
        ///
        /// Adapts the provided semantic callback implementation to this
        /// wrapper's erased callable representation.
        ///
        /// # Type Parameters
        ///
        /// * `F` - The semantic callback implementation accepted by this
        ///   wrapper.
        ///
        /// # Type Inference
        ///
        /// For callbacks that borrow arguments, explicitly annotate closure
        /// parameter types when inference is insufficient, even when the
        /// wrapper's type arguments are specified. Constructors accept
        /// semantic callback traits, which may provide less closure parameter
        /// inference than direct `Fn` bounds.
        ///
        /// # Parameters
        ///
        #[doc = concat!("* `", stringify!($source), "` - The semantic callback implementation to wrap.")]
        ///
        /// # Returns
        ///
        #[doc = concat!("A new ", $type_desc, " wrapper.")]
        #[inline]
        pub fn new<F>($source: F) -> Self
        where
            F: $($semantic_bounds)+,
        {
            let $f = $adapter_expr;
            Self {
                function: $wrapper_expr,
                metadata: $crate::internal::CallbackMetadata::unnamed(),
            }
        }

        #[doc = concat!("Creates a new named ", $type_desc, ".")]
        ///
        /// Adapts the provided semantic callback implementation and assigns
        /// the supplied name for diagnostics.
        ///
        /// # Type Parameters
        ///
        /// * `F` - The semantic callback implementation accepted by this
        ///   wrapper.
        ///
        /// # Type Inference
        ///
        /// For callbacks that borrow arguments, explicitly annotate closure
        /// parameter types when inference is insufficient, even when the
        /// wrapper's type arguments are specified. Constructors accept
        /// semantic callback traits, which may provide less closure parameter
        /// inference than direct `Fn` bounds.
        ///
        /// # Parameters
        ///
        /// * `name` - The diagnostic name assigned to the wrapper.
        #[doc = concat!("* `", stringify!($source), "` - The semantic callback implementation to wrap.")]
        ///
        /// # Returns
        ///
        #[doc = concat!("A new named ", $type_desc, " wrapper.")]
        #[inline]
        pub fn new_with_name<F>(name: &str, $source: F) -> Self
        where
            F: $($semantic_bounds)+,
        {
            let $f = $adapter_expr;
            Self {
                function: $wrapper_expr,
                metadata: $crate::internal::CallbackMetadata::named(name),
            }
        }

        #[doc = concat!("Creates a new ", $type_desc, " with an optional name.")]
        ///
        /// Adapts the provided semantic callback implementation and preserves
        /// the optional diagnostic name.
        ///
        /// # Type Parameters
        ///
        /// * `F` - The semantic callback implementation accepted by this
        ///   wrapper.
        ///
        /// # Type Inference
        ///
        /// For callbacks that borrow arguments, explicitly annotate closure
        /// parameter types when inference is insufficient, even when the
        /// wrapper's type arguments are specified. Constructors accept
        /// semantic callback traits, which may provide less closure parameter
        /// inference than direct `Fn` bounds.
        ///
        /// # Parameters
        ///
        #[doc = concat!("* `", stringify!($source), "` - The semantic callback implementation to wrap.")]
        /// * `name` - The optional diagnostic name assigned to the wrapper.
        ///
        /// # Returns
        ///
        #[doc = concat!("A new ", $type_desc, " wrapper.")]
        #[inline]
        pub fn new_with_optional_name<F>(
            $source: F,
            name: Option<String>,
        ) -> Self
        where
            F: $($semantic_bounds)+,
        {
            let $f = $adapter_expr;
            Self {
                function: $wrapper_expr,
                metadata: $crate::internal::CallbackMetadata::from_optional_name(name),
            }
        }

    };

    (
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr,
        $type_desc:literal
    ) => {
        #[doc = concat!("Creates a new ", $type_desc, ".")]
        ///
        #[doc = concat!("Wraps the provided closure in the appropriate smart pointer type for this ", $type_desc, " implementation.")]
        ///
        /// # Parameters
        ///
        /// * `function` - The closure to wrap.
        ///
        /// # Returns
        ///
        #[doc = concat!("A new ", $type_desc, " wrapper.")]
        #[inline]
        pub fn new<F>($f: F) -> Self
        where
            F: $($fn_trait_with_bounds)+,
        {
            Self {
                function: $wrapper_expr,
                metadata: $crate::internal::CallbackMetadata::unnamed(),
            }
        }

        #[doc = concat!("Creates a new named ", $type_desc, ".")]
        ///
        /// Wraps the provided closure and assigns it a name, which is
        /// useful for debugging and logging purposes.
        ///
        /// # Parameters
        ///
        /// * `name` - The diagnostic name assigned to the wrapper.
        /// * `function` - The closure to wrap.
        ///
        /// # Returns
        ///
        #[doc = concat!("A new named ", $type_desc, " wrapper.")]
        #[inline]
        pub fn new_with_name<F>(name: &str, $f: F) -> Self
        where
            F: $($fn_trait_with_bounds)+,
        {
            Self {
                function: $wrapper_expr,
                metadata: $crate::internal::CallbackMetadata::named(name),
            }
        }

        #[doc = concat!("Creates a new ", $type_desc, " with an optional name.")]
        ///
        /// Wraps the provided closure and assigns it an optional name.
        ///
        /// # Parameters
        ///
        /// * `function` - The closure to wrap.
        /// * `name` - The optional diagnostic name assigned to the wrapper.
        ///
        /// # Returns
        ///
        #[doc = concat!("A new ", $type_desc, " wrapper.")]
        #[inline]
        pub fn new_with_optional_name<F>($f: F, name: Option<String>) -> Self
        where
            F: $($fn_trait_with_bounds)+,
        {
            Self {
                function: $wrapper_expr,
                metadata: $crate::internal::CallbackMetadata::from_optional_name(name),
            }
        }

    };
}

/// Generates public constructors and the metadata-preserving constructor.
macro_rules! impl_common_new_methods {
    (
        semantic_mut ($($semantic_bounds:tt)+),
        |$source:ident| $adapter_expr:expr,
        |$f:ident| $wrapper_expr:expr,
        $type_desc:literal
    ) => {
        $crate::macros::impl_common_new_public_methods!(
            semantic_mut ($($semantic_bounds)+),
            |$source| $adapter_expr,
            |$f| $wrapper_expr,
            $type_desc
        );
        $crate::macros::impl_common_new_metadata_method!(
            semantic_mut ($($semantic_bounds)+),
            |$source| $adapter_expr,
            |$f| $wrapper_expr,
            $type_desc
        );
    };
    (
        semantic ($($semantic_bounds:tt)+),
        |$source:ident| $adapter_expr:expr,
        |$f:ident| $wrapper_expr:expr,
        $type_desc:literal
    ) => {
        $crate::macros::impl_common_new_public_methods!(
            semantic ($($semantic_bounds)+),
            |$source| $adapter_expr,
            |$f| $wrapper_expr,
            $type_desc
        );
        $crate::macros::impl_common_new_metadata_method!(
            semantic ($($semantic_bounds)+),
            |$source| $adapter_expr,
            |$f| $wrapper_expr,
            $type_desc
        );
    };
    (
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr,
        $type_desc:literal
    ) => {
        $crate::macros::impl_common_new_public_methods!(
            ($($fn_trait_with_bounds)+),
            |$f| $wrapper_expr,
            $type_desc
        );
        $crate::macros::impl_common_new_metadata_method!(
            ($($fn_trait_with_bounds)+),
            |$f| $wrapper_expr,
            $type_desc
        );
    };
}

pub(crate) use impl_common_new_methods;
pub(crate) use impl_common_new_public_methods;

/// Generates the crate-visible constructor used to preserve callback metadata.
///
/// This helper is expanded after public constructors and before other methods
/// so constructors remain grouped first in each inherent impl.
macro_rules! impl_common_new_metadata_method {
    (
        semantic_mut ($($semantic_bounds:tt)+),
        |$source:ident| $adapter_expr:expr,
        |$f:ident| $wrapper_expr:expr,
        $type_desc:literal
    ) => {
        /// Creates a wrapper while preserving existing metadata.
        ///
        /// Adapts the provided semantic callback implementation to this
        /// wrapper's erased callable representation.
        ///
        /// # Type Parameters
        ///
        /// * `F` - The callback implementation type.
        ///
        /// # Parameters
        ///
        /// * `source` - The semantic callback implementation to wrap.
        /// * `metadata` - The diagnostic metadata to preserve.
        ///
        /// # Returns
        ///
        #[doc = concat!("A new ", $type_desc, " wrapper.")]
        #[allow(dead_code, reason = "generated for metadata-preserving wrapper transformations")]
        #[inline]
        pub(crate) fn new_with_metadata<F>(
            mut $source: F,
            metadata: $crate::internal::CallbackMetadata,
        ) -> Self
        where
            F: $($semantic_bounds)+,
        {
            let $f = $adapter_expr;
            Self { function: $wrapper_expr, metadata }
        }
    };
    (
        semantic ($($semantic_bounds:tt)+),
        |$source:ident| $adapter_expr:expr,
        |$f:ident| $wrapper_expr:expr,
        $type_desc:literal
    ) => {
        /// Creates a wrapper while preserving existing metadata.
        ///
        /// # Parameters
        ///
        /// * `source` - The semantic callback implementation to wrap.
        /// * `metadata` - The diagnostic metadata to preserve.
        ///
        /// # Returns
        ///
        #[doc = concat!("A new ", $type_desc, " wrapper.")]
        #[allow(dead_code, reason = "generated for metadata-preserving wrapper transformations")]
        #[inline]
        pub(crate) fn new_with_metadata<F>(
            $source: F,
            metadata: $crate::internal::CallbackMetadata,
        ) -> Self
        where
            F: $($semantic_bounds)+,
        {
            let $f = $adapter_expr;
            Self { function: $wrapper_expr, metadata }
        }
    };
    (
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr,
        $type_desc:literal
    ) => {
        /// Creates a wrapper while preserving existing metadata.
        ///
        /// # Parameters
        ///
        /// * `function` - The closure to wrap.
        /// * `metadata` - The diagnostic metadata to preserve.
        ///
        /// # Returns
        ///
        #[doc = concat!("A new ", $type_desc, " wrapper.")]
        #[allow(dead_code, reason = "generated for metadata-preserving wrapper transformations")]
        #[inline]
        pub(crate) fn new_with_metadata<F>(
            $f: F,
            metadata: $crate::internal::CallbackMetadata,
        ) -> Self
        where
            F: $($fn_trait_with_bounds)+,
        {
            Self { function: $wrapper_expr, metadata }
        }
    };
}

pub(crate) use impl_common_new_metadata_method;
