/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Defines the `ArcStatefulTransformer` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// ArcStatefulTransformer - Arc<Mutex<dyn FnMut(T) -> R + Send>>
// ============================================================================

/// ArcStatefulTransformer - thread-safe transformer wrapper
///
/// A thread-safe, clonable transformer wrapper suitable for multi-threaded
/// scenarios. Can be called multiple times and shared across threads
/// while maintaining internal state.
///
/// # Features
///
/// - **Based on**: `Arc<Mutex<dyn FnMut(T) -> R + Send>>`
/// - **Ownership**: Shared ownership via reference counting
/// - **Reusability**: Can be called multiple times (each call consumes
///   its input)
/// - **Thread Safety**: Thread-safe (`Send` required)
/// - **Clonable**: Cheap cloning via `Arc::clone`
/// - **Statefulness**: Can modify internal state between calls
///
/// # Author
///
/// Haixing Hu
pub struct ArcStatefulTransformer<T, R> {
    pub(super) function: Arc<Mutex<dyn FnMut(T) -> R + Send>>,
    pub(super) name: Option<String>,
}

impl<T, R> ArcStatefulTransformer<T, R> {
    impl_transformer_common_methods!(
        ArcStatefulTransformer<T, R>,
        (FnMut(T) -> R + Send + 'static),
        |f| Arc::new(Mutex::new(f))
    );

    impl_shared_transformer_methods!(
        ArcStatefulTransformer<T, R>,
        ArcConditionalStatefulTransformer,
        into_arc,
        StatefulTransformer,
        Send + Sync + 'static
    );
}

// Implement constant method for ArcStatefulTransformer
impl_transformer_constant_method!(stateful thread_safe ArcStatefulTransformer<T, R>);

// Implement Debug and Display for ArcStatefulTransformer
impl_transformer_debug_display!(ArcStatefulTransformer<T, R>);

// Implement Clone for ArcStatefulTransformer
impl_transformer_clone!(ArcStatefulTransformer<T, R>);

impl<T, R> StatefulTransformer<T, R> for ArcStatefulTransformer<T, R> {
    fn apply(&mut self, input: T) -> R {
        let mut func = self.function.lock();
        func(input)
    }

    // Use macro to implement conversion methods
    impl_arc_conversions!(
        ArcStatefulTransformer<T, R>,
        BoxStatefulTransformer,
        RcStatefulTransformer,
        BoxTransformerOnce,
        FnMut(t: T) -> R
    );
}

// ============================================================================
// Blanket implementation for standard FnMut trait
// ============================================================================

// Implement StatefulTransformer<T, R> for any type that implements FnMut(T) -> R
impl_closure_trait!(
    StatefulTransformer<T, R>,
    apply,
    BoxTransformerOnce,
    FnMut(input: T) -> R
);
