// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
// qubit-style: allow explicit-imports
///
/// # ConsumerOnce Tests
///
/// Unit tests for the ConsumerOnce trait and its implementations.
use qubit_function::{
    BoxConsumerOnce,
    ConsumerOnce,
    FnConsumerOnceOps,
};
use std::sync::{
    Arc,
    Mutex,
};

// ============================================================================
// BoxConsumerOnce Tests
// ============================================================================

#[cfg(test)]
mod box_consumer_once_tests {
    use super::{
        Arc,
        BoxConsumerOnce,
        ConsumerOnce,
        Mutex,
    };

    #[test]
    fn test_new() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let consumer = BoxConsumerOnce::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });
        consumer.accept(&5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);
    }

    #[test]
    fn test_accept() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let consumer = BoxConsumerOnce::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x * 2);
        });
        consumer.accept(&5);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![10]
        );
    }

    #[test]
    fn test_and_then() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();
        let chained = BoxConsumerOnce::new(move |x: &i32| {
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
    fn test_and_then_multiple() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();
        let l3 = log.clone();
        let chained = BoxConsumerOnce::new(move |x: &i32| {
            l1.lock()
                .expect("mutex should not be poisoned")
                .push(*x * 2);
        })
        .and_then(move |x: &i32| {
            l2.lock()
                .expect("mutex should not be poisoned")
                .push(*x + 10);
        })
        .and_then(move |x: &i32| {
            l3.lock()
                .expect("mutex should not be poisoned")
                .push(*x - 1);
        });
        chained.accept(&5);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![10, 15, 4]
        );
    }

    #[test]
    fn test_noop() {
        let noop = BoxConsumerOnce::<i32>::noop();
        noop.accept(&42);
        // Should not panic
    }

    #[test]
    fn test_new_with_name() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let consumer =
            BoxConsumerOnce::new_with_name("test_consumer", move |x: &i32| {
                l.lock().expect("mutex should not be poisoned").push(*x);
            });
        assert_eq!(consumer.name(), Some("test_consumer"));
        consumer.accept(&5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);
    }

    // print and print_with methods have been removed

    #[test]
    fn test_if_then_true() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let consumer = BoxConsumerOnce::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x + 1);
        });
        let conditional = consumer.when(|x: &i32| *x > 0);
        conditional.accept(&5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![6]);
    }

    #[test]
    fn test_if_then_false() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let consumer = BoxConsumerOnce::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x + 1);
        });
        let conditional = consumer.when(|x: &i32| *x > 0);
        conditional.accept(&-5);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn test_if_then_else_true() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();
        let consumer = BoxConsumerOnce::new(move |x: &i32| {
            l1.lock()
                .expect("mutex should not be poisoned")
                .push(*x + 1);
        });
        let conditional =
            consumer.when(|x: &i32| *x > 0).or_else(move |x: &i32| {
                l2.lock()
                    .expect("mutex should not be poisoned")
                    .push(*x - 1);
            });
        conditional.accept(&5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![6]);
    }

    #[test]
    fn test_if_then_else_false() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();
        let consumer = BoxConsumerOnce::new(move |x: &i32| {
            l1.lock()
                .expect("mutex should not be poisoned")
                .push(*x + 1);
        });
        let conditional =
            consumer.when(|x: &i32| *x > 0).or_else(move |x: &i32| {
                l2.lock()
                    .expect("mutex should not be poisoned")
                    .push(*x - 1);
            });
        conditional.accept(&-5);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![-6]
        );
    }
}

// ============================================================================
// Closure Tests
// ============================================================================

#[cfg(test)]
mod closure_tests {
    use super::{
        Arc,
        ConsumerOnce,
        FnConsumerOnceOps,
        Mutex,
    };

