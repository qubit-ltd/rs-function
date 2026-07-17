// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Shared Transformer Methods Macro
//!
//! Generates `when` and `and_then` for shared Arc/Rc transformers.
//! The generated methods borrow `&self`, clone the shared wrapper, and keep
//! predicate-storage capabilities separate from chained-callback
//! capabilities.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Transformer or BiTransformer wrapper type.
//! * `$conditional_type` - Conditional wrapper returned by `when`, such as
//!   `ArcConditionalTransformer`.
//! * `$predicate_type` - Predicate wrapper used by the conditional result.
//! * `$chained_transformer_trait` - Semantic trait implemented by the chained
//!   callback, such as `Transformer`.
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

/// Generates `when` and `and_then` for shared Arc/Rc transformers.
///
/// Invoke this macro inside the target wrapper's `impl` block. Predicate
/// bounds describe the immutable predicate object stored by the conditional
/// wrapper. Chained bounds describe the callback stored by the returned
/// transformer; stateful Arc callbacks are serialized by their outer
/// mutex and therefore require `Send`, but not `Sync`.
///
/// # Capability policy
///
/// | Wrapper family | `predicate_bounds` | `chained_bounds` |
/// |----------------|--------------------|------------------|
/// | Arc stateless | `Send + Sync + 'static` | `Send + Sync + 'static` |
/// | Arc stateful | `Send + Sync + 'static` | `Send + 'static` |
/// | Rc stateless/stateful | `'static` | `'static` |
macro_rules! impl_shared_transformer_methods {
    (@let_before ArcStatefulTransformer, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_before RcStatefulTransformer, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_before ArcStatefulBiTransformer, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_before RcStatefulBiTransformer, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_before $struct_name:ident, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_after StatefulTransformer, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    (@let_after $transformer_trait:ident, $name:ident, $value:expr) => {
        let $name = $value;
    };

    // Two generic parameters
    (
        $struct_name:ident < $t:ident, $r:ident >,
        $conditional_type:ident,
        $predicate_type:ident,
        $chained_transformer_trait:ident,
        predicate_bounds = ($($predicate_bounds:tt)+),
        chained_bounds = ($($chained_bounds:tt)+)
    ) => {
        /// Creates a conditional transformer that applies this transformer
        /// only when `predicate` accepts the input.
        ///
        /// # Parameters
        ///
        /// * `predicate` - The condition evaluated before transformation.
        ///
        /// # Returns
        ///
        /// A conditional wrapper containing a shared clone of this
        /// transformer and the supplied predicate.
        #[inline]
        pub fn when<P>(&self, predicate: P) -> $conditional_type<$t, $r>
        where
            $t: 'static,
            $r: 'static,
            P: Predicate<$t> + $($predicate_bounds)+,
        {
            $conditional_type {
                transformer: self.clone(),
                predicate: $crate::$predicate_type::new(predicate),
            }
        }

        /// Chains `after` to run on the value produced by this transformer.
        ///
        /// # Parameters
        ///
        /// * `after` - The transformer applied to this transformer's output.
        ///
        /// # Returns
        ///
        /// A shared transformer that applies the two transformations in
        /// sequence.
        #[inline]
        pub fn and_then<S, F>(&self, after: F) -> $struct_name<$t, S>
        where
            $t: 'static,
            $r: 'static,
            S: 'static,
            F: $chained_transformer_trait<$r, S> + $($chained_bounds)+,
        {
            impl_shared_transformer_methods!(@let_before $struct_name, before, self.clone());
            impl_shared_transformer_methods!(@let_after $chained_transformer_trait, after, after);
            $struct_name::new(move |t| {
                let r = before.apply(t);
                after.apply(r)
            })
        }
    };

    // Three generic parameters
    (
        $struct_name:ident < $t:ident, $u:ident, $r:ident >,
        $conditional_type:ident,
        $predicate_type:ident,
        $chained_transformer_trait:ident,
        predicate_bounds = ($($predicate_bounds:tt)+),
        chained_bounds = ($($chained_bounds:tt)+)
    ) => {
        /// Creates a conditional transformer that applies this transformer
        /// only when `predicate` accepts both inputs.
        ///
        /// # Parameters
        ///
        /// * `predicate` - The condition evaluated before transformation.
        ///
        /// # Returns
        ///
        /// A conditional wrapper containing a shared clone of this
        /// transformer and the supplied predicate.
        #[inline]
        pub fn when<P>(&self, predicate: P) -> $conditional_type<$t, $u, $r>
        where
            $t: 'static,
            $u: 'static,
            $r: 'static,
            P: BiPredicate<$t, $u> + $($predicate_bounds)+,
        {
            $conditional_type {
                transformer: self.clone(),
                predicate: $crate::$predicate_type::new(predicate),
            }
        }

        /// Chains `after` to run on the value produced from both inputs.
        ///
        /// # Parameters
        ///
        /// * `after` - The transformer applied to this transformer's output.
        ///
        /// # Returns
        ///
        /// A shared two-input transformer that applies the two
        /// transformations in sequence.
        #[inline]
        pub fn and_then<S, F>(&self, after: F) -> $struct_name<$t, $u, S>
        where
            $t: 'static,
            $u: 'static,
            $r: 'static,
            S: 'static,
            F: $chained_transformer_trait<$r, S> + $($chained_bounds)+,
        {
            impl_shared_transformer_methods!(@let_before $struct_name, before, self.clone());
            impl_shared_transformer_methods!(@let_after $chained_transformer_trait, after, after);
            $struct_name::new(move |t, u| {
                let r = before.apply(t, u);
                after.apply(r)
            })
        }
    };
}

pub(crate) use impl_shared_transformer_methods;
