// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Tests for Consumer types

use std::rc::Rc;
use std::sync::Arc;

use qubit_function::ArcConsumer;
use qubit_function::BoxConsumer;
use qubit_function::Consumer;
use qubit_function::RcConsumer;

#[cfg(test)]
mod display_debug_tests {
    use super::ArcConsumer;
    use super::BoxConsumer;
    use super::RcConsumer;

    #[test]
    fn test_box_consumer_debug() {
        let consumer = BoxConsumer::new(|_x: &i32| {});
        let debug_str = format!("{:?}", consumer);
        assert!(debug_str.contains("BoxConsumer"));
        assert!(debug_str.contains("name"));
        assert!(debug_str.contains("function"));
    }

    #[test]
    fn test_box_consumer_display_without_name() {
        let consumer = BoxConsumer::new(|_x: &i32| {});
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "BoxConsumer");
    }

    #[test]
    fn test_box_consumer_display_with_name() {
        let mut consumer = BoxConsumer::new(|_x: &i32| {});
        consumer.set_name("test_consumer");
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "BoxConsumer(test_consumer)");
    }

    #[test]
    fn test_arc_consumer_debug() {
        let consumer = ArcConsumer::new(|_x: &i32| {});
        let debug_str = format!("{:?}", consumer);
        assert!(debug_str.contains("ArcConsumer"));
        assert!(debug_str.contains("name"));
        assert!(debug_str.contains("function"));
    }

    #[test]
    fn test_arc_consumer_display_without_name() {
        let consumer = ArcConsumer::new(|_x: &i32| {});
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "ArcConsumer");
    }

    #[test]
    fn test_arc_consumer_display_with_name() {
        let mut consumer = ArcConsumer::new(|_x: &i32| {});
        consumer.set_name("test_consumer");
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "ArcConsumer(test_consumer)");
    }

    #[test]
    fn test_rc_consumer_debug() {
        let consumer = RcConsumer::new(|_x: &i32| {});
        let debug_str = format!("{:?}", consumer);
        assert!(debug_str.contains("RcConsumer"));
        assert!(debug_str.contains("name"));
        assert!(debug_str.contains("function"));
    }

    #[test]
    fn test_rc_consumer_display_without_name() {
        let consumer = RcConsumer::new(|_x: &i32| {});
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "RcConsumer");
    }

    #[test]
    fn test_rc_consumer_display_with_name() {
        let mut consumer = RcConsumer::new(|_x: &i32| {});
        consumer.set_name("test_consumer");
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "RcConsumer(test_consumer)");
    }
}
// ============================================================================
// to_xxx Methods Tests - Testing non-consuming conversion methods
// ============================================================================

// ============================================================================
// to_once Tests - Testing Consumer trait default to_once implementation
// ============================================================================

// ============================================================================
// Conditional Consumer Tests
// ============================================================================
