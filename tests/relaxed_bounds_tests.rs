// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
#![cfg(feature = "full")]
use std::cell::{
    Cell,
    RefCell,
};
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::{
    Arc,
    Mutex,
    atomic::{
        AtomicI32,
        Ordering,
    },
};

use qubit_function::{
    ArcConsumer,
    ArcMutator,
    ArcPredicate,
    ArcStatefulBiConsumer,
    ArcStatefulBiTransformer,
    ArcStatefulConsumer,
    ArcStatefulFunction,
    ArcStatefulMutatingFunction,
    ArcStatefulMutator,
    ArcStatefulSupplier,
    ArcStatefulTransformer,
    ArcSupplier,
    ArcTransformer,
    BiConsumer,
    BiConsumerOnce,
    BiTransformer,
    BiTransformerOnce,
    BinaryOperator,
    BinaryOperatorOnce,
    BoxBiConsumer,
    BoxBiConsumerOnce,
    BoxConsumer,
    BoxConsumerOnce,
    BoxMutator,
    BoxMutatorOnce,
    BoxPredicate,
    BoxSupplier,
    BoxSupplierOnce,
    BoxTransformer,
    BoxTransformerOnce,
    Consumer,
    ConsumerOnce,
    Mutator,
    MutatorOnce,
    Predicate,
    RcConsumer,
    StatefulBiConsumer,
    StatefulBiTransformer,
    StatefulConsumer,
    StatefulFunction,
    StatefulMutatingFunction,
    StatefulMutator,
    StatefulSupplier,
    StatefulTransformer,
    Supplier,
    SupplierOnce,
    Transformer,
    TransformerOnce,
    UnaryOperator,
    UnaryOperatorOnce,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Borrowed<'a> {
    value: &'a i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BorrowedUnaryOp;

impl<'a> Transformer<Borrowed<'a>, Borrowed<'a>> for BorrowedUnaryOp {
    fn apply(&self, input: Borrowed<'a>) -> Borrowed<'a> {
        input
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BorrowedBinaryOp;

impl<'a> BiTransformer<Borrowed<'a>, Borrowed<'a>, Borrowed<'a>>
    for BorrowedBinaryOp
{
    fn apply(
        &self,
        first: Borrowed<'a>,
        _second: Borrowed<'a>,
    ) -> Borrowed<'a> {
        first
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BorrowedUnaryOpOnce;

impl<'a> TransformerOnce<Borrowed<'a>, Borrowed<'a>> for BorrowedUnaryOpOnce {
    fn apply(self, input: Borrowed<'a>) -> Borrowed<'a> {
        input
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BorrowedBinaryOpOnce;

impl<'a> BiTransformerOnce<Borrowed<'a>, Borrowed<'a>, Borrowed<'a>>
    for BorrowedBinaryOpOnce
{
    fn apply(self, first: Borrowed<'a>, _second: Borrowed<'a>) -> Borrowed<'a> {
        first
    }
}

#[test]
fn test_consumers_allow_non_static_generic_on_new() {
    let n = 7;
    let input = Borrowed { value: &n };

    let box_sum = Rc::new(RefCell::new(0));
    let rc_sum = Rc::new(RefCell::new(0));
    let arc_sum = Arc::new(Mutex::new(0));

    let box_consumer = BoxConsumer::new({
        let box_sum = Rc::clone(&box_sum);
        move |item: &Borrowed<'_>| {
            *box_sum.borrow_mut() += *item.value;
        }
    });
    box_consumer.accept(&input);

    let rc_consumer = RcConsumer::new({
        let rc_sum = Rc::clone(&rc_sum);
        move |item: &Borrowed<'_>| {
            *rc_sum.borrow_mut() += *item.value;
        }
    });
    rc_consumer.accept(&input);

    let arc_consumer = ArcConsumer::new({
        let arc_sum = Arc::clone(&arc_sum);
        move |item: &Borrowed<'_>| {
            *arc_sum.lock().expect("lock should succeed") += *item.value;
        }
    });
    arc_consumer.accept(&input);

    assert_eq!(*box_sum.borrow(), 7);
    assert_eq!(*rc_sum.borrow(), 7);
    assert_eq!(*arc_sum.lock().expect("lock should succeed"), 7);
}

#[test]
fn test_bi_consumer_allow_non_static_generic_on_new() {
    let n = 5;
    let input = Borrowed { value: &n };
    let sink = Rc::new(RefCell::new(String::new()));

    let bi_consumer = BoxBiConsumer::new({
        let sink = Rc::clone(&sink);
        move |prefix: &&str, item: &Borrowed<'_>| {
            *sink.borrow_mut() = format!("{}-{}", *prefix, item.value);
        }
    });

    bi_consumer.accept(&"ok", &input);
    assert_eq!(&*sink.borrow(), "ok-5");
}

#[test]
fn test_consumer_once_allow_non_static_generic_on_new() {
    let n = 3;
    let input = Borrowed { value: &n };
    let sink = Rc::new(RefCell::new(0));

    let consumer_once = BoxConsumerOnce::new({
        let sink = Rc::clone(&sink);
        move |item: &Borrowed<'_>| {
            *sink.borrow_mut() = *item.value;
        }
    });
    consumer_once.accept(&input);

    let bi_sink = Rc::new(RefCell::new(0));
    let bi_consumer_once = BoxBiConsumerOnce::new({
        let bi_sink = Rc::clone(&bi_sink);
        move |left: &Borrowed<'_>, right: &Borrowed<'_>| {
            *bi_sink.borrow_mut() = *left.value + *right.value;
        }
    });
    bi_consumer_once.accept(&input, &input);

    assert_eq!(*sink.borrow(), 3);
    assert_eq!(*bi_sink.borrow(), 6);
}

