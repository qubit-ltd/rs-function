// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
// qubit-style: allow explicit-imports
/// Tests for BiConsumer types
use qubit_function::{
    ArcBiConsumer,
    BiConsumer,
    BoxBiConsumer,
    FnBiConsumerOps,
    RcBiConsumer,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;


#[cfg(test)]
mod box_non_mutating_bi_consumer_tests {
    use super::{
        Arc,
        BiConsumer,
        BoxBiConsumer,
    };

    #[test]
    fn test_new_and_accept() {
        let consumer = BoxBiConsumer::new(|x: &i32, y: &i32| {
            std::hint::black_box(x + y);
        });
        consumer.accept(&5, &3);
    }

    #[test]
    fn test_and_then() {
        let counter = Arc::new(std::sync::Mutex::new(0));
        let c1 = counter.clone();
        let c2 = counter.clone();
        let chained = BoxBiConsumer::new(move |_x: &i32, _y: &i32| {
            *c1.lock().expect("mutex should not be poisoned") += 1;
        })
        .and_then(move |_x: &i32, _y: &i32| {
            *c2.lock().expect("mutex should not be poisoned") += 1;
        });

        chained.accept(&5, &3);
        assert_eq!(*counter.lock().expect("mutex should not be poisoned"), 2);
    }

    #[test]
    fn test_noop() {
        let noop = BoxBiConsumer::<i32, i32>::noop();
        noop.accept(&42, &10);
        // Should not panic
    }




    #[test]
    fn test_name() {
        let mut consumer = BoxBiConsumer::<i32, i32>::noop();
        assert_eq!(consumer.name(), None);

        consumer.set_name("test_consumer");
        assert_eq!(consumer.name(), Some("test_consumer"));
    }

    #[test]
    fn test_debug() {
        let consumer = BoxBiConsumer::new(|_x: &i32, _y: &i32| {});
        let debug_str = format!("{:?}", consumer);
        assert!(debug_str.contains("BoxBiConsumer"));
    }

    #[test]
    fn test_display() {
        let consumer = BoxBiConsumer::new(|_x: &i32, _y: &i32| {});
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "BoxBiConsumer");
    }

    #[test]
    fn test_display_with_name() {
        let mut consumer = BoxBiConsumer::new(|_x: &i32, _y: &i32| {});
        consumer.set_name("my_consumer");
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "BoxBiConsumer(my_consumer)");
    }

}

#[cfg(test)]
mod arc_non_mutating_bi_consumer_tests {
    use super::{
        Arc,
        ArcBiConsumer,
        BiConsumer,
    };

    #[test]
    fn test_new_and_accept() {
        let consumer = ArcBiConsumer::new(|x: &i32, y: &i32| {
            std::hint::black_box(x + y);
        });
        consumer.accept(&5, &3);
    }

    #[test]
    fn test_clone() {
        let counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let c = counter.clone();
        let consumer = ArcBiConsumer::new(move |_x: &i32, _y: &i32| {
            c.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        });

        let clone1 = consumer.clone();
        let clone2 = consumer.clone();

        clone1.accept(&5, &3);
        assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 1);

        clone2.accept(&10, &2);
        assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 2);
    }

    #[test]
    fn test_and_then() {
        let counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let c1 = counter.clone();
        let c2 = counter.clone();
        let first = ArcBiConsumer::new(move |_x: &i32, _y: &i32| {
            c1.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        });
        let second = ArcBiConsumer::new(move |_x: &i32, _y: &i32| {
            c2.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        });

        let chained = first.and_then(second);

        chained.accept(&5, &3);
        assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 2);
    }




    #[test]
    fn test_name() {
        let mut consumer = ArcBiConsumer::new(|_x: &i32, _y: &i32| {});
        assert_eq!(consumer.name(), None);

        consumer.set_name("test_consumer");
        assert_eq!(consumer.name(), Some("test_consumer"));
    }

    #[test]
    fn test_debug() {
        let consumer = ArcBiConsumer::new(|_x: &i32, _y: &i32| {});
        let debug_str = format!("{:?}", consumer);
        assert!(debug_str.contains("ArcBiConsumer"));
    }

    #[test]
    fn test_display() {
        let consumer = ArcBiConsumer::new(|_x: &i32, _y: &i32| {});
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "ArcBiConsumer");
    }

    #[test]
    fn test_display_with_name() {
        let mut consumer = ArcBiConsumer::new(|_x: &i32, _y: &i32| {});
        consumer.set_name("my_consumer");
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "ArcBiConsumer(my_consumer)");
    }



}

