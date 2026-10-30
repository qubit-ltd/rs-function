// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use qubit_function::ArcBiConsumer;
use qubit_function::ArcConsumer;
use qubit_function::ArcMutator;
#[cfg(feature = "stateful")]
use qubit_function::ArcStatefulBiConsumer;
#[cfg(feature = "stateful")]
use qubit_function::ArcStatefulConsumer;
#[cfg(feature = "stateful")]
use qubit_function::ArcStatefulMutator;
use qubit_function::BiConsumer;
#[cfg(feature = "once")]
use qubit_function::BiConsumerOnce;
use qubit_function::BoxBiConsumer;
#[cfg(feature = "once")]
use qubit_function::BoxBiConsumerOnce;
use qubit_function::BoxConsumer;
#[cfg(feature = "once")]
use qubit_function::BoxConsumerOnce;
use qubit_function::BoxMutator;
#[cfg(feature = "once")]
use qubit_function::BoxMutatorOnce;
#[cfg(feature = "stateful")]
use qubit_function::BoxStatefulBiConsumer;
#[cfg(feature = "stateful")]
use qubit_function::BoxStatefulConsumer;
#[cfg(feature = "stateful")]
use qubit_function::BoxStatefulMutator;
use qubit_function::Consumer;
#[cfg(feature = "once")]
use qubit_function::ConsumerOnce;
use qubit_function::Mutator;
#[cfg(feature = "once")]
use qubit_function::MutatorOnce;
#[cfg(feature = "rc")]
use qubit_function::RcBiConsumer;
#[cfg(feature = "rc")]
use qubit_function::RcConsumer;
#[cfg(feature = "rc")]
use qubit_function::RcMutator;
#[cfg(all(feature = "rc", feature = "stateful"))]
use qubit_function::RcStatefulBiConsumer;
#[cfg(all(feature = "rc", feature = "stateful"))]
use qubit_function::RcStatefulConsumer;
#[cfg(all(feature = "rc", feature = "stateful"))]
use qubit_function::RcStatefulMutator;
#[cfg(feature = "stateful")]
use qubit_function::StatefulBiConsumer;
#[cfg(feature = "stateful")]
use qubit_function::StatefulConsumer;
#[cfg(feature = "stateful")]
use qubit_function::StatefulMutator;

struct Borrowed<'a> {
    source: &'a i32,
    updates: usize,
}

#[test]
fn test_box_consumer_when_accepts_borrowed_input() {
    fn check<'a>(positive: &'a i32, negative: &'a i32) {
        let calls = Rc::new(Cell::new(0));
        let observed = Rc::clone(&calls);
        let consumer: BoxConsumer<&'a i32> = BoxConsumer::new(move |_: &&'a i32| observed.set(observed.get() + 1));
        let conditional = consumer.when(|value: &&'a i32| **value > 0);
        conditional.accept(&negative);
        assert_eq!(calls.get(), 0);
        conditional.accept(&positive);
        assert_eq!(calls.get(), 1);
    }
    let positive = 3;
    let negative = -1;
    check(&positive, &negative);
}

#[test]
fn test_arc_consumer_when_accepts_borrowed_input() {
    fn check<'a>(positive: &'a i32, negative: &'a i32) {
        let calls = Arc::new(AtomicUsize::new(0));
        let observed = Arc::clone(&calls);
        let consumer: ArcConsumer<&'a i32> = ArcConsumer::new(move |_: &&'a i32| {
            observed.fetch_add(1, Ordering::SeqCst);
        });
        let conditional = consumer.when(|value: &&'a i32| **value > 0);
        conditional.accept(&negative);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        conditional.accept(&positive);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
    let positive = 3;
    let negative = -1;
    check(&positive, &negative);
}

#[cfg(feature = "rc")]
#[test]
fn test_rc_consumer_when_accepts_borrowed_input() {
    fn check<'a>(positive: &'a i32, negative: &'a i32) {
        let calls = Rc::new(Cell::new(0));
        let observed = Rc::clone(&calls);
        let consumer: RcConsumer<&'a i32> = RcConsumer::new(move |_: &&'a i32| observed.set(observed.get() + 1));
        let conditional = consumer.when(|value: &&'a i32| **value > 0);
        conditional.accept(&negative);
        assert_eq!(calls.get(), 0);
        conditional.accept(&positive);
        assert_eq!(calls.get(), 1);
    }
    let positive = 3;
    let negative = -1;
    check(&positive, &negative);
}

