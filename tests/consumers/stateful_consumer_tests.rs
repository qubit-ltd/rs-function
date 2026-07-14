// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Unit tests for StatefulConsumer types

use qubit_function::{
    ArcConsumer,
    ArcStatefulConsumer,
    BoxConsumer,
    BoxStatefulConsumer,
    Consumer,
    RcConsumer,
    RcStatefulConsumer,
    StatefulConsumer,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{
    Arc,
    Mutex,
};

// ============================================================================
// BoxConsumer Tests
// ============================================================================

#[cfg(test)]
mod test_box_consumer {
    use super::{
        Arc,
        BoxConsumer,
        BoxStatefulConsumer,
        Consumer,
        Mutex,
        StatefulConsumer,
    };

    #[test]
    fn test_new() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let mut consumer = BoxStatefulConsumer::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });
        let value = 5;
        consumer.accept(&value);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);
    }

    #[test]
    fn test_with_different_types() {
        // String
        let log = Arc::new(Mutex::new(String::new()));
        let l = log.clone();
        let mut consumer = BoxStatefulConsumer::new(move |s: &String| {
            *l.lock().expect("mutex should not be poisoned") =
                format!("Got: {}", s);
        });
        let text = String::from("hello");
        consumer.accept(&text);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            "Got: hello"
        );

        // Vec
        let log = Arc::new(Mutex::new(0));
        let l = log.clone();
        let mut consumer = BoxStatefulConsumer::new(move |v: &Vec<i32>| {
            *l.lock().expect("mutex should not be poisoned") = v.len();
        });
        let numbers = vec![1, 2, 3];
        consumer.accept(&numbers);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), 3);

        // bool
        let log = Arc::new(Mutex::new(String::new()));
        let l = log.clone();
        let mut consumer = BoxStatefulConsumer::new(move |b: &bool| {
            *l.lock().expect("mutex should not be poisoned") =
                if *b { "true" } else { "false" }.to_string();
        });
        let flag = true;
        consumer.accept(&flag);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), "true");
    }

    #[test]
    fn test_and_then() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();
        let mut consumer = BoxStatefulConsumer::new(move |x: &i32| {
            l1.lock()
                .expect("mutex should not be poisoned")
                .push(*x * 2);
        })
        .and_then(move |x: &i32| {
            l2.lock()
                .expect("mutex should not be poisoned")
                .push(*x + 10);
        });

        let value = 5;
        consumer.accept(&value);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![10, 15]
        ); // 5*2=10, 5+10=15
    }

    #[test]
    fn test_and_then_multiple_chains() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();
        let l3 = log.clone();
        let mut consumer = BoxStatefulConsumer::new(move |x: &i32| {
            l1.lock()
                .expect("mutex should not be poisoned")
                .push(*x + 1);
        })
        .and_then(move |x: &i32| {
            l2.lock()
                .expect("mutex should not be poisoned")
                .push(*x * 2);
        })
        .and_then(move |x: &i32| {
            l3.lock()
                .expect("mutex should not be poisoned")
                .push(*x - 5);
        });

        let value = 10;
        consumer.accept(&value);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![11, 20, 5]
        ); // 10+1=11, 10*2=20, 10-5=5
    }

    #[test]
    fn test_and_then_with_box_consumer() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();
        let c1 = BoxStatefulConsumer::new(move |x: &i32| {
            l1.lock()
                .expect("mutex should not be poisoned")
                .push(*x * 2);
        });
        let c2 = BoxStatefulConsumer::new(move |x: &i32| {
            l2.lock()
                .expect("mutex should not be poisoned")
                .push(*x + 10);
        });
        let mut combined = c1.and_then(c2);

        let value = 5;
        combined.accept(&value);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![10, 15]
        );
    }

    #[test]
    fn test_noop() {
        let noop = BoxConsumer::<i32>::noop();
        let value = 42;
        noop.accept(&value);
        // No assertion needed, just ensure it doesn't panic
    }

    #[test]
    fn test_new_with_name() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let mut consumer = BoxStatefulConsumer::new_with_name(
            "test_consumer",
            move |x: &i32| {
                l.lock().expect("mutex should not be poisoned").push(*x);
            },
        );
        assert_eq!(consumer.name(), Some("test_consumer"));
        consumer.accept(&5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);
    }

    // print and print_with methods have been removed

    #[test]
    fn test_if_then() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let consumer = BoxStatefulConsumer::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });
        let mut conditional = consumer.when(|x: &i32| *x > 0);

        let positive = 5;
        conditional.accept(&positive);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);

        let negative = -5;
        conditional.accept(&negative);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]); // Unchanged
    }

    #[test]
    fn test_if_then_else() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();
        let consumer = BoxStatefulConsumer::new(move |x: &i32| {
            l1.lock().expect("mutex should not be poisoned").push(*x);
        });
        let mut conditional =
            consumer.when(|x: &i32| *x > 0).or_else(move |x: &i32| {
                l2.lock().expect("mutex should not be poisoned").push(-*x);
            });

        let positive = 5;
        conditional.accept(&positive);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);

        let negative = -5;
        conditional.accept(&negative);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![5, 5]
        ); // -(-5) = 5
    }

    #[test]
    fn test_debug() {
        let consumer = BoxStatefulConsumer::new(|_x: &i32| {});
        let debug_str = format!("{:?}", consumer);
        assert!(debug_str.contains("BoxStatefulConsumer"));
    }

    #[test]
    fn test_debug_with_name() {
        let mut consumer = BoxStatefulConsumer::new(|_x: &i32| {});
        consumer.set_name("test_consumer");
        let debug_str = format!("{:?}", consumer);
        assert!(debug_str.contains("BoxStatefulConsumer"));
        assert!(debug_str.contains("test_consumer"));
    }

    #[test]
    fn test_display() {
        let consumer = BoxStatefulConsumer::new(|_x: &i32| {});
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "BoxStatefulConsumer");
    }

    #[test]
    fn test_display_with_name() {
        let mut consumer = BoxStatefulConsumer::new(|_x: &i32| {});
        consumer.set_name("my_consumer");
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "BoxStatefulConsumer(my_consumer)");
    }
}

