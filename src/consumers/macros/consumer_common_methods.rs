// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! # Consumer Common Methods Macro
//!
//! Generates common Consumer methods (new, new_with_name, name,
//! set_name, noop)
//!
//! Generates constructor methods, name management methods and noop
//! constructor for Consumer structs. This macro should be called inside
//! an impl block.
//!
//! The macro automatically detects the number of generic parameters and
//! generates the appropriate implementations for single-parameter or
//! two-parameter consumers.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Struct name with generic parameters
//! * `$fn_trait_with_bounds` - Closure trait with complete bounds (e.g.,
//!   `Fn(&T) + 'static`)
//! * `$wrapper_expr` - Wrapper expression (uses `f` for the closure)
//!
//! # Usage
//!
//! The example requires the `stateful` feature.
//!
//! ```rust
//! # #[cfg(feature = "stateful")]
//! # {
//! // Single generic parameter - Consumer
//! use std::sync::Arc;
//! use std::sync::Mutex;
//! use qubit_function::{ArcStatefulConsumer, BoxBiConsumer, BoxConsumer};
//! macro_rules! impl_consumer_common_methods {
//!     ($struct_name:ident < $t:ident >, ($($fn_trait_with_bounds:tt)+), |$f:ident| $wrapper_expr:expr) => {
//!         let _ = stringify!($struct_name);
//!         let _ = stringify!($($fn_trait_with_bounds)+);
//!         let _ = stringify!($wrapper_expr);
//!     };
//!     ($struct_name:ident < $t:ident, $u:ident >, ($($fn_trait_with_bounds:tt)+), |$f:ident| $wrapper_expr:expr) => {
//!         let _ = stringify!($struct_name);
//!         let _ = stringify!($($fn_trait_with_bounds)+);
//!         let _ = stringify!($wrapper_expr);
//!     };
//! }
//! impl_consumer_common_methods!(
//!     BoxConsumer<i32>,
//!     (Fn(&i32) + 'static),
//!     |f| Box::new(f)
//! );
//!
//! // Single generic parameter - StatefulConsumer
//! impl_consumer_common_methods!(
//!     ArcStatefulConsumer<i32>,
//!     (FnMut(&i32) + Send + 'static),
//!     |f| Arc::new(Mutex::new(f))
//! );
//!
//! // Two generic parameters - BiConsumer
//! impl_consumer_common_methods!(
//!     BoxBiConsumer<i32, i32>,
//!     (Fn(&i32, &i32) + 'static),
//!     |f| Box::new(f)
//! );
//! # }
//! ```
//!
//! # Generated Methods
//!
//! * `new()` - Creates a new consumer
//! * `new_with_name()` - Creates a named consumer
//! * `name()` - Gets the name of the consumer
//! * `set_name()` - Sets the name of the consumer
//! * `noop()` - Creates a consumer that performs no operation