#[cfg(test)]
mod rc_non_mutating_bi_consumer_tests {
    use super::{
        BiConsumer,
        Rc,
        RcBiConsumer,
    };

    #[test]
    fn test_new_and_accept() {
        let consumer = RcBiConsumer::new(|x: &i32, y: &i32| {
            std::hint::black_box(x + y);
        });
        consumer.accept(&5, &3);
    }

    #[test]
    fn test_clone() {
        let counter = Rc::new(std::cell::Cell::new(0));
        let c = counter.clone();
        let consumer = RcBiConsumer::new(move |_x: &i32, _y: &i32| {
            c.set(c.get() + 1);
        });

        let clone1 = consumer.clone();
        let clone2 = consumer.clone();

        clone1.accept(&5, &3);
        assert_eq!(counter.get(), 1);

        clone2.accept(&10, &2);
        assert_eq!(counter.get(), 2);
    }

    #[test]
    fn test_and_then() {
        let counter = Rc::new(std::cell::Cell::new(0));
        let c1 = counter.clone();
        let c2 = counter.clone();
        let first = RcBiConsumer::new(move |_x: &i32, _y: &i32| {
            c1.set(c1.get() + 1);
        });
        let second = RcBiConsumer::new(move |_x: &i32, _y: &i32| {
            c2.set(c2.get() + 1);
        });

        let chained = first.and_then(second);

        chained.accept(&5, &3);
        assert_eq!(counter.get(), 2);
    }



    #[test]
    fn test_name() {
        let mut consumer = RcBiConsumer::new(|_x: &i32, _y: &i32| {});
        assert_eq!(consumer.name(), None);

        consumer.set_name("test_consumer");
        assert_eq!(consumer.name(), Some("test_consumer"));
    }

    #[test]
    fn test_debug() {
        let consumer = RcBiConsumer::new(|_x: &i32, _y: &i32| {});
        let debug_str = format!("{:?}", consumer);
        assert!(debug_str.contains("RcBiConsumer"));
    }

    #[test]
    fn test_display() {
        let consumer = RcBiConsumer::new(|_x: &i32, _y: &i32| {});
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "RcBiConsumer");
    }

    #[test]
    fn test_display_with_name() {
        let mut consumer = RcBiConsumer::new(|_x: &i32, _y: &i32| {});
        consumer.set_name("test_consumer");
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "RcBiConsumer(test_consumer)");
    }


}

#[cfg(test)]
mod closure_tests {
    use super::{
        Arc,
        BiConsumer,
        FnBiConsumerOps,
    };

    #[test]
    fn test_closure_accept() {
        let closure = |x: &i32, y: &i32| {
            std::hint::black_box(x + y);
        };
        closure.accept(&5, &3);
    }

    #[test]
    fn test_closure_and_then() {
        let counter = Arc::new(std::sync::Mutex::new(0));
        let c1 = counter.clone();
        let c2 = counter.clone();
        let chained = (move |_x: &i32, _y: &i32| {
            *c1.lock().expect("mutex should not be poisoned") += 1;
        })
        .and_then(move |_x: &i32, _y: &i32| {
            *c2.lock().expect("mutex should not be poisoned") += 1;
        });

        chained.accept(&5, &3);
        assert_eq!(*counter.lock().expect("mutex should not be poisoned"), 2);
    }


}

// ============================================================================
// Edge Cases Tests
// ============================================================================

#[cfg(test)]
mod edge_cases_tests {
    use super::{
        Arc,
        ArcBiConsumer,
        BiConsumer,
        BoxBiConsumer,
        Rc,
        RcBiConsumer,
        RefCell,
    };

    #[test]
    fn test_noop_multiple_calls() {
        let consumer = BoxBiConsumer::<i32, i32>::noop();
        consumer.accept(&5, &3);
        consumer.accept(&10, &20);
        consumer.accept(&1, &2);
        // Should do nothing
    }