    #[test]
    fn test_closure_accept() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let closure = move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x * 2);
        };
        closure.accept(&5);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![10]
        );
    }

    #[test]
    fn test_closure_and_then() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();
        let chained = (move |x: &i32| {
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
    fn test_closure_multi_step_chaining() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();
        let l3 = log.clone();
        let chained = (move |x: &i32| {
            l1.lock()
                .expect("mutex should not be poisoned")
                .push(*x * 2);
        })
        .and_then(move |x: &i32| {
            l2.lock()
                .expect("mutex should not be poisoned")
                .push(*x + 10);
        })
        .and_then(move |x: &i32| {
            l3.lock()
                .expect("mutex should not be poisoned")
                .push(*x / 2);
        });
        chained.accept(&5);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![10, 15, 2]
        );
    }
}

#[cfg(test)]
mod debug_display_tests {
    use super::{
        Arc,
        BoxConsumerOnce,
        ConsumerOnce,
        Mutex,
    };

    #[test]
    fn test_debug() {
        let consumer = BoxConsumerOnce::new(|_x: &i32| {});
        let debug_str = format!("{:?}", consumer);
        assert!(debug_str.contains("BoxConsumerOnce"));
    }

    #[test]
    fn test_display() {
        let consumer = BoxConsumerOnce::new(|_x: &i32| {});
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "BoxConsumerOnce");
    }

    #[test]
    fn test_display_with_name() {
        let mut consumer = BoxConsumerOnce::new(|_x: &i32| {});
        consumer.set_name("my_consumer");
        let display_str = format!("{}", consumer);
        assert_eq!(display_str, "BoxConsumerOnce(my_consumer)");
    }

    #[test]
    fn test_name() {
        let mut consumer = BoxConsumerOnce::new(|_x: &i32| {});
        assert_eq!(consumer.name(), None);
        consumer.set_name("test");
        assert_eq!(consumer.name(), Some("test"));
    }

    #[test]
    fn test_conditional_and_then() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();
        let consumer = BoxConsumerOnce::new(move |x: &i32| {
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
    }
}

// ============================================================================
// Custom ConsumerOnce Tests - Testing Default into_xxx() Implementation
// ============================================================================

#[cfg(test)]
mod custom_consumer_once_tests {
    use super::{
        Arc,
        ConsumerOnce,
        Mutex,
    };

    /// Custom consumer that increments a counter
    struct CustomConsumer {
        log: Arc<Mutex<Vec<i32>>>,
        multiplier: i32,
    }

    impl CustomConsumer {
        fn new(log: Arc<Mutex<Vec<i32>>>, multiplier: i32) -> Self {
            Self { log, multiplier }
        }
    }

    impl ConsumerOnce<i32> for CustomConsumer {
        fn accept(self, value: &i32) {
            self.log
                .lock()
                .expect("mutex should not be poisoned")
                .push(*value * self.multiplier);
        }

        // Note: We do not override into_box() and into_fn(),
        // but use the default implementations provided by the trait
    }

    #[test]
    fn test_custom_consumer_accept() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let consumer = CustomConsumer::new(log.clone(), 3);
        consumer.accept(&5);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![15]
        );
    }

    #[test]
    fn test_custom_consumer_with_generic_function() {
        let log = Arc::new(Mutex::new(Vec::new()));

        fn process_with_consumer<C>(consumer: C, value: &i32)
        where
            C: ConsumerOnce<i32>,
        {
            consumer.accept(value);
        }

        let consumer = CustomConsumer::new(log.clone(), 5);
        process_with_consumer(consumer, &6);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![30]
        );
    }

    /// Custom consumer with String type
    struct StringLogger {
        log: Arc<Mutex<Vec<String>>>,
        prefix: String,
    }

    impl StringLogger {
        fn new(
            log: Arc<Mutex<Vec<String>>>,
            prefix: impl Into<String>,
        ) -> Self {
            Self {
                log,
                prefix: prefix.into(),
            }
        }
    }

    impl ConsumerOnce<String> for StringLogger {
        fn accept(self, value: &String) {
            self.log
                .lock()
                .expect("mutex should not be poisoned")
                .push(format!("{}{}", self.prefix, value));
        }
    }

    /// Custom consumer that counts how many times it was supposed to be called
    struct CountingConsumer {
        counter: Arc<Mutex<usize>>,
        value_log: Arc<Mutex<Vec<i32>>>,
    }

    impl CountingConsumer {
        fn new(
            counter: Arc<Mutex<usize>>,
            value_log: Arc<Mutex<Vec<i32>>>,
        ) -> Self {
            Self { counter, value_log }
        }
    }

    impl ConsumerOnce<i32> for CountingConsumer {
        fn accept(self, value: &i32) {
            *self.counter.lock().expect("mutex should not be poisoned") += 1;
            self.value_log
                .lock()
                .expect("mutex should not be poisoned")
                .push(*value);
        }
    }
}

