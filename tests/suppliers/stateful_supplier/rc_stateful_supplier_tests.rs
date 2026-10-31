// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::panic::AssertUnwindSafe;
use std::panic::catch_unwind;

use qubit_function::RcStatefulSupplier;
use qubit_function::StatefulSupplier;

use crate::internal::isolated_test::run_isolated;

#[test]
fn test_rc_stateful_supplier_clones_share_mutable_callback_state() {
    let mut count = 0;
    let mut supplier = RcStatefulSupplier::new(move || {
        count += 1;
        count
    });
    let mut cloned = supplier.clone();

    assert_eq!(supplier.get(), 1);
    assert_eq!(cloned.get(), 2);
}

#[test]
fn test_rc_map_stateful_releases_source_before_mapper() {
    let qualified = concat!(
        module_path!(),
        "::",
        "test_rc_map_stateful_releases_source_before_mapper"
    );
    let (_, exact_name) = qualified.split_once("::").expect("test path includes crate prefix");
    run_isolated(exact_name, || {
        let mut sequence = 0;
        let mut source = RcStatefulSupplier::new(move || {
            sequence += 1;
            sequence
        });
        let mut inside_mapper = source.clone();
        let mut mapped = source.map_stateful(move |value: i32| (value, inside_mapper.get()));
        assert_eq!(mapped.get(), (1, 2));
        assert_eq!(mapped.get(), (3, 4));
        assert_eq!(source.get(), 5);
    });
}

#[test]
fn test_rc_map_stateful_recovers_after_mapper_panic() {
    let qualified = concat!(module_path!(), "::", "test_rc_map_stateful_recovers_after_mapper_panic");
    let (_, exact_name) = qualified.split_once("::").expect("test path includes crate prefix");
    run_isolated(exact_name, || {
        let mut sequence = 0;
        let mut source = RcStatefulSupplier::new(move || {
            sequence += 1;
            sequence
        });
        let mut mapper_calls = 0;
        let mut mapped = source.map_stateful(move |value: i32| {
            mapper_calls += 1;
            if mapper_calls == 1 {
                panic!("intentional mapper panic");
            }
            value + 10 * mapper_calls
        });
        let mut cloned = mapped.clone();
        let result = catch_unwind(AssertUnwindSafe(|| mapped.get()));
        assert!(result.is_err(), "the first mapper panic must propagate");
        assert_eq!(mapped.get(), 22, "source and mapper state must survive panic");
        assert_eq!(cloned.get(), 33, "clones must still share mapper state");
        assert_eq!(source.get(), 4);
    });
}