    #[test]
    fn test_and_then_with_noop() {
        let counter = Arc::new(std::sync::Mutex::new(0));
        let c = counter.clone();
        let consumer = BoxBiConsumer::new(move |_x: &i32, _y: &i32| {
            *c.lock().expect("mutex should not be poisoned") += 1;
        })
        .and_then(BoxBiConsumer::noop());
        consumer.accept(&5, &3);
        assert_eq!(*counter.lock().expect("mutex should not be poisoned"), 1);
    }

    #[test]
    fn test_complex_chain() {
        let counter = Arc::new(std::sync::Mutex::new(0));
        let c1 = counter.clone();
        let c2 = counter.clone();
        let c3 = counter.clone();
        let consumer = BoxBiConsumer::new(move |_x: &i32, _y: &i32| {
            *c1.lock().expect("mutex should not be poisoned") += 1;
        })
        .and_then(move |_x: &i32, _y: &i32| {
            *c2.lock().expect("mutex should not be poisoned") += 1;
        })
        .and_then(BoxBiConsumer::noop())
        .and_then(move |_x: &i32, _y: &i32| {
            *c3.lock().expect("mutex should not be poisoned") += 1;
        });
        consumer.accept(&5, &3);
        assert_eq!(*counter.lock().expect("mutex should not be poisoned"), 3);
    }

    #[test]
    fn test_with_different_types() {
        let counter = Arc::new(std::sync::Mutex::new(String::new()));
        let c = counter.clone();
        let consumer = BoxBiConsumer::new(move |s: &String, n: &i32| {
            *c.lock().expect("mutex should not be poisoned") =
                format!("{}: {}", s, n);
        });
        consumer.accept(&"Count".to_string(), &42);
        assert_eq!(
            *counter.lock().expect("mutex should not be poisoned"),
            "Count: 42"
        );
    }

    #[test]
    fn test_arc_consumer_multiple_threads() {
        let counter = Arc::new(std::sync::Mutex::new(0));
        let c = counter.clone();
        let consumer = ArcBiConsumer::new(move |x: &i32, y: &i32| {
            *c.lock().expect("mutex should not be poisoned") += x + y;
        });

        let handles: Vec<_> = (0..10)
            .map(|i| {
                let cons = consumer.clone();
                std::thread::spawn(move || {
                    cons.accept(&i, &1);
                })
            })
            .collect();

        for handle in handles {
            handle.join().expect("thread should not panic");
        }

        // Sum of (0+1) + (1+1) + ... + (9+1) = 55
        assert_eq!(*counter.lock().expect("mutex should not be poisoned"), 55);
    }

    #[test]
    fn test_rc_consumer_multiple_clones() {
        let counter = Rc::new(RefCell::new(0));
        let c = counter.clone();
        let consumer = RcBiConsumer::new(move |x: &i32, y: &i32| {
            *c.borrow_mut() += x + y;
        });

        let cons1 = consumer.clone();
        let cons2 = consumer.clone();
        let cons3 = consumer.clone();

        cons1.accept(&1, &2);
        cons2.accept(&3, &4);
        cons3.accept(&5, &6);

        assert_eq!(*counter.borrow(), 21); // 3 + 7 + 11
    }

    #[test]
    fn test_name_with_and_then() {
        let mut consumer1 = BoxBiConsumer::new(|_x: &i32, _y: &i32| {});
        consumer1.set_name("first");
        let consumer2 = BoxBiConsumer::new(|_x: &i32, _y: &i32| {});
        let chained = consumer1.and_then(consumer2);
        // Name is not preserved through and_then
        assert_eq!(chained.name(), None);
    }


}

// ============================================================================
// Conversion Tests
// ============================================================================

#[cfg(test)]
mod conversion_tests {







}

// ============================================================================
// Name Tests - Testing name() and set_name() methods
// ============================================================================

#[cfg(test)]
mod name_tests {
    use super::{
        ArcBiConsumer,
        BiConsumer,
        BoxBiConsumer,
        RcBiConsumer,
    };