// ============================================================================
// BoxConditionalConsumerOnce Focused Tests
// ============================================================================

#[cfg(test)]
mod box_conditional_consumer_once_tests {
    use super::{
        Arc,
        BoxConsumerOnce,
        ConsumerOnce,
        Mutex,
    };

    // Tests for accept() method

    #[test]
    fn test_accept_predicate_false() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let consumer = BoxConsumerOnce::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });
        let conditional = consumer.when(|x: &i32| *x > 0);
        conditional.accept(&-5);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn test_accept_predicate_true() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let consumer = BoxConsumerOnce::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });
        let conditional = consumer.when(|x: &i32| *x > 0);
        conditional.accept(&5);
        assert_eq!(*log.lock().expect("mutex should not be poisoned"), vec![5]);
    }

    #[test]
    fn test_accept_predicate_boundary() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let consumer = BoxConsumerOnce::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });
        let conditional = consumer.when(|x: &i32| *x > 0);
        // Test boundary case - predicate checks > 0, so 0 should be false
        conditional.accept(&0);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            Vec::<i32>::new()
        );
    }

    // Tests for into_box() method

    // Tests for into_fn() method

    // Additional tests for into_box() and into_fn() with complex predicates

    // Additional comprehensive branch coverage tests for accept() method

    #[test]
    fn test_accept_with_always_true_predicate() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let consumer = BoxConsumerOnce::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });
        let conditional = consumer.when(|_: &i32| true);
        conditional.accept(&42);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![42]
        );
    }

    #[test]
    fn test_accept_with_always_false_predicate() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let consumer = BoxConsumerOnce::new(move |x: &i32| {
            l.lock().expect("mutex should not be poisoned").push(*x);
        });
        let conditional = consumer.when(|_: &i32| false);
        conditional.accept(&42);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn test_accept_with_complex_predicate_logic() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let consumer = BoxConsumerOnce::new(move |x: &i32| {
            l.lock()
                .expect("mutex should not be poisoned")
                .push(*x * 10);
        });
        // Complex predicate: value is positive and even
        let conditional = consumer.when(|x: &i32| *x > 0 && *x % 2 == 0);
        conditional.accept(&4);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![40]
        );
    }

    #[test]
    fn test_accept_with_complex_predicate_logic_fails() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let consumer = BoxConsumerOnce::new(move |x: &i32| {
            l.lock()
                .expect("mutex should not be poisoned")
                .push(*x * 10);
        });
        // Complex predicate: value is positive and even
        let conditional = consumer.when(|x: &i32| *x > 0 && *x % 2 == 0);
        // Test with odd number - fails the even check
        conditional.accept(&3);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn test_accept_with_complex_predicate_logic_fails_negative() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l = log.clone();
        let consumer = BoxConsumerOnce::new(move |x: &i32| {
            l.lock()
                .expect("mutex should not be poisoned")
                .push(*x * 10);
        });
        // Complex predicate: value is positive and even
        let conditional = consumer.when(|x: &i32| *x > 0 && *x % 2 == 0);
        // Test with negative even number - fails the positive check
        conditional.accept(&-4);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            Vec::<i32>::new()
        );
    }

    // Tests for and_then() method with conditional consumer

    #[test]
    fn test_and_then_predicate_true() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();
        let conditional = BoxConsumerOnce::new(move |x: &i32| {
            l1.lock().expect("mutex should not be poisoned").push(*x);
        })
        .when(|x: &i32| *x > 0);

        let chained = conditional.and_then(move |x: &i32| {
            l2.lock()
                .expect("mutex should not be poisoned")
                .push(*x * 2);
        });

        chained.accept(&5);
        // First consumer executes (5), second consumer executes (10)
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![5, 10]
        );
    }

    #[test]
    fn test_and_then_predicate_false() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();
        let conditional = BoxConsumerOnce::new(move |x: &i32| {
            l1.lock().expect("mutex should not be poisoned").push(*x);
        })
        .when(|x: &i32| *x > 0);

        let chained = conditional.and_then(move |x: &i32| {
            l2.lock()
                .expect("mutex should not be poisoned")
                .push(*x * 2);
        });

        chained.accept(&-5);
        // First consumer doesn't execute (predicate false), second consumer
        // still executes (-10)
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![-10]
        );
    }

    #[test]
    fn test_and_then_multiple_conditionals() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();
        let l3 = log.clone();

        let conditional1 = BoxConsumerOnce::new(move |x: &i32| {
            l1.lock().expect("mutex should not be poisoned").push(*x);
        })
        .when(|x: &i32| *x > 0);

        let conditional2 = BoxConsumerOnce::new(move |x: &i32| {
            l2.lock()
                .expect("mutex should not be poisoned")
                .push(*x * 2);
        })
        .when(|x: &i32| *x % 2 == 0);

        let chained =
            conditional1
                .and_then(conditional2)
                .and_then(move |x: &i32| {
                    l3.lock()
                        .expect("mutex should not be poisoned")
                        .push(*x + 100);
                });

        // Test with 6: positive (first passes), even (second passes), third
        // always executes
        chained.accept(&6);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![6, 12, 106]
        );
    }
}
// ============================================================================
// to_box() and to_fn() Tests - Closure Implementation
// ============================================================================

