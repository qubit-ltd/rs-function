/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Defines the `FnStatefulTesterOps` public type.

use super::{
    StatefulTester,
    box_stateful_tester::BoxStatefulTester,
};

/// Extension trait providing logical composition methods for `FnMut() -> bool`.
///
/// This trait is implemented for all stateful zero-argument closures and
/// function objects that match `FnMut() -> bool`.
pub trait FnStatefulTesterOps: Sized + FnMut() -> bool {
    /// Returns a boxed stateful tester representing logical AND.
    #[inline]
    fn and<T>(mut self, mut other: T) -> BoxStatefulTester
    where
        Self: 'static,
        T: StatefulTester + 'static,
    {
        BoxStatefulTester::new(move || self() && other.test())
    }

    /// Returns a boxed stateful tester representing logical OR.
    #[inline]
    fn or<T>(mut self, mut other: T) -> BoxStatefulTester
    where
        Self: 'static,
        T: StatefulTester + 'static,
    {
        BoxStatefulTester::new(move || self() || other.test())
    }

    /// Returns a boxed stateful tester representing logical negation.
    #[inline]
    fn not(mut self) -> BoxStatefulTester
    where
        Self: 'static,
    {
        BoxStatefulTester::new(move || !self())
    }

    /// Returns a boxed stateful tester representing logical NAND.
    #[inline]
    fn nand<T>(mut self, mut other: T) -> BoxStatefulTester
    where
        Self: 'static,
        T: StatefulTester + 'static,
    {
        BoxStatefulTester::new(move || !(self() && other.test()))
    }

    /// Returns a boxed stateful tester representing logical XOR.
    #[inline]
    fn xor<T>(mut self, mut other: T) -> BoxStatefulTester
    where
        Self: 'static,
        T: StatefulTester + 'static,
    {
        BoxStatefulTester::new(move || self() ^ other.test())
    }

    /// Returns a boxed stateful tester representing logical NOR.
    #[inline]
    fn nor<T>(mut self, mut other: T) -> BoxStatefulTester
    where
        Self: 'static,
        T: StatefulTester + 'static,
    {
        BoxStatefulTester::new(move || !(self() || other.test()))
    }
}

impl<F> FnStatefulTesterOps for F where F: FnMut() -> bool {}