    #[test]
    fn test_box_consumer_name() {
        let mut consumer = BoxBiConsumer::new(|x: &i32, y: &i32| {
            std::hint::black_box(x + y);
        });
        assert_eq!(consumer.name(), None);

        consumer.set_name("add_printer");
        assert_eq!(consumer.name(), Some("add_printer"));
    }

    #[test]
    fn test_arc_consumer_name() {
        let mut consumer = ArcBiConsumer::new(|x: &i32, y: &i32| {
            std::hint::black_box(x + y);
        });
        assert_eq!(consumer.name(), None);

        consumer.set_name("add_printer");
        assert_eq!(consumer.name(), Some("add_printer"));
    }

    #[test]
    fn test_rc_consumer_name() {
        let mut consumer = RcBiConsumer::new(|x: &i32, y: &i32| {
            std::hint::black_box(x + y);
        });
        assert_eq!(consumer.name(), None);

        consumer.set_name("add_printer");
        assert_eq!(consumer.name(), Some("add_printer"));
    }

    #[test]
    fn test_box_consumer_name_with_accept() {
        let mut consumer = BoxBiConsumer::new(|_x: &i32, _y: &i32| {});
        consumer.set_name("test_consumer");
        assert_eq!(consumer.name(), Some("test_consumer"));
        consumer.accept(&1, &2);
        assert_eq!(consumer.name(), Some("test_consumer"));
    }

    #[test]
    fn test_arc_consumer_name_with_accept() {
        let mut consumer = ArcBiConsumer::new(|_x: &i32, _y: &i32| {});
        consumer.set_name("test_consumer");
        assert_eq!(consumer.name(), Some("test_consumer"));
        consumer.accept(&1, &2);
        assert_eq!(consumer.name(), Some("test_consumer"));
    }

    #[test]
    fn test_rc_consumer_name_with_accept() {
        let mut consumer = RcBiConsumer::new(|_x: &i32, _y: &i32| {});
        consumer.set_name("test_consumer");
        assert_eq!(consumer.name(), Some("test_consumer"));
        consumer.accept(&1, &2);
        assert_eq!(consumer.name(), Some("test_consumer"));
    }

    #[test]
    fn test_box_consumer_name_change() {
        let mut consumer = BoxBiConsumer::new(|_x: &i32, _y: &i32| {});
        consumer.set_name("name1");
        assert_eq!(consumer.name(), Some("name1"));
        consumer.set_name("name2");
        assert_eq!(consumer.name(), Some("name2"));
    }

    #[test]
    fn test_arc_consumer_name_change() {
        let mut consumer = ArcBiConsumer::new(|_x: &i32, _y: &i32| {});
        consumer.set_name("name1");
        assert_eq!(consumer.name(), Some("name1"));
        consumer.set_name("name2");
        assert_eq!(consumer.name(), Some("name2"));
    }

    #[test]
    fn test_rc_consumer_name_change() {
        let mut consumer = RcBiConsumer::new(|_x: &i32, _y: &i32| {});
        consumer.set_name("name1");
        assert_eq!(consumer.name(), Some("name1"));
        consumer.set_name("name2");
        assert_eq!(consumer.name(), Some("name2"));
    }







}

// ============================================================================
// Display and Debug Tests
// ============================================================================

#[cfg(test)]
mod display_debug_tests {
    use super::{
        ArcBiConsumer,
        BoxBiConsumer,
        RcBiConsumer,
    };

    #[test]
    fn test_box_consumer_debug() {
        let consumer = BoxBiConsumer::new(|_x: &i32, _y: &i32| {});
        let debug_str = format!("{:?}", consumer);
        assert!(debug_str.contains("BoxBiConsumer"));
        assert!(debug_str.contains("name"));
        assert!(debug_str.contains("function"));
    }

