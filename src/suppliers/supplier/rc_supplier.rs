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
//! Defines the `RcSupplier` public type.

use super::{
    ArcSupplier,
    BoxSupplier,
    BoxSupplierOnce,
    Predicate,
    Rc,
    Supplier,
    Transformer,
    impl_closure_trait,
    impl_rc_conversions,
    impl_shared_supplier_methods,
    impl_supplier_clone,
    impl_supplier_common_methods,
    impl_supplier_debug_display,
};

// ======================================================================
// RcSupplier - Single-threaded Shared Ownership
// ======================================================================

/// Single-threaded shared ownership stateless supplier.
///
/// Uses `Rc<dyn Fn() -> T>` for single-threaded shared ownership.
/// Can be cloned but not sent across threads.
///
/// # Ownership Model
///
/// Like `ArcSupplier`, methods borrow `&self` instead of
/// consuming `self`:
///
/// ```rust
/// use qubit_function::{RcSupplier, Supplier};
///
/// let source = RcSupplier::new(|| 10);
/// let mapped = source.map(|x| x * 2);
/// // source is still usable here!
/// ```
///
/// # Examples
///
/// ## Shared Factory
///
/// ```rust
/// use qubit_function::{RcSupplier, Supplier};
///
/// let factory = RcSupplier::new(|| {
///     String::from("Hello")
/// });
///
/// let f1 = factory.clone();
/// let f2 = factory.clone();
/// assert_eq!(f1.get(), "Hello");
/// assert_eq!(f2.get(), "Hello");
/// ```
///
/// ## Reusable Transformations
///
/// ```rust
/// use qubit_function::{RcSupplier, Supplier};
///
/// let base = RcSupplier::new(|| 10);
/// let doubled = base.map(|x| x * 2);
/// let tripled = base.map(|x| x * 3);
///
/// assert_eq!(base.get(), 10);
/// assert_eq!(doubled.get(), 20);
/// assert_eq!(tripled.get(), 30);
/// ```
///
pub struct RcSupplier<T> {
    pub(super) function: Rc<dyn Fn() -> T>,
    pub(super) name: Option<String>,
}

impl<T> RcSupplier<T> {
    // Generates: new(), new_with_name(), name(), set_name(), constant()
    impl_supplier_common_methods!(RcSupplier<T>, (Fn() -> T + 'static), |f| Rc::new(f));

    // Generates: map(), filter(), zip()
    impl_shared_supplier_methods!(
        RcSupplier<T>,
        Supplier,
        ('static)
    );
}

// Generates: Debug and Display implementations for RcSupplier<T>
impl_supplier_debug_display!(RcSupplier<T>);

// Generates: Clone implementation for RcSupplier<T>
impl_supplier_clone!(RcSupplier<T>);

impl<T> Supplier<T> for RcSupplier<T> {
    fn get(&self) -> T {
        (self.function)()
    }

    // Generate all conversion methods using the unified macro
    impl_rc_conversions!(
        RcSupplier<T>,
        BoxSupplier,
        BoxSupplierOnce,
        Fn() -> T
    );
}

// ======================================================================
// Implement Supplier for Closures
// ======================================================================

// Implement Supplier<T> for any type that implements Fn() -> T
impl_closure_trait!(
    Supplier<T>,
    get,
    BoxSupplierOnce,
    Fn() -> T
);

// ======================================================================
// Note on Extension Traits for Closures
// ======================================================================
//
// We don't provide `FnSupplierOps` trait for `Fn() -> T` closures
// because:
//
// 1. All `Fn` closures also implement `FnMut`, so they can use `FnSupplierOps`
//    from the `supplier` module
// 2. Providing both would cause ambiguity errors due to overlapping trait impls
// 3. Rust doesn't support negative trait bounds to exclude `FnMut`
//
// Users of `Fn` closures should use `FnSupplierOps` from `supplier` module,
// or explicitly convert to `BoxSupplier` using `.into_box()` first.
