/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Fn Ops Trait Macro
//!
//! Generate extension traits and implementations for closure types
//!
//! This macro generates extension traits for closure types that implement
//! `Fn` or `FnMut`, providing `and_then`, `compose`, and `when` methods without requiring
//! explicit wrapping as `BoxTransformer`, `RcTransformer`, or `ArcTransformer`.
//!
//! # Parameters
//!
//! * `$fn_signature` - Closure signature (in parentheses, without constraints)
//!   Examples: `(Fn(T) -> R)`, `(FnMut(T) -> R)`
//! * `$trait_name` - Name of the extension trait (e.g., `FnTransformerOps`,
//!   `FnStatefulTransformerOps`)
//! * `$box_type` - Box wrapper type (e.g., `BoxTransformer`, `BoxStatefulTransformer`)
//! * `$chained_transformer_trait` - The name of the transformer trait that is
//!   chained after the execution of this transformer (e.g., Transformer,
//!   BiTransformer)
//! * `$conditional_type` - Conditional transformer type (e.g., BoxConditionalTransformer)
//!
//! # Implementation Notes
//!
//! The macro keeps the same composition shape for `Fn` and `FnMut` closures,
//! simplifying the generated `and_then` and `when` implementations while still
//! preserving transformer value-passing semantics.
//!
//! # Usage Examples
//!
//! ```ignore
//! // Generate extension trait for Fn(T) -> R
//! impl_transformer_fn_ops_trait!(
//!     (Fn(T) -> R),
//!     FnTransformerOps,
//!     BoxTransformer,
//!     Transformer,
//!     BoxConditionalTransformer
//! );
//!
//! // Generate extension trait for FnMut(T) -> R
//! impl_transformer_fn_ops_trait!(
//!     (FnMut(T) -> R),
//!     FnStatefulTransformerOps,
//!     BoxStatefulTransformer,
//!     StatefulTransformer,
//!     BoxConditionalStatefulTransformer
//! );
//! ```
//!