#[cfg(feature = "once")]
#[test]
fn test_box_consumer_once_when_accepts_borrowed_input() {
    fn check<'a>(positive: &'a i32, negative: &'a i32) {
        let calls = Rc::new(Cell::new(0));
        BoxConsumerOnce::<&'a i32>::new({
            let observed = Rc::clone(&calls);
            move |_: &&'a i32| observed.set(observed.get() + 1)
        })
        .when(|value: &&'a i32| **value > 0)
        .accept(&negative);
        assert_eq!(calls.get(), 0);

        BoxConsumerOnce::<&'a i32>::new({
            let observed = Rc::clone(&calls);
            move |_: &&'a i32| observed.set(observed.get() + 1)
        })
        .when(|value: &&'a i32| **value > 0)
        .accept(&positive);
        assert_eq!(calls.get(), 1);
    }
    let positive = 3;
    let negative = -1;
    check(&positive, &negative);
}

#[cfg(feature = "stateful")]
#[test]
fn test_box_stateful_consumer_when_accepts_borrowed_input() {
    fn check<'a>(positive: &'a i32, negative: &'a i32) {
        let calls = Rc::new(Cell::new(0));
        let observed = Rc::clone(&calls);
        let mut state = 0;
        let consumer: BoxStatefulConsumer<&'a i32> = BoxStatefulConsumer::new(move |_: &&'a i32| {
            state += 1;
            observed.set(state);
        });
        let mut conditional = consumer.when(|value: &&'a i32| **value > 0);
        conditional.accept(&negative);
        assert_eq!(calls.get(), 0);
        conditional.accept(&positive);
        assert_eq!(calls.get(), 1);
    }
    let positive = 3;
    let negative = -1;
    check(&positive, &negative);
}

#[cfg(feature = "stateful")]
#[test]
fn test_arc_stateful_consumer_when_accepts_borrowed_input() {
    fn check<'a>(positive: &'a i32, negative: &'a i32) {
        let calls = Arc::new(AtomicUsize::new(0));
        let observed = Arc::clone(&calls);
        let mut state = 0;
        let consumer: ArcStatefulConsumer<&'a i32> = ArcStatefulConsumer::new(move |_: &&'a i32| {
            state += 1;
            observed.store(state, Ordering::SeqCst);
        });
        let mut conditional = consumer.when(|value: &&'a i32| **value > 0);
        conditional.accept(&negative);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        conditional.accept(&positive);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
    let positive = 3;
    let negative = -1;
    check(&positive, &negative);
}

#[cfg(all(feature = "rc", feature = "stateful"))]
#[test]
fn test_rc_stateful_consumer_when_accepts_borrowed_input() {
    fn check<'a>(positive: &'a i32, negative: &'a i32) {
        let calls = Rc::new(Cell::new(0));
        let observed = Rc::clone(&calls);
        let mut state = 0;
        let consumer: RcStatefulConsumer<&'a i32> = RcStatefulConsumer::new(move |_: &&'a i32| {
            state += 1;
            observed.set(state);
        });
        let mut conditional = consumer.when(|value: &&'a i32| **value > 0);
        conditional.accept(&negative);
        assert_eq!(calls.get(), 0);
        conditional.accept(&positive);
        assert_eq!(calls.get(), 1);
    }
    let positive = 3;
    let negative = -1;
    check(&positive, &negative);
}

#[test]
fn test_box_bi_consumer_when_accepts_independently_borrowed_inputs() {
    fn check<'a, 'b>(positive: &'a i32, negative: &'a i32, addend: &'b i32) {
        let calls = Rc::new(Cell::new(0));
        let observed = Rc::clone(&calls);
        let consumer: BoxBiConsumer<&'a i32, &'b i32> =
            BoxBiConsumer::new(move |_: &&'a i32, _: &&'b i32| observed.set(observed.get() + 1));
        let conditional = consumer.when(|left: &&'a i32, right: &&'b i32| **left + **right > 0);
        conditional.accept(&negative, &addend);
        assert_eq!(calls.get(), 0);
        conditional.accept(&positive, &addend);
        assert_eq!(calls.get(), 1);
    }
    let positive = 3;
    let negative = -4;
    let addend = 1;
    check(&positive, &negative, &addend);
}

