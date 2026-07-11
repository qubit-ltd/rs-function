// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

// qubit-style: allow explicit-imports
//! Tests for Consumer types

use qubit_function::{
    ArcConsumer,
    BoxConsumer,
    Consumer,
    FnConsumerOps,
    RcConsumer,
};
use std::rc::Rc;
use std::sync::Arc;


#[cfg(test)]
mod box_non_mutating_consumer_tests {
    use super::{
        Arc,
        ArcConsumer,
        BoxConsumer,
        Consumer,
    };

    #[test]
    fn test_new_and_accept() {
        let consumer = BoxConsumer::new(|x: &i32| {
            std::hint::black_box(x);
        });
        consumer.accept(&5);
    }

    #[test]
    fn test_and_then() {
        let counter = Arc::new(std::sync::Mutex::new(0));
        let c1 = counter.clone();
        let c2 = counter.clone();
        let chained = BoxConsumer::new(move |_x: &i32| {
            *c1.lock().expect("mutex should not be poisoned") += 1;
        })
        .and_then(move |_x: &i32| {
            *c2.lock().expect("mutex should not be poisoned") += 1;
        });

        chained.accept(&5);
        assert_eq!(*counter.lock().expect("mutex should not be poisoned"), 2);
    }

    #[test]
    fn test_and_then_with_box_consumer() {
        let counter = Arc::new(std::sync::Mutex::new(0));
        let c1 = counter.clone();
        let c2 = counter.clone();

        let first = BoxConsumer::new(move |_x: &i32| {
            *c1.lock().expect("mutex should not be poisoned") += 1;
        });

        let second = BoxConsumer::new(move |_x: &i32| {
            *c2.lock().expect("mutex should not be poisoned") += 1;
        });

        let chained = first.and_then(second);
        chained.accept(&5);
        assert_eq!(*counter.lock().expect("mutex should not be poisoned"), 2);
    }

    #[test]
    fn test_and_then_multiple_chains() {
        let counter = Arc::new(std::sync::Mutex::new(0));
        let c1 = counter.clone();
        let c2 = counter.clone();
        let c3 = counter.clone();

        let chained = BoxConsumer::new(move |_x: &i32| {
            *c1.lock().expect("mutex should not be poisoned") += 1;
        })
        .and_then(move |_x: &i32| {
            *c2.lock().expect("mutex should not be poisoned") += 1;
        })
        .and_then(move |_x: &i32| {
            *c3.lock().expect("mutex should not be poisoned") += 1;
        });

        chained.accept(&5);
        assert_eq!(*counter.lock().expect("mutex should not be poisoned"), 3);
    }

    #[test]
    fn test_noop() {
        let noop = BoxConsumer::<i32>::noop();
        noop.accept(&42);
        // Should not panic
    }

    #[test]
    fn test_arc_noop() {
        let noop = ArcConsumer::<i32>::noop();
        noop.accept(&42);
        // Should not panic
    }





    #[test]
    fn test_name() {
        let mut consumer = BoxConsumer::<i32>::noop();
        assert_eq!(consumer.name(), None);

        consumer.set_name("test_consumer");
        assert_eq!(consumer.name(), Some("test_consumer"));
    }

    #[test]
    fn test_debug() {
        let consumer = BoxConsumer::<i32>::noop();
        let debug_str = format!("{:?}", consumer);
        assert!(debug_str.contains("BoxConsumer"));
    }

    #[test]
    fn test_display() {
        let mut consumer = BoxConsumer::<i32>::noop();
        assert_eq!(format!("{}", consumer), "BoxConsumer");

        consumer.set_name("my_consumer");
        assert_eq!(format!("{}", consumer), "BoxConsumer(my_consumer)");
    }

    #[test]
    fn test_with_different_types() {
        let string_consumer = BoxConsumer::new(|s: &String| {
            std::hint::black_box(s);
        });
        string_consumer.accept(&"Hello".to_string());

        let vec_consumer = BoxConsumer::new(|v: &Vec<i32>| {
            std::hint::black_box(v.len());
        });
        vec_consumer.accept(&vec![1, 2, 3]);
    }
}

#[cfg(test)]
mod arc_non_mutating_consumer_tests {
    use super::{
        Arc,
        ArcConsumer,
        Consumer,
    };

    #[test]
    fn test_new_and_accept() {
        let consumer = ArcConsumer::new(|x: &i32| {
            std::hint::black_box(x);
        });
        consumer.accept(&5);
    }