#[test]
fn test_mutators_allow_non_static_generic_on_new() {
    let n = 11;
    let mut slot = Some(&n);

    let box_mutator = BoxMutator::new(|value: &mut Option<&i32>| {
        *value = None;
    });
    box_mutator.apply(&mut slot);
    assert_eq!(slot, None);

    let arc_mutator = ArcMutator::new(|value: &mut Option<&i32>| {
        if value.is_none() {
            *value = Some(&42);
        }
    });
    arc_mutator.apply(&mut slot);
    assert_eq!(slot, Some(&42));
}

#[test]
fn test_mutator_once_allow_non_static_generic_on_new() {
    let n = 9;
    let mut slot = Some(&n);

    let mutator_once = BoxMutatorOnce::new(|value: &mut Option<&i32>| {
        *value = None;
    });
    mutator_once.apply(&mut slot);

    assert_eq!(slot, None);
}

#[test]
fn test_predicate_and_transformer_allow_non_static_generic_on_new() {
    let n = 13;
    let value = Borrowed { value: &n };

    let predicate = BoxPredicate::new(|item: &Borrowed<'_>| *item.value > 10);
    assert!(predicate.test(&value));

    let arc_predicate =
        ArcPredicate::new(|item: &Borrowed<'_>| *item.value % 2 == 1);
    assert!(arc_predicate.test(&value));

    let transformer = BoxTransformer::new(|item: Borrowed<'_>| *item.value + 1);
    assert_eq!(transformer.apply(value), 14);

    let arc_transformer =
        ArcTransformer::new(|item: Borrowed<'_>| *item.value - 1);
    assert_eq!(arc_transformer.apply(value), 12);
}

#[test]
fn test_transformer_once_allow_non_static_generic_on_new() {
    let n = 8;
    let value = Borrowed { value: &n };

    let transformer_once =
        BoxTransformerOnce::new(|item: Borrowed<'_>| *item.value * 2);
    assert_eq!(transformer_once.apply(value), 16);
}

#[test]
fn test_suppliers_allow_non_static_generic_on_new() {
    let n = 21;

    let box_supplier: BoxSupplier<PhantomData<&i32>> =
        make_box_supplier_with_lifetime(&n);
    let box_supplier_once: BoxSupplierOnce<PhantomData<&i32>> =
        make_box_supplier_once_with_lifetime(&n);
    let arc_supplier: ArcSupplier<PhantomData<&i32>> =
        make_arc_supplier_with_lifetime(&n);

    assert_eq!(box_supplier.get(), PhantomData);
    assert_eq!(box_supplier_once.get(), PhantomData);
    assert_eq!(arc_supplier.get(), PhantomData);
}

fn make_box_supplier_with_lifetime(_: &i32) -> BoxSupplier<PhantomData<&i32>> {
    BoxSupplier::new(|| PhantomData)
}

fn make_box_supplier_once_with_lifetime(
    _: &i32,
) -> BoxSupplierOnce<PhantomData<&i32>> {
    BoxSupplierOnce::new(|| PhantomData)
}

fn make_arc_supplier_with_lifetime(_: &i32) -> ArcSupplier<PhantomData<&i32>> {
    ArcSupplier::new(|| PhantomData)
}

#[test]
fn test_semantic_traits_allow_non_static_closure_implementations() {
    let a = 3;

    let stateful_function = |value: &Borrowed<'_>| *value.value;
    assert_stateful_function_impl(&a, stateful_function);

    let bi_transformer_with_borrow =
        |left: Borrowed<'_>, right: Borrowed<'_>| *left.value + *right.value;
    assert_bi_transformer_impl(&a, bi_transformer_with_borrow);

    assert_unary_operator_impl(&a, BorrowedUnaryOp);
    assert_binary_operator_impl(&a, BorrowedBinaryOp);
    assert_unary_operator_once_impl(&a, BorrowedUnaryOpOnce);
    assert_binary_operator_once_impl(&a, BorrowedBinaryOpOnce);
}

fn assert_stateful_function_impl<'a, F>(_: &'a i32, f: F)
where
    F: StatefulFunction<Borrowed<'a>, i32>,
{
    let _ = f;
}

fn assert_bi_transformer_impl<'a, F>(_: &'a i32, f: F)
where
    F: BiTransformer<Borrowed<'a>, Borrowed<'a>, i32>,
{
    let _ = f;
}

fn assert_unary_operator_impl<'a, F>(_: &'a i32, f: F)
where
    F: UnaryOperator<Borrowed<'a>>,
{
    let _ = f;
}

fn assert_binary_operator_impl<'a, F>(_: &'a i32, f: F)
where
    F: BinaryOperator<Borrowed<'a>>,
{
    let _ = f;
}

fn assert_unary_operator_once_impl<'a, F>(_: &'a i32, f: F)
where
    F: UnaryOperatorOnce<Borrowed<'a>>,
{
    let _ = f;
}

