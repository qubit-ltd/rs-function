// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::testers::tester::box_tester::BoxTester;

#[test]
fn test_box_tester_observable_behavior() {
    let type_name = std::any::type_name::<BoxTester>();
    assert!(type_name.contains("BoxTester"), "{type_name}");
}

/// Verifies naming and diagnostic formatting for `BoxTester`.
#[test]
fn test_box_tester_name_and_diagnostics() {
    let mut tester =
        BoxTester::new_with_optional_name(|| true, Some("ready".to_owned()));

    assert_eq!(tester.name(), Some("ready"));
    assert_eq!(format!("{tester:?}"), "BoxTester { name: Some(\"ready\") }");
    assert_eq!(format!("{tester}"), "BoxTester(ready)");

    tester.clear_name();
    assert_eq!(tester.name(), None);
    tester.set_name("healthy");
    assert_eq!(tester.name(), Some("healthy"));
}
