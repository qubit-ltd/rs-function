use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::{
    Arc,
    Mutex,
};

use qubit_function::{
    ArcConsumer,
    ArcMutator,
    ArcPredicate,
    ArcSupplier,
    ArcTransformer,
    BiConsumer,
    BiConsumerOnce,
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
    Supplier,
    SupplierOnce,
    Transformer,
    TransformerOnce,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Borrowed<'a> {
    value: &'a i32,
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

    let arc_predicate = ArcPredicate::new(|item: &Borrowed<'_>| *item.value % 2 == 1);
    assert!(arc_predicate.test(&value));

    let transformer = BoxTransformer::new(|item: Borrowed<'_>| *item.value + 1);
    assert_eq!(transformer.apply(value), 14);

    let arc_transformer = ArcTransformer::new(|item: Borrowed<'_>| *item.value - 1);
    assert_eq!(arc_transformer.apply(value), 12);
}

#[test]
fn test_transformer_once_allow_non_static_generic_on_new() {
    let n = 8;
    let value = Borrowed { value: &n };

    let transformer_once = BoxTransformerOnce::new(|item: Borrowed<'_>| *item.value * 2);
    assert_eq!(transformer_once.apply(value), 16);
}

#[test]
fn test_suppliers_allow_non_static_generic_on_new() {
    let n = 21;

    let box_supplier: BoxSupplier<PhantomData<&i32>> = make_box_supplier_with_lifetime(&n);
    let box_supplier_once: BoxSupplierOnce<PhantomData<&i32>> =
        make_box_supplier_once_with_lifetime(&n);
    let arc_supplier: ArcSupplier<PhantomData<&i32>> = make_arc_supplier_with_lifetime(&n);

    assert_eq!(box_supplier.get(), PhantomData);
    assert_eq!(box_supplier_once.get(), PhantomData);
    assert_eq!(arc_supplier.get(), PhantomData);
}

fn make_box_supplier_with_lifetime(_: &i32) -> BoxSupplier<PhantomData<&i32>> {
    BoxSupplier::new(|| PhantomData)
}

fn make_box_supplier_once_with_lifetime(_: &i32) -> BoxSupplierOnce<PhantomData<&i32>> {
    BoxSupplierOnce::new(|| PhantomData)
}

fn make_arc_supplier_with_lifetime(_: &i32) -> ArcSupplier<PhantomData<&i32>> {
    ArcSupplier::new(|| PhantomData)
}
