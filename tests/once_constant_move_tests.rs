// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

#![cfg(feature = "once")]

use std::cell::Cell;
use std::rc::Rc;

use qubit_function::BiFunctionOnce;
use qubit_function::BiTransformer;
use qubit_function::BiTransformerOnce;
use qubit_function::BiMutatingFunctionOnce;
use qubit_function::BoxBiFunctionOnce;
use qubit_function::BoxBiMutatingFunctionOnce;
use qubit_function::BoxBiTransformer;
use qubit_function::BoxBiTransformerOnce;
use qubit_function::BoxFunction;
use qubit_function::BoxFunctionOnce;
use qubit_function::BoxSupplier;
use qubit_function::BoxSupplierOnce;
use qubit_function::BoxTransformer;
use qubit_function::BoxTransformerOnce;
use qubit_function::Function;
use qubit_function::FunctionOnce;
use qubit_function::Supplier;
use qubit_function::SupplierOnce;
use qubit_function::Transformer;
use qubit_function::TransformerOnce;

struct Resource(String);

struct CloneTracked {
    clones: Rc<Cell<usize>>,
}

impl Clone for CloneTracked {
    fn clone(&self) -> Self {
        self.clones.set(self.clones.get() + 1);
        Self {
            clones: Rc::clone(&self.clones),
        }
    }
}

struct DropTracked {
    drops: Rc<Cell<usize>>,
}

impl Drop for DropTracked {
    fn drop(&mut self) {
        self.drops.set(self.drops.get() + 1);
    }
}

#[test]
fn test_transformer_and_supplier_once_constants_move_non_clone_values() {
    let unary = BoxTransformerOnce::<i32, Resource>::constant(Resource("unary".into()));
    assert_eq!(unary.apply(7).0, "unary");

    let binary = BoxBiTransformerOnce::<i32, bool, Resource>::constant(Resource("binary".into()));
    assert_eq!(binary.apply(7, false).0, "binary");

    let supplier = BoxSupplierOnce::constant(Resource("supplier".into()));
    assert_eq!(supplier.get().0, "supplier");
}

#[test]
fn test_once_constants_do_not_clone_and_reusable_constants_still_clone() {
    let clones = Rc::new(Cell::new(0));
    let unary = BoxTransformerOnce::<i32, CloneTracked>::constant(CloneTracked {
        clones: Rc::clone(&clones),
    });
    let binary = BoxBiTransformerOnce::<i32, bool, CloneTracked>::constant(CloneTracked {
        clones: Rc::clone(&clones),
    });
    let supplier = BoxSupplierOnce::constant(CloneTracked {
        clones: Rc::clone(&clones),
    });
    let _ = unary.apply(0);
    let _ = binary.apply(0, false);
    let _ = supplier.get();
    assert_eq!(clones.get(), 0);

    let repeated = BoxTransformer::<i32, CloneTracked>::constant(CloneTracked {
        clones: Rc::clone(&clones),
    });
    let _ = repeated.apply(0);
    let _ = repeated.apply(1);
    assert_eq!(clones.get(), 2);

    let repeated_binary = BoxBiTransformer::<i32, bool, CloneTracked>::constant(CloneTracked {
        clones: Rc::clone(&clones),
    });
    let _ = repeated_binary.apply(0, false);
    let _ = repeated_binary.apply(1, true);
    assert_eq!(clones.get(), 4);

    let repeated_supplier = BoxSupplier::constant(CloneTracked {
        clones: Rc::clone(&clones),
    });
    let _ = repeated_supplier.get();
    let _ = repeated_supplier.get();
    assert_eq!(clones.get(), 6);
}

