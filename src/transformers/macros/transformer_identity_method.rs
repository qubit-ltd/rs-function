// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Transformer Identity Method Macro
//!
//! Generates an identity constructor for a unary transformer wrapper.

/// Generates an identity constructor for a unary transformer wrapper.
///
/// The constructor is implemented only for the same input and output type, so
/// the expected result type can determine the transformer type arguments.
///
/// # Example
///
/// ```ignore
/// use qubit_function::{BoxTransformer, Transformer};
///
/// let identity: BoxTransformer<String, String> = BoxTransformer::identity();
/// assert_eq!(identity.apply(String::from("item")), "item");
/// ```
macro_rules! impl_transformer_identity_method {
    (@binding Transformer) => { "" };
    (@binding StatefulTransformer) => { "mut " };
    (@binding TransformerOnce) => { "" };

    ($wrapper:ident, $semantic_trait:ident) => {
        impl<T> $wrapper<T, T> {
            #[doc = concat!("Creates a transformer that returns its owned input unchanged.\n\n", "# Returns\n\n", "A transformer that moves its input to the output without cloning.\n\n", "# Example\n\n", "```rust\n", "use qubit_function::{", stringify!($wrapper), ", ", stringify!($semantic_trait), "};\n\n", "let ", impl_transformer_identity_method!(@binding $semantic_trait), "identity: ", stringify!($wrapper), "<String, String> = ", stringify!($wrapper), "::identity();\n", "assert_eq!(identity.apply(String::from(\"item\")), \"item\");\n", "```" )]
            #[inline]
            pub fn identity() -> Self {
                Self::new(|value: T| value)
            }
        }
    };
}

pub(crate) use impl_transformer_identity_method;
