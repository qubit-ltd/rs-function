// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

// qubit-style: allow explicit-imports
//! Unit tests for Supplier types

use qubit_function::{
    ArcSupplier,
    ArcTransformer,
    BoxSupplier,
    BoxTransformer,
    RcSupplier,
    RcTransformer,
    Supplier,
};
use std::sync::Arc;
use std::thread;

// ======================================================================
// Supplier Trait Tests (for closures)
// ======================================================================


#[cfg(test)]
mod test_stateless_supplier_trait {
    use super::{
        BoxSupplier,
        Supplier,
    };


    #[test]
    fn test_closure_stateless() {
        // Test stateless closure (always returns same value)
        let boxed = BoxSupplier::new(|| 42);
        assert_eq!(boxed.get(), 42);
        assert_eq!(boxed.get(), 42);
        assert_eq!(boxed.get(), 42);
    }




    #[test]
    fn test_closure_get() {
        // Test the get method in impl<T, F> Supplier<T>
        // for F
        let closure = || 42;
        assert_eq!(closure.get(), 42);
        assert_eq!(closure.get(), 42);
    }

    #[test]
    fn test_closure_get_stateless() {
        // Test stateless closure (doesn't modify captured
        // variables)
        let value = 100;
        let closure = move || value * 2;
        assert_eq!(closure.get(), 200);
        assert_eq!(closure.get(), 200);
        assert_eq!(closure.get(), 200);
    }



}

// ======================================================================
// BoxSupplier Tests
// ======================================================================

#[cfg(test)]
mod test_box_stateless_supplier {
    use super::{
        BoxSupplier,
        Supplier,
    };

    mod test_new {
        use super::{
            BoxSupplier,
            Supplier,
        };

        #[test]
        fn test_new_basic() {
            // Test creating a new BoxSupplier
            let supplier = BoxSupplier::new(|| 42);
            assert_eq!(supplier.get(), 42);
        }

        #[test]
        fn test_new_with_closure() {
            // Test with a closure that captures variables
            let value = 100;
            let supplier = BoxSupplier::new(move || value);
            assert_eq!(supplier.get(), 100);
        }

        #[test]
        fn test_new_returns_same_value() {
            // Test that successive calls return same value
            let supplier = BoxSupplier::new(|| 42);
            assert_eq!(supplier.get(), 42);
            assert_eq!(supplier.get(), 42);
            assert_eq!(supplier.get(), 42);
        }
    }

    mod test_constant {
        use super::{
            BoxSupplier,
            Supplier,
        };

        #[test]
        fn test_constant_basic() {
            // Test constant supplier
            let constant = BoxSupplier::constant(42);
            assert_eq!(constant.get(), 42);
            assert_eq!(constant.get(), 42);
        }

        #[test]
        fn test_constant_string() {
            // Test constant with String type
            let constant = BoxSupplier::constant(String::from("hello"));
            assert_eq!(constant.get(), "hello");
            assert_eq!(constant.get(), "hello");
        }

        #[test]
        fn test_constant_vec() {
            // Test constant with Vec type
            let constant = BoxSupplier::constant(vec![1, 2, 3]);
            assert_eq!(constant.get(), vec![1, 2, 3]);
            assert_eq!(constant.get(), vec![1, 2, 3]);
        }
    }

    mod test_map {
        use super::{
            BoxSupplier,
            Supplier,
        };

        #[test]
        fn test_map_basic() {
            // Test map transformation
            let mapped = BoxSupplier::new(|| 10).map(|x| x * 2);
            assert_eq!(mapped.get(), 20);
        }

        #[test]
        fn test_map_chain() {
            // Test chained map operations
            let pipeline =
                BoxSupplier::new(|| 10).map(|x| x * 2).map(|x| x + 5);
            assert_eq!(pipeline.get(), 25);
        }

        #[test]
        fn test_map_type_conversion() {
            // Test map with type conversion
            let mapped = BoxSupplier::new(|| 42).map(|x: i32| x.to_string());
            assert_eq!(mapped.get(), "42");
        }
    }

    mod test_filter {
        use super::{
            BoxSupplier,
            Supplier,
        };

        #[test]
        fn test_filter_passes() {
            // Test filter that passes
            let filtered = BoxSupplier::new(|| 42).filter(|x: &i32| x % 2 == 0);
            assert_eq!(filtered.get(), Some(42));
        }

        #[test]
        fn test_filter_fails() {
            // Test filter that fails
            let filtered = BoxSupplier::new(|| 43).filter(|x: &i32| x % 2 == 0);
            assert_eq!(filtered.get(), None);
        }

        #[test]
        fn test_filter_with_map() {
            // Test combining filter and map
            let pipeline = BoxSupplier::new(|| 10)
                .map(|x| x * 2)
                .filter(|x: &i32| *x > 15);
            assert_eq!(pipeline.get(), Some(20));
        }
    }

