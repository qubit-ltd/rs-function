// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::cmp::Ordering;

use qubit_function::ArcComparator;
use qubit_function::Comparator;

struct ArcNaturalComparator;

impl Comparator<i32> for ArcNaturalComparator {
    fn compare(&self, left: &i32, right: &i32) -> Ordering {
        left.cmp(right)
    }
}

/// Verifies naming, diagnostics, and clone-independent metadata updates.
#[test]
fn test_arc_comparator_name_and_diagnostics() {
    let original = ArcComparator::new_with_name("ascending", |left: &i32, right: &i32| left.cmp(right));
    let renamed = original.clone().with_name("renamed");

    assert_eq!(original.name(), Some("ascending"));
    assert_eq!(original.compare(&1, &2), Ordering::Less);
    assert_eq!(renamed.name(), Some("renamed"));
    assert_eq!(format!("{original:?}"), "ArcComparator { name: Some(\"ascending\") }");
    assert_eq!(format!("{original}"), "ArcComparator(ascending)");

    let unnamed = ArcComparator::new_with_optional_name(|left: &i32, right: &i32| left.cmp(right), None);
    assert_eq!(unnamed.compare(&2, &1), Ordering::Greater);
    assert_eq!(format!("{unnamed}"), "ArcComparator");
}

/// Verifies that Arc composition accepts thread-safe semantic comparators.
#[test]
fn test_arc_comparator_then_comparing_semantic_trait() {
    let first = ArcComparator::new(|_: &i32, _: &i32| Ordering::Equal);
    let comparator = first.then_comparing(ArcNaturalComparator);

    assert_eq!(comparator.compare(&1, &2), Ordering::Less);
    assert_eq!(first.compare(&2, &1), Ordering::Equal);
}

#[test]
fn test_comparing_accepts_unsized_keys() {
    let text = ArcComparator::comparing(|value: &String| value.as_str());
    assert_eq!(text.compare(&String::from("a"), &String::from("b")), Ordering::Less);
    assert_eq!(text.compare(&String::from("a"), &String::from("a")), Ordering::Equal);
    assert_eq!(text.compare(&String::from("b"), &String::from("a")), Ordering::Greater);

    let bytes = ArcComparator::comparing(|value: &Vec<u8>| value.as_slice());
    assert_eq!(bytes.compare(&vec![1, 2], &vec![1, 3]), Ordering::Less);
    assert_eq!(bytes.compare(&vec![1, 2], &vec![1, 2]), Ordering::Equal);
    assert_eq!(bytes.compare(&vec![1, 3], &vec![1, 2]), Ordering::Greater);
}