    #[test]
    fn test_clone() {
        let counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let c = counter.clone();
        let consumer = ArcConsumer::new(move |_x: &i32| {
            c.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        });

        let clone = consumer.clone();
        consumer.accept(&5);
        clone.accept(&10);

        assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 2);
    }

    #[test]
    fn test_and_then() {
        let counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let c1 = counter.clone();
        let c2 = counter.clone();

        let first = ArcConsumer::new(move |_x: &i32| {
            c1.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        });

        let second = ArcConsumer::new(move |_x: &i32| {
            c2.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        });

        let chained = first.and_then(second.clone());
        chained.accept(&5);

        assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 2);

        // Original consumers remain usable
        first.accept(&10);
        second.accept(&15);
        assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 4);
    }






    #[test]
    fn test_name() {
        let mut consumer = ArcConsumer::new(|_x: &i32| {});
        assert_eq!(consumer.name(), None);

        consumer.set_name("test_consumer");
        assert_eq!(consumer.name(), Some("test_consumer"));
    }

    #[test]
    fn test_debug() {
        let consumer = ArcConsumer::new(|_x: &i32| {});
        let debug_str = format!("{:?}", consumer);
        assert!(debug_str.contains("ArcConsumer"));
    }

    #[test]
    fn test_display() {
        let mut consumer = ArcConsumer::new(|_x: &i32| {});
        assert_eq!(format!("{}", consumer), "ArcConsumer");

        consumer.set_name("my_consumer");
        assert_eq!(format!("{}", consumer), "ArcConsumer(my_consumer)");
    }

    #[test]
    fn test_thread_safety() {
        let counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let c = counter.clone();
        let consumer = ArcConsumer::new(move |_x: &i32| {
            c.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        });

        let handles: Vec<_> = (0..10)
            .map(|i| {
                let consumer_clone = consumer.clone();
                std::thread::spawn(move || {
                    consumer_clone.accept(&i);
                })
            })
            .collect();

        for handle in handles {
            handle.join().expect("thread should not panic");
        }

        assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 10);
    }
}

#[cfg(test)]
mod rc_non_mutating_consumer_tests {
    use super::{
        Consumer,
        Rc,
        RcConsumer,
    };

    #[test]
    fn test_new_and_accept() {
        let consumer = RcConsumer::new(|x: &i32| {
            std::hint::black_box(x);
        });
        consumer.accept(&5);
    }

    #[test]
    fn test_rc_noop() {
        let noop = RcConsumer::<i32>::noop();
        noop.accept(&42);
        // Should not panic
    }

    #[test]
    fn test_clone() {
        let counter = Rc::new(std::cell::RefCell::new(0));
        let c = counter.clone();
        let consumer = RcConsumer::new(move |_x: &i32| {
            *c.borrow_mut() += 1;
        });

        let clone = consumer.clone();
        consumer.accept(&5);
        clone.accept(&10);

        assert_eq!(*counter.borrow(), 2);
    }

    #[test]
    fn test_and_then() {
        let counter = Rc::new(std::cell::RefCell::new(0));
        let c1 = counter.clone();
        let c2 = counter.clone();

        let first = RcConsumer::new(move |_x: &i32| {
            *c1.borrow_mut() += 1;
        });

        let second = RcConsumer::new(move |_x: &i32| {
            *c2.borrow_mut() += 1;
        });

        let chained = first.and_then(second.clone());
        chained.accept(&5);

        assert_eq!(*counter.borrow(), 2);

        // Original consumers remain usable
        first.accept(&10);
        second.accept(&15);
        assert_eq!(*counter.borrow(), 4);
    }





    #[test]
    fn test_name() {
        let mut consumer = RcConsumer::new(|_x: &i32| {});
        assert_eq!(consumer.name(), None);

        consumer.set_name("test_consumer");
        assert_eq!(consumer.name(), Some("test_consumer"));
    }

    #[test]
    fn test_debug() {
        let consumer = RcConsumer::new(|_x: &i32| {});
        let debug_str = format!("{:?}", consumer);
        assert!(debug_str.contains("RcConsumer"));
    }

    #[test]
    fn test_display() {
        let mut consumer = RcConsumer::new(|_x: &i32| {});
        assert_eq!(format!("{}", consumer), "RcConsumer");

        consumer.set_name("my_consumer");
        assert_eq!(format!("{}", consumer), "RcConsumer(my_consumer)");
    }
}

#[cfg(test)]
mod closure_tests {
    use super::{
        Arc,
        Consumer,
        FnConsumerOps,
    };

