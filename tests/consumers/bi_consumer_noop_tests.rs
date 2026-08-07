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
mod noop_tests {
    use super::Arc;
    use super::ArcBiConsumer;
    use super::BiConsumer;
    use super::BoxBiConsumer;
    use super::Rc;
    use super::RcBiConsumer;

    #[test]
    fn test_box_noop_multiple_accepts() {
        let noop = BoxBiConsumer::<i32, i32>::noop();
        noop.accept(&1, &2);
        noop.accept(&3, &4);
        noop.accept(&5, &6);
        // Should not panic and do nothing
    }

    #[test]
    fn test_arc_noop_multiple_accepts() {
        let noop = ArcBiConsumer::<i32, i32>::noop();
        noop.accept(&1, &2);
        noop.accept(&3, &4);
        noop.accept(&5, &6);
        // Should not panic and do nothing
    }

    #[test]
    fn test_rc_noop_multiple_accepts() {
        let noop = RcBiConsumer::<i32, i32>::noop();
        noop.accept(&1, &2);
        noop.accept(&3, &4);
        noop.accept(&5, &6);
        // Should not panic and do nothing
    }

    #[test]
    fn test_box_noop_with_and_then() {
        let counter = Arc::new(std::sync::Mutex::new(0));
        let c = counter.clone();
        let active = BoxBiConsumer::new(move |_x: &i32, _y: &i32| {
            *c.lock().expect("mutex should not be poisoned") += 1;
        });
        let chained = active.and_then(BoxBiConsumer::noop());
        chained.accept(&1, &2);
        assert_eq!(*counter.lock().expect("mutex should not be poisoned"), 1);
    }

    #[test]
    fn test_arc_noop_with_and_then() {
        let counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let c = counter.clone();
        let active = ArcBiConsumer::new(move |_x: &i32, _y: &i32| {
            c.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        });
        let noop = ArcBiConsumer::<i32, i32>::noop();
        let chained = active.and_then(noop);
        chained.accept(&1, &2);
        assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 1);
    }

    #[test]
    fn test_rc_noop_with_and_then() {
        let counter = Rc::new(std::cell::Cell::new(0));
        let c = counter.clone();
        let active = RcBiConsumer::new(move |_x: &i32, _y: &i32| {
            c.set(c.get() + 1);
        });
        let chained = active.and_then(RcBiConsumer::<i32, i32>::noop());
        chained.accept(&1, &2);
        assert_eq!(counter.get(), 1);
    }
}
// ============================================================================
// to_once Tests - Testing BiConsumer trait default to_once implementation
// ============================================================================

// ============================================================================
// Conditional BiConsumer Tests
// ============================================================================