fn assert_binary_operator_once_impl<'a, F>(_: &'a i32, f: F)
where
    F: BinaryOperatorOnce<Borrowed<'a>>,
{
    let _ = f;
}

#[test]
fn test_stateful_arc_and_then_accepts_send_non_sync_callbacks() {
    let observed = Arc::new(AtomicI32::new(0));
    let observed_capture = Arc::clone(&observed);
    let state = Cell::new(0);
    let mut consumer =
        ArcStatefulConsumer::new(|_: &i32| {}).and_then(move |value: &i32| {
            state.set(*value);
            observed_capture.store(state.get(), Ordering::SeqCst);
        });
    consumer.accept(&3);
    assert_eq!(observed.load(Ordering::SeqCst), 3);

    let observed = Arc::new(AtomicI32::new(0));
    let observed_capture = Arc::clone(&observed);
    let state = Cell::new(0);
    let mut bi_consumer = ArcStatefulBiConsumer::new(|_: &i32, _: &i32| {})
        .and_then(move |left: &i32, right: &i32| {
            state.set(*left + *right);
            observed_capture.store(state.get(), Ordering::SeqCst);
        });
    bi_consumer.accept(&2, &4);
    assert_eq!(observed.load(Ordering::SeqCst), 6);

    let state = Cell::new(0);
    let mut function = ArcStatefulFunction::new(|value: &i32| *value + 1)
        .and_then(move |value: &i32| {
            state.set(*value);
            state.get() * 2
        });
    assert_eq!(function.apply(&2), 6);

    let state = Cell::new(0);
    let mut mutating_function =
        ArcStatefulMutatingFunction::new(|value: &mut i32| {
            *value += 1;
            *value
        })
        .and_then(move |value: &i32| {
            state.set(*value);
            state.get() * 2
        });
    let mut input = 2;
    assert_eq!(mutating_function.apply(&mut input), 6);

    let state = Cell::new(0);
    let mut mutator = ArcStatefulMutator::new(|value: &mut i32| *value += 1)
        .and_then(move |value: &mut i32| {
            state.set(*value);
            *value *= 2;
        });
    let mut input = 2;
    mutator.apply(&mut input);
    assert_eq!(input, 6);

    let state = Cell::new(0);
    let mut transformer = ArcStatefulTransformer::new(|value: i32| value + 1)
        .and_then(move |value: i32| {
            state.set(value);
            state.get() * 2
        });
    assert_eq!(transformer.apply(2), 6);

    let state = Cell::new(0);
    let mut bi_transformer =
        ArcStatefulBiTransformer::new(|left: i32, right: i32| left + right)
            .and_then(move |value: i32| {
                state.set(value);
                state.get() * 2
            });
    assert_eq!(bi_transformer.apply(2, 4), 12);
}

