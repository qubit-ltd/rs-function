// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxBiFunctionOnce` public type.

use crate::BiFunctionOnce;
use crate::BiPredicate;
use crate::BoxConditionalBiFunctionOnce;
use crate::FunctionOnce;
use crate::functions::macros::impl_box_function_methods;
use crate::functions::macros::impl_function_common_methods;
use crate::functions::macros::impl_function_debug_display;
use crate::macros::impl_closure_once_trait;

/// The erased callback representation used by this implementation.
type BoxBiFunctionOnceFn<T, U, R> = Box<dyn FnOnce(&T, &U) -> R>;

// ============================================================================
// BoxBiFunctionOnce - Box<dyn FnOnce(&T, &U) -> R>
// ============================================================================

/// BoxBiFunctionOnce - consuming bi-function wrapper based on
/// `Box<dyn FnOnce>`
///
/// A bi-function wrapper that provides single ownership with one-time use
/// semantics. Consumes self and borrows both input values.
///
/// # Features
///
/// - **Based on**: `Box<dyn FnOnce(&T, &U) -> R>`
/// - **Ownership**: Single ownership, cannot be cloned
/// - **Reusability**: Can only be called once (consumes self)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
///
/// # Type Parameters
///
/// * `T` - The type of the first borrowed input.
/// * `U` - The type of the second borrowed input.
/// * `R` - The output type produced by the callback.
///
/// # Examples
///
/// ```
/// use qubit_function::{BiFunctionOnce, BoxBiFunctionOnce};
///
/// let first = String::from("hello");
/// let second = String::from(" world");
/// let concatenate = BoxBiFunctionOnce::new(|first: &String, second: &String| {
///     format!("{first}{second}")
/// });
/// assert_eq!(concatenate.apply(&first, &second), "hello world");
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxBiFunctionOnce<T, U, R> {
    /// The wrapped callback implementation.
    pub(super) function: BoxBiFunctionOnceFn<T, U, R>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

// Implement BoxBiFunctionOnce
impl<T, U, R> BoxBiFunctionOnce<T, U, R> {
    // Generate new(), new_with_name(), new_with_optional_name(), name(),
    // set_name()
    impl_function_common_methods!(
        BoxBiFunctionOnce<T, U, R>,
        (FnOnce(&T, &U) -> R + 'static),
        |f| Box::new(f)
    );

    // Generate when(), and_then()
    impl_box_function_methods!(
        BoxBiFunctionOnce<T, U, R>,
        BoxConditionalBiFunctionOnce,
        FunctionOnce
    );

    /// Creates a function that returns the captured value when called.
    ///
    /// The value is moved into the function and returned by ownership, so `R`
    /// does not need to implement `Clone`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BiFunctionOnce, BoxBiFunctionOnce};
    ///
    /// let constant =
    ///     BoxBiFunctionOnce::<i32, bool, &'static str>::constant("hello");
    /// assert_eq!(constant.apply(&123, &true), "hello");
    /// ```
    pub fn constant(value: R) -> Self
    where
        R: 'static,
    {
        Self::new(move |_: &T, _: &U| value)
    }
}

// Implement BiFunctionOnce trait for BoxBiFunctionOnce
impl<T, U, R> BiFunctionOnce<T, U, R> for BoxBiFunctionOnce<T, U, R> {
    #[inline(always)]
    fn apply(self, first: &T, second: &U) -> R {
        (self.function)(first, second)
    }
}

// Use macro to generate Debug and Display implementations
impl_function_debug_display!(BoxBiFunctionOnce<T, U, R>);

// ============================================================================
// Blanket implementation for standard FnOnce trait
// ============================================================================

// Implement BiFunctionOnce for all FnOnce(&T, &U) -> R using macro
impl_closure_once_trait!(
    BiFunctionOnce<T, U, R>,
    apply,
    BoxBiFunctionOnce,
    FnOnce(first: &T, second: &U) -> R
);
