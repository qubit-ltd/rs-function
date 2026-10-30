// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxFunction;
use qubit_function::Function;

#[test]
fn test_box_function_borrows_input_and_returns_result() {
    let format = BoxFunction::new(|value: &i32| format!("value={value}"));
    let input = 42;

    assert_eq!(format.apply(&input), "value=42");
    assert_eq!(input, 42);
}

#[test]
fn test_box_mutating_function_and_mutator_update_input() {
    use qubit_function::BoxMutatingFunction;
    use qubit_function::BoxMutator;
    use qubit_function::MutatingFunction;
    use qubit_function::Mutator;

    let update_and_return = BoxMutatingFunction::new(|value: &mut i32| {
        *value += 3;
        *value * 2
    });
    let mut value = 4;

    assert_eq!(update_and_return.apply(&mut value), 14);
    assert_eq!(value, 7);

    let add_ten = BoxMutator::new(|value: &mut i32| *value += 10);
    add_ten.apply(&mut value);
    assert_eq!(value, 17);
}

#[test]
fn test_box_transformer_consumes_owned_string() {
    use qubit_function::BoxTransformer;
    use qubit_function::Transformer;

    let describe = BoxTransformer::new(|value: String| format!("{}:{}", value.len(), value));
    let result = describe.apply(String::from("rust"));

    assert_eq!(result, "4:rust");
}

#[test]
fn test_box_consumer_when_rejects_without_running_callback() {
    use std::cell::Cell;
    use std::rc::Rc;

    use qubit_function::BoxConsumer;
    use qubit_function::Consumer;

    let calls = Rc::new(Cell::new(0));
    let observed = Rc::clone(&calls);
    let conditional = BoxConsumer::new(move |_: &i32| observed.set(observed.get() + 1)).when(|value: &i32| *value > 0);

    conditional.accept(&-2);
    assert_eq!(calls.get(), 0);

    conditional.accept(&5);
    assert_eq!(calls.get(), 1);
}

#[test]
fn test_box_predicate_and_tester_short_circuit_and_or() {
    use std::cell::Cell;
    use std::rc::Rc;

    use qubit_function::BoxPredicate;
    use qubit_function::BoxTester;
    use qubit_function::Predicate;
    use qubit_function::Tester;

    let predicate_calls = Rc::new(Cell::new(0));
    let observed_predicate_calls = Rc::clone(&predicate_calls);
    let predicate_and = BoxPredicate::new(|_: &i32| false).and(move |_: &i32| {
        observed_predicate_calls.set(observed_predicate_calls.get() + 1);
        true
    });
    assert!(!predicate_and.test(&10));
    assert_eq!(predicate_calls.get(), 0);

    let predicate_or_calls = Rc::new(Cell::new(0));
    let observed_predicate_or_calls = Rc::clone(&predicate_or_calls);
    let predicate_or = BoxPredicate::new(|_: &i32| true).or(move |_: &i32| {
        observed_predicate_or_calls.set(observed_predicate_or_calls.get() + 1);
        false
    });
    assert!(predicate_or.test(&10));
    assert_eq!(predicate_or_calls.get(), 0);

    let tester_and_calls = Rc::new(Cell::new(0));
    let observed_tester_and_calls = Rc::clone(&tester_and_calls);
    let tester_and = BoxTester::new(|| false).and(move || {
        observed_tester_and_calls.set(observed_tester_and_calls.get() + 1);
        true
    });
    assert!(!tester_and.test());
    assert_eq!(tester_and_calls.get(), 0);

    let tester_or_calls = Rc::new(Cell::new(0));
    let observed_tester_or_calls = Rc::clone(&tester_or_calls);
    let tester_or = BoxTester::new(|| true).or(move || {
        observed_tester_or_calls.set(observed_tester_or_calls.get() + 1);
        false
    });
    assert!(tester_or.test());
    assert_eq!(tester_or_calls.get(), 0);
}

#[test]
fn test_box_supplier_map_returns_mapped_value() {
    use qubit_function::BoxSupplier;
    use qubit_function::Supplier;

    let description =
        BoxSupplier::new(|| String::from("item")).map(|value: String| format!("{}:{}", value.len(), value));

    assert_eq!(description.get(), "4:item");
}

#[test]
fn test_box_comparator_runs_secondary_only_when_primary_is_equal() {
    use std::cell::Cell;
    use std::cmp::Ordering;
    use std::rc::Rc;

    use qubit_function::BoxComparator;
    use qubit_function::Comparator;

    let secondary_calls = Rc::new(Cell::new(0));
    let observed_calls = Rc::clone(&secondary_calls);
    let comparator =
        BoxComparator::new(|left: &i32, right: &i32| left.cmp(right)).then_comparing(move |left: &i32, right: &i32| {
            observed_calls.set(observed_calls.get() + 1);
            right.cmp(left)
        });

    assert_eq!(comparator.compare(&4, &4), Ordering::Equal);
    assert_eq!(secondary_calls.get(), 1);
    assert_eq!(comparator.compare(&1, &3), Ordering::Less);
    assert_eq!(secondary_calls.get(), 1);
}

