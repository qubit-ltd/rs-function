/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Supplier Common Methods Macro
//!
//! Generates common Supplier methods (new, new_with_name, name,
//! set_name, constant)
//!
//! Generates constructor methods, name management methods and constant
//! constructor for Supplier structs. This macro should be called inside
//! an impl block.
//!
//! The macro automatically detects the number of generic parameters and
//! generates the appropriate implementations for single-parameter or
//! two-parameter suppliers.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Struct name with generic parameters
//! * `$fn_trait_with_bounds` - Closure trait with complete bounds
//!   (e.g., `Fn() -> T + 'static`)
//! * `$wrapper_expr` - Wrapper expression (uses `f` for the closure)
//!
//! # Usage
//!
//! ```ignore
//! // Single generic parameter - Supplier
//! impl_supplier_common_methods!(
//!     BoxSupplier<T>,
//!     (Fn() -> T + 'static),
//!     |f| Box::new(f)
//! );
//!
//! // Single generic parameter - StatefulSupplier
//! impl_supplier_common_methods!(
//!     ArcStatefulSupplier<T>,
//!     (FnMut() -> T + Send + 'static),
//!     |f| Arc::new(Mutex::new(f))
//! );
//! ```
//!
//! # Generated Methods
//!
//! * `new()` - Creates a new supplier
//! * `new_with_name()` - Creates a named supplier
//! * `name()` - Gets the name of the supplier
//! * `set_name()` - Sets the name of the supplier
//! * `constant()` - Creates a supplier that returns a constant value
//!
//! # Author
//!
//! Haixing Hu

/// Generates common Supplier methods (new, new_with_name, name,
/// set_name, constant)
///
/// This macro should be used inside an existing impl block for the target
/// struct. It generates individual methods but does not create a complete
/// impl block itself. Generates constructor methods, name management methods
/// and constant constructor for Supplier structs.
///
/// The macro automatically detects the number of generic parameters and
/// generates the appropriate implementations for single-parameter or
/// two-parameter suppliers.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - Struct name with generic parameters
/// * `$fn_trait_with_bounds` - Closure trait with complete bounds
///   (e.g., `Fn() -> T + 'static`)
/// * `$wrapper_expr` - Wrapper expression (uses `f` for the closure)
///
/// # Usage
///
/// ```ignore
/// // Single generic parameter - Supplier
/// impl_supplier_common_methods!(
///     BoxSupplier<T>,
///     (Fn() -> T + 'static),
///     |f| Box::new(f)
/// );
///
/// // Single generic parameter - StatefulSupplier
/// impl_supplier_common_methods!(
///     ArcStatefulSupplier<T>,
///     (FnMut() -> T + Send + 'static),
///     |f| Arc::new(Mutex::new(f))
/// );
/// ```
///
/// # Generated Methods
///
/// * `new()` - Creates a new supplier
/// * `new_with_name()` - Creates a named supplier
/// * `name()` - Gets the name of the supplier
/// * `set_name()` - Sets the name of the supplier
/// * `constant()` - Creates a supplier that returns a constant value
macro_rules! impl_supplier_common_methods {
    // Internal rule: generates new and new_with_name methods
    // Parameters:
    //   $fn_trait_with_bounds - Function trait bounds
    //   $f - Closure parameter name
    //   $wrapper_expr - Wrapper expression
    //   $type_desc - Type description for docs (e.g., "supplier" or "stateful-supplier")
    (@new_methods
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr,
        $type_desc:literal
    ) => {
        #[doc = concat!("Creates a new ", $type_desc, ".")]
        ///
        #[doc = concat!("Wraps the provided closure in the appropriate smart pointer type for this ", $type_desc, " implementation.")]
        ///
        /// # Type Parameters
        ///
        /// * `F` - The closure type
        ///
        /// # Parameters
        ///
        /// * `f` - The closure to wrap
        ///
        /// # Returns
        ///
        #[doc = concat!("Returns a new ", $type_desc, " instance wrapping the closure.")]
        pub fn new<F>($f: F) -> Self
        where
            F: $($fn_trait_with_bounds)+,
        {
            Self {
                function: $wrapper_expr,
                name: None,
            }
        }

        #[doc = concat!("Creates a new named ", $type_desc, ".")]
        ///
        /// Wraps the provided closure and assigns it a name, which is
        /// useful for debugging and logging purposes.
        ///
        /// # Type Parameters
        ///
        /// * `F` - The closure type
        ///
        /// # Parameters
        ///
        #[doc = concat!("* `name` - The name for this ", $type_desc)]
        /// * `f` - The closure to wrap
        ///
        /// # Returns
        ///
        #[doc = concat!("Returns a new named ", $type_desc, " instance wrapping the closure.")]
        pub fn new_with_name<F>(name: &str, $f: F) -> Self
        where
            F: $($fn_trait_with_bounds)+,
        {
            Self {
                function: $wrapper_expr,
                name: Some(name.to_string()),
            }
        }
    };

    // Internal rule: generates name and set_name methods
    (@name_methods $type_desc:literal) => {
        #[doc = concat!("Gets the name of this ", $type_desc, ".")]
        ///
        /// # Returns
        ///
        /// Returns `Some(&str)` if a name was set, `None` otherwise.
        pub fn name(&self) -> Option<&str> {
            self.name.as_deref()
        }

        #[doc = concat!("Sets the name of this ", $type_desc, ".")]
        ///
        /// # Parameters
        ///
        #[doc = concat!("* `name` - The name to set for this ", $type_desc)]
        pub fn set_name(&mut self, name: &str) {
            self.name = Some(name.to_string());
        }
    };

    // Single generic parameter - Supplier types
    (
        $struct_name:ident < $t:ident >,
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr
    ) => {
        impl_supplier_common_methods!(@new_methods
            ($($fn_trait_with_bounds)+),
            |$f| $wrapper_expr,
            "supplier"
        );

        impl_supplier_common_methods!(@name_methods "supplier");

        /// Creates a supplier that returns a constant value.
        ///
        /// Creates a supplier that always returns the same value. Useful for
        /// default values or placeholder implementations.
        ///
        /// # Parameters
        ///
        /// * `value` - The constant value to return
        ///
        /// # Returns
        ///
        /// Returns a new supplier instance that returns the constant value.
        pub fn constant(value: $t) -> Self
        where
            $t: Clone + 'static,
        {
            Self::new(move || value.clone())
        }
    };
}

pub(crate) use impl_supplier_common_methods;
