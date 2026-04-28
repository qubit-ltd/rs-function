/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `BoxBiTransformer` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// BoxBiTransformer - Box<dyn Fn(T, U) -> R>
// ============================================================================

/// BoxBiTransformer - bi-transformer wrapper based on `Box<dyn Fn>`
///
/// A bi-transformer wrapper that provides single ownership with reusable
/// transformation. The bi-transformer consumes both inputs and can be called
/// multiple times.
///
/// # Features
///
/// - **Based on**: `Box<dyn Fn(T, U) -> R>`
/// - **Ownership**: Single ownership, cannot be cloned
/// - **Reusability**: Can be called multiple times (each call consumes its
///   inputs)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
///
/// # Author
///
/// Haixing Hu
pub struct BoxBiTransformer<T, U, R> {
    pub(super) function: Box<dyn Fn(T, U) -> R>,
    pub(super) name: Option<String>,
}

// Implement BoxBiTransformer
impl<T, U, R> BoxBiTransformer<T, U, R> {
    impl_transformer_common_methods!(
        BoxBiTransformer<T, U, R>,
        (Fn(T, U) -> R + 'static),
        |f| Box::new(f)
    );

    impl_box_transformer_methods!(
        BoxBiTransformer<T, U, R>,
        BoxConditionalBiTransformer,
        Transformer
    );
}

// Implement constant method for BoxBiTransformer
impl_transformer_constant_method!(BoxBiTransformer<T, U, R>);

// Implement Debug and Display for BoxBiTransformer
impl_transformer_debug_display!(BoxBiTransformer<T, U, R>);

// Implement BiTransformer trait for BoxBiTransformer
impl<T, U, R> BiTransformer<T, U, R> for BoxBiTransformer<T, U, R> {
    fn apply(&self, first: T, second: U) -> R {
        (self.function)(first, second)
    }

    // Generates: into_box(), into_rc(), into_fn(), into_once()
    impl_box_conversions!(
        BoxBiTransformer<T, U, R>,
        RcBiTransformer,
        Fn(T, U) -> R,
        BoxBiTransformerOnce
    );
}
