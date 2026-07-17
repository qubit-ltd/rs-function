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
macro_rules! impl_common_new_methods {
    (
        semantic_mut ($($semantic_bounds:tt)+),
        |$source:ident| $adapter_expr:expr,
        |$f:ident| $wrapper_expr:expr,
        $type_desc:literal
    ) => {
        #[doc = concat!("Creates a new ", $type_desc, ".")]
        ///
        /// # Parameters
        ///
        /// * `source` - The semantic callback implementation to wrap.
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
        /// # Parameters
        ///
        /// * `name` - The diagnostic name assigned to the wrapper.
        /// * `source` - The semantic callback implementation to wrap.
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

        #[doc = concat!("Creates a new named ", $type_desc, " with an optional name.")]
        ///
        /// # Parameters
        ///
        /// * `source` - The semantic callback implementation to wrap.
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

        #[allow(
            dead_code,
            reason = "generated for metadata-preserving wrapper transformations"
        )]
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
        #[doc = concat!("Creates a new ", $type_desc, ".")]
        ///
        /// Adapts the provided semantic callback implementation to this
        /// wrapper's erased callable representation.
        ///
        /// # Parameters
        ///
        /// * `source` - The semantic callback implementation to wrap.
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
        /// # Parameters
        ///
        /// * `name` - The diagnostic name assigned to the wrapper.
        /// * `source` - The semantic callback implementation to wrap.
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

        #[doc = concat!("Creates a new named ", $type_desc, " with an optional name.")]
        ///
        /// Adapts the provided semantic callback implementation and preserves
        /// the optional diagnostic name.
        ///
        /// # Parameters
        ///
        /// * `source` - The semantic callback implementation to wrap.
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

        #[allow(
            dead_code,
            reason = "generated for metadata-preserving wrapper transformations"
        )]
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
        #[inline]
        pub(crate) fn new_with_metadata<F>(
            $source: F,
            metadata: $crate::internal::CallbackMetadata,
        ) -> Self
        where
            F: $($semantic_bounds)+,
        {
            let $f = $adapter_expr;
            Self {
                function: $wrapper_expr,
                metadata,
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

        #[doc = concat!("Creates a new named ", $type_desc, " with an optional name.")]
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

        #[allow(
            dead_code,
            reason = "generated for metadata-preserving wrapper transformations"
        )]
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
        #[inline]
        pub(crate) fn new_with_metadata<F>(
            $f: F,
            metadata: $crate::internal::CallbackMetadata,
        ) -> Self
        where
            F: $($fn_trait_with_bounds)+,
        {
            Self {
                function: $wrapper_expr,
                metadata,
            }
        }
    };
}

pub(crate) use impl_common_new_methods;