// ============================================================================
// ArcConsumer Tests
// ============================================================================

#[cfg(test)]
mod test_arc_consumer {
    use std::panic::{
        AssertUnwindSafe,
        catch_unwind,
    };

    use super::{
        Arc,
        ArcConsumer,
        ArcStatefulConsumer,
        Consumer,
        Mutex,
        Rc,
        RefCell,
        StatefulConsumer,
    };

    #[test]
    fn test_new() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let mut consumer = ArcStatefulConsumer::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });
        let value = 5;
        consumer.accept(&value);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);
    }

    #[test]
    fn test_clone() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let mut consumer = ArcStatefulConsumer::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });
        let mut clone = consumer.clone();

        consumer.accept(&5);
        clone.accept(&10);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![5, 10]
        );
    }

    /// Verifies that callback mutations survive a panic and that the shared
    /// consumer remains usable after unwinding.
    #[test]
    fn test_accept_after_callback_panic() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let callback_log = log.clone();
        let mut consumer = ArcStatefulConsumer::new(move |value: &i32| {
            callback_log
                .lock()
                .expect("mutex should not be poisoned")
                .push(*value);
            assert_ne!(*value, 1, "first callback should panic");
        });

        let result = catch_unwind(AssertUnwindSafe(|| consumer.accept(&1)));
        assert!(result.is_err(), "the first callback should panic");
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![1],
            "mutations completed before the panic should be preserved"
        );

        consumer.accept(&2);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![1, 2],
            "the consumer should remain usable after unwinding"
        );
    }

    #[test]
    fn test_and_then() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();
        let first = ArcStatefulConsumer::new(move |x: &i32| {
            l1.lock()
                .expect("mutex should not be poisoned")
                .push(*x * 2);
        });
        let second = ArcStatefulConsumer::new(move |x: &i32| {
            l2.lock()
                .expect("mutex should not be poisoned")
                .push(*x + 10);
        });
        let mut chained = first.and_then(second);

        let value = 5;
        chained.accept(&value);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![10, 15]
        );
    }

    #[test]
    fn test_thread_safety() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let consumer = ArcStatefulConsumer::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });

        let mut c1 = consumer.clone();
        let mut c2 = consumer.clone();

        let h1 = std::thread::spawn(move || {
            c1.accept(&1);
        });

        let h2 = std::thread::spawn(move || {
            c2.accept(&2);
        });

        h1.join().expect("thread should not panic");
        h2.join().expect("thread should not panic");

        let mut result =
            log.lock().expect("mutex should not be poisoned").clone();
        result.sort();
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_noop() {
        let noop = ArcConsumer::<i32>::noop();
        noop.accept(&42);
        // No assertion needed, just ensure it doesn't panic
    }

    #[test]
    fn test_new_with_name() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let mut consumer = ArcStatefulConsumer::new_with_name(
            "test_consumer",
            move |x: &i32| {
                l.lock().expect("mutex should not be poisoned").push(*x);
            },
        );
        assert_eq!(consumer.name(), Some("test_consumer"));
        consumer.accept(&5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);
    }

    #[test]
    fn test_noop_stateful() {
        let mut noop = ArcStatefulConsumer::<i32>::noop();
        noop.accept(&42);
        // No assertion needed, just ensure it doesn't panic
    }

    #[test]
    fn test_debug() {
        let consumer = ArcStatefulConsumer::new(|_x: &i32| {});
        let debug_str = format!("{:?}", consumer);
        assert!(debug_str.contains("ArcStatefulConsumer"));
    }

    #[test]
    fn test_debug_with_name() {
        let mut consumer = ArcStatefulConsumer::new(|_x: &i32| {});
        consumer.set_name("test_consumer");
        let debug_str = format!("{:?}", consumer);
        assert!(debug_str.contains("ArcStatefulConsumer"));
        assert!(debug_str.contains("test_consumer"));
    }

    #[test]
    fn test_display() {
        let consumer = ArcStatefulConsumer::new(|_x: &i32| {});
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "ArcStatefulConsumer");
    }

    #[test]
    fn test_display_with_name() {
        let mut consumer = ArcStatefulConsumer::new(|_x: &i32| {});
        consumer.set_name("my_consumer");
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "ArcStatefulConsumer(my_consumer)");
    }

    // ============================================================================
    // ArcConsumer ConsumerOnce Tests
    // ============================================================================

    #[test]
    fn test_accept_once() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let mut consumer = ArcStatefulConsumer::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });
        consumer.accept(&5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);
    }

    #[test]
    fn test_accept_once_with_different_types() {
        // String
        let log = Arc::new(Mutex::new(String::new()));
        let l = log.clone();
        let mut consumer = ArcStatefulConsumer::new(move |s: &String| {
            *l.lock().expect("mutex should not be poisoned") =
                format!("Got: {}", s);
        });
        let text = String::from("hello");
        consumer.accept(&text);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            "Got: hello"
        );

        // Vec
        let log = Arc::new(Mutex::new(0));
        let l = log.clone();
        let mut consumer = ArcStatefulConsumer::new(move |v: &Vec<i32>| {
            *l.lock().expect("mutex should not be poisoned") = v.len();
        });
        let numbers = vec![1, 2, 3];
        consumer.accept(&numbers);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), 3);

        // bool
        let log = Arc::new(Mutex::new(String::new()));
        let l = log.clone();
        let mut consumer = ArcStatefulConsumer::new(move |b: &bool| {
            *l.lock().expect("mutex should not be poisoned") =
                if *b { "true" } else { "false" }.to_string();
        });
        let flag = true;
        consumer.accept(&flag);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), "true");
    }

    #[test]
    fn test_consumer_once_with_state_modification() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let mut counter = 0;
        let mut consumer = ArcStatefulConsumer::new(move |x: &i32| {
            counter += 1;
            l.lock()
                .expect("mutex should not be poisoned")
                .push(*x + counter);
        });
        consumer.accept(&10);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![11]
        ); // 10 + 1
    }

    #[test]
    fn test_consumer_once_consumes_self() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let mut consumer = ArcStatefulConsumer::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });

        consumer.accept(&5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);
    }

    /// Test that ArcConsumer can work with non-Send + non-Sync types
    ///
    /// This test verifies that the relaxed generic constraints (T: 'static
    /// instead of T: Send + Sync + 'static) allow ArcConsumer to be created
    /// for types that are not thread-safe, as long as we only pass
    /// references to them.
    #[test]
    fn test_with_non_send_sync_type() {
        // Rc<RefCell<i32>> is neither Send nor Sync
        type NonSendType = Rc<RefCell<i32>>;

        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();

        // This should compile now with relaxed constraints
        let consumer =
            ArcConsumer::<NonSendType>::new(move |value: &NonSendType| {
                let val = *value.borrow();
                l.lock().expect("mutex should not be poisoned").push(val);
            });

        let value = Rc::new(RefCell::new(42));
        consumer.accept(&value);

        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![42]
        );
    }

    /// Test that ArcConsumer with non-Send type can be cloned and used
    #[test]
    fn test_clone_with_non_send_sync_type() {
        type NonSendType = Rc<RefCell<String>>;

        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();

        let consumer =
            ArcConsumer::<NonSendType>::new(move |value: &NonSendType| {
                let val = value.borrow().clone();
                l.lock().expect("mutex should not be poisoned").push(val);
            });

        let consumer2 = consumer.clone();

        let value1 = Rc::new(RefCell::new("hello".to_string()));
        let value2 = Rc::new(RefCell::new("world".to_string()));

        consumer.accept(&value1);
        consumer2.accept(&value2);

        let result = log.lock().expect("mutex should not be poisoned").clone();
        assert_eq!(result, vec!["hello".to_string(), "world".to_string()]);
    }

    /// Test that ArcConsumer with non-Send type can be chained
    #[test]
    fn test_and_then_with_non_send_sync_type() {
        type NonSendType = Rc<RefCell<i32>>;

        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();

        let first =
            ArcConsumer::<NonSendType>::new(move |value: &NonSendType| {
                let val = *value.borrow();
                l1.lock()
                    .expect("mutex should not be poisoned")
                    .push(val * 2);
            });

        let second =
            ArcConsumer::<NonSendType>::new(move |value: &NonSendType| {
                let val = *value.borrow();
                l2.lock()
                    .expect("mutex should not be poisoned")
                    .push(val + 10);
            });

        let chained = first.and_then(second);

        let value = Rc::new(RefCell::new(5));
        chained.accept(&value);

        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![10, 15]
        ); // 5*2=10, 5+10=15
    }
}