    mod test_zip {
        use super::{
            BoxSupplier,
            Supplier,
        };

        #[test]
        fn test_zip_basic() {
            // Test zipping two suppliers
            let first = BoxSupplier::new(|| 42);
            let second = BoxSupplier::new(|| "hello");
            let zipped = first.zip(second);
            assert_eq!(zipped.get(), (42, "hello"));
        }

        #[test]
        fn test_zip_different_types() {
            // Test zipping suppliers of different types
            let first = BoxSupplier::new(|| 100);
            let second = BoxSupplier::new(|| vec![1, 2, 3]);
            let zipped = first.zip(second);
            assert_eq!(zipped.get(), (100, vec![1, 2, 3]));
        }
    }

    mod test_trait_methods {
        use super::{
            BoxSupplier,
            Supplier,
        };

        #[test]
        fn test_get() {
            // Test Supplier::get method
            let supplier = BoxSupplier::new(|| 42);
            assert_eq!(supplier.get(), 42);
        }






        // Note: test_into_arc is not included here because
        // BoxSupplier cannot be converted to
        // ArcSupplier (inner function may not be Send +
        // Sync). This is enforced at compile time by trait bounds.
    }
}

// ======================================================================
// ArcSupplier Tests
// ======================================================================

#[cfg(test)]
mod test_arc_stateless_supplier {
    use super::{
        Arc,
        ArcSupplier,
        Supplier,
        thread,
    };

    mod test_new {
        use super::{
            ArcSupplier,
            Supplier,
        };

        #[test]
        fn test_new_basic() {
            // Test creating a new ArcSupplier
            let supplier = ArcSupplier::new(|| 42);
            assert_eq!(supplier.get(), 42);
        }

        #[test]
        fn test_new_with_closure() {
            // Test with a closure that captures variables
            let value = 100;
            let supplier = ArcSupplier::new(move || value);
            assert_eq!(supplier.get(), 100);
        }

        #[test]
        fn test_new_returns_same_value() {
            // Test that successive calls return same value
            let supplier = ArcSupplier::new(|| 42);
            assert_eq!(supplier.get(), 42);
            assert_eq!(supplier.get(), 42);
            assert_eq!(supplier.get(), 42);
        }
    }

    mod test_constant {
        use super::{
            ArcSupplier,
            Supplier,
        };

        #[test]
        fn test_constant_basic() {
            // Test constant supplier
            let constant = ArcSupplier::constant(42);
            assert_eq!(constant.get(), 42);
            assert_eq!(constant.get(), 42);
        }

        #[test]
        fn test_constant_string() {
            // Test constant with String type
            let constant = ArcSupplier::constant(String::from("hello"));
            assert_eq!(constant.get(), "hello");
            assert_eq!(constant.get(), "hello");
        }
    }

    mod test_map {
        use super::{
            ArcSupplier,
            Supplier,
        };

        #[test]
        fn test_map_basic() {
            // Test map transformation
            let source = ArcSupplier::new(|| 10);
            let mapped = source.map(|x| x * 2);
            assert_eq!(mapped.get(), 20);
        }

        #[test]
        fn test_map_chain() {
            // Test chained map operations
            let source = ArcSupplier::new(|| 10);
            let pipeline = source.map(|x| x * 2).map(|x| x + 5);
            assert_eq!(pipeline.get(), 25);
        }

        #[test]
        fn test_map_preserves_original() {
            // Test that mapping doesn't consume original
            let source = ArcSupplier::new(|| 10);
            let _mapped = source.map(|x| x * 2);
            // source is still usable
            assert_eq!(source.get(), 10);
        }
    }

    mod test_filter {
        use super::{
            ArcSupplier,
            Supplier,
        };

        #[test]
        fn test_filter_passes() {
            // Test filter that passes
            let source = ArcSupplier::new(|| 42);
            let filtered = source.filter(|x: &i32| x % 2 == 0);
            assert_eq!(filtered.get(), Some(42));
        }

        #[test]
        fn test_filter_fails() {
            // Test filter that fails
            let source = ArcSupplier::new(|| 43);
            let filtered = source.filter(|x: &i32| x % 2 == 0);
            assert_eq!(filtered.get(), None);
        }
    }

    mod test_zip {
        use super::{
            ArcSupplier,
            Supplier,
        };

        #[test]
        fn test_zip_basic() {
            // Test zipping two suppliers
            let first = ArcSupplier::new(|| 42);
            let second = ArcSupplier::new(|| "hello");
            let zipped = first.zip(second);
            assert_eq!(zipped.get(), (42, "hello"));
        }

