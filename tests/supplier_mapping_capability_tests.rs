// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_function::BoxSupplier;
use qubit_function::Supplier;
use qubit_function::Transformer;

struct CustomTransformer;

impl Transformer<i32, i32> for CustomTransformer {
    fn apply(&self, input: i32) -> i32 {
        input * 2
    }
}

#[test]
fn test_existing_map_still_accepts_custom_transformer_only() {
    let mapped = BoxSupplier::new(|| 21).map(CustomTransformer);
    assert_eq!(mapped.get(), 42);
}

#[cfg(feature = "once")]
mod once {
    use qubit_function::BoxSupplierOnce;
    use qubit_function::SupplierOnce;
    use qubit_function::TransformerOnce;

    struct Payload(&'static str);

    struct MovePayload(Payload);

    impl TransformerOnce<i32, (i32, Payload)> for MovePayload {
        fn apply(self, value: i32) -> (i32, Payload) {
            (value, self.0)
        }
    }

    #[test]
    fn test_map_once_moves_non_clone_capture_and_preserves_name() {
        let source = BoxSupplierOnce::new_with_name("source", || 7);
        let mapped = source.map_once(MovePayload(Payload("owned")));
        assert_eq!(mapped.name(), Some("source"));
        let (value, payload) = mapped.get();
        assert_eq!(value, 7);
        assert_eq!(payload.0, "owned");
    }
}

#[cfg(feature = "stateful")]
mod stateful {
    use std::cell::Cell;
    use std::rc::Rc;
    use std::sync::Arc;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;
    use std::thread;

    use qubit_function::ArcStatefulSupplier;
    use qubit_function::BoxStatefulSupplier;
    use qubit_function::StatefulSupplier;
    use qubit_function::StatefulTransformer;

    struct Counter(u32);

    impl StatefulTransformer<i32, i32> for Counter {
        fn apply(&mut self, value: i32) -> i32 {
            self.0 += 1;
            value + self.0 as i32
        }
    }

    #[test]
    fn test_box_map_stateful_retains_closure_and_trait_mapper_state() {
        let mut count = 0;
        let mut mapped = BoxStatefulSupplier::new(|| 10).map_stateful(move |value| {
            count += 1;
            value + count
        });
        assert_eq!(mapped.get(), 11);
        assert_eq!(mapped.get(), 12);

        let mut mapped = BoxStatefulSupplier::new(|| 10).map_stateful(Counter(0));
        assert_eq!(mapped.get(), 11);
        assert_eq!(mapped.get(), 12);
    }

    #[test]
    fn test_arc_map_stateful_shares_mapper_state_across_threads() {
        let source = ArcStatefulSupplier::new(|| 10);
        let mut mapped = source.map_stateful({
            let count = Cell::new(0);
            move |value| {
                count.set(count.get() + 1);
                value + count.get()
            }
        });
        let mut cloned = mapped.clone();

        let left = thread::spawn(move || mapped.get());
        let right = thread::spawn(move || cloned.get());
        let mut values = [
            left.join().expect("left mapper call should succeed"),
            right.join().expect("right mapper call should succeed"),
        ];
        values.sort_unstable();
        assert_eq!(values, [11, 12]);
    }

    #[test]
    fn test_arc_map_stateful_accepts_non_send_output() {
        let source = ArcStatefulSupplier::<Rc<i32>>::new(|| Rc::new(9));
        let mut mapped = source.map_stateful(|value: Rc<i32>| -> Rc<i32> { Rc::new(*value + 1) });
        assert_eq!(*mapped.get(), 10);
    }

    #[test]
    fn test_stateful_mapping_preserves_metadata_and_separate_mapper_state() {
        let calls = Arc::new(AtomicUsize::new(0));
        let source_calls = Arc::clone(&calls);
        let source =
            ArcStatefulSupplier::new_with_name("source", move || source_calls.fetch_add(1, Ordering::SeqCst) + 1);
        let first = source.map_stateful({
            let mut count = 0;
            move |value| {
                count += 1;
                value + count
            }
        });
        let second = source.map_stateful({
            let mut count = 0;
            move |value| {
                count += 1;
                value + count
            }
        });

        assert_eq!(first.name(), Some("source"));
        let mut first = first;
        let mut second = second;
        let mut source_clone = source.clone();
        assert_eq!(first.get(), 2);
        assert_eq!(second.get(), 3);
        assert_eq!(source_clone.get(), 3);
    }
}

#[cfg(all(feature = "rc", feature = "stateful"))]
mod rc {
    use std::cell::Cell;
    use std::rc::Rc;

    use qubit_function::RcStatefulSupplier;
    use qubit_function::StatefulSupplier;

    #[test]
    fn test_rc_mapping_shares_source_and_mapper_state_across_clones() {
        let calls = Rc::new(Cell::new(0));
        let source_calls = Rc::clone(&calls);
        let source = RcStatefulSupplier::new(move || {
            source_calls.set(source_calls.get() + 1);
            source_calls.get()
        });
        let mut mapped = source.map_stateful({
            let mut count = 0;
            move |value| {
                count += 10;
                value + count
            }
        });
        let mut cloned = mapped.clone();
        let mut source_clone = source.clone();

        assert_eq!(mapped.get(), 11);
        assert_eq!(cloned.get(), 22);
        assert_eq!(source_clone.get(), 3);
    }

    #[test]
    fn test_rc_map_stateful_accepts_fn_mut_mapper() {
        let source = RcStatefulSupplier::new(|| 10);
        let mut mapped = source.map_stateful({
            let mut count = 0;
            move |value| {
                count += 1;
                value + count
            }
        });
        assert_eq!(mapped.get(), 11);
        assert_eq!(mapped.get(), 12);
    }
}
