/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Conditional Consumer Conversions Macro
//!
//! Generates conversion methods for Conditional Consumer implementations
//!
//! This macro generates the conversion methods (`into_box`, `into_rc`, `into_fn`) for
//! conditional consumer types. It selects immutable or mutable captures from the
//! generated closure trait (`Fn` or `FnMut`).
//!
//! # Parameters
//!
//! * `$box_type<$t:ident>` - The box-based consumer type (e.g., `BoxConsumer<T>`)
//! * `$rc_type:ident` - The rc-based consumer type name (e.g., `RcConsumer`)
//! * `$fn_trait:ident` - The function trait (e.g., `Fn` or `FnMut`)
//!
//! # Usage Examples
//!
//! ```rust
//! use qubit_function::{BoxConsumer, RcConsumer, BoxStatefulConsumer, RcStatefulConsumer};
//! macro_rules! impl_conditional_consumer_conversions {
//!     ($box_type:ident<$t:ident>, $rc_type:ident, $fn_trait:ident) => {
//!         let _ = std::stringify!($box_type);
//!         let _ = std::stringify!($rc_type);
//!         let _ = std::stringify!($fn_trait);
//!         let _ = std::marker::PhantomData::<$t>;
//!     };
//!     ($box_type:ident<$t:ident, $u:ident>, $rc_type:ident, $fn_trait:ident) => {
//!         let _ = std::stringify!($box_type);
//!         let _ = std::stringify!($rc_type);
//!         let _ = std::stringify!($fn_trait);
//!         let _ = std::marker::PhantomData::<($t, $u)>;
//!     };
//! }
//! impl_conditional_consumer_conversions!(BoxConsumer<i32>, RcConsumer, Fn);
//! impl_conditional_consumer_conversions!(BoxStatefulConsumer<i32>, RcStatefulConsumer, FnMut);
//! ```
//!
//! # Implementation Details
//!
//! - Uses the `$fn_trait` parameter to choose immutable or mutable captures.
//! - The closures inside `into_box` and `into_rc` capture as `Fn` or `FnMut`
//!   according to the generated operation.
//! - The `into_fn` method uses the provided `$fn_trait` parameter to match the
//!   intended trait type
//!

/// Generates conversion methods for Conditional Consumer implementations
///
/// This macro should be used inside an existing impl block (typically within
/// a trait implementation block). It generates individual conversion methods
/// but does not create a complete impl block itself. This macro generates the
/// conversion methods (`into_box`, `into_rc`, `into_fn`) for conditional consumer
/// types. It selects immutable or mutable captures from the generated closure
/// trait (`Fn` or `FnMut`).
///
/// # Parameters
///
/// * `$box_type<$t:ident>` - The box-based consumer type (e.g., `BoxConsumer<T>`)
/// * `$rc_type:ident` - The rc-based consumer type name (e.g., `RcConsumer`)
/// * `$fn_trait:ident` - The function trait (e.g., `Fn` or `FnMut`)
///
/// # Usage Examples
///
/// ```rust
/// use qubit_function::{BoxConsumer, RcConsumer, BoxStatefulConsumer, RcStatefulConsumer};
/// use std::marker::PhantomData;
/// macro_rules! impl_conditional_consumer_conversions {
///     ($box_type:ident<$t:ident>, $rc_type:ident, $fn_trait:ident) => {
///         let _ = std::stringify!($box_type);
///         let _ = std::stringify!($rc_type);
///         let _ = std::stringify!($fn_trait);
///         let _ = PhantomData::<$t>;
///     };
///     ($box_type:ident<$t:ident, $u:ident>, $rc_type:ident, $fn_trait:ident) => {
///         let _ = std::stringify!($box_type);
///         let _ = std::stringify!($rc_type);
///         let _ = std::stringify!($fn_trait);
///         let _ = PhantomData::<($t, $u)>;
///     };
/// }
/// impl_conditional_consumer_conversions!(BoxConsumer<i32>, RcConsumer, Fn);
/// impl_conditional_consumer_conversions!(BoxStatefulConsumer<i32>, RcStatefulConsumer, FnMut);
/// ```
///
/// # Implementation Details
///
/// - Uses the `$fn_trait` parameter to choose immutable or mutable captures.
/// - The closures inside `into_box` and `into_rc` capture as `Fn` or `FnMut`
///   according to the generated operation.
/// - The `into_fn` method uses the provided `$fn_trait` parameter to match the
///   intended trait type
///
///
macro_rules! impl_conditional_consumer_conversions {
    (@let_consumer Fn, $name:ident, $value:expr) => {
        let $name = $value;
    };

    (@let_consumer FnMut, $name:ident, $value:expr) => {
        let mut $name = $value;
    };

    // Single generic parameter - Consumer
    (
        $box_type:ident < $t:ident >,
        $rc_type:ident,
        $fn_trait:ident
    ) => {
        fn into_box(self) -> $box_type<$t>
        where
            Self: 'static,
        {
            let pred = self.predicate;
            impl_conditional_consumer_conversions!(@let_consumer $fn_trait, consumer, self.consumer);
            $box_type::new(move |t| {
                if pred.test(t) {
                    consumer.accept(t);
                }
            })
        }

        fn into_rc(self) -> $rc_type<$t>
        where
            Self: 'static,
        {
            let pred = self.predicate.into_rc();
            impl_conditional_consumer_conversions!(@let_consumer $fn_trait, consumer, self.consumer.into_rc());
            $rc_type::new(move |t| {
                if pred.test(t) {
                    consumer.accept(t);
                }
            })
        }

        fn into_fn(self) -> impl $fn_trait(&$t) {
            let pred = self.predicate;
            impl_conditional_consumer_conversions!(@let_consumer $fn_trait, consumer, self.consumer);
            move |t: &$t| {
                if pred.test(t) {
                    consumer.accept(t);
                }
            }
        }
    };

    // Two generic parameters - BiConsumer
    (
        $box_type:ident < $t:ident, $u:ident >,
        $rc_type:ident,
        $fn_trait:ident
    ) => {
        fn into_box(self) -> $box_type<$t, $u>
        where
            Self: 'static,
        {
            let pred = self.predicate;
            impl_conditional_consumer_conversions!(@let_consumer $fn_trait, consumer, self.consumer);
            $box_type::new(move |t, u| {
                if pred.test(t, u) {
                    consumer.accept(t, u);
                }
            })
        }

        fn into_rc(self) -> $rc_type<$t, $u>
        where
            Self: 'static,
        {
            let pred = self.predicate.into_rc();
            impl_conditional_consumer_conversions!(@let_consumer $fn_trait, consumer, self.consumer.into_rc());
            $rc_type::new_with_optional_name(
                move |t, u| {
                    if pred.test(t, u) {
                        consumer.accept(t, u);
                    }
                },
                None,
            )
        }

        fn into_fn(self) -> impl $fn_trait(&$t, &$u) {
            let pred = self.predicate;
            impl_conditional_consumer_conversions!(@let_consumer $fn_trait, consumer, self.consumer);
            move |t: &$t, u: &$u| {
                if pred.test(t, u) {
                    consumer.accept(t, u);
                }
            }
        }
    };
}

pub(crate) use impl_conditional_consumer_conversions;
