// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

/// Tests for BiConsumer types
use qubit_function::ArcBiConsumer;
/// Tests for BiConsumer types
use qubit_function::BiConsumer;
/// Tests for BiConsumer types
use qubit_function::BoxBiConsumer;
/// Tests for BiConsumer types
use qubit_function::RcBiConsumer;

#[cfg(test)]
mod name_tests {
    use super::ArcBiConsumer;
    use super::BiConsumer;
    use super::BoxBiConsumer;
    use super::RcBiConsumer;

    #[test]
    fn test_box_consumer_name() {
        let mut consumer = BoxBiConsumer::new(|x: &i32, y: &i32| {
            std::hint::black_box(x + y);
        });
        assert_eq!(consumer.name(), None);

        consumer.set_name("add_printer");
        assert_eq!(consumer.name(), Some("add_printer"));
    }

    #[test]
    fn test_arc_consumer_name() {
        let mut consumer = ArcBiConsumer::new(|x: &i32, y: &i32| {
            std::hint::black_box(x + y);
        });
        assert_eq!(consumer.name(), None);

        consumer.set_name("add_printer");
        assert_eq!(consumer.name(), Some("add_printer"));
    }

    #[test]
    fn test_rc_consumer_name() {
        let mut consumer = RcBiConsumer::new(|x: &i32, y: &i32| {
            std::hint::black_box(x + y);
        });
        assert_eq!(consumer.name(), None);

        consumer.set_name("add_printer");
        assert_eq!(consumer.name(), Some("add_printer"));
    }

    #[test]
    fn test_box_consumer_name_with_accept() {
        let mut consumer = BoxBiConsumer::new(|_x: &i32, _y: &i32| {});
        consumer.set_name("test_consumer");
        assert_eq!(consumer.name(), Some("test_consumer"));
        consumer.accept(&1, &2);
        assert_eq!(consumer.name(), Some("test_consumer"));
    }

    #[test]
    fn test_arc_consumer_name_with_accept() {
        let mut consumer = ArcBiConsumer::new(|_x: &i32, _y: &i32| {});
        consumer.set_name("test_consumer");
        assert_eq!(consumer.name(), Some("test_consumer"));
        consumer.accept(&1, &2);
        assert_eq!(consumer.name(), Some("test_consumer"));
    }

    #[test]
    fn test_rc_consumer_name_with_accept() {
        let mut consumer = RcBiConsumer::new(|_x: &i32, _y: &i32| {});
        consumer.set_name("test_consumer");
        assert_eq!(consumer.name(), Some("test_consumer"));
        consumer.accept(&1, &2);
        assert_eq!(consumer.name(), Some("test_consumer"));
    }

    #[test]
    fn test_box_consumer_name_change() {
        let mut consumer = BoxBiConsumer::new(|_x: &i32, _y: &i32| {});
        consumer.set_name("name1");
        assert_eq!(consumer.name(), Some("name1"));
        consumer.set_name("name2");
        assert_eq!(consumer.name(), Some("name2"));
    }

    #[test]
    fn test_arc_consumer_name_change() {
        let mut consumer = ArcBiConsumer::new(|_x: &i32, _y: &i32| {});
        consumer.set_name("name1");
        assert_eq!(consumer.name(), Some("name1"));
        consumer.set_name("name2");
        assert_eq!(consumer.name(), Some("name2"));
    }

    #[test]
    fn test_rc_consumer_name_change() {
        let mut consumer = RcBiConsumer::new(|_x: &i32, _y: &i32| {});
        consumer.set_name("name1");
        assert_eq!(consumer.name(), Some("name1"));
        consumer.set_name("name2");
        assert_eq!(consumer.name(), Some("name2"));
    }
}

// ============================================================================
// Display and Debug Tests
// ============================================================================