        #[test]
        fn test_zip_preserves_originals() {
            // Test that zip doesn't consume originals
            let first = ArcSupplier::new(|| 42);
            let second = ArcSupplier::new(|| "hello");
            let _zipped = first.zip(second.clone());
            // Both are still usable
            assert_eq!(first.get(), 42);
            assert_eq!(second.get(), "hello");
        }
    }

    mod test_clone {
        use super::{
            ArcSupplier,
            Supplier,
        };

        #[test]
        fn test_clone_basic() {
            // Test cloning supplier
            let original = ArcSupplier::new(|| 42);
            let cloned = original.clone();
            assert_eq!(original.get(), 42);
            assert_eq!(cloned.get(), 42);
        }

        #[test]
        fn test_clone_shares_function() {
            // Test that clone shares the underlying function
            let original = ArcSupplier::new(|| String::from("hello"));
            let cloned = original.clone();
            assert_eq!(original.get(), cloned.get());
        }
    }

    mod test_thread_safety {
        use super::{
            Arc,
            ArcSupplier,
            Supplier,
            thread,
        };

        #[test]
        fn test_send_between_threads() {
            // Test that supplier can be sent between threads
            let supplier = ArcSupplier::new(|| 42);
            let handle = thread::spawn(move || supplier.get());
            assert_eq!(handle.join().expect("thread should not panic"), 42);
        }

        #[test]
        fn test_concurrent_access() {
            // Test lock-free concurrent access
            let factory = ArcSupplier::new(|| String::from("Hello, World!"));

            let handles: Vec<_> = (0..10)
                .map(|_| {
                    let f = factory.clone();
                    thread::spawn(move || f.get())
                })
                .collect();

            for h in handles {
                assert_eq!(
                    h.join().expect("thread should not panic"),
                    "Hello, World!"
                );
            }
        }

        #[test]
        fn test_shared_across_threads() {
            // Test sharing supplier across multiple threads
            let supplier = Arc::new(ArcSupplier::new(|| 100));

            let handles: Vec<_> = (0..5)
                .map(|_| {
                    let s = Arc::clone(&supplier);
                    thread::spawn(move || s.get())
                })
                .collect();

            for h in handles {
                assert_eq!(h.join().expect("thread should not panic"), 100);
            }
        }
    }

    mod test_trait_methods {
        use super::{
            ArcSupplier,
            Supplier,
        };

        #[test]
        fn test_get() {
            // Test Supplier::get method
            let supplier = ArcSupplier::new(|| 42);
            assert_eq!(supplier.get(), 42);
        }







    }
}

// ======================================================================
// RcSupplier Tests
// ======================================================================

#[cfg(test)]
mod test_rc_stateless_supplier {
    use super::{
        RcSupplier,
        Supplier,
    };

    mod test_new {
        use super::{
            RcSupplier,
            Supplier,
        };

        #[test]
        fn test_new_basic() {
            // Test creating a new RcSupplier
            let supplier = RcSupplier::new(|| 42);
            assert_eq!(supplier.get(), 42);
        }

        #[test]
        fn test_new_with_closure() {
            // Test with a closure that captures variables
            let value = 100;
            let supplier = RcSupplier::new(move || value);
            assert_eq!(supplier.get(), 100);
        }

        #[test]
        fn test_new_returns_same_value() {
            // Test that successive calls return same value
            let supplier = RcSupplier::new(|| 42);
            assert_eq!(supplier.get(), 42);
            assert_eq!(supplier.get(), 42);
            assert_eq!(supplier.get(), 42);
        }
    }

    mod test_constant {
        use super::{
            RcSupplier,
            Supplier,
        };

        #[test]
        fn test_constant_basic() {
            // Test constant supplier
            let constant = RcSupplier::constant(42);
            assert_eq!(constant.get(), 42);
            assert_eq!(constant.get(), 42);
        }

        #[test]
        fn test_constant_string() {
            // Test constant with String type
            let constant = RcSupplier::constant(String::from("hello"));
            assert_eq!(constant.get(), "hello");
            assert_eq!(constant.get(), "hello");
        }
    }

    mod test_map {
        use super::{
            RcSupplier,
            Supplier,
        };

        #[test]
        fn test_map_basic() {
            // Test map transformation
            let source = RcSupplier::new(|| 10);
            let mapped = source.map(|x| x * 2);
            assert_eq!(mapped.get(), 20);
        }

        #[test]
        fn test_map_chain() {
            // Test chained map operations
            let source = RcSupplier::new(|| 10);
            let pipeline = source.map(|x| x * 2).map(|x| x + 5);
            assert_eq!(pipeline.get(), 25);
        }

        #[test]
        fn test_map_preserves_original() {
            // Test that mapping doesn't consume original
            let source = RcSupplier::new(|| 10);
            let _mapped = source.map(|x| x * 2);
            // source is still usable
            assert_eq!(source.get(), 10);
        }
    }