// ============================================================================
// RcConsumer Tests
// ============================================================================

#[cfg(test)]
mod test_rc_consumer {
    use std::panic::{
        AssertUnwindSafe,
        catch_unwind,
    };

    use super::{
        Consumer,
        Rc,
        RcConsumer,
        RcStatefulConsumer,
        RefCell,
        StatefulConsumer,
    };

    #[test]
    fn test_new() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l = log.clone();
        let mut consumer = RcStatefulConsumer::new(move |x: &i32| {
            l.borrow_mut().push(*x);
        });
        let value = 5;
        consumer.accept(&value);
        assert_eq!(*log.borrow(), vec![5]);
    }

    #[test]
    fn test_clone() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l = log.clone();
        let mut consumer = RcStatefulConsumer::new(move |x: &i32| {
            l.borrow_mut().push(*x);
        });
        let mut clone = consumer.clone();

        consumer.accept(&5);
        clone.accept(&10);
        assert_eq!(*log.borrow(), vec![5, 10]);
    }

    /// Verifies that synchronous re-entry panics after preserving prior
    /// mutations, and that the shared consumer remains usable after unwinding.
    #[test]
    fn test_accept_reentrant_call_panics_and_recovers() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let callback_log = log.clone();
        let shared_consumer =
            Rc::new(RefCell::new(None::<RcStatefulConsumer<i32>>));
        let callback_consumer = shared_consumer.clone();
        let mut consumer = RcStatefulConsumer::new(move |value: &i32| {
            callback_log.borrow_mut().push(*value);
            if *value == 1 {
                let mut reentrant = callback_consumer
                    .borrow()
                    .as_ref()
                    .expect("shared consumer should be initialized")
                    .clone();
                reentrant.accept(&2);
            }
        });
        *shared_consumer.borrow_mut() = Some(consumer.clone());

        let result = catch_unwind(AssertUnwindSafe(|| consumer.accept(&1)));
        assert!(result.is_err(), "synchronous re-entry should panic");
        assert_eq!(
            *log.borrow(),
            vec![1],
            "mutations completed before the panic should be preserved"
        );

        consumer.accept(&3);
        assert_eq!(
            *log.borrow(),
            vec![1, 3],
            "the consumer should remain usable after unwinding"
        );

        assert!(
            shared_consumer.borrow_mut().take().is_some(),
            "shared consumer should be present during cleanup"
        );
    }

    #[test]
    fn test_and_then() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();
        let first = RcStatefulConsumer::new(move |x: &i32| {
            l1.borrow_mut().push(*x * 2);
        });
        let second = RcStatefulConsumer::new(move |x: &i32| {
            l2.borrow_mut().push(*x + 10);
        });
        let mut chained = first.and_then(second);

        let value = 5;
        chained.accept(&value);
        assert_eq!(*log.borrow(), vec![10, 15]);
    }

    #[test]
    fn test_noop() {
        let noop = RcConsumer::<i32>::noop();
        noop.accept(&42);
        // No assertion needed, just ensure it doesn't panic
    }

    #[test]
    fn test_new_with_name() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l = log.clone();
        let mut consumer = RcStatefulConsumer::new_with_name(
            "test_consumer",
            move |x: &i32| {
                l.borrow_mut().push(*x);
            },
        );
        assert_eq!(consumer.name(), Some("test_consumer"));
        consumer.accept(&5);
        assert_eq!(*log.borrow(), vec![5]);
    }

    #[test]
    fn test_noop_stateful() {
        let mut noop = RcStatefulConsumer::<i32>::noop();
        noop.accept(&42);
        // No assertion needed, just ensure it doesn't panic
    }

    #[test]
    fn test_debug() {
        let consumer = RcStatefulConsumer::new(|_x: &i32| {});
        let debug_str = format!("{:?}", consumer);
        assert!(debug_str.contains("RcStatefulConsumer"));
    }

    #[test]
    fn test_debug_with_name() {
        let mut consumer = RcStatefulConsumer::new(|_x: &i32| {});
        consumer.set_name("test_consumer");
        let debug_str = format!("{:?}", consumer);
        assert!(debug_str.contains("RcStatefulConsumer"));
        assert!(debug_str.contains("test_consumer"));
    }

    #[test]
    fn test_display() {
        let consumer = RcStatefulConsumer::new(|_x: &i32| {});
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "RcStatefulConsumer");
    }

    #[test]
    fn test_display_with_name() {
        let mut consumer = RcStatefulConsumer::new(|_x: &i32| {});
        consumer.set_name("my_consumer");
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "RcStatefulConsumer(my_consumer)");
    }
}

