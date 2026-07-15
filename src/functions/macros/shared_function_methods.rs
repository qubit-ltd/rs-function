// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Shared Function Methods Macro
//!
//! Generates `when` and `and_then` for shared Arc/Rc functions.
//! The generated methods borrow `&self`, clone the shared wrapper, and keep
//! predicate-storage capabilities separate from chained-callback
//! capabilities.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Function or BiFunction wrapper type.
//! * `$conditional_type` - Conditional wrapper returned by `when`, such as
//!   `ArcConditionalFunction`.
//! * `$predicate_type` - Predicate wrapper used by the conditional result.
//! * `$chained_function_trait` - Semantic trait implemented by the chained
//!   callback, such as `Function`.
//! * `$predicate_bounds` - Bounds required to store the predicate wrapper.
//! * `$chained_bounds` - Bounds required to store the callback produced by
//!   `and_then`.
//!
//! # Capability policy
//!
//! | Wrapper family | `predicate_bounds` | `chained_bounds` |
//! |----------------|--------------------|------------------|
//! | Arc stateless | `Send + Sync + 'static` | `Send + Sync + 'static` |
//! | Arc stateful | `Send + Sync + 'static` | `Send + 'static` |
//! | Rc stateless/stateful | `'static` | `'static` |

/// Generates `when` and `and_then` for shared Arc/Rc functions.
///
/// Invoke this macro inside the target wrapper's `impl` block. Predicate
/// bounds describe the immutable predicate object stored by the conditional
/// wrapper. Chained bounds describe the callback stored by the returned
/// function; stateful Arc callbacks are serialized by their outer
/// mutex and therefore require `Send`, but not `Sync`.
///
/// # Capability policy
///
/// | Wrapper family | `predicate_bounds` | `chained_bounds` |
/// |----------------|--------------------|------------------|
/// | Arc stateless | `Send + Sync + 'static` | `Send + Sync + 'static` |
/// | Arc stateful | `Send + Sync + 'static` | `Send + 'static` |
/// | Rc stateless/stateful | `'static` | `'static` |
macro_rules! impl_shared_function_methods {
    (@let_before ArcStatefulFunction, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_before RcStatefulFunction, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_before ArcStatefulMutatingFunction, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_before RcStatefulMutatingFunction, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_before ArcStatefulBiFunction, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_before RcStatefulBiFunction, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_before $struct_name:ident, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_after StatefulFunction, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_after StatefulMutatingFunction, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_after $function_trait:ident, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@apply_after Function, $after:ident, $value:expr) => {{
        let value = $value;
        $after.apply(&value)
    }};

    (@apply_after StatefulFunction, $after:ident, $value:expr) => {{
        let value = $value;
        $after.apply(&value)
    }};

    (@apply_after MutatingFunction, $after:ident, $value:expr) => {{
        let mut value = $value;
        $after.apply(&mut value)
    }};

    (@apply_after StatefulMutatingFunction, $after:ident, $value:expr) => {{
        let mut value = $value;
        $after.apply(&mut value)
    }};

    // Two generic parameters - Function types
    (
        $struct_name:ident < $t:ident, $r:ident >,
        $conditional_type:ident,
        $predicate_type:ident,
        $chained_function_trait:ident,
        predicate_bounds = ($($predicate_bounds:tt)+),
        chained_bounds = ($($chained_bounds:tt)+)
    ) => {
        /// Creates a conditional function that executes based on predicate
        /// result.
        ///
        /// # Parameters
        ///
        /// * `predicate` - The predicate to determine whether to execute
        ///   the function operation
        ///
        /// # Returns
        ///
        /// Returns a conditional function that only executes when the
        /// predicate returns `true`.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use qubit_function::{ArcBiFunction, BiFunction, ArcFunction, Function};
        /// use std::sync::Arc;
        ///
        /// let double = ArcFunction::new(|x: &i32| x * 2);
        /// let conditional = double.when(|value: &i32| *value > 0);
        /// assert_eq!(conditional.or_else(|_: &i32| 0).apply(&5), 10);  // executed
        /// assert_eq!(conditional.or_else(|_: &i32| 0).apply(&-3), 0);  // not executed
        /// ```
        #[inline]
        pub fn when<P>(&self, predicate: P) -> $conditional_type<$t, $r>
        where
            $t: 'static,
            $r: 'static,
            P: Predicate<$t> + $($predicate_bounds)+,
        {
            $conditional_type {
                function: self.clone(),
                predicate: $crate::$predicate_type::new(predicate),
            }
        }

        /// Chains execution with another function, executing the current
        /// function first, then the subsequent function.
        ///
        /// # Parameters
        ///
        /// * `after` - The subsequent function to execute after the current
        ///   function completes
        ///
        /// # Returns
        ///
        /// Returns a new function that executes the current function and
        /// the subsequent function in sequence.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use qubit_function::{ArcFunction, Function};
        /// use std::sync::Arc;
        ///
        /// let double = ArcFunction::new(|x: &i32| x * 2);
        /// let to_string = ArcFunction::new(|x: &i32| x.to_string());
        ///
        /// let chained = double.and_then(to_string);
        /// assert_eq!(chained.apply(&5), "10".to_string());
        /// ```
        #[inline]
        pub fn and_then<S, F>(&self, after: F) -> $struct_name<$t, S>
        where
            $t: 'static,
            $r: 'static,
            S: 'static,
            F: $chained_function_trait<$r, S> + $($chained_bounds)+,
        {
            impl_shared_function_methods!(@let_before $struct_name, before, self.clone());
            impl_shared_function_methods!(@let_after $chained_function_trait, after, after);
            $crate::functions::macros::impl_function_new_callback!($chained_function_trait, $struct_name, $t, |t| {
                impl_shared_function_methods!(@apply_after $chained_function_trait, after, before.apply(t))
            })
        }
    };

    // Three generic parameters - BiFunction types
    (
        $struct_name:ident < $t:ident, $u:ident, $r:ident >,
        $conditional_type:ident,
        $predicate_type:ident,
        $chained_function_trait:ident,
        predicate_bounds = ($($predicate_bounds:tt)+),
        chained_bounds = ($($chained_bounds:tt)+)
    ) => {
        /// Creates a conditional two-parameter function that executes based
        /// on bi-predicate result.
        ///
        /// # Parameters
        ///
        /// * `predicate` - The bi-predicate to determine whether to execute
        ///   the function operation
        ///
        /// # Returns
        ///
        /// Returns a conditional two-parameter function that only executes
        /// when the predicate returns `true`.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use qubit_function::{ArcBiFunction, BiFunction, BiMutatingFunction};
        /// use std::sync::Arc;
        ///
        /// let add = ArcBiFunction::new(|x: &i32, y: &i32| x + y);
        /// let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        /// assert_eq!(conditional.or_else(|_: &i32, _: &i32| 0).apply(&2, &3), 5);  // executed
        /// assert_eq!(conditional.or_else(|_: &i32, _: &i32| 0).apply(&-1, &3), 0); // not executed
        /// ```
        #[inline]
        pub fn when<P>(&self, predicate: P) -> $conditional_type<$t, $u, $r>
        where
            $t: 'static,
            $u: 'static,
            $r: 'static,
            P: BiPredicate<$t, $u> + $($predicate_bounds)+,
        {
            $conditional_type {
                function: self.clone(),
                predicate: $crate::$predicate_type::new(predicate),
            }
        }

        /// Chains execution with another two-parameter function, executing
        /// the current function first, then the subsequent function.
        ///
        /// # Parameters
        ///
        /// * `after` - The subsequent two-parameter function to execute after
        ///   the current function completes
        ///
        /// # Returns
        ///
        /// Returns a new two-parameter function that executes the current
        /// function and the subsequent function in sequence.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use qubit_function::{ArcBiFunction, ArcFunction, BiFunction, BiMutatingFunction, Function};
        /// use std::sync::Arc;
        ///
        /// let add = ArcBiFunction::new(|x: &i32, y: &i32| x + y);
        /// let multiply_by_two = ArcFunction::new(|x: &i32| x * 2);
        ///
        /// let chained = add.and_then(multiply_by_two);
        /// assert_eq!(chained.apply(&2, &3), 10); // (2+3) * 2 = 10
        /// ```
        #[inline]
        pub fn and_then<S, F>(&self, after: F) -> $struct_name<$t, $u, S>
        where
            $t: 'static,
            $u: 'static,
            $r: 'static,
            S: 'static,
            F: $chained_function_trait<$r, S> + $($chained_bounds)+,
        {
            impl_shared_function_methods!(@let_before $struct_name, before, self.clone());
            impl_shared_function_methods!(@let_after $chained_function_trait, after, after);
            $crate::functions::macros::impl_function_new_callback!($chained_function_trait, $struct_name, $t, $u, |t, u| {
                impl_shared_function_methods!(@apply_after $chained_function_trait, after, before.apply(t, u))
            })
        }
    };
}

pub(crate) use impl_shared_function_methods;
