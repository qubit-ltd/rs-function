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
mod name_tests {
    use super::{
        ArcConsumer,
        BoxConsumer,
        Consumer,
        RcConsumer,
    };

    #[test]
    fn test_box_consumer_name() {
        let mut consumer = qubit_function::BoxConsumer::new(|x: &i32| {
            std::hint::black_box(x);
        });
        assert_eq!(consumer.name(), None);

        consumer.set_name("printer");
        assert_eq!(consumer.name(), Some("printer"));
    }

    #[test]
    fn test_arc_consumer_name() {
        let mut consumer = ArcConsumer::new(|x: &i32| {
            std::hint::black_box(x);
        });
        assert_eq!(consumer.name(), None);

        consumer.set_name("printer");
        assert_eq!(consumer.name(), Some("printer"));
    }

    #[test]
    fn test_arc_consumer_clone_names_mutate_independently() {
        let consumer = ArcConsumer::new_with_name("original", |_x: &i32| {});
        let mut cloned = consumer.clone();

        assert_eq!(consumer.name(), Some("original"));
        assert_eq!(cloned.name(), Some("original"));

        cloned.set_name("clone");
        assert_eq!(consumer.name(), Some("original"));
        assert_eq!(cloned.name(), Some("clone"));

        cloned.clear_name();
        assert_eq!(cloned.name(), None);
    }

    #[test]
    fn test_rc_consumer_name() {
        let mut consumer = RcConsumer::new(|x: &i32| {
            std::hint::black_box(x);
        });
        assert_eq!(consumer.name(), None);

        consumer.set_name("printer");
        assert_eq!(consumer.name(), Some("printer"));
    }

    #[test]
    fn test_box_consumer_name_with_accept() {
        let mut consumer = qubit_function::BoxConsumer::new(|_x: &i32| {});
        consumer.set_name("test_consumer");
        assert_eq!(consumer.name(), Some("test_consumer"));
        consumer.accept(&1);
        assert_eq!(consumer.name(), Some("test_consumer"));
    }

    #[test]
    fn test_arc_consumer_name_with_accept() {
        let mut consumer = ArcConsumer::new(|_x: &i32| {});
        consumer.set_name("test_consumer");
        assert_eq!(consumer.name(), Some("test_consumer"));
        consumer.accept(&1);
        assert_eq!(consumer.name(), Some("test_consumer"));
    }

    #[test]
    fn test_rc_consumer_name_with_accept() {
        let mut consumer = RcConsumer::new(|_x: &i32| {});
        consumer.set_name("test_consumer");
        assert_eq!(consumer.name(), Some("test_consumer"));
        consumer.accept(&1);
        assert_eq!(consumer.name(), Some("test_consumer"));
    }
}

// ============================================================================
// Display and Debug Tests
// ============================================================================