// ============================================================================
// Conversion Tests
// ============================================================================

#[cfg(test)]
mod test_conversions {
    use super::{
        Rc,
        RcStatefulConsumer,
        RefCell,
        StatefulConsumer,
    };

    // RcConsumer cannot be converted to ArcConsumer because Rc is not Send

    // ============================================================================
    // RcConsumer ConsumerOnce Tests
    // ============================================================================

    #[test]
    fn test_accept_once() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l = log.clone();
        let mut consumer = RcStatefulConsumer::new(move |x: &i32| {
            l.borrow_mut().push(*x);
        });
        consumer.accept(&5);
        assert_eq!(*log.borrow(), vec![5]);
    }

    #[test]
    fn test_accept_once_with_different_types() {
        // String
        let log = Rc::new(RefCell::new(String::new()));
        let l = log.clone();
        let mut consumer = RcStatefulConsumer::new(move |s: &String| {
            *l.borrow_mut() = format!("Got: {}", s);
        });
        let text = String::from("hello");
        consumer.accept(&text);
        assert_eq!(*log.borrow(), "Got: hello");

        // Vec
        let log = Rc::new(RefCell::new(0));
        let l = log.clone();
        let mut consumer = RcStatefulConsumer::new(move |v: &Vec<i32>| {
            *l.borrow_mut() = v.len();
        });
        let numbers = vec![1, 2, 3];
        consumer.accept(&numbers);
        assert_eq!(*log.borrow(), 3);

        // bool
        let log = Rc::new(RefCell::new(String::new()));
        let l = log.clone();
        let mut consumer = RcStatefulConsumer::new(move |b: &bool| {
            *l.borrow_mut() = if *b { "true" } else { "false" }.to_string();
        });
        let flag = true;
        consumer.accept(&flag);
        assert_eq!(*log.borrow(), "true");
    }

    #[test]
    fn test_consumer_once_with_state_modification() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l = log.clone();
        let mut counter = 0;
        let mut consumer = RcStatefulConsumer::new(move |x: &i32| {
            counter += 1;
            l.borrow_mut().push(*x + counter);
        });
        consumer.accept(&10);
        assert_eq!(*log.borrow(), vec![11]); // 10 + 1
    }

    #[test]
    fn test_consumer_once_consumes_self() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l = log.clone();
        let mut consumer = RcStatefulConsumer::new(move |x: &i32| {
            l.borrow_mut().push(*x);
        });

        consumer.accept(&5);
        assert_eq!(*log.borrow(), vec![5]);
    }
}