    #[test]
    fn test_closure_accept() {
        let closure = |x: &i32| {
            std::hint::black_box(x);
        };
        closure.accept(&5);
    }





    #[test]
    fn test_closure_and_then() {
        let counter = Arc::new(std::sync::Mutex::new(0));
        let c1 = counter.clone();
        let c2 = counter.clone();

        let chained = (move |_x: &i32| {
            *c1.lock().expect("mutex should not be poisoned") += 1;
        })
        .and_then(move |_x: &i32| {
            *c2.lock().expect("mutex should not be poisoned") += 1;
        });

        chained.accept(&5);
        assert_eq!(*counter.lock().expect("mutex should not be poisoned"), 2);
    }

    #[test]
    fn test_closure_and_then_multiple() {
        let counter = Arc::new(std::sync::Mutex::new(0));
        let c1 = counter.clone();
        let c2 = counter.clone();
        let c3 = counter.clone();

        let chained = (move |_x: &i32| {
            *c1.lock().expect("mutex should not be poisoned") += 1;
        })
        .and_then(move |_x: &i32| {
            *c2.lock().expect("mutex should not be poisoned") += 1;
        })
        .and_then(move |_x: &i32| {
            *c3.lock().expect("mutex should not be poisoned") += 1;
        });

        chained.accept(&5);
        assert_eq!(*counter.lock().expect("mutex should not be poisoned"), 3);
    }
}

#[cfg(test)]
mod conversion_tests {






    // Note: Box and Rc cannot be converted to Arc because they don't implement
    // Send+Sync These conversions are prevented at compile time, not
    // runtime
}

#[cfg(test)]
mod generic_tests {
    use super::{
        ArcConsumer,
        BoxConsumer,
        Consumer,
        RcConsumer,
    };

    fn apply_consumer<C: Consumer<i32>>(consumer: &C, value: &i32) {
        consumer.accept(value);
    }

    #[test]
    fn test_with_box_consumer() {
        let box_consumer = BoxConsumer::new(|x: &i32| {
            std::hint::black_box(x);
        });
        apply_consumer(&box_consumer, &5);
    }

    #[test]
    fn test_with_arc_consumer() {
        let arc_consumer = ArcConsumer::new(|x: &i32| {
            std::hint::black_box(x);
        });
        apply_consumer(&arc_consumer, &5);
    }

    #[test]
    fn test_with_rc_consumer() {
        let rc_consumer = RcConsumer::new(|x: &i32| {
            std::hint::black_box(x);
        });
        apply_consumer(&rc_consumer, &5);
    }

    #[test]
    fn test_with_closure() {
        let closure = |x: &i32| {
            std::hint::black_box(x);
        };
        apply_consumer(&closure, &5);
    }
}

// ============================================================================
// Name Tests - Testing name() and set_name() methods
// ============================================================================

#[cfg(test)]
mod name_tests {
    use super::{
        ArcConsumer,
        BoxConsumer,
        Consumer,
        RcConsumer,
    };

    #[test]
    fn test_box_consumer_name() {
        let mut consumer = BoxConsumer::new(|x: &i32| {
            std::hint::black_box(x);
        });
        assert_eq!(consumer.name(), None);

        consumer.set_name("printer");
        assert_eq!(consumer.name(), Some("printer"));
    }

    #[test]
    fn test_arc_consumer_name() {
        let mut consumer = ArcConsumer::new(|x: &i32| {
            std::hint::black_box(x);
        });
        assert_eq!(consumer.name(), None);

        consumer.set_name("printer");
        assert_eq!(consumer.name(), Some("printer"));
    }

    #[test]
    fn test_rc_consumer_name() {
        let mut consumer = RcConsumer::new(|x: &i32| {
            std::hint::black_box(x);
        });
        assert_eq!(consumer.name(), None);

        consumer.set_name("printer");
        assert_eq!(consumer.name(), Some("printer"));
    }

    #[test]
    fn test_box_consumer_name_with_accept() {
        let mut consumer = BoxConsumer::new(|_x: &i32| {});
        consumer.set_name("test_consumer");
        assert_eq!(consumer.name(), Some("test_consumer"));
        consumer.accept(&1);
        assert_eq!(consumer.name(), Some("test_consumer"));
    }

    #[test]
    fn test_arc_consumer_name_with_accept() {
        let mut consumer = ArcConsumer::new(|_x: &i32| {});
        consumer.set_name("test_consumer");
        assert_eq!(consumer.name(), Some("test_consumer"));
        consumer.accept(&1);
        assert_eq!(consumer.name(), Some("test_consumer"));
    }

