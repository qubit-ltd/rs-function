// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::cmp::Ordering;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering as AtomicOrdering;

use qubit_function::comparator::ArcComparator;
use qubit_function::comparator::Comparator;

struct NoClone(i32);

#[test]
fn arc_comparator_clone_does_not_require_clone_for_values() {
    let calls = Arc::new(AtomicUsize::new(0));
    let callback_calls = Arc::clone(&calls);
    let original = ArcComparator::new_with_name("original", move |left: &NoClone, right: &NoClone| {
        callback_calls.fetch_add(1, AtomicOrdering::SeqCst);
        left.0.cmp(&right.0)
    });
    let mut cloned = original.clone();

    assert_eq!(original.compare(&NoClone(1), &NoClone(2)), Ordering::Less);
    assert_eq!(cloned.compare(&NoClone(3), &NoClone(2)), Ordering::Greater);
    assert_eq!(original.compare(&NoClone(2), &NoClone(2)), Ordering::Equal);
    assert_eq!(calls.load(AtomicOrdering::SeqCst), 3);

    cloned.set_name("clone");
    assert_eq!(original.name(), Some("original"));
    assert_eq!(cloned.name(), Some("clone"));
}

#[cfg(feature = "rc")]
#[test]
fn rc_comparator_clone_does_not_require_clone_for_values() {
    use std::cell::Cell;
    use std::rc::Rc;

    use qubit_function::comparator::RcComparator;

    let calls = Rc::new(Cell::new(0));
    let callback_calls = Rc::clone(&calls);
    let original = RcComparator::new_with_name("original", move |left: &NoClone, right: &NoClone| {
        callback_calls.set(callback_calls.get() + 1);
        left.0.cmp(&right.0)
    });
    let mut cloned = original.clone();

    assert_eq!(original.compare(&NoClone(1), &NoClone(2)), Ordering::Less);
    assert_eq!(cloned.compare(&NoClone(3), &NoClone(2)), Ordering::Greater);
    assert_eq!(original.compare(&NoClone(2), &NoClone(2)), Ordering::Equal);
    assert_eq!(calls.get(), 3);

    cloned.set_name("clone");
    assert_eq!(original.name(), Some("original"));
    assert_eq!(cloned.name(), Some("clone"));
}
