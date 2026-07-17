// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
#![allow(
    dead_code,
    unused_imports,
    reason = "split contract fixtures share support definitions"
)]
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

#[derive(Clone)]
struct SendNonSyncValue(Cell<i32>);

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

#[test]
fn test_arc_stateful_function_constant_accepts_send_non_sync_value() {
    let mut constant = ArcStatefulFunction::<(), SendNonSyncValue>::constant(
        SendNonSyncValue(Cell::new(7)),
    );
    assert_eq!(constant.apply(&()).0.get(), 7);
}
