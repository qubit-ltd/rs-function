/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
// qubit-style: allow explicit-imports
//! Defines the `ArcBiTransformer` public type.

#![allow(unused_imports)]

use super::*;

// ============================================================================
// ArcBiTransformer - Arc<dyn Fn(T, U) -> R + Send + Sync>
// ============================================================================

/// ArcBiTransformer - thread-safe bi-transformer wrapper
///
/// A thread-safe, clonable bi-transformer wrapper suitable for multi-threaded
/// scenarios. Can be called multiple times and shared across threads.
///
/// # Features
///
/// - **Based on**: `Arc<dyn Fn(T, U) -> R + Send + Sync>`
/// - **Ownership**: Shared ownership via reference counting
/// - **Reusability**: Can be called multiple times (each call consumes its
///   inputs)
/// - **Thread Safety**: Thread-safe (`Send + Sync` required)
/// - **Clonable**: Cheap cloning via `Arc::clone`
///
pub struct ArcBiTransformer<T, U, R> {
    pub(super) function: Arc<dyn Fn(T, U) -> R + Send + Sync>,
    pub(super) name: Option<String>,
}

impl<T, U, R> ArcBiTransformer<T, U, R> {
    impl_transformer_common_methods!(
        ArcBiTransformer<T, U, R>,
        (Fn(T, U) -> R + Send + Sync + 'static),
        |f| Arc::new(f)
    );

    impl_shared_transformer_methods!(
        ArcBiTransformer<T, U, R>,
        ArcConditionalBiTransformer,
        into_arc,
        Transformer,
        Send + Sync + 'static
    );
}

// Implement constant method for ArcBiTransformer
impl_transformer_constant_method!(thread_safe ArcBiTransformer<T, U, R>);

// Implement Debug and Display for ArcBiTransformer
impl_transformer_debug_display!(ArcBiTransformer<T, U, R>);

// Implement Clone for ArcBiTransformer
impl_transformer_clone!(ArcBiTransformer<T, U, R>);

// Implement BiTransformer trait for ArcBiTransformer
impl<T, U, R> BiTransformer<T, U, R> for ArcBiTransformer<T, U, R> {
    fn apply(&self, first: T, second: U) -> R {
        (self.function)(first, second)
    }

    // Use macro to implement conversion methods
    impl_arc_conversions!(
        ArcBiTransformer<T, U, R>,
        BoxBiTransformer,
        RcBiTransformer,
        BoxBiTransformerOnce,
        Fn(t: T, u: U) -> R
    );
}

// ============================================================================
// Blanket implementation for standard Fn trait
// ============================================================================

/// Implement BiTransformer<T, U, R> for any type that implements Fn(T, U) -> R
///
/// This allows closures and function pointers to be used directly with our
/// BiTransformer trait without wrapping.
///
/// # Examples
///
/// ```rust
/// use qubit_function::BiTransformer;
///
/// fn add(x: i32, y: i32) -> i32 { x + y }
///
/// assert_eq!(add.apply(20, 22), 42);
///
/// let multiply = |x: i32, y: i32| x * y;
/// assert_eq!(multiply.apply(6, 7), 42);
/// ```
///
impl<F, T, U, R> BiTransformer<T, U, R> for F
where
    F: Fn(T, U) -> R,
{
    fn apply(&self, first: T, second: U) -> R {
        self(first, second)
    }

    fn into_box(self) -> BoxBiTransformer<T, U, R>
    where
        Self: Sized + 'static,
    {
        BoxBiTransformer::new(self)
    }

    fn into_rc(self) -> RcBiTransformer<T, U, R>
    where
        Self: Sized + 'static,
    {
        RcBiTransformer::new(self)
    }

    fn into_arc(self) -> ArcBiTransformer<T, U, R>
    where
        Self: Sized + Send + Sync + 'static,
    {
        ArcBiTransformer::new(self)
    }

    fn into_fn(self) -> impl Fn(T, U) -> R
    where
        Self: Sized + 'static,
    {
        move |t: T, u: U| self(t, u)
    }

    // use the default implementation of to_box(), to_rc(), to_arc() from
    // BiTransformer trait

    fn to_fn(&self) -> impl Fn(T, U) -> R
    where
        Self: Sized + Clone + 'static,
    {
        self.clone()
    }
}
