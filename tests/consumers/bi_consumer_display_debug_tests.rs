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
mod display_debug_tests {
    use super::ArcBiConsumer;
    use super::BoxBiConsumer;
    use super::RcBiConsumer;

    #[test]
    fn test_box_consumer_debug() {
        let consumer = BoxBiConsumer::new(|_x: &i32, _y: &i32| {});
        let debug_str = format!("{:?}", consumer);
        assert!(debug_str.contains("BoxBiConsumer"));
        assert!(debug_str.contains("name"));
        assert!(debug_str.contains("function"));
    }

    #[test]
    fn test_box_consumer_display_without_name() {
        let consumer = BoxBiConsumer::new(|_x: &i32, _y: &i32| {});
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "BoxBiConsumer");
    }

    #[test]
    fn test_box_consumer_display_with_name() {
        let mut consumer = BoxBiConsumer::new(|_x: &i32, _y: &i32| {});
        consumer.set_name("test_consumer");
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "BoxBiConsumer(test_consumer)");
    }

    #[test]
    fn test_arc_consumer_debug() {
        let consumer = ArcBiConsumer::new(|_x: &i32, _y: &i32| {});
        let debug_str = format!("{:?}", consumer);
        assert!(debug_str.contains("ArcBiConsumer"));
        assert!(debug_str.contains("name"));
        assert!(debug_str.contains("function"));
    }

    #[test]
    fn test_arc_consumer_display_without_name() {
        let consumer = ArcBiConsumer::new(|_x: &i32, _y: &i32| {});
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "ArcBiConsumer");
    }

    #[test]
    fn test_arc_consumer_display_with_name() {
        let mut consumer = ArcBiConsumer::new(|_x: &i32, _y: &i32| {});
        consumer.set_name("test_consumer");
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "ArcBiConsumer(test_consumer)");
    }

    #[test]
    fn test_rc_consumer_debug() {
        let consumer = RcBiConsumer::new(|_x: &i32, _y: &i32| {});
        let debug_str = format!("{:?}", consumer);
        assert!(debug_str.contains("RcBiConsumer"));
        assert!(debug_str.contains("name"));
        assert!(debug_str.contains("function"));
    }

    #[test]
    fn test_rc_consumer_display_without_name() {
        let consumer = RcBiConsumer::new(|_x: &i32, _y: &i32| {});
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "RcBiConsumer");
    }

    #[test]
    fn test_rc_consumer_display_with_name() {
        let mut consumer = RcBiConsumer::new(|_x: &i32, _y: &i32| {});
        consumer.set_name("test_consumer");
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "RcBiConsumer(test_consumer)");
    }
}
