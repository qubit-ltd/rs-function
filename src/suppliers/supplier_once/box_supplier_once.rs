// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxSupplierOnce` public type.

use crate::Predicate;
use crate::SupplierOnce;
use crate::Transformer;
use crate::TransformerOnce;
use crate::macros::impl_closure_once_trait;
use crate::suppliers::macros::impl_box_supplier_methods;
use crate::suppliers::macros::impl_supplier_debug_display;
use crate::suppliers::macros::impl_supplier_new_methods;

// ==========================================================================
// BoxSupplierOnce - One-time Supplier Implementation
// ==========================================================================

/// Box-based one-time supplier.
///
/// Uses `Box<dyn FnOnce() -> T>` for one-time value generation.
/// Can only call `get()` once, consuming the supplier.
///
/// # Type Parameters
///
/// - `T`: The value produced when the supplier is consumed.
///
/// # Examples
///
/// ## Lazy Initialization
///
/// ```rust
/// use qubit_function::{BoxSupplierOnce, SupplierOnce};
///
/// let once = BoxSupplierOnce::new(|| {
///     println!("Expensive initialization");
///     42
/// });
///
/// let value = once.get(); // Prints: Expensive initialization
/// assert_eq!(value, 42);
/// ```
///
/// ## Moving Captured Values
///
/// ```rust
/// use qubit_function::{BoxSupplierOnce, SupplierOnce};
///
/// let resource = String::from("data");
/// let once = BoxSupplierOnce::new(move || resource);
///
/// let value = once.get();
/// assert_eq!(value, "data");
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxSupplierOnce<T> {
    /// The wrapped callback implementation.
    pub(super) function: Box<dyn FnOnce() -> T>,
    /// Diagnostic metadata associated with this callback.
    pub(super) metadata: crate::internal::CallbackMetadata,
}

impl<T> BoxSupplierOnce<T> {
    impl_supplier_new_methods!(BoxSupplierOnce<T>, |f| Box::new(f));
    crate::macros::impl_common_name_methods!("supplier");

    /// Creates a one-time supplier that returns the owned value.
    ///
    /// The value is moved into the supplier and does not need to implement
    /// `Clone`. If the supplier is dropped without being called, the captured
    /// value is dropped with it.
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_function::{BoxSupplierOnce, SupplierOnce};
    ///
    /// let constant = BoxSupplierOnce::constant(String::from("ready"));
    /// assert_eq!(constant.get(), "ready");
    /// ```
    pub fn constant(value: T) -> Self
    where
        T: 'static,
    {
        Self::new(move || value)
    }

    // Generates: map(), filter(), zip()
    impl_box_supplier_methods!(BoxSupplierOnce<T>, SupplierOnce);

    /// Maps the value produced by this supplier with a consuming transformer.
    ///
    /// The supplier and mapper are both consumed. The mapper runs after the
    /// supplier and can move captured values into its output without cloning.
    /// The source supplier's diagnostic name is preserved. Panics from either
    /// callback propagate to the caller.
    ///
    /// # Parameters
    ///
    /// * `mapper` - A one-time transformer applied to the supplied value.
    ///
    /// # Returns
    ///
    /// A one-time supplier that returns the mapped value.
    ///
    /// # Examples
    ///
    /// ```
    /// use qubit_function::{BoxSupplierOnce, SupplierOnce, TransformerOnce};
    ///
    /// let resource = String::from("owned");
    /// let mapped = BoxSupplierOnce::new(|| 7).map_once(move |value| (value, resource));
    /// assert_eq!(mapped.get(), (7, String::from("owned")));
    /// ```
    pub fn map_once<U, M>(self, mapper: M) -> BoxSupplierOnce<U>
    where
        T: 'static,
        U: 'static,
        M: TransformerOnce<T, U> + 'static,
    {
        let metadata = self.metadata;
        let source = self.function;
        BoxSupplierOnce::new_with_metadata(move || mapper.apply(source()), metadata)
    }
}

// Generates: implement SupplierOnce for BoxSupplierOnce<T>
impl<T> SupplierOnce<T> for BoxSupplierOnce<T> {
    #[inline(always)]
    fn get(self) -> T {
        (self.function)()
    }
}

// Generates: Debug and Display implementations for BoxSupplierOnce<T>
impl_supplier_debug_display!(BoxSupplierOnce<T>);

// ==========================================================================
// Implement SupplierOnce for Closures
// ==========================================================================

// Implement SupplierOnce for all FnOnce() -> T using macro
impl_closure_once_trait!(
    SupplierOnce<T>,
    get,
    BoxSupplierOnce,
    FnOnce() -> T
);