#[cfg(test)]
mod closure_to_xxx_methods_tests {
    use super::{
        Arc,
        ConsumerOnce,
        FnConsumerOnceOps,
        Mutex,
    };

    /// Test closure with and_then() through FnConsumerOnceOps
    #[test]
    fn test_closure_fnonce_ops_and_then() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();

        let chained = (move |x: &i32| {
            l1.lock()
                .expect("mutex should not be poisoned")
                .push(*x * 2);
        })
        .and_then(move |x: &i32| {
            l2.lock()
                .expect("mutex should not be poisoned")
                .push(*x + 50);
        });

        chained.accept(&5);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![10, 55]
        );
    }

    /// Test closure with multiple and_then() chains through FnConsumerOnceOps
    #[test]
    fn test_closure_fnonce_ops_multiple_and_then() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();
        let l3 = log.clone();

        let chained = (move |x: &i32| {
            l1.lock()
                .expect("mutex should not be poisoned")
                .push(*x * 2);
        })
        .and_then(move |x: &i32| {
            l2.lock()
                .expect("mutex should not be poisoned")
                .push(*x + 10);
        })
        .and_then(move |x: &i32| {
            l3.lock()
                .expect("mutex should not be poisoned")
                .push(*x * 3);
        });

        chained.accept(&5);
        // First: 5 * 2 = 10
        // Second: 5 + 10 = 15 (operates on original value, not on result of
        // first) Third: 5 * 3 = 15 (operates on original value, not on
        // result of second)
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![10, 15, 15]
        );
    }

    /// Test chain of closures with and_then() followed by conditional
    #[test]
    fn test_closure_chain_then_conditional() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let l1 = log.clone();
        let l2 = log.clone();

        let chained = (move |x: &i32| {
            l1.lock()
                .expect("mutex should not be poisoned")
                .push(*x * 2);
        })
        .and_then(move |x: &i32| {
            l2.lock()
                .expect("mutex should not be poisoned")
                .push(*x + 10);
        });

        let boxed = chained;
        let conditional = boxed.when(|x: &i32| *x < 15);
        conditional.accept(&5);
        assert_eq!(
            *log.lock().expect("mutex should not be poisoned"),
            vec![10, 15]
        ); // Both execute because condition is true (5 < 15)
    }
}
