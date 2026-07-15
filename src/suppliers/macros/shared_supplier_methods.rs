// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! # Shared Supplier Methods Macro
//!
//! Generates `map`, `filter`, and `zip` for shared Arc/Rc suppliers. Each
//! method borrows `&self` and returns a wrapper from the same ownership and
//! statefulness family.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Supplier wrapper type.
//! * `$supplier_trait` - `Supplier` or `StatefulSupplier`.
//! * `$callback_bounds` - Explicit callback bounds for Arc invocations.
//! * `$extra_bounds` - Local callback and output bounds for Rc invocations;
//!   current Rc wrappers pass only `'static`.
//!
//! # Capability policy
//!
//! | Wrapper family | Mapper/predicate/other supplier | Output |
//! |----------------|---------------------------------|--------|
//! | `ArcSupplier` | `Send + Sync + 'static` | `'static` |
//! | `ArcStatefulSupplier` | `Send + 'static` | `'static` |
//! | Rc stateless/stateful | `'static` | `'static` |

/// Generates `map`, `filter`, and `zip` for shared Arc/Rc suppliers.
///
/// Invoke this macro inside the target wrapper's `impl` block. Stateless Arc
/// callbacks require `Sync` because they are called through shared references.
/// Stateful Arc callbacks are called under an outer mutex and require only
/// `Send`. Rc callbacks remain local and require only `'static`.
///
/// # Capability policy
///
/// | Wrapper family | Callback bounds |
/// |----------------|-----------------|
/// | `ArcSupplier` | `Send + Sync + 'static` |
/// | `ArcStatefulSupplier` | `Send + 'static` |
/// | Rc stateless/stateful | `'static` |
macro_rules! impl_shared_supplier_methods {
    (@let_supplier Supplier, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_supplier StatefulSupplier, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    // Arc case: T only needs 'static; callback capabilities are selected by
    // the wrapper family at the invocation site.
    (
        $struct_name:ident < $t:ident >,
        $supplier_trait:ident,
        callback_bounds = ($($callback_bounds:tt)+)
    ) => {
        /// Maps the output using a transformation function.
        ///
        /// # Parameters
        ///
        /// * `mapper` - The transformation function to apply
        ///
        /// # Returns
        ///
        /// A new `$struct_name<U>` with the mapped output
        ///
        /// # Examples
        ///
        /// ```rust
        /// use qubit_function::{ArcSupplier, Supplier};
        ///
        /// let source = ArcSupplier::new(|| 10);
        /// let mapped = source.map(|x| x * 2);
        /// // source is still usable
        /// assert_eq!(mapped.get(), 20);
        /// ```
        #[inline]
        pub fn map<U, M>(&self, mapper: M) -> $struct_name<U>
        where
            $t: 'static,
            M: Transformer<$t, U> + $($callback_bounds)+,
            U: 'static,
        {
            let metadata = self.metadata.clone();
            impl_shared_supplier_methods!(@let_supplier $supplier_trait, self_cloned, self.clone());
            $struct_name::new_with_metadata(
                move || {
                    let value = self_cloned.get();
                    mapper.apply(value)
                },
                metadata,
            )
        }

        /// Filters output based on a predicate.
        ///
        /// # Parameters
        ///
        /// * `predicate` - The predicate to test the supplied value
        ///
        /// # Returns
        ///
        /// A new filtered `$struct_name<Option<$t>>`
        ///
        /// # Examples
        ///
        /// ```rust
        /// use qubit_function::{ArcSupplier, Supplier};
        ///
        /// let source = ArcSupplier::new(|| 42);
        /// let filtered = source.filter(|x: &i32| x % 2 == 0);
        ///
        /// assert_eq!(filtered.get(), Some(42));
        /// ```
        #[inline]
        pub fn filter<P>(&self, predicate: P) -> $struct_name<Option<$t>>
        where
            $t: 'static,
            P: Predicate<$t> + $($callback_bounds)+,
        {
            impl_shared_supplier_methods!(@let_supplier $supplier_trait, self_cloned, self.clone());
            $struct_name::new(move || {
                let value = self_cloned.get();
                if predicate.test(&value) {
                    Some(value)
                } else {
                    None
                }
            })
        }

        /// Combines this supplier with another, producing a tuple.
        ///
        /// # Parameters
        ///
        /// * `other` - The other supplier to combine with
        ///
        /// # Returns
        ///
        /// A new `$struct_name<($t, U)>`
        ///
        /// # Examples
        ///
        /// ```rust
        /// use qubit_function::{ArcSupplier, Supplier};
        ///
        /// let first = ArcSupplier::new(|| 42);
        /// let second = ArcSupplier::new(|| "hello");
        ///
        /// let zipped = first.zip(second);
        ///
        /// assert_eq!(zipped.get(), (42, "hello"));
        /// ```
        #[inline]
        pub fn zip<U, S>(&self, other: S) -> $struct_name<($t, U)>
        where
            $t: 'static,
            S: $supplier_trait<U> + $($callback_bounds)+,
            U: 'static,
        {
            impl_shared_supplier_methods!(@let_supplier $supplier_trait, self_cloned, self.clone());
            impl_shared_supplier_methods!(@let_supplier $supplier_trait, other, other);
            $struct_name::new(move || {
                let first = self_cloned.get();
                let second = other.get();
                (first, second)
            })
        }
    };

    // Generic case for other bounds (e.g., 'static for Rc)
    (
        $struct_name:ident < $t:ident >,
        $supplier_trait:ident,
        ($($extra_bounds:tt)*)
    ) => {
        /// Maps the output using a transformation function.
        ///
        /// # Parameters
        ///
        /// * `mapper` - The transformation function to apply
        ///
        /// # Returns
        ///
        /// A new `$struct_name<U>` with the mapped output
        ///
        /// # Examples
        ///
        /// ```rust
        /// use qubit_function::{RcSupplier, Supplier};
        ///
        /// let source = RcSupplier::new(|| 10);
        /// let mapped = source.map(|x| x * 2);
        /// // source is still usable
        /// assert_eq!(mapped.get(), 20);
        /// ```
        #[inline]
        pub fn map<U, M>(&self, mapper: M) -> $struct_name<U>
        where
            $t: 'static,
            M: Transformer<$t, U> + $($extra_bounds)+,
            U: $($extra_bounds)+,
        {
            let metadata = self.metadata.clone();
            impl_shared_supplier_methods!(@let_supplier $supplier_trait, self_cloned, self.clone());
            $struct_name::new_with_metadata(
                move || {
                    let value = self_cloned.get();
                    mapper.apply(value)
                },
                metadata,
            )
        }

        /// Filters output based on a predicate.
        ///
        /// # Parameters
        ///
        /// * `predicate` - The predicate to test the supplied value
        ///
        /// # Returns
        ///
        /// A new filtered `$struct_name<Option<$t>>`
        ///
        /// # Examples
        ///
        /// ```rust
        /// use qubit_function::{RcSupplier, Supplier};
        ///
        /// let source = RcSupplier::new(|| 42);
        /// let filtered = source.filter(|x: &i32| x % 2 == 0);
        ///
        /// assert_eq!(filtered.get(), Some(42));
        /// ```
        #[inline]
        pub fn filter<P>(&self, predicate: P) -> $struct_name<Option<$t>>
        where
            $t: 'static,
            P: Predicate<$t> + $($extra_bounds)+,
        {
            impl_shared_supplier_methods!(@let_supplier $supplier_trait, self_cloned, self.clone());
            $struct_name::new(move || {
                let value = self_cloned.get();
                if predicate.test(&value) {
                    Some(value)
                } else {
                    None
                }
            })
        }

        /// Combines this supplier with another, producing a tuple.
        ///
        /// # Parameters
        ///
        /// * `other` - The other supplier to combine with
        ///
        /// # Returns
        ///
        /// A new `$struct_name<($t, U)>`
        ///
        /// # Examples
        ///
        /// ```rust
        /// use qubit_function::{RcSupplier, Supplier};
        ///
        /// let first = RcSupplier::new(|| 42);
        /// let second = RcSupplier::new(|| "hello");
        ///
        /// let zipped = first.zip(second);
        ///
        /// assert_eq!(zipped.get(), (42, "hello"));
        /// ```
        #[inline]
        pub fn zip<U, S>(&self, other: S) -> $struct_name<($t, U)>
        where
            $t: 'static,
            S: $supplier_trait<U> + $($extra_bounds)+,
            U: $($extra_bounds)+,
        {
            impl_shared_supplier_methods!(@let_supplier $supplier_trait, self_cloned, self.clone());
            impl_shared_supplier_methods!(@let_supplier $supplier_trait, other, other);
            $struct_name::new(move || {
                let first = self_cloned.get();
                let second = other.get();
                (first, second)
            })
        }
    };
}

pub(crate) use impl_shared_supplier_methods;