#[test]
fn test_arc_bi_consumer_when_accepts_independently_borrowed_inputs() {
    fn check<'a, 'b>(positive: &'a i32, negative: &'a i32, addend: &'b i32) {
        let calls = Arc::new(AtomicUsize::new(0));
        let observed = Arc::clone(&calls);
        let consumer: ArcBiConsumer<&'a i32, &'b i32> = ArcBiConsumer::new(move |_: &&'a i32, _: &&'b i32| {
            observed.fetch_add(1, Ordering::SeqCst);
        });
        let conditional = consumer.when(|left: &&'a i32, right: &&'b i32| **left + **right > 0);
        conditional.accept(&negative, &addend);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        conditional.accept(&positive, &addend);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
    let positive = 3;
    let negative = -4;
    let addend = 1;
    check(&positive, &negative, &addend);
}

#[cfg(feature = "rc")]
#[test]
fn test_rc_bi_consumer_when_accepts_independently_borrowed_inputs() {
    fn check<'a, 'b>(positive: &'a i32, negative: &'a i32, addend: &'b i32) {
        let calls = Rc::new(Cell::new(0));
        let observed = Rc::clone(&calls);
        let consumer: RcBiConsumer<&'a i32, &'b i32> =
            RcBiConsumer::new(move |_: &&'a i32, _: &&'b i32| observed.set(observed.get() + 1));
        let conditional = consumer.when(|left: &&'a i32, right: &&'b i32| **left + **right > 0);
        conditional.accept(&negative, &addend);
        assert_eq!(calls.get(), 0);
        conditional.accept(&positive, &addend);
        assert_eq!(calls.get(), 1);
    }
    let positive = 3;
    let negative = -4;
    let addend = 1;
    check(&positive, &negative, &addend);
}

#[cfg(feature = "once")]
#[test]
fn test_box_bi_consumer_once_when_accepts_independently_borrowed_inputs() {
    fn check<'a, 'b>(positive: &'a i32, negative: &'a i32, addend: &'b i32) {
        let calls = Rc::new(Cell::new(0));
        BoxBiConsumerOnce::<&'a i32, &'b i32>::new({
            let observed = Rc::clone(&calls);
            move |_: &&'a i32, _: &&'b i32| observed.set(observed.get() + 1)
        })
        .when(|left: &&'a i32, right: &&'b i32| **left + **right > 0)
        .accept(&negative, &addend);
        assert_eq!(calls.get(), 0);
        BoxBiConsumerOnce::<&'a i32, &'b i32>::new({
            let observed = Rc::clone(&calls);
            move |_: &&'a i32, _: &&'b i32| observed.set(observed.get() + 1)
        })
        .when(|left: &&'a i32, right: &&'b i32| **left + **right > 0)
        .accept(&positive, &addend);
        assert_eq!(calls.get(), 1);
    }
    let positive = 3;
    let negative = -4;
    let addend = 1;
    check(&positive, &negative, &addend);
}

#[cfg(feature = "stateful")]
#[test]
fn test_box_stateful_bi_consumer_when_accepts_independently_borrowed_inputs() {
    fn check<'a, 'b>(positive: &'a i32, negative: &'a i32, addend: &'b i32) {
        let calls = Rc::new(Cell::new(0));
        let observed = Rc::clone(&calls);
        let mut state = 0;
        let consumer: BoxStatefulBiConsumer<&'a i32, &'b i32> =
            BoxStatefulBiConsumer::new(move |_: &&'a i32, _: &&'b i32| {
                state += 1;
                observed.set(state);
            });
        let mut conditional = consumer.when(|left: &&'a i32, right: &&'b i32| **left + **right > 0);
        conditional.accept(&negative, &addend);
        assert_eq!(calls.get(), 0);
        conditional.accept(&positive, &addend);
        assert_eq!(calls.get(), 1);
    }
    let positive = 3;
    let negative = -4;
    let addend = 1;
    check(&positive, &negative, &addend);
}

#[cfg(feature = "stateful")]
#[test]
fn test_arc_stateful_bi_consumer_when_accepts_independently_borrowed_inputs() {
    fn check<'a, 'b>(positive: &'a i32, negative: &'a i32, addend: &'b i32) {
        let calls = Arc::new(AtomicUsize::new(0));
        let observed = Arc::clone(&calls);
        let mut state = 0;
        let consumer: ArcStatefulBiConsumer<&'a i32, &'b i32> =
            ArcStatefulBiConsumer::new(move |_: &&'a i32, _: &&'b i32| {
                state += 1;
                observed.store(state, Ordering::SeqCst);
            });
        let mut conditional = consumer.when(|left: &&'a i32, right: &&'b i32| **left + **right > 0);
        conditional.accept(&negative, &addend);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        conditional.accept(&positive, &addend);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
    let positive = 3;
    let negative = -4;
    let addend = 1;
    check(&positive, &negative, &addend);
}

#[cfg(all(feature = "rc", feature = "stateful"))]
#[test]
fn test_rc_stateful_bi_consumer_when_accepts_independently_borrowed_inputs() {
    fn check<'a, 'b>(positive: &'a i32, negative: &'a i32, addend: &'b i32) {
        let calls = Rc::new(Cell::new(0));
        let observed = Rc::clone(&calls);
        let mut state = 0;
        let consumer: RcStatefulBiConsumer<&'a i32, &'b i32> =
            RcStatefulBiConsumer::new(move |_: &&'a i32, _: &&'b i32| {
                state += 1;
                observed.set(state);
            });
        let mut conditional = consumer.when(|left: &&'a i32, right: &&'b i32| **left + **right > 0);
        conditional.accept(&negative, &addend);
        assert_eq!(calls.get(), 0);
        conditional.accept(&positive, &addend);
        assert_eq!(calls.get(), 1);
    }
    let positive = 3;
    let negative = -4;
    let addend = 1;
    check(&positive, &negative, &addend);
}

#[test]
fn test_box_mutator_when_accepts_borrowed_input() {
    fn check<'a>(positive: &'a i32, negative: &'a i32) {
        let calls = Rc::new(Cell::new(0));
        let observed = Rc::clone(&calls);
        let mutator: BoxMutator<Borrowed<'a>> = BoxMutator::new(move |value: &mut Borrowed<'a>| {
            observed.set(observed.get() + 1);
            value.updates += 1;
        });
        let conditional = mutator.when(|value: &Borrowed<'a>| *value.source > 0);
        let mut rejected = Borrowed {
            source: negative,
            updates: 0,
        };
        conditional.apply(&mut rejected);
        assert_eq!(rejected.updates, 0);
        assert_eq!(calls.get(), 0);
        let mut accepted = Borrowed {
            source: positive,
            updates: 0,
        };
        conditional.apply(&mut accepted);
        assert_eq!(accepted.updates, 1);
        assert_eq!(calls.get(), 1);
    }
    let positive = 3;
    let negative = -1;
    check(&positive, &negative);
}

#[test]
fn test_arc_mutator_when_accepts_borrowed_input() {
    fn check<'a>(positive: &'a i32, negative: &'a i32) {
        let calls = Arc::new(AtomicUsize::new(0));
        let observed = Arc::clone(&calls);
        let mutator: ArcMutator<Borrowed<'a>> = ArcMutator::new(move |value: &mut Borrowed<'a>| {
            observed.fetch_add(1, Ordering::SeqCst);
            value.updates += 1;
        });
        let conditional = mutator.when(|value: &Borrowed<'a>| *value.source > 0);
        let mut rejected = Borrowed {
            source: negative,
            updates: 0,
        };
        conditional.apply(&mut rejected);
        assert_eq!(rejected.updates, 0);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        let mut accepted = Borrowed {
            source: positive,
            updates: 0,
        };
        conditional.apply(&mut accepted);
        assert_eq!(accepted.updates, 1);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
    let positive = 3;
    let negative = -1;
    check(&positive, &negative);
}

#[cfg(feature = "rc")]
#[test]
fn test_rc_mutator_when_accepts_borrowed_input() {
    fn check<'a>(positive: &'a i32, negative: &'a i32) {
        let calls = Rc::new(Cell::new(0));
        let observed = Rc::clone(&calls);
        let mutator: RcMutator<Borrowed<'a>> = RcMutator::new(move |value: &mut Borrowed<'a>| {
            observed.set(observed.get() + 1);
            value.updates += 1;
        });
        let conditional = mutator.when(|value: &Borrowed<'a>| *value.source > 0);
        let mut rejected = Borrowed {
            source: negative,
            updates: 0,
        };
        conditional.apply(&mut rejected);
        assert_eq!(rejected.updates, 0);
        assert_eq!(calls.get(), 0);
        let mut accepted = Borrowed {
            source: positive,
            updates: 0,
        };
        conditional.apply(&mut accepted);
        assert_eq!(accepted.updates, 1);
        assert_eq!(calls.get(), 1);
    }
    let positive = 3;
    let negative = -1;
    check(&positive, &negative);
}