    #[test]
    fn test_rc_consumer_name_with_accept() {
        let mut consumer = RcConsumer::new(|_x: &i32| {});
        consumer.set_name("test_consumer");
        assert_eq!(consumer.name(), Some("test_consumer"));
        consumer.accept(&1);
        assert_eq!(consumer.name(), Some("test_consumer"));
    }







}

// ============================================================================
// Display and Debug Tests
// ============================================================================

#[cfg(test)]
mod display_debug_tests {
    use super::{
        ArcConsumer,
        BoxConsumer,
        RcConsumer,
    };

    #[test]
    fn test_box_consumer_debug() {
        let consumer = BoxConsumer::new(|_x: &i32| {});
        let debug_str = format!("{:?}", consumer);
        assert!(debug_str.contains("BoxConsumer"));
        assert!(debug_str.contains("name"));
        assert!(debug_str.contains("function"));
    }

    #[test]
    fn test_box_consumer_display_without_name() {
        let consumer = BoxConsumer::new(|_x: &i32| {});
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "BoxConsumer");
    }

    #[test]
    fn test_box_consumer_display_with_name() {
        let mut consumer = BoxConsumer::new(|_x: &i32| {});
        consumer.set_name("test_consumer");
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "BoxConsumer(test_consumer)");
    }

    #[test]
    fn test_arc_consumer_debug() {
        let consumer = ArcConsumer::new(|_x: &i32| {});
        let debug_str = format!("{:?}", consumer);
        assert!(debug_str.contains("ArcConsumer"));
        assert!(debug_str.contains("name"));
        assert!(debug_str.contains("function"));
    }

    #[test]
    fn test_arc_consumer_display_without_name() {
        let consumer = ArcConsumer::new(|_x: &i32| {});
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "ArcConsumer");
    }

    #[test]
    fn test_arc_consumer_display_with_name() {
        let mut consumer = ArcConsumer::new(|_x: &i32| {});
        consumer.set_name("test_consumer");
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "ArcConsumer(test_consumer)");
    }

    #[test]
    fn test_rc_consumer_debug() {
        let consumer = RcConsumer::new(|_x: &i32| {});
        let debug_str = format!("{:?}", consumer);
        assert!(debug_str.contains("RcConsumer"));
        assert!(debug_str.contains("name"));
        assert!(debug_str.contains("function"));
    }

    #[test]
    fn test_rc_consumer_display_without_name() {
        let consumer = RcConsumer::new(|_x: &i32| {});
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "RcConsumer");
    }

    #[test]
    fn test_rc_consumer_display_with_name() {
        let mut consumer = RcConsumer::new(|_x: &i32| {});
        consumer.set_name("test_consumer");
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "RcConsumer(test_consumer)");
    }
}

#[cfg(test)]
mod custom_struct_tests {
    use super::Consumer;
    use std::sync::Arc;
    use std::sync::atomic::{
        AtomicUsize,
        Ordering,
    };

    pub struct MyConsumer {
        counter: Arc<AtomicUsize>,
    }

    impl MyConsumer {
        pub fn new(counter: Arc<AtomicUsize>) -> Self {
            Self { counter }
        }
    }

    impl Consumer<i32> for MyConsumer {
        fn accept(&self, _value: &i32) {
            self.counter.fetch_add(1, Ordering::SeqCst);
        }
    }


    impl Clone for MyConsumer {
        fn clone(&self) -> Self {
            Self {
                counter: self.counter.clone(),
            }
        }
    }

}

// ============================================================================
// to_xxx Methods Tests - Testing non-consuming conversion methods
// ============================================================================

#[cfg(test)]
mod to_xxx_methods_tests {



    // BoxConsumer cannot implement Clone because it uses Box<dyn Fn>
    // So it cannot have to_box, to_rc, to_fn methods
    // It can only have into_xxx methods














}

// ============================================================================
// to_once Tests - Testing Consumer trait default to_once implementation
// ============================================================================

#[cfg(test)]
mod to_once_tests {







}

// ============================================================================
// Conditional Consumer Tests
// ============================================================================

#[cfg(test)]
mod box_conditional_consumer_tests {
    use super::{
        Arc,
        BoxConsumer,
        Consumer,
    };
    use std::sync::Mutex;

    #[test]
    fn test_box_conditional_and_then() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();

        let consumer = BoxConsumer::new(move |x: &i32| {
            l1.lock().expect("mutex should not be poisoned").push(*x);
        });

