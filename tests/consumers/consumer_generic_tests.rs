// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Tests for Consumer types

use qubit_function::{
    ArcConsumer,
    BoxConsumer,
    Consumer,
    RcConsumer,
};
use std::rc::Rc;
use std::sync::Arc;

#[cfg(test)]
mod generic_tests {
    use super::{
        ArcConsumer,
        BoxConsumer,
        Consumer,
        RcConsumer,
    };

    fn apply_consumer<C: Consumer<i32>>(consumer: &C, value: &i32) {
        consumer.accept(value);
    }

    #[test]
    fn test_with_box_consumer() {
        let box_consumer = qubit_function::BoxConsumer::new(|x: &i32| {
            std::hint::black_box(x);
        });
        apply_consumer(&box_consumer, &5);
    }

    #[test]
    fn test_with_arc_consumer() {
        let arc_consumer = ArcConsumer::new(|x: &i32| {
            std::hint::black_box(x);
        });
        apply_consumer(&arc_consumer, &5);
    }

    #[test]
    fn test_with_rc_consumer() {
        let rc_consumer = RcConsumer::new(|x: &i32| {
            std::hint::black_box(x);
        });
        apply_consumer(&rc_consumer, &5);
    }

    #[test]
    fn test_with_closure() {
        let closure = |x: &i32| {
            std::hint::black_box(x);
        };
        apply_consumer(&closure, &5);
    }
}

// ============================================================================
// Name Tests - Testing name() and set_name() methods
// ============================================================================