#[cfg(feature = "once")]
#[test]
fn test_box_mutator_once_when_accepts_borrowed_input() {
    fn check<'a>(positive: &'a i32, negative: &'a i32) {
        let calls = Rc::new(Cell::new(0));
        let make = || {
            let observed = Rc::clone(&calls);
            BoxMutatorOnce::<Borrowed<'a>>::new(move |value: &mut Borrowed<'a>| {
                observed.set(observed.get() + 1);
                value.updates += 1;
            })
        };
        let mut rejected = Borrowed {
            source: negative,
            updates: 0,
        };
        make()
            .when(|value: &Borrowed<'a>| *value.source > 0)
            .apply(&mut rejected);
        assert_eq!(rejected.updates, 0);
        assert_eq!(calls.get(), 0);
        let mut accepted = Borrowed {
            source: positive,
            updates: 0,
        };
        make()
            .when(|value: &Borrowed<'a>| *value.source > 0)
            .apply(&mut accepted);
        assert_eq!(accepted.updates, 1);
        assert_eq!(calls.get(), 1);
    }
    let positive = 3;
    let negative = -1;
    check(&positive, &negative);
}

#[cfg(feature = "stateful")]
#[test]
fn test_box_stateful_mutator_when_accepts_borrowed_input() {
    fn check<'a>(positive: &'a i32, negative: &'a i32) {
        let calls = Rc::new(Cell::new(0));
        let observed = Rc::clone(&calls);
        let mut state = 0;
        let mutator: BoxStatefulMutator<Borrowed<'a>> = BoxStatefulMutator::new(move |value: &mut Borrowed<'a>| {
            state += 1;
            observed.set(state);
            value.updates += 1;
        });
        let mut conditional = mutator.when(|value: &Borrowed<'a>| *value.source > 0);
        let mut rejected = Borrowed {
            source: negative,
            updates: 0,
        };
        conditional.apply(&mut rejected);
        assert_eq!(rejected.updates, 0);
        assert_eq!(calls.get(), 0);
        let mut accepted = Borrowed {
            source: positive,
            updates: 0,
        };
        conditional.apply(&mut accepted);
        assert_eq!(accepted.updates, 1);
        assert_eq!(calls.get(), 1);
    }
    let positive = 3;
    let negative = -1;
    check(&positive, &negative);
}

