// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! # Box Supplier Methods Macro
//!
//! Generates map, filter, and zip method implementations for Box-based Supplier
//!
//! Generates transformation methods for Box-based suppliers that consume self
//! (because Box cannot be cloned).
//!
//! This macro supports single-parameter suppliers.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - The struct name with its generic parameters
//!   - Single parameter: `BoxSupplier<T>`
//! * `$supplier_trait` - Supplier trait name (e.g., Supplier, StatefulSupplier)
//!
//! # Parameter Usage Comparison
//!
//! | Supplier Type | Struct Signature | `$supplier_trait` |
//! |---------------|-----------------|------------------|
//! | **Supplier** | `BoxSupplier<T>` | Supplier |
//! | **SupplierOnce** | `BoxSupplierOnce<T>` | SupplierOnce |
//! | **StatefulSupplier** | `BoxStatefulSupplier<T>` | StatefulSupplier |
//!
//! # Examples
//!
//! ```text
//! // Example usage pattern:
//! // impl_box_supplier_methods!(
//! //     BoxSupplier<T>,
//! //     Supplier
//! // );
//! // (private to crate implementation)
//! ```

/// Generates map, filter, and zip method implementations for Box-based Supplier
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete impl block with methods for the specified struct.
/// Generates transformation methods for Box-based suppliers that consume self
/// (because Box cannot be cloned).
///
/// This macro supports single-parameter suppliers.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - The struct name with its generic parameters
///   - Single parameter: `BoxSupplier<T>`
/// * `$supplier_trait` - Supplier trait name (e.g., Supplier, StatefulSupplier)
///
/// # Parameter Usage Comparison
///
/// | Supplier Type | Struct Signature | `$supplier_trait` |
/// |---------------|-----------------|------------------|
/// | **Supplier** | `BoxSupplier<T>` | Supplier |
/// | **SupplierOnce** | `BoxSupplierOnce<T>` | SupplierOnce |
/// | **StatefulSupplier** | `BoxStatefulSupplier<T>` | StatefulSupplier |
///
/// # Examples
///
/// ```text
/// // The macro is only available within the crate where it is defined.
/// // Use this macro from `qubit_function::suppliers` when implementing
/// // supplier methods in that module.
/// ```
macro_rules! impl_box_supplier_methods {
    (@map_doc Supplier, $supplier:ident) => {
        concat!("```rust\nuse qubit_function::{", stringify!($supplier), ", Supplier};\n",
            "let supplier = ", stringify!($supplier), "::new(|| 10);\n",
            "let mapped = supplier.map(|value| value * 2);\n",
            "assert_eq!(mapped.get(), 20);\n```")
    };
    (@map_doc StatefulSupplier, $supplier:ident) => {
        concat!("```rust\nuse qubit_function::{", stringify!($supplier), ", StatefulSupplier};\n",
            "let mut source_calls = 0;\n",
            "let supplier = ", stringify!($supplier), "::new(move || { source_calls += 1; source_calls * 10 });\n",
            "let mut mapped = supplier.map(|value| value * 2);\n",
            "assert_eq!(mapped.get(), 20);\n```")
    };
    (@map_doc SupplierOnce, $supplier:ident) => {
        concat!("```rust\nuse qubit_function::{", stringify!($supplier), ", SupplierOnce};\n",
            "let payload = String::from(\"item\");\n",
            "let supplier = ", stringify!($supplier), "::new(move || payload);\n",
            "let mapped = supplier.map(|value| format!(\"{value}!\"));\n",
            "assert_eq!(mapped.get(), \"item!\");\n```")
    };
    (@filter_doc Supplier, $supplier:ident) => {
        concat!("```rust\nuse qubit_function::{", stringify!($supplier), ", Supplier};\n",
            "let supplier = ", stringify!($supplier), "::new(|| 42);\n",
            "let filtered = supplier.filter(|value: &i32| *value % 2 == 0);\n",
            "assert_eq!(filtered.get(), Some(42));\n```")
    };
    (@filter_doc StatefulSupplier, $supplier:ident) => {
        concat!("```rust\nuse qubit_function::{", stringify!($supplier), ", StatefulSupplier};\n",
            "let mut source_calls = 0;\n",
            "let supplier = ", stringify!($supplier), "::new(move || { source_calls += 1; 42 });\n",
            "let mut filtered = supplier.filter(|value: &i32| *value % 2 == 0);\n",
            "assert_eq!(filtered.get(), Some(42));\n```")
    };
    (@filter_doc SupplierOnce, $supplier:ident) => {
        concat!("```rust\nuse qubit_function::{", stringify!($supplier), ", SupplierOnce};\n",
            "let payload = String::from(\"item\");\n",
            "let supplier = ", stringify!($supplier), "::new(move || payload);\n",
            "let filtered = supplier.filter(|value: &String| value == \"item\");\n",
            "assert_eq!(filtered.get(), Some(String::from(\"item\")));\n```")
    };
    (@zip_doc Supplier, $supplier:ident) => {
        concat!("```rust\nuse qubit_function::{", stringify!($supplier), ", Supplier};\n",
            "let first = ", stringify!($supplier), "::new(|| 42);\n",
            "let second = ", stringify!($supplier), "::new(|| \"hello\");\n",
            "let zipped = first.zip(second);\n",
            "assert_eq!(zipped.get(), (42, \"hello\"));\n```")
    };
    (@zip_doc StatefulSupplier, $supplier:ident) => {
        concat!("```rust\nuse qubit_function::{", stringify!($supplier), ", StatefulSupplier};\n",
            "let mut first_calls = 0;\nlet mut second_calls = 0;\n",
            "let first = ", stringify!($supplier), "::new(move || { first_calls += 1; first_calls });\n",
            "let second = ", stringify!($supplier), "::new(move || { second_calls += 1; second_calls });\n",
            "let mut zipped = first.zip(second);\n",
            "assert_eq!(zipped.get(), (1, 1));\n```")
    };
    (@zip_doc SupplierOnce, $supplier:ident) => {
        concat!("```rust\nuse qubit_function::{", stringify!($supplier), ", SupplierOnce};\n",
            "let payload = String::from(\"item\");\n",
            "let first = ", stringify!($supplier), "::new(move || payload);\n",
            "let second = ", stringify!($supplier), "::new(|| 7);\n",
            "let zipped = first.zip(second);\n",
            "assert_eq!(zipped.get(), (String::from(\"item\"), 7));\n```")
    };

    (@let_supplier_fn Supplier, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_supplier_fn SupplierOnce, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_supplier_fn StatefulSupplier, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_supplier Supplier, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_supplier SupplierOnce, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_supplier StatefulSupplier, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    // Single generic parameter - Supplier
    (
        $struct_name:ident < $t:ident >,
        $supplier_trait:ident
    ) => {
        /// Maps the output using a transformation function.
        ///
        /// Consumes self and returns a new supplier that applies the
        /// mapper to each output.
        ///
        /// # Parameters
        ///
        /// * `mapper` - The transformer to apply to the output. Can be a
        ///   closure, function pointer, or any type implementing
        ///   `Transformer<T, U>`.
        ///
        /// # Returns
        ///
        /// A new mapped supplier
        ///
        /// # Examples
        ///
        #[doc = concat!(impl_box_supplier_methods!(@map_doc $supplier_trait, $struct_name))]
        pub fn map<U, M>(self, mapper: M) -> $struct_name<U>
        where
            $t: 'static,
            M: Transformer<$t, U> + 'static,
            U: 'static,
        {
            let metadata = self.metadata;
            impl_box_supplier_methods!(@let_supplier_fn $supplier_trait, self_fn, self.function);
            $struct_name::new_with_metadata(
                move || mapper.apply(self_fn()),
                metadata,
            )
        }

        /// Filters output based on a predicate.
        ///
        /// Returns a new supplier that returns `Some(value)` if the
        /// predicate is satisfied, `None` otherwise.
        ///
        /// # Parameters
        ///
        /// * `predicate` - The predicate to test the supplied value
        ///
        /// # Returns
        ///
        /// A new filtered supplier
        ///
        /// # Examples
        ///
        #[doc = concat!(impl_box_supplier_methods!(@filter_doc $supplier_trait, $struct_name))]
        pub fn filter<P>(self, predicate: P) -> $struct_name<Option<$t>>
        where
            $t: 'static,
            P: Predicate<$t> + 'static,
        {
            impl_box_supplier_methods!(@let_supplier_fn $supplier_trait, self_fn, self.function);
            $struct_name::new(move || {
                let value = self_fn();
                if predicate.test(&value) {
                    Some(value)
                } else {
                    None
                }
            })
        }

        /// Combines this supplier with another, producing a tuple.
        ///
        /// Consumes both suppliers and returns a new supplier that
        /// produces tuples.
        ///
        /// # Parameters
        ///
        /// * `other` - The other supplier to combine with
        ///
        /// # Returns
        ///
        /// A new supplier that produces tuples
        ///
        /// # Examples
        ///
        #[doc = concat!(impl_box_supplier_methods!(@zip_doc $supplier_trait, $struct_name))]
        pub fn zip<U, S>(self, other: S) -> $struct_name<($t, U)>
        where
            $t: 'static,
            S: $supplier_trait<U> + 'static,
            U: 'static,
        {
            impl_box_supplier_methods!(@let_supplier_fn $supplier_trait, self_fn, self.function);
            impl_box_supplier_methods!(@let_supplier $supplier_trait, other, other);
            $struct_name::new(move || (self_fn(), other.get()))
        }
    };
}

pub(crate) use impl_box_supplier_methods;