    mod test_filter {
        use super::{
            RcSupplier,
            Supplier,
        };

        #[test]
        fn test_filter_passes() {
            // Test filter that passes
            let source = RcSupplier::new(|| 42);
            let filtered = source.filter(|x: &i32| x % 2 == 0);
            assert_eq!(filtered.get(), Some(42));
        }

        #[test]
        fn test_filter_fails() {
            // Test filter that fails
            let source = RcSupplier::new(|| 43);
            let filtered = source.filter(|x: &i32| x % 2 == 0);
            assert_eq!(filtered.get(), None);
        }
    }

    mod test_zip {
        use super::{
            RcSupplier,
            Supplier,
        };

        #[test]
        fn test_zip_basic() {
            // Test zipping two suppliers
            let first = RcSupplier::new(|| 42);
            let second = RcSupplier::new(|| "hello");
            let zipped = first.zip(second);
            assert_eq!(zipped.get(), (42, "hello"));
        }

        #[test]
        fn test_zip_preserves_originals() {
            // Test that zip doesn't consume originals
            let first = RcSupplier::new(|| 42);
            let second = RcSupplier::new(|| "hello");
            let _zipped = first.zip(second.clone());
            // Both are still usable
            assert_eq!(first.get(), 42);
            assert_eq!(second.get(), "hello");
        }
    }

    mod test_clone {
        use super::{
            RcSupplier,
            Supplier,
        };

        #[test]
        fn test_clone_basic() {
            // Test cloning supplier
            let original = RcSupplier::new(|| 42);
            let cloned = original.clone();
            assert_eq!(original.get(), 42);
            assert_eq!(cloned.get(), 42);
        }

        #[test]
        fn test_clone_shares_function() {
            // Test that clone shares the underlying function
            let original = RcSupplier::new(|| String::from("hello"));
            let cloned = original.clone();
            assert_eq!(original.get(), cloned.get());
        }
    }

    mod test_trait_methods {
        use super::{
            RcSupplier,
            Supplier,
        };

        #[test]
        fn test_get() {
            // Test Supplier::get method
            let supplier = RcSupplier::new(|| 42);
            assert_eq!(supplier.get(), 42);
        }






        // Note: test_into_arc is not included here because
        // RcSupplier cannot be converted to
        // ArcSupplier (Rc is not Send + Sync). This is
        // enforced at compile time by trait bounds.
    }
}

// ======================================================================
// Integration Tests
// ======================================================================

#[cfg(test)]
mod test_integration {
    use super::{
        Arc,
        ArcSupplier,
        BoxSupplier,
        Supplier,
        thread,
    };

    #[test]
    fn test_usage_in_read_only_context() {
        // Test using supplier in read-only struct methods
        struct Executor {
            error_supplier: ArcSupplier<String>,
        }

        impl Executor {
            fn execute(&self) -> Result<(), String> {
                // Can call supplier in &self method!
                Err(self.error_supplier.get())
            }
        }

        let executor = Executor {
            error_supplier: ArcSupplier::new(|| String::from("Error occurred")),
        };

        assert_eq!(executor.execute(), Err(String::from("Error occurred")));
    }

    #[test]
    fn test_factory_pattern() {
        // Test using as a factory for creating instances
        #[derive(Debug, PartialEq)]
        struct Config {
            timeout: u64,
        }

        let factory = BoxSupplier::new(|| Config { timeout: 30 });

        let config1 = factory.get();
        let config2 = factory.get();

        assert_eq!(config1, Config { timeout: 30 });
        assert_eq!(config2, Config { timeout: 30 });
    }

    #[test]
    fn test_concurrent_factory() {
        // Test using as factory in concurrent context
        let factory = Arc::new(ArcSupplier::new(|| vec![1, 2, 3, 4, 5]));

        let handles: Vec<_> = (0..10)
            .map(|_| {
                let f = Arc::clone(&factory);
                thread::spawn(move || f.get())
            })
            .collect();

        for h in handles {
            assert_eq!(
                h.join().expect("thread should not panic"),
                vec![1, 2, 3, 4, 5]
            );
        }
    }

    #[test]
    fn test_mixed_transformations() {
        // Test combining multiple transformation methods
        let pipeline = BoxSupplier::new(|| 10)
            .map(|x| x * 2)
            .filter(|x: &i32| *x > 15)
            .map(|opt: Option<i32>| opt.map(|x| x.to_string()));

        assert_eq!(pipeline.get(), Some(String::from("20")));
    }

}

// ======================================================================
// Map with Transformer Tests - BoxSupplier
// ======================================================================

#[cfg(test)]
mod test_box_stateless_supplier_map_with_transformer {
    use super::{
        BoxSupplier,
        BoxTransformer,
        Supplier,
    };

    // Helper function pointers
    fn double(x: i32) -> i32 {
        x * 2
    }

