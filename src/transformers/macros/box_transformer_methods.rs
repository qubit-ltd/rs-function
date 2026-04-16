/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Box Transformer Methods Macro
//!
//! Generates when and and_then method implementations for Box-based Transformer
//!
//! Generates conditional execution when method and chaining and_then method
//! for Box-based transformers that consume self (because Box cannot be cloned).
//!
//! This macro supports both single-parameter and two-parameter transformers through
//! pattern matching on the struct signature.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - The struct name with its generic parameters
//!   - Single parameter: `BoxTransformer<T, U>`
//!   - Two parameters: `BoxBiTransformer<T, U, V>`
//! * `$conditional_type` - The conditional transformer type for when (e.g., BoxConditionalTransformer)
//! * `$transformer_trait` - Transformer trait name (e.g., Transformer, BiTransformer)
//!
//! # Parameter Usage Comparison
//!
//! | Transformer Type | Struct Signature | `$conditional_type` |
//! |------------------|-----------------|----------------|
//! | **Transformer** | `BoxTransformer<T, U>` | BoxConditionalTransformer |
//! | **TransformerOnce** | `BoxTransformerOnce<T, U>` | BoxConditionalTransformerOnce |
//! | **StatefulTransformer** | `BoxStatefulTransformer<T, U>` | BoxConditionalStatefulTransformer |
//! | **BiTransformer** | `BoxBiTransformer<T, U, V>` | BoxConditionalBiTransformer |
//! | **BiTransformerOnce** | `BoxBiTransformerOnce<T, U, V>` | BoxConditionalBiTransformerOnce |
//! | **StatefulBiTransformer** | `BoxStatefulBiTransformer<T, U, V>` | BoxConditionalStatefulBiTransformer |
//!
//! | `$transformer_trait` |
//! |---------------------|
//! | Transformer |
//! | TransformerOnce |
//! | StatefulTransformer |
//! | BiTransformer |
//! | BiTransformerOnce |
//! | StatefulBiTransformer |
//!
//! # Examples
//!
//! ```ignore
//! // Single-parameter transformer
//! impl_box_transformer_methods!(
//!     BoxTransformer<T, U>,
//!     BoxConditionalTransformer,
//!     Transformer
//! );
//!
//! // Two-parameter transformer
//! impl_box_transformer_methods!(
//!     BoxBiTransformer<T, U, V>,
//!     BoxConditionalBiTransformer,
//!     BiTransformer
//! );
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates when and and_then method implementations for Box-based Transformer
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete impl block with methods for the specified struct.
/// Generates conditional execution when method and chaining and_then method
/// for Box-based transformers that consume self (because Box cannot be cloned).
///
/// This macro supports both single-parameter and two-parameter transformers through
/// pattern matching on the struct signature.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - The struct name with its generic parameters
///   - Single parameter: `BoxTransformer<T, U>`
///   - Two parameters: `BoxBiTransformer<T, U, V>`
/// * `$conditional_type` - The conditional transformer type for when (e.g.,
///   BoxConditionalTransformer)
/// * `$chained_transformer_trait` - The name of the transformer trait that chained
///   after the execution of this transformer (e.g., Transformer, BiTransformer)
///
/// # Parameter Usage Comparison
///
/// | Transformer Type | Struct Signature | `$conditional_type` | `$chained_transformer_trait` |
// |------------------|-----------------|----------------|---------------------|
// | **Transformer** | `BoxTransformer<T, U>` | BoxConditionalTransformer | Transformer |
// | **TransformerOnce** | `BoxTransformerOnce<T, U>` | BoxConditionalTransformerOnce | TransformerOnce |
// | **StatefulTransformer** | `BoxStatefulTransformer<T, U>` | BoxConditionalStatefulTransformer | StatefulTransformer |
// | **BiTransformer** | `BoxBiTransformer<T, U, V>` | BoxConditionalBiTransformer | BiTransformer |
// | **BiTransformerOnce** | `BoxBiTransformerOnce<T, U, V>` | BoxConditionalBiTransformerOnce | BiTransformerOnce |
// | **StatefulBiTransformer** | `BoxStatefulBiTransformer<T, U, V>` | BoxConditionalStatefulBiTransformer | StatefulBiTransformer |
//
/// # Examples
///
/// ```ignore
/// // Single-parameter transformer
/// impl_box_transformer_methods!(
///     BoxTransformer<T, U>,
///     BoxConditionalTransformer,
///     Transformer
/// );
///
/// // Two-parameter transformer
/// impl_box_transformer_methods!(
///     BoxBiTransformer<T, U, V>,
///     BoxConditionalBiTransformer,
///     BiTransformer
/// );
/// ```
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_box_transformer_methods {
    // Two generic parameter - Transformer
    (
        $struct_name:ident < $t:ident, $r:ident >,
        $conditional_type:ident,
        $chained_transformer_trait:ident
    ) => {
        /// Creates a conditional transformer that executes based on predicate
        /// result.
        ///
        /// # Parameters
        ///
        /// * `predicate` - The predicate to determine whether to execute
        ///   the transformation operation
        ///
        /// # Returns
        ///
        /// Returns a conditional transformer that only executes when the
        /// predicate returns `true`.
        ///
        /// # Examples
        ///
        /// ```rust,ignore
        /// use std::sync::Arc;
        /// use std::sync::atomic::{AtomicI32, Ordering};
        /// use qubit_function::transformers::*;
        ///
        /// let transformer = BoxTransformer::new({
        ///     |value: &i32| value * 2
        /// });
        ///
        /// let conditional = transformer.when(|value: &i32| *value > 0);
        /// assert_eq!(conditional.transform(&5), 10);  // transformed
        /// assert_eq!(conditional.transform(&-1), -1); // identity (unchanged)
        /// ```
        #[inline]
        pub fn when<P>(self, predicate: P) -> $conditional_type<$t, $r>
        where
            $t: 'static,
            $r: 'static,
            P: Predicate<$t> + 'static,
        {
            $conditional_type {
                transformer: self,
                predicate: predicate.into_box(),
            }
        }

        /// Chains execution with another transformer, executing the current
        /// transformer first, then the subsequent transformer.
        ///
        /// # Parameters
        ///
        /// * `after` - The subsequent transformer to execute after the current
        ///   transformer completes
        ///
        /// # Returns
        ///
        /// Returns a new transformer that executes the current transformer and
        /// the subsequent transformer in sequence.
        ///
        /// # Examples
        ///
        /// ```rust,ignore
        /// use qubit_function::transformers::*;
        ///
        /// let transformer1 = BoxTransformer::new({
        ///     |value: &i32| value + 1
        /// });
        ///
        /// let transformer2 = BoxTransformer::new({
        ///     |value: &i32| value * 2
        /// });
        ///
        /// let chained = transformer1.and_then(transformer2);
        /// assert_eq!(chained.transform(&5), 12); // (5 + 1) * 2 = 12
        /// ```
        #[allow(unused_mut)]
        #[inline]
        pub fn and_then<S, F>(self, mut after: F) -> $struct_name<$t, S>
        where
            $t: 'static,
            $r: 'static,
            S: 'static,
            F: $chained_transformer_trait<$r, S> + 'static,
        {
            let mut before = self.function;
            $struct_name::new(move |t| {
                let r = before(t);
                after.apply(r)
            })
        }
    };

    // Three generic parameters - BiTransformer
    (
        $struct_name:ident < $t:ident, $u:ident, $r:ident >,
        $conditional_type:ident,
        $chained_transformer_trait:ident
    ) => {
        /// Creates a conditional two-parameter transformer that executes based
        /// on bi-predicate result.
        ///
        /// # Parameters
        ///
        /// * `predicate` - The bi-predicate to determine whether to execute
        ///   the transformation operation
        ///
        /// # Returns
        ///
        /// Returns a conditional two-parameter transformer that only executes
        /// when the predicate returns `true`.
        ///
        /// # Examples
        ///
        /// ```rust,ignore
        /// use qubit_function::transformers::*;
        ///
        /// let bi_transformer = BoxBiTransformer::new({
        ///     |key: &String, value: &i32| format!("{}: {}", key, value)
        /// });
        ///
        /// let conditional = bi_transformer.when(|key: &String, value: &i32| *value > 0);
        /// assert_eq!(conditional.transform(&"test".to_string(), &5), "test: 5".to_string());  // transformed
        /// assert_eq!(conditional.transform(&"test".to_string(), &-1), "test".to_string());    // identity (key unchanged)
        /// ```
        #[inline]
        pub fn when<P>(self, predicate: P) -> $conditional_type<$t, $u, $r>
        where
            $t: 'static,
            $u: 'static,
            $r: 'static,
            P: BiPredicate<$t, $u> + 'static,
        {
            $conditional_type {
                transformer: self,
                predicate: predicate.into_box(),
            }
        }

        /// Chains execution with another two-parameter transformer, executing
        /// the current transformer first, then the subsequent transformer.
        ///
        /// # Parameters
        ///
        /// * `after` - The subsequent two-parameter transformer to execute after
        ///   the current transformer completes
        ///
        /// # Returns
        ///
        /// Returns a new two-parameter transformer that executes the current
        /// transformer and the subsequent transformer in sequence.
        ///
        /// # Examples
        ///
        /// ```rust,ignore
        /// use qubit_function::transformers::*;
        ///
        /// let bi_transformer1 = BoxBiTransformer::new({
        ///     |key: &String, value: &i32| (key.clone(), *value + 1)
        /// });
        ///
        /// let bi_transformer2 = BoxBiTransformer::new({
        ///     |key: &String, value: &i32| format!("{}: {}", key, value)
        /// });
        ///
        /// let chained = bi_transformer1.and_then(bi_transformer2);
        /// let result = chained.transform(&"test".to_string(), &5);
        /// assert_eq!(result, "test: 6"); // (value + 1) = 6
        /// ```
        #[allow(unused_mut)]
        #[inline]
        pub fn and_then<S, F>(self, mut after: F) -> $struct_name<$t, $u, S>
        where
            $t: 'static,
            $u: 'static,
            $r: 'static,
            S: 'static,
            F: $chained_transformer_trait<$r, S> + 'static,
        {
            let mut before = self.function;
            $struct_name::new(move |t, u| {
                let mut r = before(t, u);
                after.apply(r)
            })
        }
    };
}

pub(crate) use impl_box_transformer_methods;
