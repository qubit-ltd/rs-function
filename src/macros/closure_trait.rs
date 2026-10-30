// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

/// Implement trait for closures with automatic type inference
///
/// This macro generates blanket implementations for closures implementing
/// various function traits (Consumer, Function, Predicate, etc.). It
/// automatically infers everything from the function signature and trait name.
///
/// # Parameters
///
/// * `$trait_name<$(generics),*>` - Full trait name with generics (e.g.,
///   `Consumer<T>`, `Function<T, R>`)
/// * `$method_name` - Core method name (e.g., `accept`, `apply`, `test`)
/// * `$once_type` - Optional once wrapper type (e.g., `BoxConsumerOnce`)
/// * `$fn_signature` - Function signature (e.g., `Fn(value: &T)`, `FnMut(input:
///   &T) -> R`)
///
/// # Generated implementation
///
/// Generates a blanket implementation for all closures matching the signature,
/// forwarding the trait's core method directly to the closure. The optional
/// once-wrapper argument is accepted for compatibility but does not generate
/// conversion methods.
///
/// # Examples
///
/// These are crate-internal expansion forms. Identifiers in the trait's type
/// parameter list are declared as generics; they are not concrete argument
/// types. The optional wrapper token is accepted only for compatibility with
/// older syntax.
///
/// ```text
/// impl_closure_trait!(Consumer<T>, accept, BoxConsumerOnce, Fn(value: &T));
/// impl_closure_trait!(Function<T, R>, apply, BoxFunctionOnce, Fn(input: &T) -> R);
/// impl_closure_trait!(Predicate<T>, test, Fn(value: &T) -> bool);
/// impl_closure_trait!(StatefulConsumer<T>, accept, BoxConsumerOnce, FnMut(value: &T));
/// ```
macro_rules! impl_closure_trait {
    (
        $trait_name:ident < $($generics:ident),* >,
        $method_name:ident,
        $once_type:ident,
        Fn($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?
    ) => {
        impl_closure_trait!(
            $trait_name<$($generics),*>,
            $method_name,
            Fn($($arg : $arg_ty),*) $(-> $ret)?
        );
    };
    (
        $trait_name:ident < $($generics:ident),* >,
        $method_name:ident,
        Fn($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?
    ) => {
        impl<$($generics,)* F> $trait_name<$($generics),*> for F
        where
            F: Fn($($arg_ty),*) $(-> $ret)?,
        {
            #[inline(always)]
            fn $method_name(&self, $($arg: $arg_ty),*) $(-> $ret)? {
                self($($arg),*)
            }
        }
    };
    (
        $trait_name:ident < $($generics:ident),* >,
        $method_name:ident,
        $once_type:ident,
        FnMut($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?
    ) => {
        impl_closure_trait!(
            $trait_name<$($generics),*>,
            $method_name,
            FnMut($($arg : $arg_ty),*) $(-> $ret)?
        );
    };
    (
        $trait_name:ident < $($generics:ident),* >,
        $method_name:ident,
        FnMut($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?
    ) => {
        impl<$($generics,)* F> $trait_name<$($generics),*> for F
        where
            F: FnMut($($arg_ty),*) $(-> $ret)?,
        {
            #[inline(always)]
            fn $method_name(&mut self, $($arg: $arg_ty),*) $(-> $ret)? {
                self($($arg),*)
            }
        }
    };
}

pub(crate) use impl_closure_trait;
