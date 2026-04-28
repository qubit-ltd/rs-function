/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `BoxTransformer` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// BoxTransformer - Box<dyn Fn(T) -> R>
// ============================================================================

/// BoxTransformer - transformer wrapper based on `Box<dyn Fn>`
///
/// A transformer wrapper that provides single ownership with reusable
/// transformation. The transformer consumes the input and can be called
/// multiple times.
///
/// # Features
///
/// - **Based on**: `Box<dyn Fn(T) -> R>`
/// - **Ownership**: Single ownership, cannot be cloned
/// - **Reusability**: Can be called multiple times (each call consumes its
///   input)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
///
/// # Author
///
/// Haixing Hu
pub struct BoxTransformer<T, R> {
    pub(super) function: Box<dyn Fn(T) -> R>,
    pub(super) name: Option<String>,
}

// Implement BoxTransformer
impl<T, R> BoxTransformer<T, R> {
    impl_transformer_common_methods!(
        BoxTransformer<T, R>,
        (Fn(T) -> R + 'static),
        |f| Box::new(f)
    );

    impl_box_transformer_methods!(
        BoxTransformer<T, R>,
        BoxConditionalTransformer,
        Transformer
    );
}

// Implement constant method for BoxTransformer
impl_transformer_constant_method!(BoxTransformer<T, R>);

// Implement Debug and Display for BoxTransformer
impl_transformer_debug_display!(BoxTransformer<T, R>);

// Implement Transformer for BoxTransformer
impl<T, R> Transformer<T, R> for BoxTransformer<T, R> {
    fn apply(&self, input: T) -> R {
        (self.function)(input)
    }

    // Generates: into_box(), into_rc(), into_fn(), into_once()
    impl_box_conversions!(
        BoxTransformer<T, R>,
        RcTransformer,
        Fn(T) -> R,
        BoxTransformerOnce
    );
}