#[test]
fn test_stateful_arc_conditionals_accept_send_non_sync_callbacks() {
    let state = Cell::new(0);
    let observed = Arc::new(AtomicI32::new(0));
    let observed_capture = Arc::clone(&observed);
    let mut consumer = ArcStatefulConsumer::new(|_: &i32| {})
        .when(|value: &i32| *value > 0)
        .or_else(move |value: &i32| {
            state.set(*value);
            observed_capture.store(state.get(), Ordering::SeqCst);
        });
    consumer.accept(&-2);
    assert_eq!(observed.load(Ordering::SeqCst), -2);

    let state = Cell::new(0);
    let observed = Arc::new(AtomicI32::new(0));
    let observed_capture = Arc::clone(&observed);
    let mut bi_consumer = ArcStatefulBiConsumer::new(|_: &i32, _: &i32| {})
        .when(|left: &i32, right: &i32| *left > 0 && *right > 0)
        .or_else(move |left: &i32, right: &i32| {
            state.set(*left + *right);
            observed_capture.store(state.get(), Ordering::SeqCst);
        });
    bi_consumer.accept(&-2, &4);
    assert_eq!(observed.load(Ordering::SeqCst), 2);

    let state = Cell::new(0);
    let mut function = ArcStatefulFunction::new(|value: &i32| *value)
        .when(|value: &i32| *value > 0)
        .or_else(move |value: &i32| {
            state.set(*value);
            state.get()
        });
    assert_eq!(function.apply(&-2), -2);

    let state = Cell::new(0);
    let mut mutating_function =
        ArcStatefulMutatingFunction::new(|value: &mut i32| *value)
            .when(|value: &i32| *value > 0)
            .or_else(move |value: &mut i32| {
                state.set(*value);
                *value -= 1;
                *value
            });
    let mut input = -2;
    assert_eq!(mutating_function.apply(&mut input), -3);

    let state = Cell::new(0);
    let mut mutator = ArcStatefulMutator::new(|value: &mut i32| *value += 1)
        .when(|value: &i32| *value > 0)
        .or_else(move |value: &mut i32| {
            state.set(*value);
            *value -= 1;
        });
    let mut input = -2;
    mutator.apply(&mut input);
    assert_eq!(input, -3);

    let state = Cell::new(0);
    let mut transformer = ArcStatefulTransformer::new(|value: i32| value + 1)
        .when(|value: &i32| *value > 0)
        .or_else(move |value: i32| {
            state.set(value);
            state.get() - 1
        });
    assert_eq!(transformer.apply(-2), -3);

    let state = Cell::new(0);
    let mut bi_transformer =
        ArcStatefulBiTransformer::new(|left: i32, right: i32| left + right)
            .when(|left: &i32, right: &i32| *left > 0 && *right > 0)
            .or_else(move |left: i32, right: i32| {
                state.set(left + right);
                state.get() - 1
            });
    assert_eq!(bi_transformer.apply(-2, 4), 1);
}

#[test]
fn test_arc_stateful_supplier_combinators_accept_send_non_sync_callbacks() {
    let map_state = Cell::new(0);
    let mut mapped = ArcStatefulSupplier::new(|| 2).map(move |value| {
        map_state.set(value);
        map_state.get() * 2
    });
    assert_eq!(mapped.get(), 4);

    let filter_state = Cell::new(0);
    let mut filtered =
        ArcStatefulSupplier::new(|| 2).filter(move |value: &i32| {
            filter_state.set(*value);
            filter_state.get() % 2 == 0
        });
    assert_eq!(filtered.get(), Some(2));

    let zip_state = Cell::new(0);
    let mut zipped = ArcStatefulSupplier::new(|| 2).zip(move || {
        zip_state.set(zip_state.get() + 1);
        zip_state.get()
    });
    assert_eq!(zipped.get(), (2, 1));
}

#[derive(Clone)]
struct SendNonSyncValue(Cell<i32>);

#[test]
fn test_arc_stateful_function_constant_accepts_send_non_sync_value() {
    let mut constant = ArcStatefulFunction::<(), SendNonSyncValue>::constant(
        SendNonSyncValue(Cell::new(7)),
    );
    assert_eq!(constant.apply(&()).0.get(), 7);
}