/// Generate extension traits and implementations for closure types
///
/// This macro generates an extension trait that provides composition methods
/// (`and_then`, `compose`, `when`) for closures implementing the specified
/// closure trait, without requiring explicit wrapping.
///
/// # Unified Implementation Strategy
///
/// The macro uses a unified implementation approach for `Fn` and `FnMut`
/// transformer closures. This avoids duplicating the generated composition
/// methods while preserving the by-value input and output flow of transformers.
///
/// # Parameters
///
/// * `$fn_signature` - Closure signature (in parentheses, without constraints)
/// * `$trait_name` - Name of the extension trait
/// * `$box_type` - Box wrapper type
/// * `$chained_transformer_trait` - The name of the transformer trait that is
///   chained after the execution of this transformer (e.g., Transformer,
///   BiTransformer)
/// * `$conditional_type` - Conditional transformer type
///
/// # Generated Code
///
/// Generates a trait definition and a blanket implementation, containing:
/// - `and_then<S, F>` - Chain composition method
/// - `compose<S, F>` - Reverse composition method
/// - `when<P>` - Conditional execution method
///
/// # Examples
///
/// ```ignore
/// // Fn(T) -> R version
/// impl_transformer_fn_ops_trait!(
///     (Fn(T) -> R),
///     FnTransformerOps,
///     BoxTransformer,
///     Transformer,
///     BoxConditionalTransformer
/// );
///
/// // FnMut(T) -> R version
/// impl_transformer_fn_ops_trait!(
///     (FnMut(T) -> R),
///     FnStatefulTransformerOps,
///     BoxStatefulTransformer,
///     StatefulTransformer,
///     BoxConditionalStatefulTransformer
/// );
/// ```
///
macro_rules! impl_transformer_fn_ops_trait {
    (@let_self BoxStatefulTransformer, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_self $box_type:ident, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_transformer StatefulTransformer, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_transformer $transformer_trait:ident, $name:ident, $value:expr) => {
        let $name = $value;
    };

    // Unified implementation - accepts closure signature (without constraints)
    (
        ($($fn_signature:tt)+),
        $trait_name:ident,
        $box_type:ident,
        $chained_transformer_trait:ident,
        $conditional_type:ident
    ) => {
        /// Extension trait for closures implementing the base transformer trait
        ///
        /// Provides composition methods (`and_then`, `compose`, `when`) for closures
        /// and function pointers without requiring explicit wrapping.
        ///
        /// This trait is automatically implemented for all closures and function
        /// pointers that implement the base transformer trait.
        ///
        /// # Design Rationale
        ///
        /// While closures automatically implement the base transformer trait
        /// through blanket implementation, they don't have access to instance
        /// methods like `and_then`, `compose`, and `when`. This extension trait
        /// provides those methods, returning the appropriate Box-based
        /// transformer type for maximum flexibility.
        ///
        pub trait $trait_name<T, R>: $($fn_signature)+ + Sized {
            /// Chain composition - applies self first, then after
            ///
            /// Creates a new transformer that applies this transformer first, then
            /// applies the after transformer to the result. Consumes self and returns
            /// a Box-based transformer.
            ///
            /// # Type Parameters
            ///
            /// * `S` - The output type of the after transformer
            /// * `F` - The type of the after transformer (must implement the transformer trait)
            ///
            /// # Parameters
            ///
            /// * `after` - The transformer to apply after self. **Note: This parameter
            ///   is passed by value and will transfer ownership.** If you need to
            ///   preserve the original transformer, clone it first (if it implements
            ///   `Clone`). Can be:
            ///   - A closure
            ///   - A function pointer
            ///   - A Box-based transformer
            ///   - An Rc-based transformer
            ///   - An Arc-based transformer
            ///   - Any type implementing the transformer trait
            ///
            /// # Returns
            ///
            /// A new Box-based transformer representing the composition
            ///
            /// # Examples
            ///
            /// ## Direct value passing (ownership transfer)
            ///
            /// ```rust
            /// use qubit_function::{BoxTransformer, FnTransformerOps, Transformer};
            ///
            /// let double = |x: i32| x * 2;
            /// let to_string = BoxTransformer::new(|x: i32| x.to_string());
            ///
            /// // to_string is moved here
            /// let composed = double.and_then(to_string);
            /// assert_eq!(composed.apply(21), "42");
            /// // to_string.apply(5); // Would not compile - moved
            /// ```
            ///
            /// ## Preserving behavior with separate closures
            ///
            /// ```rust
            /// use qubit_function::{BoxTransformer, FnTransformerOps, Transformer};
            ///
            /// let double = |x: i32| x * 2;
            /// let to_string_for_validation = |x: i32| x.to_string();
            ///
            /// let composed = double.and_then(BoxTransformer::new(|x: i32| x.to_string()));
            /// assert_eq!(composed.apply(21), "42");
            ///
            /// assert_eq!(to_string_for_validation(5), "5");
            /// ```
            #[inline]
            fn and_then<S, F>(self, after: F) -> $box_type<T, S>
            where
                Self: 'static,
                S: 'static,
                F: $chained_transformer_trait<R, S> + 'static,
                T: 'static,
                R: 'static,
            {
                impl_transformer_fn_ops_trait!(@let_self $box_type, this, self);
                impl_transformer_fn_ops_trait!(@let_transformer $chained_transformer_trait, after, after);
                $box_type::new(move |x| {
                  let r = this(x);
                  after.apply(r)
                })
            }

            /// Reverse composition - applies before first, then self.
            ///
            /// Creates a new transformer that applies the before transformer
            /// first, then applies this transformer to the result. Consumes
            /// self and returns a Box-based transformer.
            ///
            /// # Type Parameters
            ///
            /// * `S` - The input type of the before transformer
            /// * `F` - The type of the before transformer
            ///
            /// # Parameters
            ///
            /// * `before` - The transformer to apply before self.
            ///
            /// # Returns
            ///
            /// A new Box-based transformer representing the reverse
            /// composition.
            #[inline]
            fn compose<S, F>(self, before: F) -> $box_type<S, R>
            where
                Self: 'static,
                S: 'static,
                F: $chained_transformer_trait<S, T> + 'static,
                T: 'static,
                R: 'static,
            {
                impl_transformer_fn_ops_trait!(@let_self $box_type, this, self);
                impl_transformer_fn_ops_trait!(@let_transformer $chained_transformer_trait, before, before);
                $box_type::new(move |x| {
                    let t = before.apply(x);
                    this(t)
                })
            }

            /// Creates a conditional transformer
            ///
            /// Returns a transformer that only executes when a predicate is satisfied.
            /// You must call `or_else()` to provide an alternative transformer for when
            /// the condition is not satisfied.
            ///
            /// # Parameters
            ///
            /// * `predicate` - The condition to check. **Note: This parameter is passed
            ///   by value and will transfer ownership.** If you need to preserve the
            ///   original predicate, clone it first (if it implements `Clone`). Can be:
            ///   - A closure: `|x: &T| -> bool`
            ///   - A function pointer: `fn(&T) -> bool`
            ///   - A `BoxPredicate<T>`
            ///   - An `RcPredicate<T>`
            ///   - An `ArcPredicate<T>`
            ///   - Any type implementing `Predicate<T>`
            ///
            /// # Returns
            ///
            /// Returns the appropriate conditional transformer type
            ///
            /// # Examples
            ///
            /// ## Basic usage with or_else
            ///
            /// ```rust
            /// use qubit_function::{Transformer, FnTransformerOps};
            ///
            /// let double = |x: i32| x * 2;
            /// let conditional = double.when(|x: &i32| *x > 0).or_else(|x: i32| -x);
            ///
            /// assert_eq!(conditional.apply(5), 10);
            /// assert_eq!(conditional.apply(-5), 5);
            /// ```
            ///
            /// ## Reusing equivalent predicate logic
            ///
            /// ```rust
            /// use qubit_function::{FnTransformerOps, Transformer};
            ///
            /// let double = |x: i32| x * 2;
            /// let is_positive_for_validation = |x: &i32| *x > 0;
            ///
            /// let conditional = double.when(|x: &i32| *x > 0)
            ///     .or_else(|x: i32| -x);
            ///
            /// assert_eq!(conditional.apply(5), 10);
            /// assert!(is_positive_for_validation(&3));
            /// ```
            #[inline]
            fn when<P>(self, predicate: P) -> $conditional_type<T, R>
            where
                Self: 'static,
                P: Predicate<T> + 'static,
                T: 'static,
                R: 'static,
            {
                $box_type::new(self).when(predicate)
            }
        }

        /// Blanket implementation for all closures
        ///
        /// Automatically implements the extension trait for any type that
        /// implements the base transformer trait.
        ///
        impl<T, R, F> $trait_name<T, R> for F where F: $($fn_signature)+ {}
    };
}

pub(crate) use impl_transformer_fn_ops_trait;
