// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxTransformerOnce` public type.

use crate::BoxConditionalTransformerOnce;
use crate::Predicate;
use crate::TransformerOnce;
use crate::macros::impl_closure_once_trait;
use crate::transformers::macros::impl_box_transformer_methods;
use crate::transformers::macros::impl_transformer_common_methods;
use crate::transformers::macros::impl_transformer_debug_display;
use crate::transformers::macros::impl_transformer_identity_method;

// ============================================================================
// BoxTransformerOnce - Box<dyn FnOnce(T) -> R>
// ============================================================================

/// BoxTransformerOnce - consuming transformer wrapper based on
/// `Box<dyn FnOnce>`
///
/// A transformer wrapper that provides single ownership with one-time use
/// semantics. Consumes both self and the input value.
///
/// # Features
///
/// - **Based on**: `Box<dyn FnOnce(T) -> R>`
/// - **Ownership**: Single ownership, cannot be cloned
/// - **Reusability**: Can only be called once (consumes self and input)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
///
/// # Examples
///
/// ```
/// use qubit_function::{BoxTransformerOnce, TransformerOnce};
///
/// let double = BoxTransformerOnce::new(|value: i32| value * 2);
/// assert_eq!(double.apply(21), 42);
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxTransformerOnce<T, R> {
    /// The wrapped callback implementation.
    pub(super) function: Box<dyn FnOnce(T) -> R>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

// Implement BoxTransformerOnce
impl<T, R> BoxTransformerOnce<T, R> {
    impl_transformer_common_methods!(
        BoxTransformerOnce<T, R>,
        |f| Box::new(f)
    );

    impl_box_transformer_methods!(
        BoxTransformerOnce<T, R>,
        BoxConditionalTransformerOnce,
        TransformerOnce
    );
}

impl_transformer_identity_method!(BoxTransformerOnce, TransformerOnce);

// Implement TransformerOnce trait for BoxTransformerOnce
impl<T, R> TransformerOnce<T, R> for BoxTransformerOnce<T, R> {
    #[inline(always)]
    fn apply(self, input: T) -> R {
        (self.function)(input)
    }
}

impl<T, R> BoxTransformerOnce<T, R> {
    /// Creates a one-time transformer that returns the owned value.
    ///
    /// The value is moved into the transformer and does not need to implement
    /// `Clone`. If the transformer is dropped without being called, the
    /// captured value is dropped with it.
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_function::{BoxTransformerOnce, TransformerOnce};
    ///
    /// let constant = BoxTransformerOnce::constant(String::from("ready"));
    /// assert_eq!(constant.apply(0), "ready");
    /// ```
    pub fn constant(value: R) -> Self
    where
        R: 'static,
    {
        Self::new(move |_| value)
    }
}

// Use macro to generate Debug and Display implementations
impl_transformer_debug_display!(BoxTransformerOnce<T, R>);

// ============================================================================
// Blanket implementation for standard FnOnce trait
// ============================================================================

// Implement TransformerOnce for all FnOnce(T) -> R using macro
impl_closure_once_trait!(
    TransformerOnce<T, R>,
    apply,
    BoxTransformerOnce,
    FnOnce(input: T) -> R
);