    fn to_string(x: i32) -> String {
        x.to_string()
    }

    #[test]
    fn test_map_with_closure() {
        // Test map accepts closure
        let supplier = BoxSupplier::new(|| 10);
        let mapped = supplier.map(|x| x * 2);
        assert_eq!(mapped.get(), 20);
    }

    #[test]
    fn test_map_with_function_pointer() {
        // Test map accepts function pointer
        let supplier = BoxSupplier::new(|| 10);
        let mapped = supplier.map(double);
        assert_eq!(mapped.get(), 20);
    }

    #[test]
    fn test_map_with_box_transformer() {
        // Test map accepts BoxTransformer object
        let supplier = BoxSupplier::new(|| 10);
        let transformer = BoxTransformer::new(|x| x * 3);
        let mapped = supplier.map(transformer);
        assert_eq!(mapped.get(), 30);
    }

    #[test]
    fn test_map_chain_with_different_types() {
        // Test chained calls, each map uses different type of transformer
        let supplier = BoxSupplier::new(|| 10);
        let step1 = supplier.map(|x| x * 2); // closure
        let step2 = step1.map(double); // function pointer
        let step3 = step2.map(BoxTransformer::new(|x| x + 5)); // BoxTransformer
        assert_eq!(step3.get(), 45); // (10 * 2) * 2 + 5 = 45
    }

    #[test]
    fn test_map_with_closure_capturing_variables() {
        // Test map uses closure capturing variables
        let multiplier = 3;
        let supplier = BoxSupplier::new(|| 10);
        let mapped = supplier.map(move |x| x * multiplier);
        assert_eq!(mapped.get(), 30);
    }

    #[test]
    fn test_map_with_type_conversion() {
        // Test map performs type conversion
        let supplier = BoxSupplier::new(|| 42);

        // Use closure to convert type
        let mapped1 = supplier.map(|x: i32| x.to_string());
        assert_eq!(mapped1.get(), "42");

        // Use BoxTransformer to convert type
        let supplier2 = BoxSupplier::new(|| 42);
        let transformer = BoxTransformer::new(to_string);
        let mapped2 = supplier2.map(transformer);
        assert_eq!(mapped2.get(), "42");
    }

    #[test]
    fn test_map_with_complex_transformer() {
        // Test map uses complex Transformer
        #[derive(Debug, PartialEq)]
        struct Data {
            value: i32,
        }

        let supplier = BoxSupplier::new(|| 10);
        let transformer = BoxTransformer::new(|x| Data { value: x * 2 });
        let mapped = supplier.map(transformer);
        assert_eq!(mapped.get(), Data { value: 20 });
    }
}

// ======================================================================
// Map with Transformer Tests - ArcSupplier
// ======================================================================

#[cfg(test)]
mod test_arc_stateless_supplier_map_with_transformer {
    use super::{
        ArcSupplier,
        ArcTransformer,
        Supplier,
        thread,
    };

    // Helper function pointers
    fn double(x: i32) -> i32 {
        x * 2
    }

    fn to_string(x: i32) -> String {
        x.to_string()
    }

    #[test]
    fn test_map_with_closure() {
        // Test map accepts closure
        let supplier = ArcSupplier::new(|| 10);
        let mapped = supplier.map(|x| x * 2);
        assert_eq!(mapped.get(), 20);
    }

    #[test]
    fn test_map_with_function_pointer() {
        // Test map accepts function pointer
        let supplier = ArcSupplier::new(|| 10);
        let mapped = supplier.map(double);
        assert_eq!(mapped.get(), 20);
    }

    #[test]
    fn test_map_with_arc_transformer() {
        // Test map accepts ArcTransformer object
        let supplier = ArcSupplier::new(|| 10);
        let transformer = ArcTransformer::new(|x| x * 3);
        let mapped = supplier.map(transformer);
        assert_eq!(mapped.get(), 30);
    }

    #[test]
    fn test_map_chain_with_different_types() {
        // Test chained calls, each map uses different type of transformer
        let supplier = ArcSupplier::new(|| 10);
        let step1 = supplier.map(|x| x * 2); // closure
        let step2 = step1.map(double); // function pointer
        let step3 = step2.map(ArcTransformer::new(|x| x + 5)); // ArcTransformer
        assert_eq!(step3.get(), 45); // (10 * 2) * 2 + 5 = 45
    }

    #[test]
    fn test_map_with_closure_capturing_variables() {
        // Test map uses closure capturing variables
        let multiplier = 3;
        let supplier = ArcSupplier::new(|| 10);
        let mapped = supplier.map(move |x| x * multiplier);
        assert_eq!(mapped.get(), 30);
    }