    #[test]
    fn test_box_consumer_display_without_name() {
        let consumer = BoxBiConsumer::new(|_x: &i32, _y: &i32| {});
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "BoxBiConsumer");
    }

    #[test]
    fn test_box_consumer_display_with_name() {
        let mut consumer = BoxBiConsumer::new(|_x: &i32, _y: &i32| {});
        consumer.set_name("test_consumer");
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "BoxBiConsumer(test_consumer)");
    }

    #[test]
    fn test_arc_consumer_debug() {
        let consumer = ArcBiConsumer::new(|_x: &i32, _y: &i32| {});
        let debug_str = format!("{:?}", consumer);
        assert!(debug_str.contains("ArcBiConsumer"));
        assert!(debug_str.contains("name"));
        assert!(debug_str.contains("function"));
    }

    #[test]
    fn test_arc_consumer_display_without_name() {
        let consumer = ArcBiConsumer::new(|_x: &i32, _y: &i32| {});
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "ArcBiConsumer");
    }

    #[test]
    fn test_arc_consumer_display_with_name() {
        let mut consumer = ArcBiConsumer::new(|_x: &i32, _y: &i32| {});
        consumer.set_name("test_consumer");
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "ArcBiConsumer(test_consumer)");
    }

    #[test]
    fn test_rc_consumer_debug() {
        let consumer = RcBiConsumer::new(|_x: &i32, _y: &i32| {});
        let debug_str = format!("{:?}", consumer);
        assert!(debug_str.contains("RcBiConsumer"));
        assert!(debug_str.contains("name"));
        assert!(debug_str.contains("function"));
    }

    #[test]
    fn test_rc_consumer_display_without_name() {
        let consumer = RcBiConsumer::new(|_x: &i32, _y: &i32| {});
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "RcBiConsumer");
    }

    #[test]
    fn test_rc_consumer_display_with_name() {
        let mut consumer = RcBiConsumer::new(|_x: &i32, _y: &i32| {});
        consumer.set_name("test_consumer");
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "RcBiConsumer(test_consumer)");
    }
}

// ============================================================================
// Custom BiConsumer Implementation Tests - Testing default into_xxx methods
// ============================================================================

#[cfg(test)]
mod custom_non_mutating_bi_consumer_tests {
    use super::{
        Arc,
        BiConsumer,
    };

    /// Custom BiConsumer implementation for testing trait's default methods
    struct CustomBiConsumer<T, U> {
        counter: Arc<std::sync::Mutex<i32>>,
        _phantom: std::marker::PhantomData<(T, U)>,
    }

    impl<T, U> CustomBiConsumer<T, U> {
        fn new(counter: Arc<std::sync::Mutex<i32>>) -> Self {
            Self {
                counter,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl<T, U> BiConsumer<T, U> for CustomBiConsumer<T, U> {
        fn accept(&self, _first: &T, _second: &U) {
            *self.counter.lock().expect("mutex should not be poisoned") += 1;
        }
        // Use default into_xxx implementations from the trait
    }











}

#[cfg(test)]
mod noop_tests {
    use super::{
        Arc,
        ArcBiConsumer,
        BiConsumer,
        BoxBiConsumer,
        Rc,
        RcBiConsumer,
    };

    #[test]
    fn test_box_noop_multiple_accepts() {
        let noop = BoxBiConsumer::<i32, i32>::noop();
        noop.accept(&1, &2);
        noop.accept(&3, &4);
        noop.accept(&5, &6);
        // Should not panic and do nothing
    }

    #[test]
    fn test_arc_noop_multiple_accepts() {
        let noop = ArcBiConsumer::<i32, i32>::noop();
        noop.accept(&1, &2);
        noop.accept(&3, &4);
        noop.accept(&5, &6);
        // Should not panic and do nothing
    }

    #[test]
    fn test_rc_noop_multiple_accepts() {
        let noop = RcBiConsumer::<i32, i32>::noop();
        noop.accept(&1, &2);
        noop.accept(&3, &4);
        noop.accept(&5, &6);
        // Should not panic and do nothing
    }

    #[test]
    fn test_box_noop_with_and_then() {
        let counter = Arc::new(std::sync::Mutex::new(0));
        let c = counter.clone();
        let active = BoxBiConsumer::new(move |_x: &i32, _y: &i32| {
            *c.lock().expect("mutex should not be poisoned") += 1;
        });
        let chained = active.and_then(BoxBiConsumer::noop());
        chained.accept(&1, &2);
        assert_eq!(*counter.lock().expect("mutex should not be poisoned"), 1);
    }

    #[test]
    fn test_arc_noop_with_and_then() {
        let counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let c = counter.clone();
        let active = ArcBiConsumer::new(move |_x: &i32, _y: &i32| {
            c.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        });
        let noop = ArcBiConsumer::<i32, i32>::noop();
        let chained = active.and_then(noop);
        chained.accept(&1, &2);
        assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 1);
    }

    #[test]
    fn test_rc_noop_with_and_then() {
        let counter = Rc::new(std::cell::Cell::new(0));
        let c = counter.clone();
        let active = RcBiConsumer::new(move |_x: &i32, _y: &i32| {
            c.set(c.get() + 1);
        });
        let chained = active.and_then(RcBiConsumer::<i32, i32>::noop());
        chained.accept(&1, &2);
        assert_eq!(counter.get(), 1);
    }
}

// ============================================================================
// to_xxx Methods Tests - Testing non-consuming conversion methods
// ============================================================================

#[cfg(test)]
mod to_methods_tests {
    use super::{
        Arc,
        BiConsumer,
    };