#[cfg(feature = "stateful")]
#[test]
fn test_arc_stateful_mutator_when_accepts_borrowed_input() {
    fn check<'a>(positive: &'a i32, negative: &'a i32) {
        let calls = Arc::new(AtomicUsize::new(0));
        let observed = Arc::clone(&calls);
        let mut state = 0;
        let mutator: ArcStatefulMutator<Borrowed<'a>> = ArcStatefulMutator::new(move |value: &mut Borrowed<'a>| {
            state += 1;
            observed.store(state, Ordering::SeqCst);
            value.updates += 1;
        });
        let mut conditional = mutator.when(|value: &Borrowed<'a>| *value.source > 0);
        let mut rejected = Borrowed {
            source: negative,
            updates: 0,
        };
        conditional.apply(&mut rejected);
        assert_eq!(rejected.updates, 0);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        let mut accepted = Borrowed {
            source: positive,
            updates: 0,
        };
        conditional.apply(&mut accepted);
        assert_eq!(accepted.updates, 1);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
    let positive = 3;
    let negative = -1;
    check(&positive, &negative);
}

#[cfg(all(feature = "rc", feature = "stateful"))]
#[test]
fn test_rc_stateful_mutator_when_accepts_borrowed_input() {
    fn check<'a>(positive: &'a i32, negative: &'a i32) {
        let calls = Rc::new(Cell::new(0));
        let observed = Rc::clone(&calls);
        let mut state = 0;
        let mutator: RcStatefulMutator<Borrowed<'a>> = RcStatefulMutator::new(move |value: &mut Borrowed<'a>| {
            state += 1;
            observed.set(state);
            value.updates += 1;
        });
        let mut conditional = mutator.when(|value: &Borrowed<'a>| *value.source > 0);
        let mut rejected = Borrowed {
            source: negative,
            updates: 0,
        };
        conditional.apply(&mut rejected);
        assert_eq!(rejected.updates, 0);
        assert_eq!(calls.get(), 0);
        let mut accepted = Borrowed {
            source: positive,
            updates: 0,
        };
        conditional.apply(&mut accepted);
        assert_eq!(accepted.updates, 1);
        assert_eq!(calls.get(), 1);
    }
    let positive = 3;
    let negative = -1;
    check(&positive, &negative);
}