    #[test]
    fn test_map_preserves_original_with_transformer() {
        // Test original supplier still usable after using transformer
        let supplier = ArcSupplier::new(|| 10);
        let transformer = ArcTransformer::new(|x| x * 2);
        let mapped = supplier.map(transformer);

        // Original supplier still usable
        assert_eq!(supplier.get(), 10);
        assert_eq!(mapped.get(), 20);
    }

    #[test]
    fn test_map_thread_safety_with_transformer() {
        // Test map with transformer in multi-threaded environment
        let supplier = ArcSupplier::new(|| 10);
        let transformer = ArcTransformer::new(|x| x * 2);
        let mapped = supplier.map(transformer);

        let handles: Vec<_> = (0..10)
            .map(|_| {
                let m = mapped.clone();
                thread::spawn(move || m.get())
            })
            .collect();

        for h in handles {
            assert_eq!(h.join().expect("thread should not panic"), 20);
        }
    }

    #[test]
    fn test_map_with_type_conversion() {
        // Test map performs type conversion
        let supplier = ArcSupplier::new(|| 42);

        // Use closure to convert type
        let mapped1 = supplier.map(|x: i32| x.to_string());
        assert_eq!(mapped1.get(), "42");

        // Use ArcTransformer to convert type
        let transformer = ArcTransformer::new(to_string);
        let mapped2 = supplier.map(transformer);
        assert_eq!(mapped2.get(), "42");
    }

    #[test]
    fn test_map_with_shared_transformer() {
        // Test multiple suppliers sharing the same transformer
        let supplier1 = ArcSupplier::new(|| 10);
        let supplier2 = ArcSupplier::new(|| 20);

        let transformer = ArcTransformer::new(|x| x * 2);
        let mapped1 = supplier1.map(transformer.clone());
        let mapped2 = supplier2.map(transformer);

        assert_eq!(mapped1.get(), 20);
        assert_eq!(mapped2.get(), 40);
    }
}

// ======================================================================
// Map with Transformer Tests - RcSupplier
// ======================================================================

#[cfg(test)]
mod test_rc_stateless_supplier_map_with_transformer {
    use super::{
        RcSupplier,
        RcTransformer,
        Supplier,
    };

    // Helper function pointers
    fn double(x: i32) -> i32 {
        x * 2
    }

    fn to_string(x: i32) -> String {
        x.to_string()
    }

    #[test]
    fn test_map_with_closure() {
        // Test map accepts closure
        let supplier = RcSupplier::new(|| 10);
        let mapped = supplier.map(|x| x * 2);
        assert_eq!(mapped.get(), 20);
    }

    #[test]
    fn test_map_with_function_pointer() {
        // Test map accepts function pointer
        let supplier = RcSupplier::new(|| 10);
        let mapped = supplier.map(double);
        assert_eq!(mapped.get(), 20);
    }

    #[test]
    fn test_map_with_rc_transformer() {
        // Test map accepts RcTransformer object
        let supplier = RcSupplier::new(|| 10);
        let transformer = RcTransformer::new(|x| x * 3);
        let mapped = supplier.map(transformer);
        assert_eq!(mapped.get(), 30);
    }

    #[test]
    fn test_map_chain_with_different_types() {
        // Test chained calls, each map uses different type of transformer
        let supplier = RcSupplier::new(|| 10);
        let step1 = supplier.map(|x| x * 2); // closure
        let step2 = step1.map(double); // function pointer
        let step3 = step2.map(RcTransformer::new(|x| x + 5)); // RcTransformer
        assert_eq!(step3.get(), 45); // (10 * 2) * 2 + 5 = 45
    }

    #[test]
    fn test_map_with_closure_capturing_variables() {
        // Test map uses closure capturing variables
        let multiplier = 3;
        let supplier = RcSupplier::new(|| 10);
        let mapped = supplier.map(move |x| x * multiplier);
        assert_eq!(mapped.get(), 30);
    }

    #[test]
    fn test_map_preserves_original_with_transformer() {
        // Test original supplier still usable after using transformer
        let supplier = RcSupplier::new(|| 10);
        let transformer = RcTransformer::new(|x| x * 2);
        let mapped = supplier.map(transformer);

        // Original supplier still usable
        assert_eq!(supplier.get(), 10);
        assert_eq!(mapped.get(), 20);
    }

    #[test]
    fn test_map_with_type_conversion() {
        // Test map performs type conversion
        let supplier = RcSupplier::new(|| 42);

        // Use closure to convert type
        let mapped1 = supplier.map(|x: i32| x.to_string());
        assert_eq!(mapped1.get(), "42");

        // Use RcTransformer to convert type
        let transformer = RcTransformer::new(to_string);
        let mapped2 = supplier.map(transformer);
        assert_eq!(mapped2.get(), "42");
    }

