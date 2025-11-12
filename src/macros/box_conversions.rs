/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Box Conversions Macro
//!
//! Generate common conversion methods for all Box-based function wrappers.
//!
//! This macro uses a unified pattern to generate standard conversion methods
//! for all Box-based function wrapper types (into_box, into_rc, into_fn,
//! into_once).
//!
//! # Author
//!
//! Hu Haixing

/// Implement common conversion methods for Box types
///
/// This macro generates standard conversion methods for Box-based function
/// wrappers.
///
/// # Parameters
///
/// * `$box_type<$(generics),*>` - Box wrapper type (e.g., `BoxConsumer<T>`)
/// * `$rc_type` - Corresponding Rc wrapper type (e.g., `RcConsumer`)
/// * `$fn_trait` - Function trait (e.g., `Fn(&T)`, `Fn(&T) -> bool`)
/// * `$once_type` - Corresponding once wrapper type (optional, e.g.,
///   `BoxConsumerOnce`)
///
/// # Generated methods
///
/// * `into_box(self) -> BoxType` - Zero-cost conversion, returns self
/// * `into_rc(self) -> RcType` - Convert to Rc-based wrapper
/// * `into_fn(self) -> impl FnTrait` - Extract underlying function
/// * `into_once(self) -> OnceType` - Convert to once wrapper (only when
///   once_type is provided)
///
/// # Examples
///
/// ```ignore
/// // 3-parameter version (no once type)
/// impl_box_conversions!(
///     BoxPredicate<T>,
///     RcPredicate,
///     Fn(&T) -> bool
/// );
///
/// // 4-parameter version (with once type)
/// impl_box_conversions!(
///     BoxConsumer<T>,
///     RcConsumer,
///     Fn(&T),
///     BoxConsumerOnce
/// );
/// ```
///
/// # Author
///
/// Hu Haixing
macro_rules! impl_box_conversions {
    // 3-parameter pattern: box_type, rc_type, fn_trait (no once_type)
    (
        $box_type:ident < $($generics:ident),* >,
        $rc_type:ident,
        $fn_trait:path
    ) => {
        fn into_box(self) -> $box_type<$($generics),*>
        where
            $($generics: 'static),*
        {
            self
        }

        fn into_rc(self) -> $rc_type<$($generics),*>
        where
            $($generics: 'static),*
        {
            $rc_type::new_with_optional_name(self.function, self.name)
        }

        fn into_fn(self) -> impl $fn_trait
        {
            self.function
        }
    };

    // 4-parameter pattern: box_type, rc_type, fn_trait, once_type
    // Reuse 3-parameter version to generate into_box, into_rc, into_fn
    (
        $box_type:ident < $($generics:ident),* >,
        $rc_type:ident,
        $fn_trait:path,
        $once_type:ident
    ) => {
        // Reuse 3-parameter version to generate into_box, into_rc, into_fn
        impl_box_conversions!(
            $box_type < $($generics),* >,
            $rc_type,
            $fn_trait
        );

        fn into_once(self) -> $once_type<$($generics),*>
        where
            $($generics: 'static),*
        {
            $once_type::new_with_optional_name(self.function, self.name)
        }
    };
}

pub(crate) use impl_box_conversions;

/// Implement common conversion methods for Box*Once types
///
/// This macro generates standard conversion methods for all Box*Once types
/// that implement their respective traits (into_box, into_fn).
///
/// The macro unifies the pattern for both void-returning functions (like
/// Consumer, Mutator) and value-returning functions (like Function,
/// Transformer, Supplier).
///
/// # Parameters
///
/// * `$box_type_with_generics` - Box type with generics (e.g.,
///   `BoxConsumerOnce<T>`, `BoxBiConsumerOnce<T, U>`)
/// * `$trait_name` - Trait name (for documentation, unused in expansion)
/// * `$fn_trait` - Function trait type (e.g., `FnOnce(&T)`,
///   `FnOnce(&T) -> R`, `FnOnce() -> T`)
///
/// # Generated methods
///
/// * `into_box(self) -> BoxType` - Zero-cost conversion, returns self
/// * `into_fn(self) -> impl FnOnce(...)` - Extract underlying function
///
/// # Examples
///
/// ```ignore
/// // Consumer: (&T) -> ()
/// impl_box_once_conversions!(BoxConsumerOnce<T>, ConsumerOnce, FnOnce(&T));
///
/// // BiConsumer: (&T, &U) -> ()
/// impl_box_once_conversions!(BoxBiConsumerOnce<T, U>, BiConsumerOnce,
///     FnOnce(&T, &U));
///
/// // Function: (&T) -> R
/// impl_box_once_conversions!(BoxFunctionOnce<T, R>, FunctionOnce,
///     FnOnce(&T) -> R);
///
/// // Transformer: (T) -> R
/// impl_box_once_conversions!(BoxTransformerOnce<T, R>, TransformerOnce,
///     FnOnce(T) -> R);
///
/// // Supplier: () -> T
/// impl_box_once_conversions!(BoxSupplierOnce<T>, SupplierOnce, FnOnce() -> T);
/// ```
///
/// # Author
///
/// Hu Haixing
macro_rules! impl_box_once_conversions {
    (
        $box_type:ident < $($generics:ident),* >,
        $trait_name:ident,
        $fn_trait:path
    ) => {
        fn into_box(self) -> $box_type<$($generics),*>
        where
            $($generics: 'static),*
        {
            self
        }

        fn into_fn(self) -> impl $fn_trait
        where
            $($generics: 'static),*
        {
            self.function
        }
    };
}

pub(crate) use impl_box_once_conversions;

/// Implement common conversion methods for closure once traits
///
/// This macro generates standard conversion methods for all once traits
/// that are implemented by closures. It automatically infers everything from
/// the function signature and trait name.
///
/// # Parameters
///
/// * `$trait_name<$(generics),*>` - Full trait name with generics (e.g., `ConsumerOnce<T>`, `BiFunctionOnce<T, U, R>`)
/// * `$method_name` - Core method name (e.g., `accept`, `apply`)
/// * `$box_type` - Box wrapper type (e.g., `BoxConsumerOnce`, `BoxBiFunctionOnce`)
/// * `$fn_trait` - Function signature (e.g., `FnOnce(value: &T)`, `FnOnce(first: &T, second: &U) -> R`)
///
/// # Generated implementation
///
/// ```ignore
/// impl<F, Generics...> TraitName<Generics...> for F
/// where
///     F: FnOnce(...),
/// {
///     fn method_name(self, ...) {
///         self(...)
///     }
///     fn into_box(self) -> BoxType<...> { ... }
///     fn into_fn(self) -> impl FnOnce(...) { ... }
/// }
/// ```
///
/// # Examples
///
/// ```ignore
/// // ConsumerOnce<T>
/// impl_closure_once_trait!(
///     ConsumerOnce<T>,
///     accept,
///     BoxConsumerOnce,
///     FnOnce(value: &T)
/// );
///
/// // BiConsumerOnce<T, U>
/// impl_closure_once_trait!(
///     BiConsumerOnce<T, U>,
///     accept,
///     BoxBiConsumerOnce,
///     FnOnce(first: &T, second: &U)
/// );
///
/// // FunctionOnce<T, R>
/// impl_closure_once_trait!(
///     FunctionOnce<T, R>,
///     apply,
///     BoxFunctionOnce,
///     FnOnce(input: &T) -> R
/// );
/// ```
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_closure_once_trait {
    // ==================== Internal Implementation ====================

    // Core implementation: Generate complete trait implementation
    (
        @impl
        $trait_name:ident < $($generics:ident),* >,
        $method_name:ident,
        $box_type:ident,
        ($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?
    ) => {
        impl<F, $($generics),*> $trait_name<$($generics),*> for F
        where
            F: FnOnce($($arg_ty),*) $(-> $ret)?,
        {
            // Core method: Direct closure call
            fn $method_name(self, $($arg : $arg_ty),*) $(-> $ret)? {
                self($($arg),*)
            }

            // into_box: Convert to Box type
            fn into_box(self) -> $box_type<$($generics),*>
            where
                Self: Sized + 'static,
                $($generics: 'static),*
            {
                $box_type::new(self)
            }

            // into_fn: Convert to closure (always return self directly since F is already FnOnce)
            fn into_fn(self) -> impl FnOnce($($arg_ty),*) $(-> $ret)?
            where
                Self: Sized + 'static,
            {
                // F is already FnOnce with the correct signature, return directly (zero cost)
                self
            }
        }
    };

    // ==================== Public Interface ====================

    // No return value version
    (
        $trait_name:ident < $($generics:ident),* >,
        $method_name:ident,
        $box_type:ident,
        FnOnce($($arg:ident : $arg_ty:ty),*)
    ) => {
        impl_closure_once_trait!(
            @impl
            $trait_name<$($generics),*>,
            $method_name,
            $box_type,
            ($($arg : $arg_ty),*)
        );
    };

    // With return value version
    (
        $trait_name:ident < $($generics:ident),* >,
        $method_name:ident,
        $box_type:ident,
        FnOnce($($arg:ident : $arg_ty:ty),*) -> $ret:ty
    ) => {
        impl_closure_once_trait!(
            @impl
            $trait_name<$($generics),*>,
            $method_name,
            $box_type,
            ($($arg : $arg_ty),*) -> $ret
        );
    };
}

pub(crate) use impl_closure_once_trait;