// ============================================================================
// Unified Interface Tests
// ============================================================================

#[cfg(test)]
mod test_unified_interface {
    use super::{
        Arc,
        ArcConsumer,
        BoxConsumer,
        Consumer,
        Mutex,
        Rc,
        RcConsumer,
        RefCell,
    };

    fn apply_consumer<C: Consumer<i32>>(consumer: &mut C, value: &i32) -> i32 {
        consumer.accept(value);
        *value // Return original value since Consumer doesn't modify input
    }

    #[test]
    fn test_with_box_consumer() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let mut consumer = BoxConsumer::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x * 2);
        });
        let result = apply_consumer(&mut consumer, &5);
        assert_eq!(result, 5);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![10]
        );
    }

    #[test]
    fn test_with_arc_consumer() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let mut consumer = ArcConsumer::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x * 2);
        });
        let result = apply_consumer(&mut consumer, &5);
        assert_eq!(result, 5);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![10]
        );
    }

    #[test]
    fn test_with_rc_consumer() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l = log.clone();
        let mut consumer = RcConsumer::new(move |x: &i32| {
            l.borrow_mut().push(*x * 2);
        });
        let result = apply_consumer(&mut consumer, &5);
        assert_eq!(result, 5);
        assert_eq!(*log.borrow(), vec![10]);
    }
}

// ============================================================================
// BoxConsumer chaining test
// ============================================================================

#[cfg(test)]
mod test_box_consumer_chaining {
    use super::{
        Arc,
        Consumer,
        Mutex,
        StatefulConsumer,
    };

    #[test]
    fn test_and_then_with_closure() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();
        let chained = qubit_function::BoxConsumer::new(move |x: &i32| {
            l1.lock()
                .expect("mutex should not be poisoned")
                .push(*x * 2);
        })
        .and_then(move |x: &i32| {
            l2.lock()
                .expect("mutex should not be poisoned")
                .push(*x + 10);
        });

        let value = 5;
        chained.accept(&value);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![10, 15]
        );
    }
}

// ============================================================================
// Name Tests
// ============================================================================

#[cfg(test)]
mod test_consumer_names {
    use super::{
        Arc,
        ArcConsumer,
        ArcStatefulConsumer,
        BoxConsumer,
        BoxStatefulConsumer,
        Consumer,
        Mutex,
        Rc,
        RcConsumer,
        RcStatefulConsumer,
        RefCell,
    };