    #[test]
    fn test_map_with_shared_transformer() {
        // Test multiple suppliers sharing the same transformer
        let supplier1 = RcSupplier::new(|| 10);
        let supplier2 = RcSupplier::new(|| 20);

        let transformer = RcTransformer::new(|x| x * 2);
        let mapped1 = supplier1.map(transformer.clone());
        let mapped2 = supplier2.map(transformer);

        assert_eq!(mapped1.get(), 20);
        assert_eq!(mapped2.get(), 40);
    }
}

// ======================================================================
// Integration Tests for Map with Transformer
// ======================================================================

#[cfg(test)]
mod test_map_transformer_integration {
    use super::{
        ArcSupplier,
        ArcTransformer,
        BoxSupplier,
        Supplier,
    };

    #[test]
    fn test_mixed_transformer_types_in_pipeline() {
        // Test mixing different types of transformers in pipeline
        let supplier = BoxSupplier::new(|| 5);

        let pipeline = supplier
            .map(|x| x * 2) // closure
            .map(|x: i32| -> i32 { x + 3 }) // closure with explicit type annotation
            .map(|x: i32| x.to_string()); // type conversion closure

        assert_eq!(pipeline.get(), "13");
    }

    #[test]
    fn test_transformer_with_complex_logic() {
        // Test transformer with complex logic
        #[derive(Debug, PartialEq)]
        struct Result {
            doubled: i32,
            squared: i32,
        }

        let supplier = ArcSupplier::new(|| 5);
        let transformer = ArcTransformer::new(|x| Result {
            doubled: x * 2,
            squared: x * x,
        });

        let mapped = supplier.map(transformer);
        assert_eq!(
            mapped.get(),
            Result {
                doubled: 10,
                squared: 25
            }
        );
    }

    #[test]
    fn test_function_pointer_with_generic_supplier() {
        // Test function pointer with generic supplier
        fn process(x: i32) -> String {
            format!("Value: {}", x * 2)
        }

        let supplier = ArcSupplier::new(|| 21);
        let mapped = supplier.map(process);
        assert_eq!(mapped.get(), "Value: 42");
    }

    #[test]
    fn test_transformer_reusability() {
        // Test reusability of Transformer
        let transformer = ArcTransformer::new(|x: i32| x * 10);

        let supplier1 = ArcSupplier::new(|| 1);
        let supplier2 = ArcSupplier::new(|| 2);
        let supplier3 = ArcSupplier::new(|| 3);

        let mapped1 = supplier1.map(transformer.clone());
        let mapped2 = supplier2.map(transformer.clone());
        let mapped3 = supplier3.map(transformer);

        assert_eq!(mapped1.get(), 10);
        assert_eq!(mapped2.get(), 20);
        assert_eq!(mapped3.get(), 30);
    }
}

// ======================================================================
// Default Implementation Tests for Custom Types
// ======================================================================

#[cfg(test)]
mod test_custom_stateless_supplier_default_impl {
    use super::Supplier;

    /// A simple custom type that implements Supplier with
    /// only the core `get` method, relying on default
    /// implementations for `into_box`, `into_rc`, and `into_arc`.
    struct CounterSupplier {
        /// The value to return each time `get` is called.
        value: i32,
    }

    impl CounterSupplier {
        /// Creates a new CounterSupplier with the given value.
        fn new(value: i32) -> Self {
            Self { value }
        }
    }

    impl Supplier<i32> for CounterSupplier {
        fn get(&self) -> i32 {
            self.value
        }

        // All into_xxx methods use default implementations
    }

    #[test]
    fn test_custom_supplier_get() {
        // Test that the custom supplier correctly implements the
        // core get method
        let supplier = CounterSupplier::new(42);
        assert_eq!(supplier.get(), 42);
        assert_eq!(supplier.get(), 42);
    }
















    // Implement Clone for CounterSupplier to enable to_* methods
    impl Clone for CounterSupplier {
        fn clone(&self) -> Self {
            Self { value: self.value }
        }
    }
}

// ======================================================================
// Tests for to_* Methods
// ======================================================================

#[cfg(test)]
mod test_to_methods {


    // ============================================================
    // Tests for ArcSupplier to_* methods
    // ============================================================

    mod test_arc_stateless_supplier_to_methods {







    }

    // ============================================================
    // Tests for RcSupplier to_* methods
    // ============================================================

    mod test_rc_stateless_supplier_to_methods {






        // Note: to_arc is not implemented for RcSupplier
        // because Rc is not Send + Sync. If you try to call it,
        // the compiler will fail with a trait bound error.
    }

    // ============================================================
    // Tests for Closure to_* methods
    // ============================================================

    mod test_closure_to_methods {







    }
}

// ======================================================================
// Debug and Display Trait Tests
// ======================================================================

#[cfg(test)]
mod test_supplier_debug_display {
    use super::{
        ArcSupplier,
        BoxSupplier,
        RcSupplier,
    };

