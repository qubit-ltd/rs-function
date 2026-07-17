// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the `BoxSupplier` public type.

use {
    crate::Predicate,
    crate::Transformer,
};
use {
    crate::Supplier,
    crate::suppliers::macros::impl_box_supplier_methods,
    crate::suppliers::macros::impl_supplier_common_methods,
    crate::suppliers::macros::impl_supplier_debug_display,
};

// ======================================================================
// BoxSupplier - Single Ownership Implementation
// ======================================================================

/// Box-based single ownership stateless supplier.
///
/// Uses `Box<dyn Fn() -> T>` for single ownership scenarios. This
/// is the most lightweight stateless supplier with zero reference
/// counting overhead.
///
/// # Ownership Model
///
/// Methods consume `self` (move semantics) or borrow `&self` for
/// read-only operations. When you call methods like `map()`, the
/// original supplier is consumed and you get a new one:
///
///
/// ```rust
/// # {
/// use qubit_function::{BoxSupplier, Supplier};
///
/// let supplier = BoxSupplier::new(|| 10);
/// let mapped = supplier.map(|x| x * 2);
/// // supplier is no longer usable here
/// # }
/// ```
///
/// # Examples
///
/// ## Constant Factory
///
/// ```rust
/// use qubit_function::{BoxSupplier, Supplier};
///
/// let factory = BoxSupplier::new(|| 42);
/// assert_eq!(factory.get(), 42);
/// assert_eq!(factory.get(), 42);
/// ```
///
/// ## Method Chaining
///
///
/// ```rust
/// # {
/// use qubit_function::{BoxSupplier, Supplier};
///
/// let pipeline = BoxSupplier::new(|| 10)
///     .map(|x| x * 2)
///     .map(|x| x + 5);
///
/// assert_eq!(pipeline.get(), 25);
/// # }
/// ```
#[must_use = "callback wrappers do nothing unless stored or invoked"]
pub struct BoxSupplier<T> {
    pub(super) function: Box<dyn Fn() -> T>,
    pub(super) metadata: crate::callback_metadata::CallbackMetadata,
}

impl<T> BoxSupplier<T> {
    // Generates: new(), new_with_name(), name(), set_name(), constant()
    impl_supplier_common_methods!(BoxSupplier<T>, (Fn() -> T + 'static), |f| {
        Box::new(f)
    });

    // Generates: map(), filter(), zip()
    impl_box_supplier_methods!(BoxSupplier<T>, Supplier);
}

// Generates: Debug and Display implementations for BoxSupplier<T>
impl_supplier_debug_display!(BoxSupplier<T>);

impl<T> Supplier<T> for BoxSupplier<T> {
    fn get(&self) -> T {
        (self.function)()
    }
}