    #[test]
    fn test_box_consumer_with_name() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let mut consumer = BoxConsumer::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });
        consumer.set_name("logger");
        assert_eq!(consumer.name(), Some("logger"));
        consumer.accept(&5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);
    }

    #[test]
    fn test_box_consumer_set_name() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let mut consumer = BoxStatefulConsumer::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });
        assert_eq!(consumer.name(), None);
        consumer.set_name("my_consumer");
        assert_eq!(consumer.name(), Some("my_consumer"));
    }

    #[test]
    fn test_arc_consumer_with_name() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let mut consumer = ArcConsumer::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });
        consumer.set_name("logger");
        assert_eq!(consumer.name(), Some("logger"));
        consumer.accept(&5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);
    }

    #[test]
    fn test_arc_consumer_set_name() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let mut consumer = ArcStatefulConsumer::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });
        assert_eq!(consumer.name(), None);
        consumer.set_name("my_consumer");
        assert_eq!(consumer.name(), Some("my_consumer"));
    }

    #[test]
    fn test_rc_consumer_with_name() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l = log.clone();
        let mut consumer = RcConsumer::new(move |x: &i32| {
            l.borrow_mut().push(*x);
        });
        consumer.set_name("logger");
        assert_eq!(consumer.name(), Some("logger"));
        consumer.accept(&5);
        assert_eq!(*log.borrow(), vec![5]);
    }

    #[test]
    fn test_rc_consumer_set_name() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l = log.clone();
        let mut consumer = RcStatefulConsumer::new(move |x: &i32| {
            l.borrow_mut().push(*x);
        });
        assert_eq!(consumer.name(), None);
        consumer.set_name("my_consumer");
        assert_eq!(consumer.name(), Some("my_consumer"));
    }
}

// ============================================================================
// Edge Cases Tests
// ============================================================================

#[cfg(test)]
mod test_edge_cases {
    use super::{
        Arc,
        ArcStatefulConsumer,
        BoxConsumer,
        BoxStatefulConsumer,
        Consumer,
        Mutex,
        Rc,
        RcStatefulConsumer,
        RefCell,
        StatefulConsumer,
    };

    #[test]
    fn test_noop_with_name() {
        let mut consumer = BoxConsumer::<i32>::noop();
        consumer.set_name("noop_consumer");
        assert_eq!(consumer.name(), Some("noop_consumer"));
        consumer.accept(&5); // Should do nothing
    }

    // print and print_with methods have been removed

