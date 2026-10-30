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
    (@map_doc Supplier, $supplier:ident) => {
        concat!("```rust\nuse qubit_function::{", stringify!($supplier), ", Supplier};\n",
            "let source = ", stringify!($supplier), "::new(|| 10);\n",
            "let mapped = source.map(|value| value * 2);\n",
            "assert_eq!(mapped.get(), 20);\nassert_eq!(source.get(), 10);\n```")
    };
    (@map_doc StatefulSupplier, $supplier:ident) => {
        concat!("```rust\nuse qubit_function::{", stringify!($supplier), ", StatefulSupplier};\n",
            "let mut source_calls = 0;\n",
            "let mut source = ", stringify!($supplier), "::new(move || { source_calls += 1; source_calls * 10 });\n",
            "let mut mapped = source.map(|value| value * 2);\n",
            "assert_eq!(mapped.get(), 20);\nassert_eq!(source.get(), 20);\n```")
    };
    (@filter_doc Supplier, $supplier:ident) => {
        concat!("```rust\nuse qubit_function::{", stringify!($supplier), ", Supplier};\n",
            "let source = ", stringify!($supplier), "::new(|| 42);\n",
            "let filtered = source.filter(|value: &i32| *value % 2 == 0);\n",
            "assert_eq!(filtered.get(), Some(42));\nassert_eq!(source.get(), 42);\n```")
    };
    (@filter_doc StatefulSupplier, $supplier:ident) => {
        concat!("```rust\nuse qubit_function::{", stringify!($supplier), ", StatefulSupplier};\n",
            "let mut source_calls = 0;\n",
            "let mut source = ", stringify!($supplier), "::new(move || { source_calls += 1; 42 });\n",
            "let mut filtered = source.filter(|value: &i32| *value % 2 == 0);\n",
            "assert_eq!(filtered.get(), Some(42));\nassert_eq!(source.get(), 42);\n```")
    };
    (@zip_doc Supplier, $supplier:ident) => {
        concat!("```rust\nuse qubit_function::{", stringify!($supplier), ", Supplier};\n",
            "let first = ", stringify!($supplier), "::new(|| 42);\n",
            "let second = ", stringify!($supplier), "::new(|| \"hello\");\n",
            "let zipped = first.zip(second);\n",
            "assert_eq!(zipped.get(), (42, \"hello\"));\nassert_eq!(first.get(), 42);\n```")
    };
    (@zip_doc StatefulSupplier, $supplier:ident) => {
        concat!("```rust\nuse qubit_function::{", stringify!($supplier), ", StatefulSupplier};\n",
            "let mut first_calls = 0;\nlet mut second_calls = 0;\n",
            "let mut first = ", stringify!($supplier), "::new(move || { first_calls += 1; first_calls });\n",
            "let second = ", stringify!($supplier), "::new(move || { second_calls += 1; second_calls });\n",
            "let mut zipped = first.zip(second);\n",
            "assert_eq!(zipped.get(), (1, 1));\nassert_eq!(first.get(), 2);\n```")
    };

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
        #[doc = concat!(impl_shared_supplier_methods!(@map_doc $supplier_trait, $struct_name))]
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
        #[doc = concat!(impl_shared_supplier_methods!(@filter_doc $supplier_trait, $struct_name))]
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
        #[doc = concat!(impl_shared_supplier_methods!(@zip_doc $supplier_trait, $struct_name))]
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
        #[doc = concat!(impl_shared_supplier_methods!(@map_doc $supplier_trait, $struct_name))]
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
        #[doc = concat!(impl_shared_supplier_methods!(@filter_doc $supplier_trait, $struct_name))]
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
        #[doc = concat!(impl_shared_supplier_methods!(@zip_doc $supplier_trait, $struct_name))]
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
