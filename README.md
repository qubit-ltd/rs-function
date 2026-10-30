# Qubit Function

[![Rust CI](https://github.com/qubit-ltd/rs-function/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-function/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-function/coverage-badge.json)](https://qubit-ltd.github.io/rs-function/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-function.svg?color=blue)](https://crates.io/crates/qubit-function)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![中文文档](https://img.shields.io/badge/文档-中文版-blue.svg)](README.zh_CN.md)

Semantic callback objects for Rust: traits for domain constraints plus Box, Rc,
and Arc wrappers for storing, naming, sharing, and composing callbacks.

## Overview

This crate turns closures and custom callback implementations into explicit
semantic objects. Traits such as `Consumer`, `Predicate`, and `Runnable` express
the invocation contract, while concrete wrappers select single ownership,
single-threaded sharing, or thread-safe sharing. Chaining is implemented on the
wrappers so callbacks can be stored in fields and composed without ambiguous
extension traits.

## Key Features

- **Complete Functional Interface Suite**: broad functional abstraction families with reusable, one-time, stateful, mutating, and fallible variants
- **Thread-safe callback adapters**: Arc stateful adapters serialize callback execution with a `parking_lot::Mutex`
- **Multiple Ownership Models**: Box-based single ownership, LocalBox task wrappers for non-`Send` captures, Arc-based thread-safe sharing, and Rc-based single-threaded sharing
- **Flexible API Design**: Trait-based unified interface with concrete implementations optimized for different scenarios
- **Type-Oriented Module Layout**: Public source files are organized around a single exported type, keeping modules shorter and easier to scan
- **Explicit Method Chaining**: fluent composition starts from a concrete Box, Rc, or Arc wrapper
- **Diagnostic Names**: callback wrappers support chainable `with_name` naming and expose names through `Debug` and `Display`
- **Thread-Safety Options**: Choose between thread-safe (Arc) and efficient single-threaded (Rc) implementations
- **Ergonomic callback abstractions**: Box uses dynamic dispatch, Rc/Arc add reference counting, and stateful Arc adapters add locking

Cargo features keep optional API and dependency costs explicit. `rc` enables
single-threaded shared wrappers, including `RefCell`-backed task wrappers;
`once` enables one-shot families; and `stateful` enables explicit `Stateful*`
families plus `parking_lot::Mutex`-backed Arc task wrappers. Task-oriented
`BoxCallable` and `BoxRunnable` families erase `Send` callbacks so their
composed values can cross executor boundaries; matching `LocalBox*` task
wrappers retain support for non-`Send` captures. Wrapper composition is part
of the baseline API. `full` enables all optional families; the default feature
set is empty.

## Installation

For the core API without optional features, add this to your `Cargo.toml`:

```toml
[dependencies]
qubit-function = "0.18"
```

To enable every optional API used by the examples below:

```toml
[dependencies]
qubit-function = { version = "0.18", features = ["full"] }
```

Unless noted otherwise, examples below assume the `full` feature is enabled.

## Quick Start

When a constructor accepts a semantic callback trait, closure parameter
inference may require an explicit type annotation even if the wrapper's type
arguments are specified. Use `&T` for shared-input callbacks and `&mut T` for
mutable-input callbacks:

```rust
use qubit_function::ArcConsumer;
use qubit_function::ArcMutatingFunction;
use qubit_function::BoxFunction;
use qubit_function::Consumer;
use qubit_function::Function;
use qubit_function::MutatingFunction;

fn main() {
    let function = BoxFunction::<i32, i32>::new(|x: &i32| *x + 1);
    assert_eq!(function.apply(&41), 42);
    let mutator = ArcMutatingFunction::<i32, i32>::new(|x: &mut i32| {
        *x += 1;
        *x
    });
    let mut input = 41;
    assert_eq!(mutator.apply(&mut input), 42);
    assert_eq!(input, 42);
    let consumer = ArcConsumer::<i32>::new(|x: &i32| assert_eq!(*x, 42));
    consumer.accept(&input);
}
```

When an API only invokes a callback immediately, it can accept the semantic
trait directly. This application-defined generic function takes ownership of
the callback value for the call. The callback trait has no `'static` bound, so
the closure may borrow local state that remains valid during the call. This is
an example, not a new method provided by this crate:

```rust
use qubit_function::Consumer;

fn observe<T, C: Consumer<T>>(input: &T, callback: C) {
    callback.accept(input);
}

fn main() {
    let expected = String::from("ready");
    observe(&String::from("ready"), |value: &String| {
        assert_eq!(value, &expected);
    });
    assert_eq!(expected, "ready");
}
```

If the caller needs to keep and reuse the callback, accept it by reference:

```rust
use qubit_function::Consumer;

fn observe_borrowed<T, C: Consumer<T>>(input: &T, callback: &C) {
    callback.accept(input);
}

fn main() {
    let expected = String::from("ready");
    let callback = |value: &String| assert_eq!(value, &expected);
    observe_borrowed(&expected, &callback);
    observe_borrowed(&expected, &callback);
    callback.accept(&expected);
}
```

An application can store an optional hook and notify it after processing an
event. The application calls `notify_processed`; the library only stores,
shares, and invokes the callback. `ArcConsumer` expresses the thread-safe
callback bounds. When no hook is registered, the application skips the
notification. This example shows the wiring only; it does not implement CAS or
HTTP integration, scheduling, error recovery, or a production observability
system:

```rust
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use qubit_function::ArcConsumer;
use qubit_function::Consumer;

struct ProcessedEvent {
    records: usize,
}

#[derive(Default)]
struct ProcessingHooks {
    on_processed: Option<ArcConsumer<ProcessedEvent>>,
}

impl ProcessingHooks {
    fn on_processed<C>(mut self, callback: C) -> Self
    where
        C: Consumer<ProcessedEvent> + Send + Sync + 'static,
    {
        self.on_processed = Some(ArcConsumer::new(callback));
        self
    }

    fn notify_processed(&self, records: usize) {
        if let Some(callback) = &self.on_processed {
            callback.accept(&ProcessedEvent { records });
        }
    }
}

fn main() {
    let completed = Arc::new(AtomicUsize::new(0));
    let observed = Arc::clone(&completed);
    let hooks = ProcessingHooks::default().on_processed(move |event: &ProcessedEvent| {
        observed.fetch_add(event.records, Ordering::SeqCst);
    });
    hooks.notify_processed(3);
    hooks.notify_processed(2);
    assert_eq!(completed.load(Ordering::SeqCst), 5);

    let disabled_hooks = ProcessingHooks::default();
    disabled_hooks.notify_processed(7);
    assert_eq!(completed.load(Ordering::SeqCst), 5);
}
```

Use a wrapper when a callback needs type erasure, storage, sharing, naming, or
composition. Existing wrappers require captured values to be `'static`; this
does not mean the wrapper must live for the whole process or that its input
must be `'static`:

```rust
use qubit_function::BoxConsumer;
use qubit_function::Consumer;

fn main() {
    let expected = String::from("ready");
    let callback = BoxConsumer::new(move |value: &String| {
        assert_eq!(value, &expected);
    });
    callback.accept(&String::from("ready"));
}
```

`move` transfers captured values into the closure but does not extend a
borrow's lifetime. Move owned values such as `String` or `Vec` when storage is
needed, or use `Rc`/`Arc` for shared ownership. `LocalBox*` wrappers remove the
`Send` requirement, but stored captures still need to be `'static`; they do not
store arbitrary stack borrows. The semantic traits themselves impose no
uniform `'static` or thread bound: the API that receives a callback sets those
requirements. If an API accepts a semantic trait directly, pass the callback
without wrapping it unless type erasure, storage, sharing, naming, or
composition is needed.

Directly calling a wrapper can accept an input type with a short-lived borrow.
Here, `ArcTransformer` stores a closure that captures no borrowed state, while
the `&'a str` input is used only during `apply`:

```rust
use qubit_function::{ArcTransformer, Transformer};

fn length<'a>(input: &'a str) -> usize {
    ArcTransformer::<&'a str, usize>::new(|value: &'a str| value.len()).apply(input)
}

let input = String::from("abc");
assert_eq!(length(input.as_str()), 3);
```

Transformer composition methods add separate type bounds: the current
`when`, `or_else`, and `and_then` methods require the relevant input and output
types to be `'static`. These bounds apply to types used in the composition;
they are distinct from the `'static` requirement on a wrapper's stored
callback capture. The existing contract test
[`transformer_when_lifetime_contract_tests.rs`](tests/transformer_when_lifetime_contract_tests.rs)
shows a direct call accepting a borrowed input while a `when(...).or_else(...)`
chain rejects that short-lived type.

Wrappers can also carry diagnostic names for debugging and display. Methods
such as `map` and `map_err` preserve the source name; `and_then` returns a new
unnamed composition and does not concatenate names from its steps. Cloning a
wrapper shares callback state, while each clone can be named independently.
Set a name explicitly on the final composition when needed:

```rust
use qubit_function::{BoxCallable, Callable};

let mut task = BoxCallable::new_with_name("load", || Ok::<i32, String>(20))
    .map(|value| value + 1)
    .and_then(|value| Ok(value * 2))
    .with_name("load-and-transform");
assert_eq!(task.name(), Some("load-and-transform"));
assert_eq!(task.call(), Ok(42));
```

Use a task `Box` wrapper when a composed callback will be submitted to an
executor. The wrapper, including every returned composition, is `Send`:

```rust
use qubit_function::{BoxCallable, Callable};

fn require_send<T: Send>(value: T) -> T {
    value
}

let mut task = require_send(
    BoxCallable::new(|| Ok::<i32, String>(20))
        .map(|value| value + 1)
        .and_then(|value| Ok(value * 2)),
);
assert_eq!(task.call(), Ok(42));
```

Choose `Arc*` for a callback shared by concurrent owners. Choose a task
`LocalBox*` when a single-threaded hook or event loop must capture `Rc` data:

```rust
use std::rc::Rc;
use qubit_function::{Callable, LocalBoxCallable};

let suffix = Rc::new(String::from("!"));
let mut callback = LocalBoxCallable::<String, String>::new(|| {
    Ok(String::from("ready"))
})
.map(move |value| format!("{value}{suffix}"));

assert_eq!(callback.call(), Ok(String::from("ready!")));
```

Stateful Arc wrappers place a `Send` callback behind a mutex. The callback
itself need not be `Sync`; the wrapper provides synchronized shared access:

```rust
use std::cell::Cell;
use qubit_function::{ArcStatefulSupplier, StatefulSupplier};

let counter = Cell::new(0);
let mut next = ArcStatefulSupplier::new(move || {
    counter.set(counter.get() + 1);
    counter.get()
});

assert_eq!(next.get(), 1);
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

#### StatefulPredicate - Stateful Single-Argument Predicate

Use `StatefulPredicate<T>` when the predicate needs native
`FnMut(&T) -> bool` semantics and updates its own state while testing values.

**Trait**: `StatefulPredicate<T>`
**Core Method**: `test(&mut self, value: &T) -> bool`
**Closure Equivalent**: `FnMut(&T) -> bool`

**Implementations**:
- `BoxStatefulPredicate<T>` - Single ownership
- `ArcStatefulPredicate<T>` - Thread-safe with parking_lot::Mutex
- `RcStatefulPredicate<T>` - Single-threaded with RefCell

**Example**:
```rust
use qubit_function::{StatefulPredicate, BoxStatefulPredicate};

let mut seen = 0;
let mut every_other_positive = BoxStatefulPredicate::new(move |x: &i32| {
    seen += 1;
    seen % 2 == 0 && *x > 0
});

assert!(!every_other_positive.test(&5));
assert!(every_other_positive.test(&5));
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

#### StatefulBiPredicate - Stateful Two-Argument Predicate

Use `StatefulBiPredicate<T, U>` when the predicate needs native
`FnMut(&T, &U) -> bool` semantics and updates its own state while testing
two borrowed values.

**Trait**: `StatefulBiPredicate<T, U>`
**Core Method**: `test(&mut self, first: &T, second: &U) -> bool`
**Closure Equivalent**: `FnMut(&T, &U) -> bool`

**Implementations**:
- `BoxStatefulBiPredicate<T, U>` - Single ownership
- `ArcStatefulBiPredicate<T, U>` - Thread-safe with parking_lot::Mutex
- `RcStatefulBiPredicate<T, U>` - Single-threaded with RefCell

**Example**:
```rust
use qubit_function::{StatefulBiPredicate, BoxStatefulBiPredicate};

let mut seen = 0;
let mut every_other_positive_sum =
    BoxStatefulBiPredicate::new(move |x: &i32, y: &i32| {
        seen += 1;
        seen % 2 == 0 && x + y > 0
    });

assert!(!every_other_positive_sum.test(&3, &4));
assert!(every_other_positive_sum.test(&3, &4));
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

### 7. Mutator - Shared-Receiver In-Place Mutator

Modifies the target value in place via `&mut T` with no return value. It is
invoked with `&self` (equivalent to `Fn(&mut T)`), so calling it does not
require `&mut self`; interior mutability and external side effects remain
possible.

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

### 9. Supplier - Shared-Receiver Value Supplier

Returns a value of type `T` on each `get` call with no input. The
supplier uses `&self` (equivalent to `Fn() -> T`). Calling it does not require
`&mut self`; interior mutability and external side effects remain possible.

**Trait**: `Supplier<T>`
**Core Method**: `get(&self) -> T`
**Closure Equivalent**: `Fn() -> T`

**Implementations**:
- `BoxSupplier<T>` - Single ownership
- `ArcSupplier<T>` - Thread-safe; adapter adds no mutex (the callback may still block)
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
- `BoxCallable<R, E>` - Reusable `Send` single ownership for executor tasks
- `LocalBoxCallable<R, E>` - Reusable local ownership for non-`Send` captures
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
- `BoxRunnable<E>` - Reusable `Send` single ownership for executor tasks
- `LocalBoxRunnable<E>` - Reusable local ownership for non-`Send` captures
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
- `BoxCallableWith<T, R, E>` - Reusable `Send` ownership for executor tasks
- `LocalBoxCallableWith<T, R, E>` - Reusable local ownership for non-`Send` captures
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
- `BoxRunnableWith<T, E>` - Reusable `Send` ownership for executor tasks
- `LocalBoxRunnableWith<T, E>` - Reusable local ownership for non-`Send` captures
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
**Core Method**: `call_once(self) -> Result<R, E>`
**Closure Equivalent**: `FnOnce() -> Result<R, E>`

**Implementations**:
- `BoxCallableOnce<R, E>` - Sendable single ownership, one-time use
- `LocalBoxCallableOnce<R, E>` - Local single ownership for non-`Send` captures

**Example**:
```rust
use qubit_function::{BoxCallableOnce, CallableOnce};

let task = BoxCallableOnce::new(|| Ok::<i32, String>(42));
assert_eq!(task.call_once(), Ok(42));
```

### 16. RunnableOnce - Single-Use Fallible Action

Executes a zero-argument action once and reports success or failure
(equivalent to `FnOnce() -> Result<(), E>`).

**Trait**: `RunnableOnce<E>`
**Core Method**: `run_once(self) -> Result<(), E>`
**Closure Equivalent**: `FnOnce() -> Result<(), E>`

**Implementations**:
- `BoxRunnableOnce<E>` - Sendable single ownership, one-time use
- `LocalBoxRunnableOnce<E>` - Local single ownership for non-`Send` captures

**Example**:
```rust
use qubit_function::{BoxRunnableOnce, RunnableOnce};

let task = BoxRunnableOnce::new(|| Ok::<(), String>(()));
assert_eq!(task.run_once(), Ok(()));
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

Supplier mapping keeps the existing `map` method for `Transformer`
implementations. Use `map_once` to transfer captured values with a
`TransformerOnce`, or `map_stateful` when the mapper needs `FnMut` state:

```rust
use qubit_function::{
    BoxStatefulSupplier, BoxSupplierOnce, StatefulSupplier, SupplierOnce,
};

let resource = String::from("owned");
let once = BoxSupplierOnce::new(|| 7).map_once(move |value| (value, resource));
assert_eq!(once.get(), (7, String::from("owned")));

let mut count = 0;
let mut mapped = BoxStatefulSupplier::new(|| 10).map_stateful(move |value| {
    count += 1;
    value + count
});
assert_eq!(mapped.get(), 11);
assert_eq!(mapped.get(), 12);
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

`BoxTransformerOnce::constant`, `BoxBiTransformerOnce::constant`, and
`BoxSupplierOnce::constant` move the value into the one-shot wrapper and return
that original value when called, so the value need not implement `Clone`.
Reusable constants such as `BoxTransformer::constant`,
`BoxBiTransformer::constant`, and `BoxSupplier::constant` still clone on each
call. The Once behavior removes the previous clone side effects: the value is
returned to the caller instead of a clone, and if the wrapper is dropped
without being called, its captured value is dropped then. To preserve the old
per-call cloning behavior, migrate each call site independently:

```rust
use qubit_function::{BoxTransformerOnce, TransformerOnce};

let value = String::from("payload");
let once = BoxTransformerOnce::new(move |_: i32| value.clone());
assert_eq!(once.apply(7), "payload");
```

```rust
use qubit_function::{BiTransformerOnce, BoxBiTransformerOnce};

let value = String::from("payload");
let once = BoxBiTransformerOnce::new(move |_: i32, _: bool| value.clone());
assert_eq!(once.apply(7, false), "payload");
```

```rust
use qubit_function::{BoxSupplierOnce, SupplierOnce};

let value = String::from("payload");
let once = BoxSupplierOnce::new(move || value.clone());
assert_eq!(once.get(), "payload");
```

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

`and_then` continues to accept a `Function` callback. Use
`and_then_stateful` when the next stage needs to retain state through
`StatefulFunction` or an `FnMut` closure. It requires the `stateful` feature;
the Rc variants also require `rc`. Box consumes its source, while Rc and Arc
borrow it and share its source state. Clones of a composed Rc or Arc wrapper
also share the mapper state. A composed wrapper starts without a diagnostic
name and can be named with `with_name`.

```rust
use qubit_function::BoxStatefulFunction;
use qubit_function::BoxStatefulMutatingFunction;
use qubit_function::StatefulMutatingFunction;

let mut source_calls = 0;
let source = BoxStatefulMutatingFunction::new(move |input: &mut i32| {
    source_calls += 1;
    *input += source_calls;
    *input
});
let mut mapper_calls = 0;
let mapper = BoxStatefulFunction::new(move |result: &i32| {
    mapper_calls += 1;
    *result + mapper_calls
});
let mut pipeline = source.and_then_stateful(mapper);
let mut input = 5;
assert_eq!(pipeline.apply(&mut input), 7);
assert_eq!(pipeline.apply(&mut input), 10);
assert_eq!(input, 8);
```

The two outputs are `7` and `10`, and the final input is `8`.

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
use qubit_function::{BoxTester, Tester};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

let flag = Arc::new(AtomicBool::new(true));
let flag_clone = flag.clone();
let tester = BoxTester::new(move || flag_clone.load(Ordering::Relaxed));

assert!(tester.test());
flag.store(false, Ordering::Relaxed);
assert!(!tester.test());
```

#### StatefulTester - Stateful Zero-Argument Condition Checker

Use `StatefulTester` when a zero-argument condition needs native
`FnMut() -> bool` semantics and updates its own state while testing.

**Trait**: `StatefulTester`
**Core Method**: `test(&mut self) -> bool`
**Closure Equivalent**: `FnMut() -> bool`

**Implementations**:
- `BoxStatefulTester` - Single ownership
- `ArcStatefulTester` - Thread-safe with parking_lot::Mutex
- `RcStatefulTester` - Single-threaded with RefCell

**Example**:
```rust
use qubit_function::{BoxStatefulTester, StatefulTester};

let mut calls = 0;
let mut every_second_call = BoxStatefulTester::new(move || {
    calls += 1;
    calls % 2 == 0
});

assert!(!every_second_call.test());
assert!(every_second_call.test());
```

## Trait and Closure Correspondence Table

| Trait | Core Method Signature | Equivalent Closure Type |
|-------|----------------------|------------------------|
| `Predicate<T>` | `test(&self, value: &T) -> bool` | `Fn(&T) -> bool` |
| `StatefulPredicate<T>` | `test(&mut self, value: &T) -> bool` | `FnMut(&T) -> bool` |
| `BiPredicate<T, U>` | `test(&self, first: &T, second: &U) -> bool` | `Fn(&T, &U) -> bool` |
| `StatefulBiPredicate<T, U>` | `test(&mut self, first: &T, second: &U) -> bool` | `FnMut(&T, &U) -> bool` |
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
| `CallableOnce<R, E>` | `call_once(self) -> Result<R, E>` | `FnOnce() -> Result<R, E>` |
| `Runnable<E>` | `run(&mut self) -> Result<(), E>` | `FnMut() -> Result<(), E>` |
| `RunnableWith<T, E>` | `run_with(&mut self, input: &mut T) -> Result<(), E>` | `FnMut(&mut T) -> Result<(), E>` |
| `RunnableOnce<E>` | `run_once(self) -> Result<(), E>` | `FnOnce() -> Result<(), E>` |
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
| `StatefulTester` | `test(&mut self) -> bool` | `FnMut() -> bool` |

## Implementation Types Comparison

Each trait has multiple implementations based on ownership model:

| Trait | Box (Single) | Arc (Thread-Safe) | Rc (Single-Thread) |
|-------|--------------|-------------------|-------------------|
| Predicate | BoxPredicate | ArcPredicate | RcPredicate |
| StatefulPredicate | BoxStatefulPredicate | ArcStatefulPredicate | RcStatefulPredicate |
| BiPredicate | BoxBiPredicate | ArcBiPredicate | RcBiPredicate |
| StatefulBiPredicate | BoxStatefulBiPredicate | ArcStatefulBiPredicate | RcStatefulBiPredicate |
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
| Callable | BoxCallable, LocalBoxCallable | ArcCallable | RcCallable |
| CallableWith | BoxCallableWith, LocalBoxCallableWith | ArcCallableWith | RcCallableWith |
| CallableOnce | BoxCallableOnce, LocalBoxCallableOnce | - | - |
| Runnable | BoxRunnable, LocalBoxRunnable | ArcRunnable | RcRunnable |
| RunnableWith | BoxRunnableWith, LocalBoxRunnableWith | ArcRunnableWith | RcRunnableWith |
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
| StatefulTester | BoxStatefulTester | ArcStatefulTester | RcStatefulTester |

**Legend**:
- **Box**: Single ownership and dynamic dispatch; task Box wrappers are `Send`
- **LocalBox**: Single ownership for task callbacks with non-`Send` captures
- **Arc**: Shared ownership, thread-safe, cloneable
- **Rc**: Shared ownership, single-threaded, cloneable
- **-**: Not applicable (Once types don't need sharing)

## Design Philosophy

This crate adopts the **Trait + Multiple Implementations** pattern:

1. **Unified Interface**: Each functional type has a trait defining core behavior
2. **Specialized Implementations**: Multiple concrete types optimized for different scenarios
3. **Ownership-Aware Composition**: Applicable composition methods return the same wrapper family
4. **Ownership Flexibility**: Choose between single ownership, thread-safe sharing, or single-threaded sharing
5. **Thread-safe callbacks**: Stateful Arc adapters serialize calls with a mutex; callbacks run while the lock is held
6. **Ergonomic API**: Natural method chaining and functional composition

Stateful Rc wrappers share one `RefCell`-backed callback across clones and hold
the mutable borrow while user code runs; synchronous re-entry panics. Stateful
Arc wrappers similarly share one `parking_lot::Mutex`-backed callback and hold
the lock while user code runs; synchronous re-entry deadlocks. A panic does not
roll back state changes made before the panic, and `parking_lot::Mutex` is not
poisoned.

## Examples

The `examples/` directory contains demonstrations for every major abstraction
family. Run examples with:

```bash
cargo run --features full --example predicate_demo
cargo run --features full --example consumer_demo
cargo run --features full --example function_family_demo
cargo run --features full --example transformer_demo
cargo run --features full --example task_demo
cargo run --features full --example comparator_demo
cargo run --features full --example tester_demo
```

## Learn More

- [API documentation](https://docs.rs/qubit-function/0.18.3/qubit_function/) for the complete type and method reference.
- The online API page depends on this version being published and its docs.rs build succeeding. To build the docs from the repository root, run `cargo doc --all-features --no-deps --open`; without a desktop environment, omit `--open`. By default, the entry page is `target/doc/qubit_function/index.html`; when `CARGO_TARGET_DIR` is set, use that target directory instead.
- **Submit a task and obtain its result**: start with [`Callable`](https://docs.rs/qubit-function/0.18.3/qubit_function/trait.Callable.html) ([source](src/tasks/callable/callable.rs)) or [`Runnable`](https://docs.rs/qubit-function/0.18.3/qubit_function/trait.Runnable.html) ([source](src/tasks/runnable/runnable.rs)). These describe callable tasks; the application or executor handles submission, scheduling, and result processing.
- **Share observation notifications**: see [`ArcConsumer`](https://docs.rs/qubit-function/0.18.3/qubit_function/struct.ArcConsumer.html) ([source](src/consumers/consumer/arc_consumer.rs)). The crate provides a thread-safe shared callback wrapper; the application decides when notifications are delivered and how to handle them.
- **Modify HTTP request data**: see [`ArcMutatingFunction`](https://docs.rs/qubit-function/0.18.3/qubit_function/struct.ArcMutatingFunction.html) ([source](src/functions/mutating_function/arc_mutating_function.rs)). The crate provides a mutable-input callback; HTTP integration, invocation timing, and business handling belong to the application or downstream.
- [简体中文 README](README.zh_CN.md)

## Testing

```bash
# Run tests with the default feature set
cargo test

# Run tests with all declared features
cargo test --all-features

# Project CI checks
./.infra/bin/ci-check.sh

# Check code coverage
./.infra/bin/coverage.sh
```

## License

Copyright (c) 2025 - 2026. Haixing Hu. All rights reserved.

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for the
full license text.

## Contributing

Contributions are welcome. Please follow the Rust API guidelines, keep public
API documentation and tests current, and run `./.infra/bin/align-ci.sh` to format code and
`./.infra/bin/ci-check.sh` to satisfy CI requirements before submitting a pull request.

## Author

**Haixing Hu** - *Qubit Co. Ltd.*

Repository: [https://github.com/qubit-ltd/rs-function](https://github.com/qubit-ltd/rs-function)