#[test]
fn test_once_constants_release_resources_at_the_owner_boundary() {
    let drops = Rc::new(Cell::new(0));
    let unary = BoxTransformerOnce::<i32, DropTracked>::constant(DropTracked {
        drops: Rc::clone(&drops),
    });
    let output = unary.apply(0);
    assert_eq!(drops.get(), 0);
    drop(output);
    assert_eq!(drops.get(), 1);
    drop(BoxTransformerOnce::<i32, DropTracked>::constant(DropTracked {
        drops: Rc::clone(&drops),
    }));
    assert_eq!(drops.get(), 2);

    let binary = BoxBiTransformerOnce::<i32, bool, DropTracked>::constant(DropTracked {
        drops: Rc::clone(&drops),
    });
    let output = binary.apply(0, false);
    assert_eq!(drops.get(), 2);
    drop(output);
    assert_eq!(drops.get(), 3);
    drop(BoxBiTransformerOnce::<i32, bool, DropTracked>::constant(DropTracked {
        drops: Rc::clone(&drops),
    }));
    assert_eq!(drops.get(), 4);

    let supplier = BoxSupplierOnce::constant(DropTracked {
        drops: Rc::clone(&drops),
    });
    let output = supplier.get();
    assert_eq!(drops.get(), 4);
    drop(output);
    assert_eq!(drops.get(), 5);
    drop(BoxSupplierOnce::constant(DropTracked {
        drops: Rc::clone(&drops),
    }));
    assert_eq!(drops.get(), 6);
}

#[test]
fn test_once_constant_inputs_can_borrow_local_values() {
    let text = String::from("local");
    let unary = BoxTransformerOnce::<&str, Resource>::constant(Resource("unary".into()));
    assert_eq!(unary.apply(text.as_str()).0, "unary");
    let binary = BoxBiTransformerOnce::<&str, &str, Resource>::constant(Resource("binary".into()));
    assert_eq!(binary.apply(text.as_str(), text.as_str()).0, "binary");
}

#[test]
fn test_once_constants_keep_name_and_call_paths() {
    let unary = BoxTransformerOnce::<i32, Resource>::constant(Resource("unary".into()))
        .with_name("unary-constant");
    assert_eq!(unary.name(), Some("unary-constant"));
    assert_eq!(unary.apply(7).0, "unary");

    let binary = BoxBiTransformerOnce::<i32, bool, Resource>::constant(Resource("binary".into()))
        .with_name("binary-constant");
    assert_eq!(binary.name(), Some("binary-constant"));
    assert_eq!(binary.apply(7, false).0, "binary");

    let supplier = BoxSupplierOnce::constant(Resource("supplier".into()))
        .with_name("supplier-constant");
    assert_eq!(supplier.name(), Some("supplier-constant"));
    assert_eq!(supplier.get().0, "supplier");
}

#[test]
fn once_constants_move_non_clone_values_without_cloning() {
    let function = BoxFunctionOnce::<i32, Resource>::constant(Resource("function".into()));
    assert_eq!(function.apply(&0).0, "function");

    let bi_function = BoxBiFunctionOnce::<i32, bool, Resource>::constant(Resource("bi-function".into()));
    assert_eq!(bi_function.apply(&0, &false).0, "bi-function");

    let bi_mutating_function =
        BoxBiMutatingFunctionOnce::<i32, bool, Resource>::constant(Resource("bi-mutating-function".into()));
    let mut first = 1;
    let mut second = true;
    assert_eq!(
        bi_mutating_function.apply(&mut first, &mut second).0,
        "bi-mutating-function"
    );
    assert_eq!(first, 1);
    assert!(second);

    let clones = Rc::new(Cell::new(0));
    let once_function = BoxFunctionOnce::<i32, CloneTracked>::constant(CloneTracked {
        clones: Rc::clone(&clones),
    });
    let _ = once_function.apply(&0);
    assert_eq!(clones.get(), 0);

    let bi_clones = Rc::new(Cell::new(0));
    let once_bi_function = BoxBiFunctionOnce::<i32, bool, CloneTracked>::constant(CloneTracked {
        clones: Rc::clone(&bi_clones),
    });
    let _ = once_bi_function.apply(&0, &false);
    assert_eq!(bi_clones.get(), 0);

    let bi_mutating_clones = Rc::new(Cell::new(0));
    let once_bi_mutating_function = BoxBiMutatingFunctionOnce::<i32, bool, CloneTracked>::constant(CloneTracked {
        clones: Rc::clone(&bi_mutating_clones),
    });
    let _ = once_bi_mutating_function.apply(&mut first, &mut second);
    assert_eq!(bi_mutating_clones.get(), 0);

    let repeated_clones = Rc::new(Cell::new(0));
    let repeated_function = BoxFunction::<i32, CloneTracked>::constant(CloneTracked {
        clones: Rc::clone(&repeated_clones),
    });
    let _ = repeated_function.apply(&0);
    let _ = repeated_function.apply(&1);
    assert_eq!(repeated_clones.get(), 2);
}
