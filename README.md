# Qubit Function

[![Rust CI](https://github.com/qubit-ltd/rs-function/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-function/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-function/coverage-badge.json)](https://qubit-ltd.github.io/rs-function/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-function.svg?color=blue)](https://crates.io/crates/qubit-function)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![中文文档](https://img.shields.io/badge/文档-中文版-blue.svg)](README.zh_CN.md)

Comprehensive functional programming abstractions for Rust, providing Java-style functional interfaces adapted to Rust's ownership model.

## Overview

This crate provides a complete set of functional programming abstractions inspired by Java's functional interfaces, carefully adapted to Rust's ownership system. It offers multiple implementations for each abstraction (Box/Arc/Rc) to cover various use cases from simple single-threaded scenarios to complex multi-threaded applications.

## Key Features

- **Complete Functional Interface Suite**: broad functional abstraction families with reusable, one-time, stateful, mutating, and fallible variants
- **High-Performance Concurrency**: Uses parking_lot Mutex for superior thread synchronization performance
- **Multiple Ownership Models**: Box-based single ownership, Arc-based thread-safe sharing, and Rc-based single-threaded sharing
- **Flexible API Design**: Trait-based unified interface with concrete implementations optimized for different scenarios
- **Type-Oriented Module Layout**: Public source files are organized around a single exported type, keeping modules shorter and easier to scan
- **Method Chaining**: All types support fluent API and functional composition
- **Thread-Safety Options**: Choose between thread-safe (Arc) and efficient single-threaded (Rc) implementations
- **Zero-Cost Abstractions**: Efficient implementations with minimal runtime overhead

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
qubit-function = "0.14.0"
```

## Core Abstractions

This crate provides a broad set of functional abstractions, each with ownership-aware implementations where appropriate. The sections below introduce the main families, while the summary tables cover the additional mutating, bi-function, and operator variants.

### 1. Predicate - Single-Argument Predicate

Tests whether a value satisfies a condition, returning `bool`.

**Trait**: `Predicate<T>`
**Core Method**: `test(&self, value: &T) -> bool`
**Closure Equivalent**: `Fn(&T) -> bool`

**Implementations**:
- `BoxPredicate<T>` - Single ownership, non-cloneable
- `ArcPredicate<T>` - Thread-safe, cloneable
- `RcPredicate<T>` - Single-threaded, cloneable

**Example**:
```rust
use qubit_function::{Predicate, ArcPredicate};

let is_even = ArcPredicate::new(|x: &i32| x % 2 == 0);
let is_positive = ArcPredicate::new(|x: &i32| *x > 0);

let combined = is_even.and(is_positive.clone());
assert!(combined.test(&4));
assert!(!combined.test(&-2));
```

### 2. BiPredicate - Two-Argument Predicate

Tests whether two values satisfy a condition, returning `bool`.

**Trait**: `BiPredicate<T, U>`
**Core Method**: `test(&self, first: &T, second: &U) -> bool`
**Closure Equivalent**: `Fn(&T, &U) -> bool`

**Implementations**:
- `BoxBiPredicate<T, U>` - Single ownership
- `ArcBiPredicate<T, U>` - Thread-safe
- `RcBiPredicate<T, U>` - Single-threaded

**Example**:
```rust
use qubit_function::{BiPredicate, BoxBiPredicate};

let sum_positive = BoxBiPredicate::new(|x: &i32, y: &i32| x + y > 0);
assert!(sum_positive.test(&3, &4));
assert!(!sum_positive.test(&-5, &2));
```

### 3. Consumer - Non-Mutating Consumer

Accepts a value reference and performs side effects without returning a
result. The API uses shared references and does not grant mutable access to
the consumer wrapper or input value.

**Trait**: `Consumer<T>`
**Core Method**: `accept(&self, value: &T)`
**Closure Equivalent**: `Fn(&T)`

**Implementations**:
- `BoxConsumer<T>` - Single ownership
- `ArcConsumer<T>` - Thread-safe
- `RcConsumer<T>` - Single-threaded

**Example**:
```rust
use qubit_function::{Consumer, BoxConsumer};

let logger = BoxConsumer::new(|x: &i32| {
    println!("Value: {}", x);
});
logger.accept(&42);
```

### 4. ConsumerOnce - Single-Use Non-Mutating Consumer

Accepts a value reference and performs side effects once.

**Trait**: `ConsumerOnce<T>`
**Core Method**: `accept(self, value: &T)`
**Closure Equivalent**: `FnOnce(&T)`

**Implementations**:
- `BoxConsumerOnce<T>` - Single ownership, one-time use

### 5. BiConsumer - Two-Argument Non-Mutating Consumer

Accepts two value references and performs side effects without returning a
result. The API uses shared references and does not grant mutable access to
the consumer wrapper or input values.

**Trait**: `BiConsumer<T, U>`
**Core Method**: `accept(&self, first: &T, second: &U)`
**Closure Equivalent**: `Fn(&T, &U)`

**Implementations**:
- `BoxBiConsumer<T, U>` - Single ownership
- `ArcBiConsumer<T, U>` - Thread-safe
- `RcBiConsumer<T, U>` - Single-threaded

**Example**:
```rust
use qubit_function::{BiConsumer, BoxBiConsumer};

let sum_logger = BoxBiConsumer::new(|x: &i32, y: &i32| {
    println!("Sum: {}", x + y);
});
sum_logger.accept(&10, &20);
```

### 6. BiConsumerOnce - Single-Use Two-Argument Non-Mutating Consumer

Accepts two value references and performs side effects once.

**Trait**: `BiConsumerOnce<T, U>`
**Core Method**: `accept(self, first: &T, second: &U)`
**Closure Equivalent**: `FnOnce(&T, &U)`

**Implementations**:
- `BoxBiConsumerOnce<T, U>` - Single ownership, one-time use

### 7. Mutator - Stateless In-Place Mutator

Modifies the target value in place via `&mut T` with no return value. The mutator itself is **stateless** and is invoked with `&self` (equivalent to `Fn(&mut T)`).

**Trait**: `Mutator<T>`
**Core Method**: `apply(&self, value: &mut T)`
**Closure Equivalent**: `Fn(&mut T)`

**Implementations**:
- `BoxMutator<T>` - Single ownership
- `ArcMutator<T>` - Thread-safe
- `RcMutator<T>` - Single-threaded

**Example**:
```rust
use qubit_function::{Mutator, BoxMutator};

let mut doubler = BoxMutator::new(|x: &mut i32| *x *= 2);
let mut value = 10;
doubler.apply(&mut value);
assert_eq!(value, 20);
```

### 8. MutatorOnce - Single-Use In-Place Mutator

May be invoked once to mutate the target in place via `&mut T` (equivalent to `FnOnce(&mut T)`).

**Trait**: `MutatorOnce<T>`
**Core Method**: `apply(self, value: &mut T)`
**Closure Equivalent**: `FnOnce(&mut T)`

**Implementations**:
- `BoxMutatorOnce<T>` - Single ownership, one-time use

### StatefulMutator - Stateful In-Place Mutator

Modifies the target value in place while allowing mutable internal state
(equivalent to `FnMut(&mut T)`).

**Trait**: `StatefulMutator<T>`
**Core Method**: `apply(&mut self, value: &mut T)`
**Closure Equivalent**: `FnMut(&mut T)`

**Implementations**:
- `BoxStatefulMutator<T>` - Single ownership
- `ArcStatefulMutator<T>` - Thread-safe with parking_lot::Mutex
- `RcStatefulMutator<T>` - Single-threaded with RefCell

### 9. Supplier - Stateless Value Supplier

Returns a value of type `T` on each `get` call with no input. The
supplier itself is **stateless** and uses `&self` (equivalent to
`Fn() -> T`).

**Trait**: `Supplier<T>`
**Core Method**: `get(&self) -> T`
**Closure Equivalent**: `Fn() -> T`

**Implementations**:
- `BoxSupplier<T>` - Single ownership, lock-free
- `ArcSupplier<T>` - Thread-safe, lock-free
- `RcSupplier<T>` - Single-threaded

**Example**:
```rust
use qubit_function::{Supplier, BoxSupplier};

let factory = BoxSupplier::new(|| String::from("Hello"));
assert_eq!(factory.get(), "Hello");
```

### 10. SupplierOnce - Single-Use Value Supplier

May invoke `get` only once to return a single `T` (equivalent to
`FnOnce() -> T`).

**Trait**: `SupplierOnce<T>`
**Core Method**: `get(self) -> T`
**Closure Equivalent**: `FnOnce() -> T`

**Implementations**:
- `BoxSupplierOnce<T>` - Single ownership, one-time use

### 11. Callable - Reusable Fallible Computation

Executes a zero-argument computation and returns either a success value
or an error (equivalent to `FnMut() -> Result<R, E>`).

**Trait**: `Callable<R, E>`
**Core Method**: `call(&mut self) -> Result<R, E>`
**Closure Equivalent**: `FnMut() -> Result<R, E>`

**Implementations**:
- `BoxCallable<R, E>` - Reusable single ownership
- `RcCallable<R, E>` - Reusable single-threaded shared ownership
- `ArcCallable<R, E>` - Reusable thread-safe ownership

**Example**:
```rust
use qubit_function::{Callable, BoxCallable};

let mut task = BoxCallable::new(|| Ok::<i32, String>(42));
assert_eq!(task.call(), Ok(42));
```

### 12. Runnable - Reusable Fallible Action

Executes a zero-argument action and reports success or failure
(equivalent to `FnMut() -> Result<(), E>`).

**Trait**: `Runnable<E>`
**Core Method**: `run(&mut self) -> Result<(), E>`
**Closure Equivalent**: `FnMut() -> Result<(), E>`

**Implementations**:
- `BoxRunnable<E>` - Reusable single ownership
- `RcRunnable<E>` - Reusable single-threaded shared ownership
- `ArcRunnable<E>` - Reusable thread-safe ownership

**Example**:
```rust
use qubit_function::{Runnable, BoxRunnable};

let mut task = BoxRunnable::new(|| Ok::<(), String>(()));
assert_eq!(task.run(), Ok(()));
```

### 13. CallableWith - Reusable Fallible Mutable-Input Computation

Executes a computation with caller-provided mutable input and returns either a
success value or an error (equivalent to `FnMut(&mut T) -> Result<R, E>`).

**Trait**: `CallableWith<T, R, E>`
**Core Method**: `call_with(&mut self, input: &mut T) -> Result<R, E>`
**Closure Equivalent**: `FnMut(&mut T) -> Result<R, E>`

**Implementations**:
- `BoxCallableWith<T, R, E>` - Reusable single ownership
- `RcCallableWith<T, R, E>` - Reusable single-threaded shared ownership
- `ArcCallableWith<T, R, E>` - Reusable thread-safe ownership

**Example**:
```rust
use qubit_function::{CallableWith, BoxCallableWith};

let mut value = 40;
let mut task = BoxCallableWith::new(|input: &mut i32| {
    *input += 2;
    Ok::<i32, String>(*input)
});
assert_eq!(task.call_with(&mut value), Ok(42));
```

### 14. RunnableWith - Reusable Fallible Mutable-Input Action

Executes an action with caller-provided mutable input and reports success or
failure (equivalent to `FnMut(&mut T) -> Result<(), E>`).

**Trait**: `RunnableWith<T, E>`
**Core Method**: `run_with(&mut self, input: &mut T) -> Result<(), E>`
**Closure Equivalent**: `FnMut(&mut T) -> Result<(), E>`

**Implementations**:
- `BoxRunnableWith<T, E>` - Reusable single ownership
- `RcRunnableWith<T, E>` - Reusable single-threaded shared ownership
- `ArcRunnableWith<T, E>` - Reusable thread-safe ownership

**Example**:
```rust
use qubit_function::{RunnableWith, BoxRunnableWith};

let mut value = 40;
let mut task = BoxRunnableWith::new(|input: &mut i32| {
    *input += 2;
    Ok::<(), String>(())
});
assert_eq!(task.run_with(&mut value), Ok(()));
assert_eq!(value, 42);
```

### 15. CallableOnce - Single-Use Fallible Computation

Executes a zero-argument computation once and returns either a success value
or an error (equivalent to `FnOnce() -> Result<R, E>`).

**Trait**: `CallableOnce<R, E>`
**Core Method**: `call(self) -> Result<R, E>`
**Closure Equivalent**: `FnOnce() -> Result<R, E>`

**Implementations**:
- `BoxCallableOnce<R, E>` - Sendable single ownership, one-time use
- `LocalBoxCallableOnce<R, E>` - Local single ownership for non-`Send` captures

**Example**:
```rust
use qubit_function::{BoxCallableOnce, CallableOnce};

let task = BoxCallableOnce::new(|| Ok::<i32, String>(42));
assert_eq!(task.call(), Ok(42));
```

### 16. RunnableOnce - Single-Use Fallible Action

Executes a zero-argument action once and reports success or failure
(equivalent to `FnOnce() -> Result<(), E>`).

**Trait**: `RunnableOnce<E>`
**Core Method**: `run(self) -> Result<(), E>`
**Closure Equivalent**: `FnOnce() -> Result<(), E>`

**Implementations**:
- `BoxRunnableOnce<E>` - Sendable single ownership, one-time use
- `LocalBoxRunnableOnce<E>` - Local single ownership for non-`Send` captures

**Example**:
```rust
use qubit_function::{BoxRunnableOnce, RunnableOnce};

let task = BoxRunnableOnce::new(|| Ok::<(), String>(()));
assert_eq!(task.run(), Ok(()));
```

### 17. StatefulSupplier - Stateful Value Supplier

Supplies a `T` using mutable internal state; successive `get` calls may differ (equivalent to `FnMut() -> T`).

**Trait**: `StatefulSupplier<T>`
**Core Method**: `get(&mut self) -> T`
**Closure Equivalent**: `FnMut() -> T`

**Implementations**:
- `BoxStatefulSupplier<T>` - Single ownership
- `ArcStatefulSupplier<T>` - Thread-safe with parking_lot::Mutex
- `RcStatefulSupplier<T>` - Single-threaded with RefCell

**Example**:
```rust
use qubit_function::{StatefulSupplier, BoxStatefulSupplier};

let mut counter = {
    let mut count = 0;
    BoxStatefulSupplier::new(move || {
        count += 1;
        count
    })
};

assert_eq!(counter.get(), 1);
assert_eq!(counter.get(), 2);
```

### 18. Function - Borrowed-Input Function

Computes a result from a borrowed input without consuming the input.

**Trait**: `Function<T, R>`
**Core Method**: `apply(&self, input: &T) -> R`
**Closure Equivalent**: `Fn(&T) -> R`

**Implementations**:
- `BoxFunction<T, R>` - Single ownership
- `ArcFunction<T, R>` - Thread-safe
- `RcFunction<T, R>` - Single-threaded

**Example**:
```rust
use qubit_function::{Function, BoxFunction};

let to_string = BoxFunction::new(|x: &i32| format!("Value: {}", x));
assert_eq!(to_string.apply(&42), "Value: 42");
```

### 19. FunctionOnce - Single-Use Borrowed-Input Function

Computes a result from a borrowed input once.

**Trait**: `FunctionOnce<T, R>`
**Core Method**: `apply(self, input: &T) -> R`
**Closure Equivalent**: `FnOnce(&T) -> R`

**Implementations**:
- `BoxFunctionOnce<T, R>` - Single ownership, one-time use

### 20. StatefulFunction - Stateful Borrowed-Input Function

Computes a result from a borrowed input while allowing mutable internal
state.

**Trait**: `StatefulFunction<T, R>`
**Core Method**: `apply(&mut self, input: &T) -> R`
**Closure Equivalent**: `FnMut(&T) -> R`

**Implementations**:
- `BoxStatefulFunction<T, R>` - Single ownership
- `ArcStatefulFunction<T, R>` - Thread-safe with parking_lot::Mutex
- `RcStatefulFunction<T, R>` - Single-threaded with RefCell

### Additional Function Variants

The function family also includes borrowed bi-input and mutable-input forms:

| Trait | Core Method Signature | Equivalent Closure Type |
|-------|----------------------|------------------------|
| `BiFunction<T, U, R>` | `apply(&self, first: &T, second: &U) -> R` | `Fn(&T, &U) -> R` |
| `BiFunctionOnce<T, U, R>` | `apply(self, first: &T, second: &U) -> R` | `FnOnce(&T, &U) -> R` |
| `MutatingFunction<T, R>` | `apply(&self, value: &mut T) -> R` | `Fn(&mut T) -> R` |
| `MutatingFunctionOnce<T, R>` | `apply(self, value: &mut T) -> R` | `FnOnce(&mut T) -> R` |
| `StatefulMutatingFunction<T, R>` | `apply(&mut self, value: &mut T) -> R` | `FnMut(&mut T) -> R` |
| `BiMutatingFunction<T, U, R>` | `apply(&self, first: &mut T, second: &mut U) -> R` | `Fn(&mut T, &mut U) -> R` |
| `BiMutatingFunctionOnce<T, U, R>` | `apply(self, first: &mut T, second: &mut U) -> R` | `FnOnce(&mut T, &mut U) -> R` |

### 21. Transformer - Value Transformer

Consumes an input value of type `T` and transforms it into a value of
type `R`.

**Trait**: `Transformer<T, R>`
**Core Method**: `apply(&self, input: T) -> R`
**Closure Equivalent**: `Fn(T) -> R`

**Implementations**:
- `BoxTransformer<T, R>` - Single ownership
- `ArcTransformer<T, R>` - Thread-safe
- `RcTransformer<T, R>` - Single-threaded

**Operator Marker and Aliases**: `UnaryOperator<T>` is a marker trait for
`Transformer<T, T>`. `BoxUnaryOperator<T>`, `ArcUnaryOperator<T>`, and
`RcUnaryOperator<T>` are aliases for same-input/output transformer
implementations.

**Example**:
```rust
use qubit_function::{Transformer, BoxTransformer};

let parse = BoxTransformer::new(|s: String| s.parse::<i32>().unwrap_or(0));
assert_eq!(parse.apply("42".to_string()), 42);
```

### 22. TransformerOnce - Single-Use Value Transformer

Consumes an input value once and transforms it into a value of type `R`.

**Trait**: `TransformerOnce<T, R>`
**Core Method**: `apply(self, input: T) -> R`
**Closure Equivalent**: `FnOnce(T) -> R`

**Implementations**:
- `BoxTransformerOnce<T, R>` - Single ownership, one-time use

**Operator Marker and Alias**: `UnaryOperatorOnce<T>` is a marker trait for
`TransformerOnce<T, T>`. `BoxUnaryOperatorOnce<T>` is an alias for
`BoxTransformerOnce<T, T>`.

### 23. StatefulTransformer - Stateful Value Transformer

Consumes an input value and transforms it into a value of type `R`
while allowing mutable internal state.

**Trait**: `StatefulTransformer<T, R>`
**Core Method**: `apply(&mut self, input: T) -> R`
**Closure Equivalent**: `FnMut(T) -> R`

**Implementations**:
- `BoxStatefulTransformer<T, R>` - Single ownership
- `ArcStatefulTransformer<T, R>` - Thread-safe with parking_lot::Mutex
- `RcStatefulTransformer<T, R>` - Single-threaded with RefCell

### 24. BiTransformer - Two-Argument Value Transformer

Consumes two input values and transforms them into a result.

**Trait**: `BiTransformer<T, U, R>`
**Core Method**: `apply(&self, first: T, second: U) -> R`
**Closure Equivalent**: `Fn(T, U) -> R`

**Implementations**:
- `BoxBiTransformer<T, U, R>` - Single ownership
- `ArcBiTransformer<T, U, R>` - Thread-safe
- `RcBiTransformer<T, U, R>` - Single-threaded

**Operator Marker and Aliases**: `BinaryOperator<T>` is a marker trait for
`BiTransformer<T, T, T>`. `BoxBinaryOperator<T>`,
`ArcBinaryOperator<T>`, and `RcBinaryOperator<T>` are aliases for same-type
binary transformer implementations.

**Example**:
```rust
use qubit_function::{BiTransformer, BoxBiTransformer};

let add = BoxBiTransformer::new(|x: i32, y: i32| x + y);
assert_eq!(add.apply(10, 20), 30);
```

### 25. StatefulBiTransformer - Stateful Two-Argument Value Transformer

Consumes two input values and transforms them into a result while
allowing mutable internal state.

**Trait**: `StatefulBiTransformer<T, U, R>`
**Core Method**: `apply(&mut self, first: T, second: U) -> R`
**Closure Equivalent**: `FnMut(T, U) -> R`

**Implementations**:
- `BoxStatefulBiTransformer<T, U, R>` - Single ownership
- `ArcStatefulBiTransformer<T, U, R>` - Thread-safe with parking_lot::Mutex
- `RcStatefulBiTransformer<T, U, R>` - Single-threaded with RefCell

**Stateful Operator Marker and Aliases**:
- `StatefulBinaryOperator<T>` is a marker trait for `StatefulBiTransformer<T, T, T>`
- `BoxStatefulBinaryOperator<T>`, `ArcStatefulBinaryOperator<T>`, `RcStatefulBinaryOperator<T>`

### 26. BiTransformerOnce - Single-Use Two-Argument Value Transformer

Consumes two input values once and transforms them into a result.

**Trait**: `BiTransformerOnce<T, U, R>`
**Core Method**: `apply(self, first: T, second: U) -> R`
**Closure Equivalent**: `FnOnce(T, U) -> R`

**Implementations**:
- `BoxBiTransformerOnce<T, U, R>` - Single ownership, one-time use

**Operator Marker and Alias**: `BinaryOperatorOnce<T>` is a marker trait for
`BiTransformerOnce<T, T, T>`. `BoxBinaryOperatorOnce<T>` is an alias for
`BoxBiTransformerOnce<T, T, T>`.

### 27. StatefulConsumer - Stateful Consumer

Accepts a value reference and performs side effects while allowing
mutable internal state.

**Trait**: `StatefulConsumer<T>`
**Core Method**: `accept(&mut self, value: &T)`
**Closure Equivalent**: `FnMut(&T)`

**Implementations**:
- `BoxStatefulConsumer<T>` - Single ownership
- `ArcStatefulConsumer<T>` - Thread-safe with parking_lot::Mutex
- `RcStatefulConsumer<T>` - Single-threaded with RefCell

### 28. StatefulBiConsumer - Stateful Two-Argument Consumer

Accepts two value references and performs side effects while allowing
mutable internal state.

**Trait**: `StatefulBiConsumer<T, U>`
**Core Method**: `accept(&mut self, first: &T, second: &U)`
**Closure Equivalent**: `FnMut(&T, &U)`

**Implementations**:
- `BoxStatefulBiConsumer<T, U>` - Single ownership
- `ArcStatefulBiConsumer<T, U>` - Thread-safe with parking_lot::Mutex
- `RcStatefulBiConsumer<T, U>` - Single-threaded with RefCell

### 29. Comparator - Ordering Comparator

Compares two values and returns an `Ordering`.

**Trait**: `Comparator<T>`
**Core Method**: `compare(&self, a: &T, b: &T) -> Ordering`
**Closure Equivalent**: `Fn(&T, &T) -> Ordering`

**Implementations**:
- `BoxComparator<T>` - Single ownership
- `ArcComparator<T>` - Thread-safe
- `RcComparator<T>` - Single-threaded

**Example**:
```rust
use qubit_function::{Comparator, BoxComparator};
use std::cmp::Ordering;

let cmp = BoxComparator::new(|a: &i32, b: &i32| a.cmp(b));
assert_eq!(cmp.compare(&5, &3), Ordering::Greater);
```

### 30. Tester - Zero-Argument Condition Checker

Checks whether a condition or state holds without taking input.

**Trait**: `Tester`
**Core Method**: `test(&self) -> bool`
**Closure Equivalent**: `Fn() -> bool`

**Implementations**:
- `BoxTester` - Single ownership
- `ArcTester` - Thread-safe
- `RcTester` - Single-threaded

**Example**:
```rust
use qubit_function::{Tester, BoxTester};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

let flag = Arc::new(AtomicBool::new(true));
let flag_clone = flag.clone();
let tester = BoxTester::new(move || flag_clone.load(Ordering::Relaxed));

assert!(tester.test());
flag.store(false, Ordering::Relaxed);
assert!(!tester.test());
```

## Trait and Closure Correspondence Table

| Trait | Core Method Signature | Equivalent Closure Type |
|-------|----------------------|------------------------|
| `Predicate<T>` | `test(&self, value: &T) -> bool` | `Fn(&T) -> bool` |
| `BiPredicate<T, U>` | `test(&self, first: &T, second: &U) -> bool` | `Fn(&T, &U) -> bool` |
| `Consumer<T>` | `accept(&self, value: &T)` | `Fn(&T)` |
| `ConsumerOnce<T>` | `accept(self, value: &T)` | `FnOnce(&T)` |
| `StatefulConsumer<T>` | `accept(&mut self, value: &T)` | `FnMut(&T)` |
| `BiConsumer<T, U>` | `accept(&self, first: &T, second: &U)` | `Fn(&T, &U)` |
| `BiConsumerOnce<T, U>` | `accept(self, first: &T, second: &U)` | `FnOnce(&T, &U)` |
| `StatefulBiConsumer<T, U>` | `accept(&mut self, first: &T, second: &U)` | `FnMut(&T, &U)` |
| `Mutator<T>` | `apply(&self, value: &mut T)` | `Fn(&mut T)` |
| `MutatorOnce<T>` | `apply(self, value: &mut T)` | `FnOnce(&mut T)` |
| `StatefulMutator<T>` | `apply(&mut self, value: &mut T)` | `FnMut(&mut T)` |
| `Supplier<T>` | `get(&self) -> T` | `Fn() -> T` |
| `SupplierOnce<T>` | `get(self) -> T` | `FnOnce() -> T` |
| `Callable<R, E>` | `call(&mut self) -> Result<R, E>` | `FnMut() -> Result<R, E>` |
| `CallableWith<T, R, E>` | `call_with(&mut self, input: &mut T) -> Result<R, E>` | `FnMut(&mut T) -> Result<R, E>` |
| `CallableOnce<R, E>` | `call(self) -> Result<R, E>` | `FnOnce() -> Result<R, E>` |
| `Runnable<E>` | `run(&mut self) -> Result<(), E>` | `FnMut() -> Result<(), E>` |
| `RunnableWith<T, E>` | `run_with(&mut self, input: &mut T) -> Result<(), E>` | `FnMut(&mut T) -> Result<(), E>` |
| `RunnableOnce<E>` | `run(self) -> Result<(), E>` | `FnOnce() -> Result<(), E>` |
| `StatefulSupplier<T>` | `get(&mut self) -> T` | `FnMut() -> T` |
| `Function<T, R>` | `apply(&self, input: &T) -> R` | `Fn(&T) -> R` |
| `FunctionOnce<T, R>` | `apply(self, input: &T) -> R` | `FnOnce(&T) -> R` |
| `StatefulFunction<T, R>` | `apply(&mut self, input: &T) -> R` | `FnMut(&T) -> R` |
| `BiFunction<T, U, R>` | `apply(&self, first: &T, second: &U) -> R` | `Fn(&T, &U) -> R` |
| `BiFunctionOnce<T, U, R>` | `apply(self, first: &T, second: &U) -> R` | `FnOnce(&T, &U) -> R` |
| `MutatingFunction<T, R>` | `apply(&self, value: &mut T) -> R` | `Fn(&mut T) -> R` |
| `MutatingFunctionOnce<T, R>` | `apply(self, value: &mut T) -> R` | `FnOnce(&mut T) -> R` |
| `StatefulMutatingFunction<T, R>` | `apply(&mut self, value: &mut T) -> R` | `FnMut(&mut T) -> R` |
| `BiMutatingFunction<T, U, R>` | `apply(&self, first: &mut T, second: &mut U) -> R` | `Fn(&mut T, &mut U) -> R` |
| `BiMutatingFunctionOnce<T, U, R>` | `apply(self, first: &mut T, second: &mut U) -> R` | `FnOnce(&mut T, &mut U) -> R` |
| `Transformer<T, R>` | `apply(&self, input: T) -> R` | `Fn(T) -> R` |
| `TransformerOnce<T, R>` | `apply(self, input: T) -> R` | `FnOnce(T) -> R` |
| `StatefulTransformer<T, R>` | `apply(&mut self, input: T) -> R` | `FnMut(T) -> R` |
| `BiTransformer<T, U, R>` | `apply(&self, first: T, second: U) -> R` | `Fn(T, U) -> R` |
| `StatefulBiTransformer<T, U, R>` | `apply(&mut self, first: T, second: U) -> R` | `FnMut(T, U) -> R` |
| `BiTransformerOnce<T, U, R>` | `apply(self, first: T, second: U) -> R` | `FnOnce(T, U) -> R` |
| `Comparator<T>` | `compare(&self, a: &T, b: &T) -> Ordering` | `Fn(&T, &T) -> Ordering` |
| `Tester` | `test(&self) -> bool` | `Fn() -> bool` |

For stateful traits, closure conversions expose `into_fn` / `to_fn`; types that
return mutable closures also expose `into_mut_fn` / `to_mut_fn` for explicitness.

## Implementation Types Comparison

Each trait has multiple implementations based on ownership model:

| Trait | Box (Single) | Arc (Thread-Safe) | Rc (Single-Thread) |
|-------|--------------|-------------------|-------------------|
| Predicate | BoxPredicate | ArcPredicate | RcPredicate |
| BiPredicate | BoxBiPredicate | ArcBiPredicate | RcBiPredicate |
| Consumer | BoxConsumer | ArcConsumer | RcConsumer |
| ConsumerOnce | BoxConsumerOnce | - | - |
| StatefulConsumer | BoxStatefulConsumer | ArcStatefulConsumer | RcStatefulConsumer |
| BiConsumer | BoxBiConsumer | ArcBiConsumer | RcBiConsumer |
| BiConsumerOnce | BoxBiConsumerOnce | - | - |
| StatefulBiConsumer | BoxStatefulBiConsumer | ArcStatefulBiConsumer | RcStatefulBiConsumer |
| Mutator | BoxMutator | ArcMutator | RcMutator |
| MutatorOnce | BoxMutatorOnce | - | - |
| StatefulMutator | BoxStatefulMutator | ArcStatefulMutator | RcStatefulMutator |
| Supplier | BoxSupplier | ArcSupplier | RcSupplier |
| SupplierOnce | BoxSupplierOnce | - | - |
| Callable | BoxCallable | ArcCallable | RcCallable |
| CallableWith | BoxCallableWith | ArcCallableWith | RcCallableWith |
| CallableOnce | BoxCallableOnce, LocalBoxCallableOnce | - | - |
| Runnable | BoxRunnable | ArcRunnable | RcRunnable |
| RunnableWith | BoxRunnableWith | ArcRunnableWith | RcRunnableWith |
| RunnableOnce | BoxRunnableOnce, LocalBoxRunnableOnce | - | - |
| StatefulSupplier | BoxStatefulSupplier | ArcStatefulSupplier | RcStatefulSupplier |
| Function | BoxFunction | ArcFunction | RcFunction |
| FunctionOnce | BoxFunctionOnce | - | - |
| StatefulFunction | BoxStatefulFunction | ArcStatefulFunction | RcStatefulFunction |
| BiFunction | BoxBiFunction | ArcBiFunction | RcBiFunction |
| BiFunctionOnce | BoxBiFunctionOnce | - | - |
| MutatingFunction | BoxMutatingFunction | ArcMutatingFunction | RcMutatingFunction |
| MutatingFunctionOnce | BoxMutatingFunctionOnce | - | - |
| StatefulMutatingFunction | BoxStatefulMutatingFunction | ArcStatefulMutatingFunction | RcStatefulMutatingFunction |
| BiMutatingFunction | BoxBiMutatingFunction | ArcBiMutatingFunction | RcBiMutatingFunction |
| BiMutatingFunctionOnce | BoxBiMutatingFunctionOnce | - | - |
| Transformer | BoxTransformer | ArcTransformer | RcTransformer |
| TransformerOnce | BoxTransformerOnce | - | - |
| StatefulTransformer | BoxStatefulTransformer | ArcStatefulTransformer | RcStatefulTransformer |
| BiTransformer | BoxBiTransformer | ArcBiTransformer | RcBiTransformer |
| UnaryOperator | BoxUnaryOperator | ArcUnaryOperator | RcUnaryOperator |
| UnaryOperatorOnce | BoxUnaryOperatorOnce | - | - |
| BinaryOperator | BoxBinaryOperator | ArcBinaryOperator | RcBinaryOperator |
| BinaryOperatorOnce | BoxBinaryOperatorOnce | - | - |
| StatefulBinaryOperator | BoxStatefulBinaryOperator | ArcStatefulBinaryOperator | RcStatefulBinaryOperator |
| StatefulBiTransformer | BoxStatefulBiTransformer | ArcStatefulBiTransformer | RcStatefulBiTransformer |
| BiTransformerOnce | BoxBiTransformerOnce | - | - |
| Comparator | BoxComparator | ArcComparator | RcComparator |
| Tester | BoxTester | ArcTester | RcTester |

**Legend**:
- **Box**: Single ownership, cannot be cloned, consumes self
- **Arc**: Shared ownership, thread-safe, cloneable
- **Rc**: Shared ownership, single-threaded, cloneable
- **-**: Not applicable (Once types don't need sharing)

## Design Philosophy

This crate adopts the **Trait + Multiple Implementations** pattern:

1. **Unified Interface**: Each functional type has a trait defining core behavior
2. **Specialized Implementations**: Multiple concrete types optimized for different scenarios
3. **Type Preservation**: Composition methods return the same concrete type
4. **Ownership Flexibility**: Choose between single ownership, thread-safe sharing, or single-threaded sharing
5. **High-Performance Concurrency**: Uses parking_lot Mutex for superior synchronization performance
6. **Ergonomic API**: Natural method chaining and functional composition

## Examples

The `examples/` directory contains demonstrations for every major abstraction
family. Run examples with:

```bash
cargo run --example predicate_demo
cargo run --example consumer_demo
cargo run --example function_family_demo
cargo run --example transformer_demo
cargo run --example task_demo
cargo run --example comparator_demo
cargo run --example tester_demo
```

## Documentation

The README and rustdoc are the authoritative user-facing API references.

## License

Licensed under Apache License, Version 2.0.

## Author

Haixing Hu <starfish.hu@gmail.com>
