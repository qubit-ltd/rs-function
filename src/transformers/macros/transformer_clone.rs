// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
/// Generates a `Clone` implementation for a Transformer type.
///
/// This macro should be used at the top level (outside of any impl block) as
/// it expands to a complete `impl Clone for $struct_name` block. The target
/// struct must have `function` and `metadata` fields that implement `Clone`.
/// Cloning shared pointers such as `Arc` or `Rc` in the `function` field only
/// clones the pointer.
///
/// # Parameters
///
/// * `$struct_name` - The Transformer struct identifier.
/// * `$t` and `$r` - The two type parameters for a Transformer.
/// * `$u` - The additional middle type parameter for a three-parameter
///   Transformer.
///
/// A two-parameter invocation has the form `impl_transformer_clone!(Name<T,
/// R>);`; a three-parameter invocation has the form
/// `impl_transformer_clone!(Name<T, U, R>);`. Invoke the macro at module scope,
/// outside any `impl` block.
macro_rules! impl_transformer_clone {
    // Two generic parameters
    ($struct_name:ident < $t:ident, $r:ident >) => {
        impl<$t, $r> Clone for $struct_name<$t, $r> {
            fn clone(&self) -> Self {
                Self {
                    function: self.function.clone(),
                    metadata: self.metadata.clone(),
                }
            }
        }
    };
    // Three generic parameters
    ($struct_name:ident < $t:ident, $u:ident, $r:ident >) => {
        impl<$t, $u, $r> Clone for $struct_name<$t, $u, $r> {
            fn clone(&self) -> Self {
                Self {
                    function: self.function.clone(),
                    metadata: self.metadata.clone(),
                }
            }
        }
    };
}

pub(crate) use impl_transformer_clone;