    #[test]
    fn test_if_then_with_always_true() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let consumer = BoxStatefulConsumer::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });
        let mut conditional = consumer.when(|_: &i32| true);
        conditional.accept(&5);
        conditional.accept(&10);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![5, 10]
        );
    }

    #[test]
    fn test_if_then_with_always_false() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let consumer = BoxStatefulConsumer::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });
        let mut conditional = consumer.when(|_: &i32| false);
        conditional.accept(&5);
        conditional.accept(&10);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn test_if_then_else_all_true() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();
        let consumer = BoxStatefulConsumer::new(move |x: &i32| {
            l1.lock().expect("mutex should not be poisoned").push(*x);
        });
        let mut conditional =
            consumer.when(|_: &i32| true).or_else(move |x: &i32| {
                l2.lock()
                    .expect("mutex should not be poisoned")
                    .push(*x * 100);
            });
        conditional.accept(&5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);
    }

    #[test]
    fn test_if_then_else_all_false() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();
        let consumer = BoxStatefulConsumer::new(move |x: &i32| {
            l1.lock().expect("mutex should not be poisoned").push(*x);
        });
        let mut conditional =
            consumer.when(|_: &i32| false).or_else(move |x: &i32| {
                l2.lock()
                    .expect("mutex should not be poisoned")
                    .push(*x * 100);
            });
        conditional.accept(&5);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![500]
        );
    }

    #[test]
    fn test_and_then_with_noop() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let mut consumer = BoxStatefulConsumer::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        })
        .and_then(BoxStatefulConsumer::noop());
        consumer.accept(&5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);
    }

    #[test]
    fn test_complex_chain() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();
        let l3 = log.clone();
        let l4 = log.clone();
        let mut consumer = BoxStatefulConsumer::new(move |x: &i32| {
            l1.lock().expect("mutex should not be poisoned").push(*x);
        })
        .and_then(move |x: &i32| {
            l2.lock()
                .expect("mutex should not be poisoned")
                .push(*x * 2);
        })
        .and_then(BoxStatefulConsumer::noop())
        .and_then(move |x: &i32| {
            l3.lock()
                .expect("mutex should not be poisoned")
                .push(*x + 10);
        })
        .and_then(move |x: &i32| {
            l4.lock()
                .expect("mutex should not be poisoned")
                .push(*x - 5);
        });
        consumer.accept(&5);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![5, 10, 15, 0]
        );
    }

    #[test]
    fn test_box_conditional_and_then() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();
        let consumer = BoxStatefulConsumer::new(move |x: &i32| {
            l1.lock().expect("mutex should not be poisoned").push(*x);
        });
        let conditional = consumer.when(|x: &i32| *x > 0);
        let mut chained = conditional.and_then(move |x: &i32| {
            l2.lock()
                .expect("mutex should not be poisoned")
                .push(*x * 2);
        });
        chained.accept(&5);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![5, 10]
        );
        chained.accept(&-5);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![5, 10, -10]
        );
    }

    #[test]
    fn test_arc_when() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let consumer = ArcStatefulConsumer::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });
        let mut conditional = consumer.when(|x: &i32| *x > 0);
        conditional.accept(&5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);
        conditional.accept(&-5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);
    }

    #[test]
    fn test_arc_conditional_clone() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let consumer = ArcStatefulConsumer::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });
        let conditional = consumer.when(|x: &i32| *x > 0);
        let mut clone1 = conditional.clone();
        let mut clone2 = conditional.clone();

        clone1.accept(&5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);
        clone2.accept(&10);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![5, 10]
        );
    }

    #[test]
    fn test_arc_conditional_or_else() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();
        let consumer = ArcStatefulConsumer::new(move |x: &i32| {
            l1.lock().expect("mutex should not be poisoned").push(*x);
        });
        let conditional = consumer.when(|x: &i32| *x > 0);
        let mut with_else = conditional.or_else(move |x: &i32| {
            l2.lock()
                .expect("mutex should not be poisoned")
                .push(*x * 2);
        });
        with_else.accept(&5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);
        with_else.accept(&-5);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![5, -10]
        );
    }

    #[test]
    fn test_arc_conditional_debug() {
        let consumer = ArcStatefulConsumer::new(|_x: &i32| {});
        let conditional = consumer.when(|x: &i32| *x > 0);
        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("ArcConditionalStatefulConsumer"));
        assert!(debug_str.contains("consumer"));
        assert!(debug_str.contains("predicate"));
    }

    #[test]
    fn test_arc_conditional_display() {
        let consumer = ArcStatefulConsumer::new(|_x: &i32| {});
        let conditional = consumer.when(|x: &i32| *x > 0);
        let display_str = format!("{}", conditional);
        assert!(display_str.contains("ArcConditionalStatefulConsumer"));
    }

    #[test]
    fn test_rc_when() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l = log.clone();
        let consumer = RcStatefulConsumer::new(move |x: &i32| {
            l.borrow_mut().push(*x);
        });
        let mut conditional = consumer.when(|x: &i32| *x > 0);
        conditional.accept(&5);
        assert_eq!(*log.borrow(), vec![5]);
        conditional.accept(&-5);
        assert_eq!(*log.borrow(), vec![5]);
    }

    #[test]
    fn test_rc_conditional_clone() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l = log.clone();
        let consumer = RcStatefulConsumer::new(move |x: &i32| {
            l.borrow_mut().push(*x);
        });
        let conditional = consumer.when(|x: &i32| *x > 0);
        let mut clone1 = conditional.clone();
        let mut clone2 = conditional.clone();

        clone1.accept(&5);
        assert_eq!(*log.borrow(), vec![5]);
        clone2.accept(&10);
        assert_eq!(*log.borrow(), vec![5, 10]);
    }

    #[test]
    fn test_rc_conditional_or_else() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();
        let consumer = RcStatefulConsumer::new(move |x: &i32| {
            l1.borrow_mut().push(*x);
        });
        let conditional = consumer.when(|x: &i32| *x > 0);
        let mut with_else = conditional.or_else(move |x: &i32| {
            l2.borrow_mut().push(*x * 2);
        });
        with_else.accept(&5);
        assert_eq!(*log.borrow(), vec![5]);
        with_else.accept(&-5);
        assert_eq!(*log.borrow(), vec![5, -10]);
    }

    #[test]
    fn test_rc_conditional_debug() {
        let consumer = RcStatefulConsumer::new(|_x: &i32| {});
        let conditional = consumer.when(|x: &i32| *x > 0);
        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("RcConditionalStatefulConsumer"));
        assert!(debug_str.contains("consumer"));
        assert!(debug_str.contains("predicate"));
    }

    #[test]
    fn test_rc_conditional_display() {
        let consumer = RcStatefulConsumer::new(|_x: &i32| {});
        let conditional = consumer.when(|x: &i32| *x > 0);
        let display_str = format!("{}", conditional);
        assert!(display_str.contains("RcConditionalStatefulConsumer"));
    }
}
// ============================================================================
// Closure StatefulConsumer Tests
// ============================================================================

#[cfg(test)]
mod test_closure_to_methods {
    use super::{
        Arc,
        BoxStatefulConsumer,
        Mutex,
        StatefulConsumer,
    };

    // Note: closures must implement Clone to use to_xxx methods
    // We need to use cloneable closures or wrapper types

    // ============================================================================
    // BoxConsumer ConsumerOnce Tests
    // ============================================================================

