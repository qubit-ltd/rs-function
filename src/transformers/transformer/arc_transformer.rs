/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `ArcTransformer` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// ArcTransformer - Arc<dyn Fn(T) -> R + Send + Sync>
// ============================================================================

/// ArcTransformer - thread-safe transformer wrapper
///
/// A thread-safe, clonable transformer wrapper suitable for multi-threaded
/// scenarios. Can be called multiple times and shared across threads.
///
/// # Features
///
/// - **Based on**: `Arc<dyn Fn(T) -> R + Send + Sync>`
/// - **Ownership**: Shared ownership via reference counting
/// - **Reusability**: Can be called multiple times (each call consumes its
///   input)
/// - **Thread Safety**: Thread-safe (`Send + Sync` required)
/// - **Clonable**: Cheap cloning via `Arc::clone`
///
/// # Author
///
/// Haixing Hu
pub struct ArcTransformer<T, R> {
    pub(super) function: Arc<dyn Fn(T) -> R + Send + Sync>,
    pub(super) name: Option<String>,
}

// Implement ArcTransformer
impl<T, R> ArcTransformer<T, R> {
    impl_transformer_common_methods!(
        ArcTransformer<T, R>,
        (Fn(T) -> R + Send + Sync + 'static),
        |f| Arc::new(f)
    );

    impl_shared_transformer_methods!(
        ArcTransformer<T, R>,
        ArcConditionalTransformer,
        into_arc,
        Transformer,
        Send + Sync + 'static
    );
}

// Implement constant method for ArcTransformer
impl_transformer_constant_method!(thread_safe ArcTransformer<T, R>);

// Implement Debug and Display for ArcTransformer
impl_transformer_debug_display!(ArcTransformer<T, R>);

// Implement Clone for ArcTransformer
impl_transformer_clone!(ArcTransformer<T, R>);

// Implement Transformer for ArcTransformer
impl<T, R> Transformer<T, R> for ArcTransformer<T, R> {
    fn apply(&self, input: T) -> R {
        (self.function)(input)
    }

    // Use macro to implement conversion methods
    impl_arc_conversions!(
        ArcTransformer<T, R>,
        BoxTransformer,
        RcTransformer,
        BoxTransformerOnce,
        Fn(t: T) -> R
    );
}

// ============================================================================
// Blanket implementation for standard Fn trait
// ============================================================================

// Implement Transformer<T, R> for any type that implements Fn(T) -> R
impl_closure_trait!(
    Transformer<T, R>,
    apply,
    BoxTransformerOnce,
    Fn(input: T) -> R
);