    // ========================================================================
    // ArcBiConsumer to_xxx tests
    // ========================================================================






    // ========================================================================
    // RcBiConsumer to_xxx tests
    // ========================================================================





    // ========================================================================
    // Closure to_xxx tests
    // ========================================================================





    // ========================================================================
    // Custom BiConsumer to_xxx tests
    // ========================================================================

    /// Custom BiConsumer implementation for testing default to_xxx methods
    #[derive(Clone)]
    pub struct CustomConsumer {
        counter: Arc<std::sync::Mutex<i32>>,
    }

    impl CustomConsumer {
        pub fn new(counter: Arc<std::sync::Mutex<i32>>) -> Self {
            Self { counter }
        }
    }

    impl BiConsumer<i32, i32> for CustomConsumer {
        fn accept(&self, first: &i32, second: &i32) {
            *self.counter.lock().expect("mutex should not be poisoned") +=
                first + second;
        }
        // Use default to_xxx implementations from the trait
    }

    unsafe impl Send for CustomConsumer {}
    unsafe impl Sync for CustomConsumer {}











}

// ============================================================================
// to_once Tests - Testing BiConsumer trait default to_once implementation
// ============================================================================

#[cfg(test)]
mod to_once_tests {





}

// ============================================================================
// Conditional BiConsumer Tests
// ============================================================================

#[cfg(test)]
mod box_conditional_bi_consumer_tests {
    use super::{
        Arc,
        BiConsumer,
        BoxBiConsumer,
    };
    use std::sync::Mutex;

    #[test]
    fn test_box_conditional_and_then() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();

        let consumer = BoxBiConsumer::new(move |x: &i32, y: &i32| {
            l1.lock()
                .expect("mutex should not be poisoned")
                .push(*x + *y);
        });

        let conditional = consumer.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let chained = conditional.and_then(move |x: &i32, y: &i32| {
            l2.lock()
                .expect("mutex should not be poisoned")
                .push(*x * *y);
        });

        chained.accept(&5, &3);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![8, 15]
        );

        chained.accept(&-5, &3);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![8, 15, -15]
        );
    }

    #[test]
    fn test_box_conditional_or_else() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();

        let consumer = BoxBiConsumer::new(move |x: &i32, y: &i32| {
            l1.lock()
                .expect("mutex should not be poisoned")
                .push(*x + *y);
        });

        let conditional = consumer
            .when(|x: &i32, y: &i32| *x > 0 && *y > 0)
            .or_else(move |x: &i32, y: &i32| {
                l2.lock()
                    .expect("mutex should not be poisoned")
                    .push(*x * *y);
            });

        conditional.accept(&5, &3);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8]);

        conditional.accept(&-5, &3);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![8, -15]
        );
    }

    #[test]
    fn test_box_conditional_accept() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();

        let consumer = BoxBiConsumer::new(move |x: &i32, y: &i32| {
            l.lock()
                .expect("mutex should not be poisoned")
                .push(*x + *y);
        });

        let conditional = consumer.when(|x: &i32, y: &i32| *x > 0 && *y > 0);

        conditional.accept(&5, &3);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8]);

        conditional.accept(&-5, &3);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![8]);
    }



}