    #[test]
    fn test_accept_once() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let mut consumer = BoxStatefulConsumer::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });
        consumer.accept(&5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);
    }

    #[test]
    fn test_accept_once_with_different_types() {
        // String
        let log = Arc::new(Mutex::new(String::new()));
        let l = log.clone();
        let mut consumer = BoxStatefulConsumer::new(move |s: &String| {
            *l.lock().expect("mutex should not be poisoned") =
                format!("Got: {}", s);
        });
        let text = String::from("hello");
        consumer.accept(&text);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            "Got: hello"
        );

        // Vec
        let log = Arc::new(Mutex::new(0));
        let l = log.clone();
        let mut consumer = BoxStatefulConsumer::new(move |v: &Vec<i32>| {
            *l.lock().expect("mutex should not be poisoned") = v.len();
        });
        let numbers = vec![1, 2, 3];
        consumer.accept(&numbers);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), 3);

        // bool
        let log = Arc::new(Mutex::new(String::new()));
        let l = log.clone();
        let mut consumer = BoxStatefulConsumer::new(move |b: &bool| {
            *l.lock().expect("mutex should not be poisoned") =
                if *b { "true" } else { "false" }.to_string();
        });
        let flag = true;
        consumer.accept(&flag);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), "true");
    }

    #[test]
    fn test_consumer_once_with_state_modification() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let mut counter = 0;
        let mut consumer = BoxStatefulConsumer::new(move |x: &i32| {
            counter += 1;
            l.lock()
                .expect("mutex should not be poisoned")
                .push(*x + counter);
        });
        consumer.accept(&10);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![11]
        ); // 10 + 1
    }

    #[test]
    fn test_consumer_once_consumes_self() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let mut consumer = BoxStatefulConsumer::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });

        consumer.accept(&5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);
    }
}

// ============================================================================
// ConsumerOnce Implementation Tests
// ============================================================================

#[cfg(test)]
mod consumer_once_trait_tests {
    use super::{
        Arc,
        ArcConsumer,
        BoxConsumer,
        Consumer,
        Mutex,
        Rc,
        RcConsumer,
        RefCell,
    };

    #[test]
    fn test_box_consumer_accept_once() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let consumer = BoxConsumer::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });

        consumer.accept(&100);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![100]
        );
    }

    #[test]
    fn test_arc_consumer_accept_once() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let consumer = ArcConsumer::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x * 3);
        });

        consumer.accept(&7);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![21]
        );
    }

    #[test]
    fn test_rc_consumer_accept_once() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l = log.clone();
        let consumer = RcConsumer::new(move |x: &i32| {
            l.borrow_mut().push(*x - 5);
        });

        consumer.accept(&15);
        assert_eq!(*log.borrow(), vec![10]);
    }
}

// ============================================================================
// BoxStatefulConsumer and_then Tests
// ============================================================================

#[cfg(test)]
mod test_box_stateful_consumer_chaining {
    use super::{
        Arc,
        ArcStatefulConsumer,
        BoxStatefulConsumer,
        Mutex,
        StatefulConsumer,
    };

    #[test]
    fn test_closure_and_then() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();

        let mut chained = BoxStatefulConsumer::new(move |x: &i32| {
            l1.lock()
                .expect("mutex should not be poisoned")
                .push(*x * 2);
        })
        .and_then(move |x: &i32| {
            l2.lock()
                .expect("mutex should not be poisoned")
                .push(*x + 10);
        });

        chained.accept(&5);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![10, 15]
        );
    }

    #[test]
    fn test_closure_and_then_with_box_consumer() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();

        let second = BoxStatefulConsumer::new(move |x: &i32| {
            l2.lock()
                .expect("mutex should not be poisoned")
                .push(*x + 10);
        });

        let mut chained = BoxStatefulConsumer::new(move |x: &i32| {
            l1.lock()
                .expect("mutex should not be poisoned")
                .push(*x * 2);
        })
        .and_then(second);

        chained.accept(&5);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![10, 15]
        );
    }

    #[test]
    fn test_closure_and_then_multiple() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();
        let l3 = log.clone();

        let first = move |x: &i32| {
            l1.lock().expect("mutex should not be poisoned").push(*x);
        };
        let second = move |x: &i32| {
            l2.lock()
                .expect("mutex should not be poisoned")
                .push(*x * 2);
        };
        let third = BoxStatefulConsumer::new(move |x: &i32| {
            l3.lock()
                .expect("mutex should not be poisoned")
                .push(*x + 100);
        });

        let chained = BoxStatefulConsumer::new(first).and_then(second);
        let mut chained = chained.and_then(third);

        chained.accept(&5);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![5, 10, 105]
        );
    }

    #[test]
    fn test_closure_and_then_with_arc_consumer() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();

        let second = ArcStatefulConsumer::new(move |x: &i32| {
            l2.lock()
                .expect("mutex should not be poisoned")
                .push(*x * 3);
        });

        let mut chained = BoxStatefulConsumer::new(move |x: &i32| {
            l1.lock()
                .expect("mutex should not be poisoned")
                .push(*x + 1);
        })
        .and_then(second);

        chained.accept(&5);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![6, 15]
        );
    }

    #[test]
    fn test_closure_and_then_with_arc_consumer_clone() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();

        let second = ArcStatefulConsumer::new(move |x: &i32| {
            l2.lock()
                .expect("mutex should not be poisoned")
                .push(*x + 10);
        });

        // Clone second to preserve it
        let mut chained = BoxStatefulConsumer::new(move |x: &i32| {
            l1.lock()
                .expect("mutex should not be poisoned")
                .push(*x * 2);
        })
        .and_then(second.clone());

        chained.accept(&5);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![10, 15]
        );

        // Original second still usable
        let mut second_copy = second;
        second_copy.accept(&3);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![10, 15, 13]
        );
    }
}
