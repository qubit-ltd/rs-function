// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

// qubit-style: allow explicit-imports
//! Unit tests for StatefulSupplier types

use qubit_function::{
    ArcStatefulSupplier,
    BoxStatefulSupplier,
    FnStatefulSupplierOps,
    RcStatefulSupplier,
    StatefulSupplier,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{
    Arc,
    Mutex,
};
use std::thread;

// ==========================================================================
// StatefulSupplier Trait Tests (for closures)
// ==========================================================================

#[cfg(test)]
mod test_stateful_supplier_trait {
    use super::{
        BoxStatefulSupplier,
        StatefulSupplier,
    };

    #[test]
    fn test_closure_stateful() {
        let mut counter = 0;
        let mut boxed = BoxStatefulSupplier::new(move || {
            counter += 1;
            counter
        });
        assert_eq!(boxed.get(), 1);
        assert_eq!(boxed.get(), 2);
        assert_eq!(boxed.get(), 3);
    }

    #[test]
    fn test_closure_get() {
        // Test the get method in impl<T, F> StatefulSupplier<T> for F
        let mut closure = || 42;
        assert_eq!(closure.get(), 42);
        assert_eq!(closure.get(), 42);
    }

    #[test]
    fn test_closure_get_fn() {
        // Test an Fn closure that returns the captured value without mutation.
        let value = 42;
        let mut closure = move || value;
        assert_eq!(closure.get(), 42);
        assert_eq!(closure.get(), 42);
        assert_eq!(closure.get(), 42);
    }
}

// ==========================================================================
// BoxStatefulSupplier Tests
// ==========================================================================

#[cfg(test)]
mod test_box_stateful_supplier {
    use super::{
        BoxStatefulSupplier,
        StatefulSupplier,
    };

    mod test_new {
        use super::{
            BoxStatefulSupplier,
            StatefulSupplier,
        };

        #[test]
        fn test_creates_stateful_supplier() {
            let mut supplier = BoxStatefulSupplier::new(|| 42);
            assert_eq!(supplier.get(), 42);
        }

        #[test]
        fn test_with_i32() {
            let mut supplier = BoxStatefulSupplier::new(|| 42);
            assert_eq!(supplier.get(), 42);
        }

        #[test]
        fn test_with_string() {
            let mut supplier =
                BoxStatefulSupplier::new(|| String::from("hello"));
            assert_eq!(supplier.get(), "hello");
        }

        #[test]
        fn test_with_vec() {
            let mut supplier = BoxStatefulSupplier::new(|| vec![1, 2, 3]);
            assert_eq!(supplier.get(), vec![1, 2, 3]);
        }

        #[test]
        fn test_with_bool() {
            let mut supplier = BoxStatefulSupplier::new(|| true);
            assert!(supplier.get());
        }
    }

    mod test_constant {
        use super::{
            BoxStatefulSupplier,
            StatefulSupplier,
        };

        #[test]
        fn test_returns_same_value() {
            let mut constant = BoxStatefulSupplier::constant(42);
            assert_eq!(constant.get(), 42);
            assert_eq!(constant.get(), 42);
            assert_eq!(constant.get(), 42);
        }

        #[test]
        fn test_with_string() {
            let mut constant =
                BoxStatefulSupplier::constant(String::from("hello"));
            assert_eq!(constant.get(), "hello");
            assert_eq!(constant.get(), "hello");
        }
    }

    mod test_get {
        use super::{
            BoxStatefulSupplier,
            StatefulSupplier,
        };

        #[test]
        fn test_can_be_called_multiple_times() {
            let mut supplier = BoxStatefulSupplier::new(|| 42);
            assert_eq!(supplier.get(), 42);
            assert_eq!(supplier.get(), 42);
            assert_eq!(supplier.get(), 42);
        }

        #[test]
        fn test_stateful_counter() {
            let mut counter = 0;
            let mut supplier = BoxStatefulSupplier::new(move || {
                counter += 1;
                counter
            });

            assert_eq!(supplier.get(), 1);
            assert_eq!(supplier.get(), 2);
            assert_eq!(supplier.get(), 3);
        }
    }

    mod test_map {
        use super::{
            BoxStatefulSupplier,
            StatefulSupplier,
        };

        #[test]
        fn test_transforms_value() {
            let mut mapped = BoxStatefulSupplier::new(|| 10).map(|x| x * 2);
            assert_eq!(mapped.get(), 20);
        }

        #[test]
        fn test_multiple_chains() {
            let mut chained =
                BoxStatefulSupplier::new(|| 5).map(|x| x * 2).map(|x| x + 5);
            assert_eq!(chained.get(), 15);
        }

        #[test]
        fn test_type_conversion() {
            let mut converted =
                BoxStatefulSupplier::new(|| 42).map(|x: i32| x.to_string());
            assert_eq!(converted.get(), "42");
        }

        #[test]
        fn test_with_stateful_stateful_supplier() {
            let mut counter = 0;
            let mut mapped = BoxStatefulSupplier::new(move || {
                counter += 1;
                counter
            })
            .map(|x| x * 10);

            assert_eq!(mapped.get(), 10);
            assert_eq!(mapped.get(), 20);
            assert_eq!(mapped.get(), 30);
        }

        // Test with function pointer
        #[test]
        fn test_with_function_pointer() {
            fn double(x: i32) -> i32 {
                x * 2
            }
            let mut mapped = BoxStatefulSupplier::new(|| 10).map(double);
            assert_eq!(mapped.get(), 20);
        }
    }

    mod test_filter {
        use super::{
            BoxStatefulSupplier,
            StatefulSupplier,
        };

        #[test]
        fn test_filters_even_numbers() {
            let mut counter = 0;
            let mut filtered = BoxStatefulSupplier::new(move || {
                counter += 1;
                counter
            })
            .filter(|x: &i32| x % 2 == 0);

            assert_eq!(filtered.get(), None); // 1 is odd
            assert_eq!(filtered.get(), Some(2)); // 2 is even
            assert_eq!(filtered.get(), None); // 3 is odd
            assert_eq!(filtered.get(), Some(4)); // 4 is even
        }

        #[test]
        fn test_with_constant_stateful_supplier() {
            let mut filtered =
                BoxStatefulSupplier::constant(5).filter(|x: &i32| x % 2 == 0);
            assert_eq!(filtered.get(), None); // 5 is odd
            assert_eq!(filtered.get(), None);
        }
    }

    mod test_zip {
        use super::{
            BoxStatefulSupplier,
            StatefulSupplier,
        };

        #[test]
        fn test_combines_two_stateful_suppliers() {
            let first = BoxStatefulSupplier::new(|| 42);
            let second = BoxStatefulSupplier::new(|| "hello");
            let mut zipped = first.zip(second);

            assert_eq!(zipped.get(), (42, "hello"));
        }

        #[test]
        fn test_with_stateful_stateful_suppliers() {
            let mut counter1 = 0;
            let first = BoxStatefulSupplier::new(move || {
                counter1 += 1;
                counter1
            });
            let mut counter2 = 0;
            let second = BoxStatefulSupplier::new(move || {
                counter2 += 10;
                counter2
            });
            let mut zipped = first.zip(second);

            assert_eq!(zipped.get(), (1, 10));
            assert_eq!(zipped.get(), (2, 20));
        }
    }

    mod test_memoize {
        use super::{
            BoxStatefulSupplier,
            StatefulSupplier,
        };

        #[test]
        fn test_caches_first_value() {
            // Use a shared counter to verify memoization
            use std::cell::Cell;
            let call_count = Cell::new(0);
            let mut memoized = BoxStatefulSupplier::new(move || {
                call_count.set(call_count.get() + 1);
                42
            })
            .memoize();

            assert_eq!(memoized.get(), 42);
            assert_eq!(memoized.get(), 42);
            assert_eq!(memoized.get(), 42);
        }

        #[test]
        fn test_with_stateful_stateful_supplier() {
            let mut counter = 0;
            let mut memoized = BoxStatefulSupplier::new(move || {
                counter += 1;
                counter
            })
            .memoize();

            assert_eq!(memoized.get(), 1); // First call
            assert_eq!(memoized.get(), 1); // Cached
            assert_eq!(memoized.get(), 1); // Cached
        }
    }
}

// ==========================================================================
// ArcStatefulSupplier Tests
// ==========================================================================

#[cfg(test)]
mod test_arc_stateful_supplier {
    use super::{
        Arc,
        ArcStatefulSupplier,
        Mutex,
        StatefulSupplier,
        thread,
    };

    mod test_new {
        use super::{
            ArcStatefulSupplier,
            StatefulSupplier,
        };

        #[test]
        fn test_creates_stateful_supplier() {
            let supplier = ArcStatefulSupplier::new(|| 42);
            let mut s = supplier;
            assert_eq!(s.get(), 42);
        }

        #[test]
        fn test_with_i32() {
            let supplier = ArcStatefulSupplier::new(|| 42);
            let mut s = supplier;
            assert_eq!(s.get(), 42);
        }

        #[test]
        fn test_with_string() {
            let supplier = ArcStatefulSupplier::new(|| String::from("hello"));
            let mut s = supplier;
            assert_eq!(s.get(), "hello");
        }
    }

    mod test_constant {
        use super::{
            ArcStatefulSupplier,
            StatefulSupplier,
        };

        #[test]
        fn test_returns_same_value() {
            let constant = ArcStatefulSupplier::constant(42);
            let mut s = constant;
            assert_eq!(s.get(), 42);
            assert_eq!(s.get(), 42);
        }
    }

    mod test_get {
        use super::{
            Arc,
            ArcStatefulSupplier,
            Mutex,
            StatefulSupplier,
        };

        #[test]
        fn test_can_be_called_multiple_times() {
            let supplier = ArcStatefulSupplier::new(|| 42);
            let mut s = supplier;
            assert_eq!(s.get(), 42);
            assert_eq!(s.get(), 42);
        }

        #[test]
        fn test_stateful_counter() {
            let counter = Arc::new(Mutex::new(0));
            let counter_clone = Arc::clone(&counter);
            let supplier = ArcStatefulSupplier::new(move || {
                let mut c =
                    counter_clone.lock().expect("mutex should not be poisoned");
                *c += 1;
                *c
            });

            let mut s = supplier;
            assert_eq!(s.get(), 1);
            assert_eq!(s.get(), 2);
            assert_eq!(s.get(), 3);
        }
    }

    mod test_clone {
        use super::{
            Arc,
            ArcStatefulSupplier,
            Mutex,
            StatefulSupplier,
        };

        #[test]
        fn test_can_be_cloned() {
            let supplier = ArcStatefulSupplier::new(|| 42);
            let clone1 = supplier.clone();
            let clone2 = supplier.clone();

            let mut s1 = clone1;
            let mut s2 = clone2;
            assert_eq!(s1.get(), 42);
            assert_eq!(s2.get(), 42);
        }

        #[test]
        fn test_clones_share_state() {
            let counter = Arc::new(Mutex::new(0));
            let counter_clone = Arc::clone(&counter);
            let supplier = ArcStatefulSupplier::new(move || {
                let mut c =
                    counter_clone.lock().expect("mutex should not be poisoned");
                *c += 1;
                *c
            });

            let mut s1 = supplier.clone();
            let mut s2 = supplier.clone();

            assert_eq!(s1.get(), 1);
            assert_eq!(s2.get(), 2);
        }
    }

    mod test_map {
        use super::{
            Arc,
            ArcStatefulSupplier,
            Mutex,
            StatefulSupplier,
            thread,
        };

        #[test]
        fn test_transforms_value() {
            let source = ArcStatefulSupplier::new(|| 10);
            let mapped = source.map(|x| x * 2);
            let mut s = mapped;
            assert_eq!(s.get(), 20);
        }

        #[test]
        fn test_original_remains_usable() {
            let source = ArcStatefulSupplier::new(|| 10);
            let _mapped = source.map(|x| x * 2);
            // source is still usable
            let mut s = source;
            assert_eq!(s.get(), 10);
        }

        #[test]
        fn test_multiple_maps_from_same_source() {
            let source = ArcStatefulSupplier::new(|| 10);
            let doubled = source.map(|x| x * 2);
            let tripled = source.map(|x| x * 3);

            let mut d = doubled;
            let mut t = tripled;
            assert_eq!(d.get(), 20);
            assert_eq!(t.get(), 30);
        }

        // Test with function pointer
        #[test]
        fn test_with_function_pointer() {
            fn triple(x: i32) -> i32 {
                x * 3
            }
            let source = ArcStatefulSupplier::new(|| 10);
            let mapped = source.map(triple);
            let mut s = mapped;
            assert_eq!(s.get(), 30);
        }

        // Test thread safety with mapper
        #[test]
        fn test_thread_safety_with_mapper() {
            let counter = Arc::new(Mutex::new(0));
            let counter_clone = Arc::clone(&counter);
            let source = ArcStatefulSupplier::new(move || {
                let mut c =
                    counter_clone.lock().expect("mutex should not be poisoned");
                *c += 1;
                *c
            });

            let mapped = source.map(|x| x * 10);
            let mut s1 = mapped.clone();
            let mut s2 = mapped.clone();

            let h1 = thread::spawn(move || s1.get());
            let h2 = thread::spawn(move || s2.get());

            let v1 = h1.join().expect("thread should not panic");
            let v2 = h2.join().expect("thread should not panic");

            // Both should get different values (10 and 20)
            assert!(v1 == 10 || v1 == 20);
            assert!(v2 == 10 || v2 == 20);
            assert_ne!(v1, v2);
        }
    }

    mod test_filter {
        use super::{
            Arc,
            ArcStatefulSupplier,
            Mutex,
            StatefulSupplier,
        };

        #[test]
        fn test_filters_even_numbers() {
            let counter = Arc::new(Mutex::new(0));
            let counter_clone = Arc::clone(&counter);
            let source = ArcStatefulSupplier::new(move || {
                let mut c =
                    counter_clone.lock().expect("mutex should not be poisoned");
                *c += 1;
                *c
            });
            let filtered = source.filter(|x: &i32| x % 2 == 0);

            let mut s = filtered;
            assert_eq!(s.get(), None); // 1 is odd
            assert_eq!(s.get(), Some(2)); // 2 is even
        }
    }

    mod test_zip {
        use super::{
            ArcStatefulSupplier,
            StatefulSupplier,
        };

        #[test]
        fn test_combines_two_stateful_suppliers() {
            let first = ArcStatefulSupplier::new(|| 42);
            let second = ArcStatefulSupplier::new(|| "hello");
            let zipped = first.zip(second.clone());

            let mut z = zipped;
            assert_eq!(z.get(), (42, "hello"));
        }

        #[test]
        fn test_originals_remain_usable() {
            let first = ArcStatefulSupplier::new(|| 42);
            let second = ArcStatefulSupplier::new(|| "hello");
            let _zipped = first.zip(second.clone());

            // Both originals still usable
            let mut f = first;
            let mut s = second;
            assert_eq!(f.get(), 42);
            assert_eq!(s.get(), "hello");
        }
    }

    mod test_memoize {
        use super::{
            Arc,
            ArcStatefulSupplier,
            Mutex,
            StatefulSupplier,
        };

        #[test]
        fn test_caches_first_value() {
            let call_count = Arc::new(Mutex::new(0));
            let call_count_clone = Arc::clone(&call_count);
            let source = ArcStatefulSupplier::new(move || {
                let mut c = call_count_clone
                    .lock()
                    .expect("mutex should not be poisoned");
                *c += 1;
                42
            });
            let memoized = source.memoize();

            let mut s = memoized;
            assert_eq!(s.get(), 42);
            assert_eq!(s.get(), 42);
            assert_eq!(
                *call_count.lock().expect("mutex should not be poisoned"),
                1
            );
        }
    }

    mod test_thread_safety {
        use super::{
            Arc,
            ArcStatefulSupplier,
            Mutex,
            StatefulSupplier,
            thread,
        };

        #[test]
        fn test_can_be_sent_across_threads() {
            let counter = Arc::new(Mutex::new(0));
            let counter_clone = Arc::clone(&counter);
            let supplier = ArcStatefulSupplier::new(move || {
                let mut c =
                    counter_clone.lock().expect("mutex should not be poisoned");
                *c += 1;
                *c
            });

            let mut s1 = supplier.clone();
            let mut s2 = supplier.clone();

            let h1 = thread::spawn(move || s1.get());
            let h2 = thread::spawn(move || s2.get());

            let v1 = h1.join().expect("thread should not panic");
            let v2 = h2.join().expect("thread should not panic");

            assert!(v1 != v2);
            assert_eq!(
                *counter.lock().expect("mutex should not be poisoned"),
                2
            );
        }
    }
}

// ==========================================================================
// RcStatefulSupplier Tests
// ==========================================================================

#[cfg(test)]
mod test_rc_stateful_supplier {
    use super::{
        Rc,
        RcStatefulSupplier,
        RefCell,
        StatefulSupplier,
    };

    mod test_new {
        use super::{
            RcStatefulSupplier,
            StatefulSupplier,
        };

        #[test]
        fn test_creates_stateful_supplier() {
            let supplier = RcStatefulSupplier::new(|| 42);
            let mut s = supplier;
            assert_eq!(s.get(), 42);
        }

        #[test]
        fn test_with_i32() {
            let supplier = RcStatefulSupplier::new(|| 42);
            let mut s = supplier;
            assert_eq!(s.get(), 42);
        }

        #[test]
        fn test_with_string() {
            let supplier = RcStatefulSupplier::new(|| String::from("hello"));
            let mut s = supplier;
            assert_eq!(s.get(), "hello");
        }
    }

    mod test_constant {
        use super::{
            RcStatefulSupplier,
            StatefulSupplier,
        };

        #[test]
        fn test_returns_same_value() {
            let constant = RcStatefulSupplier::constant(42);
            let mut s = constant;
            assert_eq!(s.get(), 42);
            assert_eq!(s.get(), 42);
        }
    }

    mod test_get {
        use super::{
            Rc,
            RcStatefulSupplier,
            RefCell,
            StatefulSupplier,
        };

        #[test]
        fn test_can_be_called_multiple_times() {
            let supplier = RcStatefulSupplier::new(|| 42);
            let mut s = supplier;
            assert_eq!(s.get(), 42);
            assert_eq!(s.get(), 42);
        }

        #[test]
        fn test_stateful_counter() {
            let counter = Rc::new(RefCell::new(0));
            let counter_clone = Rc::clone(&counter);
            let supplier = RcStatefulSupplier::new(move || {
                let mut c = counter_clone.borrow_mut();
                *c += 1;
                *c
            });

            let mut s = supplier;
            assert_eq!(s.get(), 1);
            assert_eq!(s.get(), 2);
            assert_eq!(s.get(), 3);
        }
    }

    mod test_clone {
        use super::{
            Rc,
            RcStatefulSupplier,
            RefCell,
            StatefulSupplier,
        };

        #[test]
        fn test_can_be_cloned() {
            let supplier = RcStatefulSupplier::new(|| 42);
            let clone1 = supplier.clone();
            let clone2 = supplier.clone();

            let mut s1 = clone1;
            let mut s2 = clone2;
            assert_eq!(s1.get(), 42);
            assert_eq!(s2.get(), 42);
        }

        #[test]
        fn test_clones_share_state() {
            let counter = Rc::new(RefCell::new(0));
            let counter_clone = Rc::clone(&counter);
            let supplier = RcStatefulSupplier::new(move || {
                let mut c = counter_clone.borrow_mut();
                *c += 1;
                *c
            });

            let mut s1 = supplier.clone();
            let mut s2 = supplier.clone();

            assert_eq!(s1.get(), 1);
            assert_eq!(s2.get(), 2);
        }
    }

    mod test_map {
        use super::{
            Rc,
            RcStatefulSupplier,
            RefCell,
            StatefulSupplier,
        };

        #[test]
        fn test_transforms_value() {
            let source = RcStatefulSupplier::new(|| 10);
            let mapped = source.map(|x| x * 2);
            let mut s = mapped;
            assert_eq!(s.get(), 20);
        }

        #[test]
        fn test_original_remains_usable() {
            let source = RcStatefulSupplier::new(|| 10);
            let _mapped = source.map(|x| x * 2);
            // source is still usable
            let mut s = source;
            assert_eq!(s.get(), 10);
        }

        #[test]
        fn test_multiple_maps_from_same_source() {
            let source = RcStatefulSupplier::new(|| 10);
            let doubled = source.map(|x| x * 2);
            let tripled = source.map(|x| x * 3);

            let mut d = doubled;
            let mut t = tripled;
            assert_eq!(d.get(), 20);
            assert_eq!(t.get(), 30);
        }

        // Test with function pointer
        #[test]
        fn test_with_function_pointer() {
            fn quadruple(x: i32) -> i32 {
                x * 4
            }
            let source = RcStatefulSupplier::new(|| 10);
            let mapped = source.map(quadruple);
            let mut s = mapped;
            assert_eq!(s.get(), 40);
        }

        // Test shared state with cloned StatefulSuppliers
        #[test]
        fn test_shared_state_with_mapper() {
            let counter = Rc::new(RefCell::new(0));
            let counter_clone = Rc::clone(&counter);
            let source = RcStatefulSupplier::new(move || {
                let mut c = counter_clone.borrow_mut();
                *c += 1;
                *c
            });

            let mapped = source.map(|x| x * 10);
            let mut s1 = mapped.clone();
            let mut s2 = mapped.clone();

            assert_eq!(s1.get(), 10); // counter = 1, 1 * 10
            assert_eq!(s2.get(), 20); // counter = 2, 2 * 10
            assert_eq!(s1.get(), 30); // counter = 3, 3 * 10
        }
    }

    mod test_filter {
        use super::{
            Rc,
            RcStatefulSupplier,
            RefCell,
            StatefulSupplier,
        };

        #[test]
        fn test_filters_even_numbers() {
            let counter = Rc::new(RefCell::new(0));
            let counter_clone = Rc::clone(&counter);
            let source = RcStatefulSupplier::new(move || {
                let mut c = counter_clone.borrow_mut();
                *c += 1;
                *c
            });
            let filtered = source.filter(|x: &i32| x % 2 == 0);

            let mut s = filtered;
            assert_eq!(s.get(), None); // 1 is odd
            assert_eq!(s.get(), Some(2)); // 2 is even
        }
    }

    mod test_zip {
        use super::{
            RcStatefulSupplier,
            StatefulSupplier,
        };

        #[test]
        fn test_combines_two_stateful_suppliers() {
            let first = RcStatefulSupplier::new(|| 42);
            let second = RcStatefulSupplier::new(|| "hello");
            let zipped = first.zip(second.clone());

            let mut z = zipped;
            assert_eq!(z.get(), (42, "hello"));
        }

        #[test]
        fn test_originals_remain_usable() {
            let first = RcStatefulSupplier::new(|| 42);
            let second = RcStatefulSupplier::new(|| "hello");
            let _zipped = first.zip(second.clone());

            // Both originals still usable
            let mut f = first;
            let mut s = second;
            assert_eq!(f.get(), 42);
            assert_eq!(s.get(), "hello");
        }
    }

    mod test_memoize {
        use super::{
            Rc,
            RcStatefulSupplier,
            RefCell,
            StatefulSupplier,
        };

        #[test]
        fn test_caches_first_value() {
            let call_count = Rc::new(RefCell::new(0));
            let call_count_clone = Rc::clone(&call_count);
            let source = RcStatefulSupplier::new(move || {
                let mut c = call_count_clone.borrow_mut();
                *c += 1;
                42
            });
            let memoized = source.memoize();

            let mut s = memoized;
            assert_eq!(s.get(), 42);
            assert_eq!(s.get(), 42);
            assert_eq!(*call_count.borrow(), 1);
        }
    }

    // Note: RcStatefulSupplier cannot be converted to ArcStatefulSupplier
    // because Rc is not Send. This is prevented at compile time by the
    // trait bound, so we don't test it.
}

// ==========================================================================
// SupplierOnce Implementation Tests for BoxStatefulSupplier
// ==========================================================================

#[cfg(test)]
mod test_box_stateful_supplier_once {
    use super::{
        BoxStatefulSupplier,
        StatefulSupplier,
    };

    mod test_get {
        use super::{
            BoxStatefulSupplier,
            StatefulSupplier,
        };

        #[test]
        fn test_consumes_stateful_supplier() {
            let mut supplier = BoxStatefulSupplier::new(|| 42);
            let value = supplier.get();
            assert_eq!(value, 42);
            // StatefulSupplier is consumed, cannot be used again
        }

        #[test]
        fn test_with_string() {
            let mut supplier =
                BoxStatefulSupplier::new(|| String::from("hello"));
            let value = supplier.get();
            assert_eq!(value, "hello");
        }

        #[test]
        fn test_with_vec() {
            let mut supplier = BoxStatefulSupplier::new(|| vec![1, 2, 3]);
            let value = supplier.get();
            assert_eq!(value, vec![1, 2, 3]);
        }

        #[test]
        fn test_moves_captured_value() {
            let data = String::from("captured");
            let mut supplier = BoxStatefulSupplier::new(move || data.clone());
            let value = supplier.get();
            assert_eq!(value, "captured");
        }

        #[test]
        fn test_with_stateful_closure() {
            let mut counter = 0;
            let mut supplier = BoxStatefulSupplier::new(move || {
                counter += 1;
                counter
            });
            let value = supplier.get();
            assert_eq!(value, 1);
        }
    }
}

// ==========================================================================
// SupplierOnce Implementation Tests for ArcStatefulSupplier
// ==========================================================================

#[cfg(test)]
mod test_arc_stateful_supplier_once {
    use super::{
        Arc,
        ArcStatefulSupplier,
        Mutex,
        StatefulSupplier,
    };

    mod test_get {
        use super::{
            Arc,
            ArcStatefulSupplier,
            Mutex,
            StatefulSupplier,
        };

        #[test]
        fn test_consumes_stateful_supplier() {
            let mut supplier = ArcStatefulSupplier::new(|| 42);
            let value = supplier.get();
            assert_eq!(value, 42);
            // StatefulSupplier is consumed, cannot be used again
        }

        #[test]
        fn test_with_string() {
            let mut supplier =
                ArcStatefulSupplier::new(|| String::from("hello"));
            let value = supplier.get();
            assert_eq!(value, "hello");
        }

        #[test]
        fn test_with_vec() {
            let mut supplier = ArcStatefulSupplier::new(|| vec![1, 2, 3]);
            let value = supplier.get();
            assert_eq!(value, vec![1, 2, 3]);
        }

        #[test]
        fn test_with_shared_state() {
            let counter = Arc::new(Mutex::new(0));
            let counter_clone = Arc::clone(&counter);
            let mut supplier = ArcStatefulSupplier::new(move || {
                let mut c =
                    counter_clone.lock().expect("mutex should not be poisoned");
                *c += 1;
                *c
            });
            let value = supplier.get();
            assert_eq!(value, 1);
            assert_eq!(
                *counter.lock().expect("mutex should not be poisoned"),
                1
            );
        }

        #[test]
        fn test_cloned_stateful_suppliers_share_state() {
            let counter = Arc::new(Mutex::new(0));
            let counter_clone1 = Arc::clone(&counter);

            let stateful_supplier1 = ArcStatefulSupplier::new(move || {
                let mut c = counter_clone1
                    .lock()
                    .expect("mutex should not be poisoned");
                *c += 1;
                *c
            });

            let stateful_supplier2 = stateful_supplier1.clone();

            let mut stateful_supplier1 = stateful_supplier1;
            let mut stateful_supplier2 = stateful_supplier2;
            let value1 = stateful_supplier1.get();
            let value2 = stateful_supplier2.get();

            // Both should increment the same counter
            assert_eq!(value1 + value2, 3); // 1 + 2
            assert_eq!(
                *counter.lock().expect("mutex should not be poisoned"),
                2
            );
        }
    }
}

// ==========================================================================
// SupplierOnce Implementation Tests for RcStatefulSupplier
// ==========================================================================

#[cfg(test)]
mod test_rc_stateful_supplier_once {
    use super::{
        Rc,
        RcStatefulSupplier,
        RefCell,
        StatefulSupplier,
    };

    mod test_get {
        use super::{
            Rc,
            RcStatefulSupplier,
            RefCell,
            StatefulSupplier,
        };

        #[test]
        fn test_consumes_stateful_supplier() {
            let mut supplier = RcStatefulSupplier::new(|| 42);
            let value = supplier.get();
            assert_eq!(value, 42);
            // StatefulSupplier is consumed, cannot be used again
        }

        #[test]
        fn test_with_string() {
            let mut supplier =
                RcStatefulSupplier::new(|| String::from("hello"));
            let value = supplier.get();
            assert_eq!(value, "hello");
        }

        #[test]
        fn test_with_vec() {
            let mut supplier = RcStatefulSupplier::new(|| vec![1, 2, 3]);
            let value = supplier.get();
            assert_eq!(value, vec![1, 2, 3]);
        }

        #[test]
        fn test_with_shared_state() {
            let counter = Rc::new(RefCell::new(0));
            let counter_clone = Rc::clone(&counter);
            let mut supplier = RcStatefulSupplier::new(move || {
                let mut c = counter_clone.borrow_mut();
                *c += 1;
                *c
            });
            let value = supplier.get();
            assert_eq!(value, 1);
            assert_eq!(*counter.borrow(), 1);
        }

        #[test]
        fn test_cloned_stateful_suppliers_share_state() {
            let counter = Rc::new(RefCell::new(0));
            let counter_clone1 = Rc::clone(&counter);

            let stateful_supplier1 = RcStatefulSupplier::new(move || {
                let mut c = counter_clone1.borrow_mut();
                *c += 1;
                *c
            });

            let stateful_supplier2 = stateful_supplier1.clone();

            let mut stateful_supplier1 = stateful_supplier1;
            let mut stateful_supplier2 = stateful_supplier2;
            let value1 = stateful_supplier1.get();
            let value2 = stateful_supplier2.get();

            // Both should increment the same counter
            assert_eq!(value1 + value2, 3); // 1 + 2
            assert_eq!(*counter.borrow(), 2);
        }
    }
}
// ==========================================================================
// FnStatefulSupplierOps Extension Trait Tests
// ==========================================================================

#[cfg(test)]
mod test_fn_stateful_supplier_ops {
    use super::{
        BoxStatefulSupplier,
        FnStatefulSupplierOps,
        StatefulSupplier,
    };

    #[test]
    fn test_closure_map() {
        // Test map method on closure
        let mut mapped = (|| 10).map(|x| x * 2);
        assert_eq!(mapped.get(), 20);
        assert_eq!(mapped.get(), 20);
    }

    #[test]
    fn test_closure_map_chain() {
        // Test chaining multiple map operations
        let mut mapped = (|| 10).map(|x| x * 2).map(|x| x + 5);
        assert_eq!(mapped.get(), 25);
        assert_eq!(mapped.get(), 25);
    }

    #[test]
    fn test_closure_map_stateful() {
        // Test map on stateful closure
        let mut counter = 0;
        let mut mapped = (move || {
            counter += 1;
            counter
        })
        .map(|x| x * 2);

        assert_eq!(mapped.get(), 2);
        assert_eq!(mapped.get(), 4);
        assert_eq!(mapped.get(), 6);
    }

    #[test]
    fn test_closure_map_type_conversion() {
        // Test map with type conversion
        let mut mapped = (|| 42).map(|x: i32| x.to_string());
        assert_eq!(mapped.get(), "42");
    }

    #[test]
    fn test_closure_filter() {
        // Test filter method on closure
        let mut counter = 0;
        let mut filtered = (move || {
            counter += 1;
            counter
        })
        .filter(|x: &i32| x % 2 == 0);

        assert_eq!(filtered.get(), None); // 1 is odd
        assert_eq!(filtered.get(), Some(2)); // 2 is even
        assert_eq!(filtered.get(), None); // 3 is odd
        assert_eq!(filtered.get(), Some(4)); // 4 is even
    }

    #[test]
    fn test_closure_filter_always_pass() {
        // Test filter that always passes
        let mut filtered = (|| 42).filter(|_: &i32| true);
        assert_eq!(filtered.get(), Some(42));
        assert_eq!(filtered.get(), Some(42));
    }

    #[test]
    fn test_closure_filter_always_fail() {
        // Test filter that always fails
        let mut filtered = (|| 42).filter(|_: &i32| false);
        assert_eq!(filtered.get(), None);
        assert_eq!(filtered.get(), None);
    }

    #[test]
    fn test_closure_filter_with_map() {
        // Test combining filter and map
        let mut counter = 0;
        let mut pipeline = (move || {
            counter += 1;
            counter
        })
        .filter(|x: &i32| x % 2 == 0)
        .map(|opt: Option<i32>| opt.map(|x| x * 10));

        assert_eq!(pipeline.get(), None); // 1 is odd
        assert_eq!(pipeline.get(), Some(20)); // 2 is even, doubled to 20
        assert_eq!(pipeline.get(), None); // 3 is odd
        assert_eq!(pipeline.get(), Some(40)); // 4 is even, doubled to 40
    }

    #[test]
    fn test_closure_zip() {
        // Test zip method on closure
        let first = || 42;
        let second = BoxStatefulSupplier::new(|| "hello");
        let mut zipped = first.zip(second);

        assert_eq!(zipped.get(), (42, "hello"));
        assert_eq!(zipped.get(), (42, "hello"));
    }

    #[test]
    fn test_closure_zip_stateful() {
        // Test zip with stateful closures
        let mut counter1 = 0;
        let first = move || {
            counter1 += 1;
            counter1
        };

        let mut counter2 = 100;
        let second = BoxStatefulSupplier::new(move || {
            counter2 += 1;
            counter2
        });

        let mut zipped = first.zip(second);

        assert_eq!(zipped.get(), (1, 101));
        assert_eq!(zipped.get(), (2, 102));
        assert_eq!(zipped.get(), (3, 103));
    }

    #[test]
    fn test_closure_zip_different_types() {
        // Test zip with different types
        let first = || 42;
        let second = BoxStatefulSupplier::new(|| "world");
        let mut zipped = first.zip(second);

        let result = zipped.get();
        assert_eq!(result.0, 42);
        assert_eq!(result.1, "world");
    }

    #[test]
    fn test_closure_memoize() {
        // Test memoize method on closure
        let mut memoized = (|| 42).memoize();

        // First call executes the closure
        assert_eq!(memoized.get(), 42);
        // Subsequent calls return cached value
        assert_eq!(memoized.get(), 42);
        assert_eq!(memoized.get(), 42);
    }

    #[test]
    fn test_closure_memoize_with_map() {
        // Test combining memoize and map
        let mut pipeline = (|| 10).memoize().map(|x| x * 2);

        assert_eq!(pipeline.get(), 20);
        assert_eq!(pipeline.get(), 20);
        assert_eq!(pipeline.get(), 20);
    }

    #[test]
    fn test_closure_complex_pipeline() {
        // Test complex pipeline with multiple operations
        let mut counter = 0;
        let mut pipeline = (move || {
            counter += 1;
            counter
        })
        .map(|x| x * 2)
        .filter(|x: &i32| x % 4 == 0)
        .map(|opt: Option<i32>| opt.unwrap_or(0));

        assert_eq!(pipeline.get(), 0); // 1*2=2, 2%4!=0, filtered out
        assert_eq!(pipeline.get(), 4); // 2*2=4, 4%4==0, passed
        assert_eq!(pipeline.get(), 0); // 3*2=6, 6%4!=0, filtered out
        assert_eq!(pipeline.get(), 8); // 4*2=8, 8%4==0, passed
    }

    #[test]
    fn test_closure_map_then_zip() {
        // Test combining map and zip
        let first = (|| 10).map(|x| x * 2);
        let second = BoxStatefulSupplier::new(|| 5);
        let mut zipped = first.zip(second);

        assert_eq!(zipped.get(), (20, 5));
    }

    #[test]
    fn test_closure_filter_then_zip() {
        // Test combining filter and zip
        let mut counter = 0;
        let filtered = (move || {
            counter += 1;
            counter
        })
        .filter(|x: &i32| x % 2 == 0);

        let second = BoxStatefulSupplier::new(|| "test");
        let mut zipped = filtered.zip(second);

        assert_eq!(zipped.get(), (None, "test")); // 1 is odd
        assert_eq!(zipped.get(), (Some(2), "test")); // 2 is even
    }

    #[test]
    fn test_closure_all_operations() {
        // Test using all operations in one pipeline
        let mut counter = 0;
        let mut pipeline = (move || {
            counter += 1;
            counter
        })
        .map(|x| x * 2) // Double the counter
        .filter(|x: &i32| x % 4 == 0) // Keep only multiples of 4
        .map(|opt| match opt {
            Some(x) => x / 2, // Convert back
            None => 0,
        });

        assert_eq!(pipeline.get(), 0); // 1*2=2, not multiple of 4
        assert_eq!(pipeline.get(), 2); // 2*2=4, multiple of 4, 4/2=2
        assert_eq!(pipeline.get(), 0); // 3*2=6, not multiple of 4
        assert_eq!(pipeline.get(), 4); // 4*2=8, multiple of 4, 8/2=4
    }

    #[test]
    fn test_function_pointer_map() {
        // Test map with function pointer
        fn double(x: i32) -> i32 {
            x * 2
        }

        let supplier = || 10;
        let mut mapped = supplier.map(double);
        assert_eq!(mapped.get(), 20);
    }

    #[test]
    fn test_function_pointer_filter() {
        // Test filter with function pointer
        fn is_even(x: &i32) -> bool {
            x % 2 == 0
        }

        let mut counter = 0;
        let mut filtered = (move || {
            counter += 1;
            counter
        })
        .filter(is_even);

        assert_eq!(filtered.get(), None); // 1 is odd
        assert_eq!(filtered.get(), Some(2)); // 2 is even
    }

    #[test]
    fn test_closure_string_operations() {
        // Test with String type
        let mut mapped =
            (|| "hello".to_string()).map(|s: String| s.to_uppercase());
        assert_eq!(mapped.get(), "HELLO");
    }

    #[test]
    fn test_closure_vec_operations() {
        // Test with Vec type
        let mut mapped = (|| vec![1, 2, 3]).map(|v: Vec<i32>| v.len());
        assert_eq!(mapped.get(), 3);
    }

    #[test]
    fn test_closure_option_operations() {
        // Test with Option type
        let mut mapped = (|| Some(42)).map(|opt: Option<i32>| opt.unwrap_or(0));
        assert_eq!(mapped.get(), 42);

        let mut mapped_none =
            (|| None::<i32>).map(|opt: Option<i32>| opt.unwrap_or(0));
        assert_eq!(mapped_none.get(), 0);
    }

    #[test]
    fn test_closure_result_operations() {
        // Test with Result type
        let mut mapped = (|| Ok::<i32, String>(42))
            .map(|res: Result<i32, String>| res.unwrap_or(0));
        assert_eq!(mapped.get(), 42);

        let mut mapped_err = (|| Err::<i32, String>("error".to_string()))
            .map(|res: Result<i32, String>| res.unwrap_or(0));
        assert_eq!(mapped_err.get(), 0);
    }

    #[test]
    fn test_closure_tuple_operations() {
        // Test with tuple type
        let mut mapped = (|| (1, 2)).map(|(a, b)| a + b);
        assert_eq!(mapped.get(), 3);
    }

    #[test]
    fn test_closure_nested_map() {
        // Test nested map operations
        let mut mapped = (|| 5)
            .map(|x| x + 1)
            .map(|x| x * 2)
            .map(|x| x - 3)
            .map(|x| x / 2);
        assert_eq!(mapped.get(), 4); // (5+1)*2-3 = 9, 9/2 = 4
    }

    #[test]
    fn test_closure_memoize_clone_behavior() {
        // Test that memoize caches the cloned value
        let mut memoized = (|| vec![1, 2, 3]).memoize();

        let result1 = memoized.get();
        let result2 = memoized.get();

        assert_eq!(result1, vec![1, 2, 3]);
        assert_eq!(result2, vec![1, 2, 3]);
        // Verify they are separate clones
        assert_eq!(result1, result2);
    }
}
// ======================================================================
// Debug and Display Trait Tests
// ======================================================================

#[cfg(test)]
mod test_stateful_supplier_debug_display {
    use super::{
        ArcStatefulSupplier,
        BoxStatefulSupplier,
        RcStatefulSupplier,
    };

    // ============================================================
    // BoxStatefulSupplier Debug and Display Tests
    // ============================================================

    mod test_box_stateful_supplier_debug_display {
        use super::BoxStatefulSupplier;

        #[test]
        fn test_debug_without_name() {
            // Test Debug formatting for BoxStatefulSupplier without name
            let supplier = BoxStatefulSupplier::new(|| 42);
            let debug_str = format!("{:?}", supplier);
            assert!(debug_str.contains("BoxStatefulSupplier"));
            assert!(debug_str.contains("name: None"));
            assert!(debug_str.contains("function: \"<function>\""));
        }

        #[test]
        fn test_debug_with_name() {
            // Test Debug formatting for BoxStatefulSupplier with name
            let supplier =
                BoxStatefulSupplier::new_with_name("test_supplier", || 42);
            let debug_str = format!("{:?}", supplier);
            assert!(debug_str.contains("BoxStatefulSupplier"));
            assert!(debug_str.contains("name: Some(\"test_supplier\")"));
            assert!(debug_str.contains("function: \"<function>\""));
        }

        #[test]
        fn test_display_without_name() {
            // Test Display formatting for BoxStatefulSupplier without name
            let supplier = BoxStatefulSupplier::new(|| 42);
            let display_str = format!("{}", supplier);
            assert_eq!(display_str, "BoxStatefulSupplier");
        }

        #[test]
        fn test_display_with_name() {
            // Test Display formatting for BoxStatefulSupplier with name
            let supplier =
                BoxStatefulSupplier::new_with_name("test_supplier", || 42);
            let display_str = format!("{}", supplier);
            assert_eq!(display_str, "BoxStatefulSupplier(test_supplier)");
        }
    }

    // ============================================================
    // ArcStatefulSupplier Debug and Display Tests
    // ============================================================

    mod test_arc_stateful_supplier_debug_display {
        use super::ArcStatefulSupplier;

        #[test]
        fn test_debug_without_name() {
            // Test Debug formatting for ArcStatefulSupplier without name
            let supplier = ArcStatefulSupplier::new(|| 42);
            let debug_str = format!("{:?}", supplier);
            assert!(debug_str.contains("ArcStatefulSupplier"));
            assert!(debug_str.contains("name: None"));
            assert!(debug_str.contains("function: \"<function>\""));
        }

        #[test]
        fn test_debug_with_name() {
            // Test Debug formatting for ArcStatefulSupplier with name
            let supplier =
                ArcStatefulSupplier::new_with_name("test_supplier", || 42);
            let debug_str = format!("{:?}", supplier);
            assert!(debug_str.contains("ArcStatefulSupplier"));
            assert!(debug_str.contains("name: Some(\"test_supplier\")"));
            assert!(debug_str.contains("function: \"<function>\""));
        }

        #[test]
        fn test_display_without_name() {
            // Test Display formatting for ArcStatefulSupplier without name
            let supplier = ArcStatefulSupplier::new(|| 42);
            let display_str = format!("{}", supplier);
            assert_eq!(display_str, "ArcStatefulSupplier");
        }

        #[test]
        fn test_display_with_name() {
            // Test Display formatting for ArcStatefulSupplier with name
            let supplier =
                ArcStatefulSupplier::new_with_name("test_supplier", || 42);
            let display_str = format!("{}", supplier);
            assert_eq!(display_str, "ArcStatefulSupplier(test_supplier)");
        }
    }

    // ============================================================
    // RcStatefulSupplier Debug and Display Tests
    // ============================================================

    mod test_rc_stateful_supplier_debug_display {
        use super::RcStatefulSupplier;

        #[test]
        fn test_debug_without_name() {
            // Test Debug formatting for RcStatefulSupplier without name
            let supplier = RcStatefulSupplier::new(|| 42);
            let debug_str = format!("{:?}", supplier);
            assert!(debug_str.contains("RcStatefulSupplier"));
            assert!(debug_str.contains("name: None"));
            assert!(debug_str.contains("function: \"<function>\""));
        }

        #[test]
        fn test_debug_with_name() {
            // Test Debug formatting for RcStatefulSupplier with name
            let supplier =
                RcStatefulSupplier::new_with_name("test_supplier", || 42);
            let debug_str = format!("{:?}", supplier);
            assert!(debug_str.contains("RcStatefulSupplier"));
            assert!(debug_str.contains("name: Some(\"test_supplier\")"));
            assert!(debug_str.contains("function: \"<function>\""));
        }

        #[test]
        fn test_display_without_name() {
            // Test Display formatting for RcStatefulSupplier without name
            let supplier = RcStatefulSupplier::new(|| 42);
            let display_str = format!("{}", supplier);
            assert_eq!(display_str, "RcStatefulSupplier");
        }

        #[test]
        fn test_display_with_name() {
            // Test Display formatting for RcStatefulSupplier with name
            let supplier =
                RcStatefulSupplier::new_with_name("test_supplier", || 42);
            let display_str = format!("{}", supplier);
            assert_eq!(display_str, "RcStatefulSupplier(test_supplier)");
        }
    }
}

// ============================================================================
// StatefulSupplier Trait Default Methods Tests - into_once, to_once
// ============================================================================