#[cfg(test)]
mod arc_conditional_bi_consumer_tests {
    use super::{
        Arc,
        ArcBiConsumer,
        BiConsumer,
    };
    use std::sync::atomic::{
        AtomicUsize,
        Ordering,
    };

    #[test]
    fn test_arc_conditional_and_then() {
        let counter = Arc::new(AtomicUsize::new(0));
        let c1 = counter.clone();
        let c2 = counter.clone();

        let consumer = ArcBiConsumer::new(move |_x: &i32, _y: &i32| {
            c1.fetch_add(1, Ordering::SeqCst);
        });

        let conditional = consumer.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let chained = conditional.and_then(move |_x: &i32, _y: &i32| {
            c2.fetch_add(10, Ordering::SeqCst);
        });

        chained.accept(&5, &3);
        assert_eq!(counter.load(Ordering::SeqCst), 11);

        chained.accept(&-5, &3);
        assert_eq!(counter.load(Ordering::SeqCst), 21);
    }

    #[test]
    fn test_arc_conditional_or_else() {
        let counter = Arc::new(AtomicUsize::new(0));
        let c1 = counter.clone();
        let c2 = counter.clone();

        let consumer = ArcBiConsumer::new(move |_x: &i32, _y: &i32| {
            c1.fetch_add(1, Ordering::SeqCst);
        });

        let conditional = consumer
            .when(|x: &i32, y: &i32| *x > 0 && *y > 0)
            .or_else(move |_x: &i32, _y: &i32| {
                c2.fetch_add(100, Ordering::SeqCst);
            });

        conditional.accept(&5, &3);
        assert_eq!(counter.load(Ordering::SeqCst), 1);

        conditional.accept(&-5, &3);
        assert_eq!(counter.load(Ordering::SeqCst), 101);
    }

    #[test]
    fn test_arc_conditional_accept() {
        let counter = Arc::new(AtomicUsize::new(0));
        let c = counter.clone();

        let consumer = ArcBiConsumer::new(move |_x: &i32, _y: &i32| {
            c.fetch_add(1, Ordering::SeqCst);
        });

        let conditional = consumer.when(|x: &i32, y: &i32| *x > 0 && *y > 0);

        conditional.accept(&5, &3);
        assert_eq!(counter.load(Ordering::SeqCst), 1);

        conditional.accept(&-5, &3);
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }








}

#[cfg(test)]
mod rc_conditional_bi_consumer_tests {
    use super::{
        BiConsumer,
        Rc,
        RcBiConsumer,
    };
    use std::cell::RefCell;

    #[test]
    fn test_rc_conditional_and_then() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();

        let consumer = RcBiConsumer::new(move |x: &i32, y: &i32| {
            l1.borrow_mut().push(*x + *y);
        });

        let conditional = consumer.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let chained = conditional.and_then(move |x: &i32, y: &i32| {
            l2.borrow_mut().push(*x * *y);
        });

        chained.accept(&5, &3);
        assert_eq!(*log.borrow(), vec![8, 15]);

        chained.accept(&-5, &3);
        assert_eq!(*log.borrow(), vec![8, 15, -15]);
    }

    #[test]
    fn test_rc_conditional_or_else() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();

        let consumer = RcBiConsumer::new(move |x: &i32, y: &i32| {
            l1.borrow_mut().push(*x + *y);
        });

        let conditional = consumer
            .when(|x: &i32, y: &i32| *x > 0 && *y > 0)
            .or_else(move |x: &i32, y: &i32| {
                l2.borrow_mut().push(*x * *y);
            });

        conditional.accept(&5, &3);
        assert_eq!(*log.borrow(), vec![8]);

        conditional.accept(&-5, &3);
        assert_eq!(*log.borrow(), vec![8, -15]);
    }

    #[test]
    fn test_rc_conditional_accept() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l = log.clone();

        let consumer = RcBiConsumer::new(move |x: &i32, y: &i32| {
            l.borrow_mut().push(*x + *y);
        });

        let conditional = consumer.when(|x: &i32, y: &i32| *x > 0 && *y > 0);

        conditional.accept(&5, &3);
        assert_eq!(*log.borrow(), vec![8]);

        conditional.accept(&-5, &3);
        assert_eq!(*log.borrow(), vec![8]);
    }






}