#[test]
fn test_box_callable_and_runnable_skip_next_after_error() {
    use std::sync::Arc;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;

    use qubit_function::BoxCallable;
    use qubit_function::BoxRunnable;
    use qubit_function::Callable;
    use qubit_function::Runnable;

    let callable_next_calls = Arc::new(AtomicUsize::new(0));
    let observed_callable_calls = Arc::clone(&callable_next_calls);
    let mut callable = BoxCallable::new(|| Err::<i32, _>("call failed")).and_then(move |_| {
        observed_callable_calls.fetch_add(1, Ordering::Relaxed);
        Ok(10)
    });
    assert_eq!(callable.call(), Err("call failed"));
    assert_eq!(callable_next_calls.load(Ordering::Relaxed), 0);

    let runnable_next_calls = Arc::new(AtomicUsize::new(0));
    let observed_runnable_calls = Arc::clone(&runnable_next_calls);
    let mut runnable = BoxRunnable::new(|| Err::<(), _>("run failed")).and_then(move || {
        observed_runnable_calls.fetch_add(1, Ordering::Relaxed);
        Ok(())
    });
    assert_eq!(runnable.run(), Err("run failed"));
    assert_eq!(runnable_next_calls.load(Ordering::Relaxed), 0);
}

#[test]
fn test_box_with_tasks_short_circuit_and_preserve_mutated_input() {
    use std::sync::Arc;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;

    use qubit_function::BoxCallableWith;
    use qubit_function::BoxRunnableWith;
    use qubit_function::CallableWith;
    use qubit_function::RunnableWith;

    let callable_next_calls = Arc::new(AtomicUsize::new(0));
    let observed_callable_calls = Arc::clone(&callable_next_calls);
    let mut callable = BoxCallableWith::new(|value: &mut i32| {
        *value += 2;
        Err::<i32, _>("call-with failed")
    })
    .and_then(move |_, value: &mut i32| {
        observed_callable_calls.fetch_add(1, Ordering::Relaxed);
        Ok(*value)
    });
    let mut callable_input = 3;

    assert_eq!(callable.call_with(&mut callable_input), Err("call-with failed"));
    assert_eq!(callable_input, 5);
    assert_eq!(callable_next_calls.load(Ordering::Relaxed), 0);

    let runnable_next_calls = Arc::new(AtomicUsize::new(0));
    let observed_runnable_calls = Arc::clone(&runnable_next_calls);
    let mut runnable = BoxRunnableWith::new(|value: &mut i32| {
        *value += 4;
        Err::<(), _>("run-with failed")
    })
    .and_then(move |value: &mut i32| {
        observed_runnable_calls.fetch_add(1, Ordering::Relaxed);
        *value += 100;
        Ok(())
    });
    let mut runnable_input = 1;

    assert_eq!(runnable.run_with(&mut runnable_input), Err("run-with failed"));
    assert_eq!(runnable_input, 5);
    assert_eq!(runnable_next_calls.load(Ordering::Relaxed), 0);
}

#[cfg(feature = "rc")]
#[test]
fn test_rc_function_clone_shares_callback_and_copies_name() {
    use std::cell::Cell;
    use std::rc::Rc;

    use qubit_function::RcFunction;

    let calls = Rc::new(Cell::new(0));
    let observed_calls = Rc::clone(&calls);
    let mut original = RcFunction::new_with_name("original", move |value: &i32| {
        observed_calls.set(observed_calls.get() + 1);
        value * 2
    });
    let cloned = original.clone();
    original.set_name("renamed");

    assert_eq!(original.name(), Some("renamed"));
    assert_eq!(cloned.name(), Some("original"));
    assert_eq!(original.apply(&3), 6);
    assert_eq!(cloned.apply(&4), 8);
    assert_eq!(calls.get(), 2);
}

#[cfg(feature = "once")]
#[test]
fn test_box_callable_once_consumes_non_clone_capture() {
    use qubit_function::BoxCallableOnce;
    use qubit_function::CallableOnce;

    let payload = String::from("owned once");
    let task = BoxCallableOnce::new(move || Ok::<String, ()>(payload));

    assert_eq!(task.call_once(), Ok(String::from("owned once")));
}

#[cfg(feature = "stateful")]
#[test]
fn test_arc_stateful_supplier_clones_share_sequence() {
    use qubit_function::ArcStatefulSupplier;
    use qubit_function::StatefulSupplier;

    let mut first = ArcStatefulSupplier::new({
        let mut value = 0;
        move || {
            value += 1;
            value
        }
    });
    let mut second = first.clone();

    assert_eq!(first.get(), 1);
    assert_eq!(second.get(), 2);
    assert_eq!(first.get(), 3);
}

#[cfg(all(feature = "rc", feature = "stateful"))]
#[test]
fn test_rc_stateful_supplier_clones_share_sequence() {
    use qubit_function::RcStatefulSupplier;
    use qubit_function::StatefulSupplier;

    let mut first = RcStatefulSupplier::new({
        let mut value = 0;
        move || {
            value += 1;
            value
        }
    });
    let mut second = first.clone();

    assert_eq!(first.get(), 1);
    assert_eq!(second.get(), 2);
    assert_eq!(first.get(), 3);
}