        let conditional = consumer.when(|x: &i32| *x > 0);
        let chained = conditional.and_then(move |x: &i32| {
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
    fn test_box_conditional_or_else() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();

        let consumer = BoxConsumer::new(move |x: &i32| {
            l1.lock().expect("mutex should not be poisoned").push(*x);
        });

        let conditional =
            consumer.when(|x: &i32| *x > 0).or_else(move |x: &i32| {
                l2.lock()
                    .expect("mutex should not be poisoned")
                    .push(*x * 10);
            });

        conditional.accept(&5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);

        conditional.accept(&-5);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![5, -50]
        );
    }

    #[test]
    fn test_box_conditional_accept() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();

        let consumer = BoxConsumer::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });

        let conditional = consumer.when(|x: &i32| *x > 0);

        conditional.accept(&5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);

        conditional.accept(&-5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);
    }



}

#[cfg(test)]
mod arc_conditional_consumer_tests {
    use super::{
        Arc,
        ArcConsumer,
        Consumer,
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

        let consumer = ArcConsumer::new(move |_x: &i32| {
            c1.fetch_add(1, Ordering::SeqCst);
        });

        let conditional = consumer.when(|x: &i32| *x > 0);
        let chained = conditional.and_then(move |_x: &i32| {
            c2.fetch_add(10, Ordering::SeqCst);
        });

        chained.accept(&5);
        assert_eq!(counter.load(Ordering::SeqCst), 11);

        chained.accept(&-5);
        assert_eq!(counter.load(Ordering::SeqCst), 21);
    }

    #[test]
    fn test_arc_conditional_or_else() {
        let counter = Arc::new(AtomicUsize::new(0));
        let c1 = counter.clone();
        let c2 = counter.clone();

        let consumer = ArcConsumer::new(move |_x: &i32| {
            c1.fetch_add(1, Ordering::SeqCst);
        });

        let conditional =
            consumer.when(|x: &i32| *x > 0).or_else(move |_x: &i32| {
                c2.fetch_add(100, Ordering::SeqCst);
            });

        conditional.accept(&5);
        assert_eq!(counter.load(Ordering::SeqCst), 1);

        conditional.accept(&-5);
        assert_eq!(counter.load(Ordering::SeqCst), 101);
    }

    #[test]
    fn test_arc_conditional_accept() {
        let counter = Arc::new(AtomicUsize::new(0));
        let c = counter.clone();

        let consumer = ArcConsumer::new(move |_x: &i32| {
            c.fetch_add(1, Ordering::SeqCst);
        });

        let conditional = consumer.when(|x: &i32| *x > 0);

        conditional.accept(&5);
        assert_eq!(counter.load(Ordering::SeqCst), 1);

        conditional.accept(&-5);
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }








}

#[cfg(test)]
mod rc_conditional_consumer_tests {
    use super::{
        Consumer,
        Rc,
        RcConsumer,
    };
    use std::cell::RefCell;

    #[test]
    fn test_rc_conditional_and_then() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();

        let consumer = RcConsumer::new(move |x: &i32| {
            l1.borrow_mut().push(*x);
        });

        let conditional = consumer.when(|x: &i32| *x > 0);
        let chained = conditional.and_then(move |x: &i32| {
            l2.borrow_mut().push(*x * 2);
        });

        chained.accept(&5);
        assert_eq!(*log.borrow(), vec![5, 10]);

        chained.accept(&-5);
        assert_eq!(*log.borrow(), vec![5, 10, -10]);
    }

    #[test]
    fn test_rc_conditional_or_else() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();

        let consumer = RcConsumer::new(move |x: &i32| {
            l1.borrow_mut().push(*x);
        });

        let conditional =
            consumer.when(|x: &i32| *x > 0).or_else(move |x: &i32| {
                l2.borrow_mut().push(*x * 10);
            });

        conditional.accept(&5);
        assert_eq!(*log.borrow(), vec![5]);

        conditional.accept(&-5);
        assert_eq!(*log.borrow(), vec![5, -50]);
    }

    #[test]
    fn test_rc_conditional_accept() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let l = log.clone();

        let consumer = RcConsumer::new(move |x: &i32| {
            l.borrow_mut().push(*x);
        });

        let conditional = consumer.when(|x: &i32| *x > 0);

        conditional.accept(&5);
        assert_eq!(*log.borrow(), vec![5]);

        conditional.accept(&-5);
        assert_eq!(*log.borrow(), vec![5]);
    }






}