/// Generates common Consumer methods (new, new_with_name, name,
/// set_name, noop)
///
/// This macro should be used inside an existing impl block for the target
/// struct. It generates individual methods but does not create a complete
/// impl block itself. Generates constructor methods, name management methods
/// and noop constructor for Consumer structs.
///
/// The macro automatically detects the number of generic parameters and
/// generates the appropriate implementations for single-parameter or
/// two-parameter consumers.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - Struct name with generic parameters
/// * `$fn_trait_with_bounds` - Closure trait with complete bounds (e.g.,
///   `Fn(&T) + 'static`)
/// * `$wrapper_expr` - Wrapper expression (uses `f` for the closure)
///
/// # Usage
///
/// The example requires the `stateful` feature.
///
/// ```rust
/// # #[cfg(feature = "stateful")]
/// # {
/// // Single generic parameter - Consumer
/// use qubit_function::{ArcStatefulConsumer, BoxBiConsumer, BoxConsumer};
/// use std::sync::Arc;
/// use std::sync::Mutex;
/// macro_rules! impl_consumer_common_methods {
///     ($struct_name:ident < $t:ident >, ($($fn_trait_with_bounds:tt)+), |$f:ident| $wrapper_expr:expr) => {
///         let _ = stringify!($struct_name);
///         let _ = stringify!($($fn_trait_with_bounds)+);
///         let _ = stringify!($wrapper_expr);
///     };
///     ($struct_name:ident < $t:ident, $u:ident >, ($($fn_trait_with_bounds:tt)+), |$f:ident| $wrapper_expr:expr) => {
///         let _ = stringify!($struct_name);
///         let _ = stringify!($($fn_trait_with_bounds)+);
///         let _ = stringify!($wrapper_expr);
///     };
/// }
/// impl_consumer_common_methods!(
///     BoxConsumer<i32>,
///     (Fn(&i32) + 'static),
///     |f| Box::new(f)
/// );
///
/// // Single generic parameter - StatefulConsumer
/// impl_consumer_common_methods!(
///     ArcStatefulConsumer<i32>,
///     (FnMut(&i32) + Send + 'static),
///     |f| Arc::new(Mutex::new(f))
/// );
///
/// // Two generic parameters - BiConsumer
/// impl_consumer_common_methods!(
///     BoxBiConsumer<i32, i32>,
///     (Fn(&i32, &i32) + 'static),
///     |f| Box::new(f)
/// );
/// # }
/// ```
///
/// # Generated Methods
///
/// * `new()` - Creates a new consumer
/// * `new_with_name()` - Creates a named consumer
/// * `name()` - Gets the name of the consumer
/// * `set_name()` - Sets the name of the consumer
/// * `noop()` - Creates a consumer that performs no operation
macro_rules! impl_consumer_new_methods {
    (BoxConsumer<$t:ident>, |$f:ident| $wrapper:expr) => { $crate::consumers::macros::impl_consumer_new_methods!(@one Consumer, $t, ('static), |$f| $wrapper); };
    (RcConsumer<$t:ident>, |$f:ident| $wrapper:expr) => { $crate::consumers::macros::impl_consumer_new_methods!(@one Consumer, $t, ('static), |$f| $wrapper); };
    (ArcConsumer<$t:ident>, |$f:ident| $wrapper:expr) => { $crate::consumers::macros::impl_consumer_new_methods!(@one Consumer, $t, (Send + Sync + 'static), |$f| $wrapper); };
    (BoxStatefulConsumer<$t:ident>, |$f:ident| $wrapper:expr) => { $crate::consumers::macros::impl_consumer_new_methods!(@one_mut StatefulConsumer, $t, ('static), |$f| $wrapper); };
    (RcStatefulConsumer<$t:ident>, |$f:ident| $wrapper:expr) => { $crate::consumers::macros::impl_consumer_new_methods!(@one_mut StatefulConsumer, $t, ('static), |$f| $wrapper); };
    (ArcStatefulConsumer<$t:ident>, |$f:ident| $wrapper:expr) => { $crate::consumers::macros::impl_consumer_new_methods!(@one_mut StatefulConsumer, $t, (Send + 'static), |$f| $wrapper); };
    (BoxConsumerOnce<$t:ident>, |$f:ident| $wrapper:expr) => { $crate::consumers::macros::impl_consumer_new_methods!(@one_once ConsumerOnce, $t, ('static), |$f| $wrapper); };
    (BoxBiConsumer<$t:ident, $u:ident>, |$f:ident| $wrapper:expr) => { $crate::consumers::macros::impl_consumer_new_methods!(@two BiConsumer, $t, $u, ('static), |$f| $wrapper); };
    (RcBiConsumer<$t:ident, $u:ident>, |$f:ident| $wrapper:expr) => { $crate::consumers::macros::impl_consumer_new_methods!(@two BiConsumer, $t, $u, ('static), |$f| $wrapper); };
    (ArcBiConsumer<$t:ident, $u:ident>, |$f:ident| $wrapper:expr) => { $crate::consumers::macros::impl_consumer_new_methods!(@two BiConsumer, $t, $u, (Send + Sync + 'static), |$f| $wrapper); };
    (BoxStatefulBiConsumer<$t:ident, $u:ident>, |$f:ident| $wrapper:expr) => { $crate::consumers::macros::impl_consumer_new_methods!(@two_mut StatefulBiConsumer, $t, $u, ('static), |$f| $wrapper); };
    (RcStatefulBiConsumer<$t:ident, $u:ident>, |$f:ident| $wrapper:expr) => { $crate::consumers::macros::impl_consumer_new_methods!(@two_mut StatefulBiConsumer, $t, $u, ('static), |$f| $wrapper); };
    (ArcStatefulBiConsumer<$t:ident, $u:ident>, |$f:ident| $wrapper:expr) => { $crate::consumers::macros::impl_consumer_new_methods!(@two_mut StatefulBiConsumer, $t, $u, (Send + 'static), |$f| $wrapper); };
    (BoxBiConsumerOnce<$t:ident, $u:ident>, |$f:ident| $wrapper:expr) => { $crate::consumers::macros::impl_consumer_new_methods!(@two_once BiConsumerOnce, $t, $u, ('static), |$f| $wrapper); };
    (@one $trait:ident, $t:ident, ($($bounds:tt)+), |$f:ident| $wrapper:expr) => { crate::macros::impl_common_new_methods!(semantic ($trait<$t> + $($bounds)+), |source| move |value: &$t| source.accept(value), |$f| $wrapper, "consumer"); };
    (@one_mut $trait:ident, $t:ident, ($($bounds:tt)+), |$f:ident| $wrapper:expr) => { crate::macros::impl_common_new_methods!(semantic_mut ($trait<$t> + $($bounds)+), |source| move |value: &$t| source.accept(value), |$f| $wrapper, "consumer"); };
    (@one_once $trait:ident, $t:ident, ($($bounds:tt)+), |$f:ident| $wrapper:expr) => { crate::macros::impl_common_new_methods!(semantic ($trait<$t> + $($bounds)+), |source| move |value: &$t| source.accept(value), |$f| $wrapper, "consumer"); };
    (@two $trait:ident, $t:ident, $u:ident, ($($bounds:tt)+), |$f:ident| $wrapper:expr) => { crate::macros::impl_common_new_methods!(semantic ($trait<$t, $u> + $($bounds)+), |source| move |first: &$t, second: &$u| source.accept(first, second), |$f| $wrapper, "bi-consumer"); };
    (@two_mut $trait:ident, $t:ident, $u:ident, ($($bounds:tt)+), |$f:ident| $wrapper:expr) => { crate::macros::impl_common_new_methods!(semantic_mut ($trait<$t, $u> + $($bounds)+), |source| move |first: &$t, second: &$u| source.accept(first, second), |$f| $wrapper, "bi-consumer"); };
    (@two_once $trait:ident, $t:ident, $u:ident, ($($bounds:tt)+), |$f:ident| $wrapper:expr) => { crate::macros::impl_common_new_methods!(semantic ($trait<$t, $u> + $($bounds)+), |source| move |first: &$t, second: &$u| source.accept(first, second), |$f| $wrapper, "bi-consumer"); };
}

macro_rules! impl_consumer_common_methods {
    // Single generic parameter - Consumer types
    (
        $struct_name:ident < $t:ident >,
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr
    ) => {
        $crate::consumers::macros::impl_consumer_new_methods!($struct_name<$t>, |$f| $wrapper_expr);
        crate::macros::impl_common_name_methods!("consumer");

        /// Creates a no-operation consumer.
        ///
        /// Creates a consumer that does nothing when called. Useful for
        /// default values or placeholder implementations.
        ///
        /// # Returns
        ///
        /// Returns a new consumer instance that performs no operation.
        #[inline]
        pub fn noop() -> Self {
            Self::new(|_: &$t| {})
        }
    };

    // Two generic parameters - BiConsumer types
    (
        $struct_name:ident < $t:ident, $u:ident >,
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr
    ) => {
        $crate::consumers::macros::impl_consumer_new_methods!($struct_name<$t, $u>, |$f| $wrapper_expr);
        crate::macros::impl_common_name_methods!("bi-consumer");

        /// Creates a no-operation bi-consumer.
        ///
        /// Creates a bi-consumer that does nothing when called. Useful
        /// for default values or placeholder implementations.
        ///
        /// # Returns
        ///
        /// Returns a new bi-consumer instance that performs no operation.
        #[inline]
        pub fn noop() -> Self {
            Self::new(|_: &$t, _: &$u| {})
        }
    };
}

pub(crate) use impl_consumer_common_methods;
pub(crate) use impl_consumer_new_methods;