    // ============================================================
    // BoxSupplier Debug and Display Tests
    // ============================================================

    mod test_box_supplier_debug_display {
        use super::BoxSupplier;

        #[test]
        fn test_debug_without_name() {
            // Test Debug formatting for BoxSupplier without name
            let supplier = BoxSupplier::new(|| 42);
            let debug_str = format!("{:?}", supplier);
            assert!(debug_str.contains("BoxSupplier"));
            assert!(debug_str.contains("name: None"));
            assert!(debug_str.contains("function: \"<function>\""));
        }

        #[test]
        fn test_debug_with_name() {
            // Test Debug formatting for BoxSupplier with name
            let supplier = BoxSupplier::new_with_name("test_supplier", || 42);
            let debug_str = format!("{:?}", supplier);
            assert!(debug_str.contains("BoxSupplier"));
            assert!(debug_str.contains("name: Some(\"test_supplier\")"));
            assert!(debug_str.contains("function: \"<function>\""));
        }

        #[test]
        fn test_display_without_name() {
            // Test Display formatting for BoxSupplier without name
            let supplier = BoxSupplier::new(|| 42);
            let display_str = format!("{}", supplier);
            assert_eq!(display_str, "BoxSupplier");
        }

        #[test]
        fn test_display_with_name() {
            // Test Display formatting for BoxSupplier with name
            let supplier = BoxSupplier::new_with_name("test_supplier", || 42);
            let display_str = format!("{}", supplier);
            assert_eq!(display_str, "BoxSupplier(test_supplier)");
        }
    }

    // ============================================================
    // ArcSupplier Debug and Display Tests
    // ============================================================

    mod test_arc_supplier_debug_display {
        use super::ArcSupplier;

        #[test]
        fn test_debug_without_name() {
            // Test Debug formatting for ArcSupplier without name
            let supplier = ArcSupplier::new(|| 42);
            let debug_str = format!("{:?}", supplier);
            assert!(debug_str.contains("ArcSupplier"));
            assert!(debug_str.contains("name: None"));
            assert!(debug_str.contains("function: \"<function>\""));
        }

        #[test]
        fn test_debug_with_name() {
            // Test Debug formatting for ArcSupplier with name
            let supplier = ArcSupplier::new_with_name("test_supplier", || 42);
            let debug_str = format!("{:?}", supplier);
            assert!(debug_str.contains("ArcSupplier"));
            assert!(debug_str.contains("name: Some(\"test_supplier\")"));
            assert!(debug_str.contains("function: \"<function>\""));
        }

        #[test]
        fn test_display_without_name() {
            // Test Display formatting for ArcSupplier without name
            let supplier = ArcSupplier::new(|| 42);
            let display_str = format!("{}", supplier);
            assert_eq!(display_str, "ArcSupplier");
        }

        #[test]
        fn test_display_with_name() {
            // Test Display formatting for ArcSupplier with name
            let supplier = ArcSupplier::new_with_name("test_supplier", || 42);
            let display_str = format!("{}", supplier);
            assert_eq!(display_str, "ArcSupplier(test_supplier)");
        }
    }

    // ============================================================
    // RcSupplier Debug and Display Tests
    // ============================================================

    mod test_rc_supplier_debug_display {
        use super::RcSupplier;

        #[test]
        fn test_debug_without_name() {
            // Test Debug formatting for RcSupplier without name
            let supplier = RcSupplier::new(|| 42);
            let debug_str = format!("{:?}", supplier);
            assert!(debug_str.contains("RcSupplier"));
            assert!(debug_str.contains("name: None"));
            assert!(debug_str.contains("function: \"<function>\""));
        }

        #[test]
        fn test_debug_with_name() {
            // Test Debug formatting for RcSupplier with name
            let supplier = RcSupplier::new_with_name("test_supplier", || 42);
            let debug_str = format!("{:?}", supplier);
            assert!(debug_str.contains("RcSupplier"));
            assert!(debug_str.contains("name: Some(\"test_supplier\")"));
            assert!(debug_str.contains("function: \"<function>\""));
        }

        #[test]
        fn test_display_without_name() {
            // Test Display formatting for RcSupplier without name
            let supplier = RcSupplier::new(|| 42);
            let display_str = format!("{}", supplier);
            assert_eq!(display_str, "RcSupplier");
        }

        #[test]
        fn test_display_with_name() {
            // Test Display formatting for RcSupplier with name
            let supplier = RcSupplier::new_with_name("test_supplier", || 42);
            let display_str = format!("{}", supplier);
            assert_eq!(display_str, "RcSupplier(test_supplier)");
        }
    }
}

// ============================================================================
// Supplier Trait Default Methods Tests - into_once, to_once
// ============================================================================

#[cfg(test)]
mod test_supplier_trait_default_methods {





}
