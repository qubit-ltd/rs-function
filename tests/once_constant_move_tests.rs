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
use qubit_function::BiMutatingFunctionOnce;
use qubit_function::BoxBiFunctionOnce;
use qubit_function::BoxBiMutatingFunctionOnce;
use qubit_function::BoxFunction;
use qubit_function::BoxFunctionOnce;
use qubit_function::Function;
use qubit_function::FunctionOnce;

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
