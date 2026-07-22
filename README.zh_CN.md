# Qubit Function

[![Rust CI](https://github.com/qubit-ltd/rs-function/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-function/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-function/coverage-badge.json)](https://qubit-ltd.github.io/rs-function/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-function.svg?color=blue)](https://crates.io/crates/qubit-function)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![English Document](https://img.shields.io/badge/Document-English-blue.svg)](README.md)

为 Rust 提供语义回调对象：用 trait 表达领域约束，用 Box、Rc 和 Arc
包装器保存、命名、共享并组合回调。

## 概述

本 crate 把闭包和自定义回调实现转换为显式的语义对象。`Consumer`、
`Predicate`、`Runnable` 等 trait 表达调用契约，具体包装器分别表达单一所有权、
单线程共享和线程安全共享。链式组合定义在包装器上，因此回调可以方便地保存在字段中，
同时避免闭包扩展 trait 的方法歧义。

## 核心特性

- **完整的函数式接口套件**: 覆盖可复用、一次性、有状态、可变输入和可失败任务等函数式抽象家族
- **线程安全回调适配器**: Arc 有状态适配器通过 `parking_lot::Mutex` 串行执行回调
- **多种所有权模型**: 基于 Box 的单一所有权、支持非 `Send` 捕获的 LocalBox 任务包装器、基于 Arc 的线程安全共享、基于 Rc 的单线程共享
- **灵活的 API 设计**: 基于 trait 的统一接口,针对不同场景优化的具体实现
- **面向类型的模块布局**: 公开源码文件围绕单一导出类型组织,模块更短,更易阅读和定位
- **显式方法链式调用**: 从具体的 Box、Rc 或 Arc 包装器开始流畅组合
- **诊断命名**: 回调包装器支持链式 `with_name` 命名，并通过 `Debug` 和 `Display` 输出名称
- **线程安全选项**: 在线程安全(Arc)和高效单线程(Rc)实现之间选择
- **易用的回调抽象**: Box 包含动态分发成本，Rc/Arc 包含引用计数成本，有状态 Arc 适配器还包含加锁成本

Cargo feature 明确划分可选 API 和依赖成本：`rc` 启用单线程共享包装器，
包括基于 `RefCell` 的任务包装器；`once` 启用一次性调用家族；`stateful`
启用显式的 `Stateful*` 家族，以及基于 `parking_lot::Mutex` 的 Arc 任务包装器。
面向任务的 `BoxCallable` 和 `BoxRunnable` 家族擦除为 `Send` 回调，因此组合后的值
可以跨越执行器边界；对应的 `LocalBox*` 任务包装器继续支持非 `Send` 捕获。
包装器组合属于基础 API。`full` 启用全部可选家族；默认 feature 为空。

## 安装

只使用不含可选 feature 的核心 API 时，在 `Cargo.toml` 中添加：

```toml
[dependencies]
qubit-function = "0.17"
```

如需启用下文示例使用的全部可选 API：

```toml
[dependencies]
qubit-function = { version = "0.17", features = ["full"] }
```

除非特别说明，下文示例均假定已启用 `full` feature。

## 快速开始

组合后的回调需要提交给执行器时，使用任务 `Box` 包装器。包装器及其组合结果均为
`Send`：

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

并发所有者需要共享回调时选择 `Arc*`。单线程钩子或事件循环需要捕获 `Rc` 数据时，
选择任务 `LocalBox*`：

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

有状态 Arc 包装器将 `Send` 回调放在互斥锁之后。回调自身无需实现 `Sync`，同步共享
访问由包装器提供：

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

## 核心抽象

本 crate 提供一组广泛的函数式抽象,并在适合的地方提供所有权感知的实现。下方章节介绍主要家族,汇总表覆盖额外的 mutating、bi-function 和 operator 变体。

### 1. Predicate - 单参数谓词

判断一个值是否满足条件,返回 `bool`。

**Trait**: `Predicate<T>`
**核心方法**: `test(&self, value: &T) -> bool`
**等价闭包**: `Fn(&T) -> bool`

**实现类型**:
- `BoxPredicate<T>` - 单一所有权,不可克隆
- `ArcPredicate<T>` - 线程安全,可克隆
- `RcPredicate<T>` - 单线程,可克隆

**示例**:
```rust
use qubit_function::{Predicate, ArcPredicate};

let is_even = ArcPredicate::new(|x: &i32| x % 2 == 0);
let is_positive = ArcPredicate::new(|x: &i32| *x > 0);

let combined = is_even.and(is_positive.clone());
assert!(combined.test(&4));
assert!(!combined.test(&-2));
```

#### StatefulPredicate - 有状态单参数谓词

当谓词需要原生 `FnMut(&T) -> bool` 语义,并且在判断值时更新自身状态,
使用 `StatefulPredicate<T>`。

**Trait**: `StatefulPredicate<T>`
**核心方法**: `test(&mut self, value: &T) -> bool`
**等价闭包**: `FnMut(&T) -> bool`

**实现类型**:
- `BoxStatefulPredicate<T>` - 单一所有权
- `ArcStatefulPredicate<T>` - 线程安全(使用 parking_lot::Mutex)
- `RcStatefulPredicate<T>` - 单线程(使用 RefCell)

**示例**:
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

### 2. BiPredicate - 双参数谓词

判断两个值是否满足条件,返回 `bool`。

**Trait**: `BiPredicate<T, U>`
**核心方法**: `test(&self, first: &T, second: &U) -> bool`
**等价闭包**: `Fn(&T, &U) -> bool`

**实现类型**:
- `BoxBiPredicate<T, U>` - 单一所有权
- `ArcBiPredicate<T, U>` - 线程安全
- `RcBiPredicate<T, U>` - 单线程

**示例**:
```rust
use qubit_function::{BiPredicate, BoxBiPredicate};

let sum_positive = BoxBiPredicate::new(|x: &i32, y: &i32| x + y > 0);
assert!(sum_positive.test(&3, &4));
assert!(!sum_positive.test(&-5, &2));
```

#### StatefulBiPredicate - 有状态双参数谓词

当谓词需要原生 `FnMut(&T, &U) -> bool` 语义,并且在判断两个借用值时更新自身状态,
使用 `StatefulBiPredicate<T, U>`。

**Trait**: `StatefulBiPredicate<T, U>`
**核心方法**: `test(&mut self, first: &T, second: &U) -> bool`
**等价闭包**: `FnMut(&T, &U) -> bool`

**实现类型**:
- `BoxStatefulBiPredicate<T, U>` - 单一所有权
- `ArcStatefulBiPredicate<T, U>` - 线程安全(使用 parking_lot::Mutex)
- `RcStatefulBiPredicate<T, U>` - 单线程(使用 RefCell)

**示例**:
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

### 3. Consumer - 非修改型消费者

接受值引用并执行带副作用的操作,不返回结果。API 使用共享引用,
不会向消费者包装器或输入值授予可变访问权。

**Trait**: `Consumer<T>`
**核心方法**: `accept(&self, value: &T)`
**等价闭包**: `Fn(&T)`

**实现类型**:
- `BoxConsumer<T>` - 单一所有权
- `ArcConsumer<T>` - 线程安全
- `RcConsumer<T>` - 单线程

**示例**:
```rust
use qubit_function::{Consumer, BoxConsumer};

let logger = BoxConsumer::new(|x: &i32| {
    println!("值: {}", x);
});
logger.accept(&42);
```

### 4. ConsumerOnce - 一次性非修改型消费者

接受值引用并执行一次带副作用的操作。

**Trait**: `ConsumerOnce<T>`
**核心方法**: `accept(self, value: &T)`
**等价闭包**: `FnOnce(&T)`

**实现类型**:
- `BoxConsumerOnce<T>` - 单一所有权,一次性使用

### 5. BiConsumer - 双参数非修改型消费者

接受两个值引用并执行带副作用的操作,不返回结果。API 使用共享引用,
不会向消费者包装器或输入值授予可变访问权。

**Trait**: `BiConsumer<T, U>`
**核心方法**: `accept(&self, first: &T, second: &U)`
**等价闭包**: `Fn(&T, &U)`

**实现类型**:
- `BoxBiConsumer<T, U>` - 单一所有权
- `ArcBiConsumer<T, U>` - 线程安全
- `RcBiConsumer<T, U>` - 单线程

**示例**:
```rust
use qubit_function::{BiConsumer, BoxBiConsumer};

let sum_logger = BoxBiConsumer::new(|x: &i32, y: &i32| {
    println!("和: {}", x + y);
});
sum_logger.accept(&10, &20);
```

### 6. BiConsumerOnce - 一次性双参数非修改型消费者

接受两个值引用并执行一次带副作用的操作。

**Trait**: `BiConsumerOnce<T, U>`
**核心方法**: `accept(self, first: &T, second: &U)`
**等价闭包**: `FnOnce(&T, &U)`

**实现类型**:
- `BoxBiConsumerOnce<T, U>` - 单一所有权,一次性使用

### 7. Mutator - 共享接收者原地修改器

通过可变引用**原地**修改目标值,无返回值; 以 `&self` 调用(对应
`Fn(&mut T)`),因此调用不需要 `&mut self`; 仍可使用内部可变性或产生外部副作用。

**Trait**: `Mutator<T>`
**核心方法**: `apply(&self, value: &mut T)`
**等价闭包**: `Fn(&mut T)`

**实现类型**:
- `BoxMutator<T>` - 单一所有权
- `ArcMutator<T>` - 线程安全
- `RcMutator<T>` - 单线程

**示例**:
```rust
use qubit_function::{Mutator, BoxMutator};

let mut doubler = BoxMutator::new(|x: &mut i32| *x *= 2);
let mut value = 10;
doubler.apply(&mut value);
assert_eq!(value, 20);
```

### 8. MutatorOnce - 一次性原地修改器

仅可调用一次,通过可变引用原地修改目标值(对应 `FnOnce(&mut T)`)。

**Trait**: `MutatorOnce<T>`
**核心方法**: `apply(self, value: &mut T)`
**等价闭包**: `FnOnce(&mut T)`

**实现类型**:
- `BoxMutatorOnce<T>` - 单一所有权,一次性使用

### StatefulMutator - 有状态原地修改器

通过可变引用原地修改目标值,同时允许修改自身内部状态(对应
`FnMut(&mut T)`)。

**Trait**: `StatefulMutator<T>`
**核心方法**: `apply(&mut self, value: &mut T)`
**等价闭包**: `FnMut(&mut T)`

**实现类型**:
- `BoxStatefulMutator<T>` - 单一所有权
- `ArcStatefulMutator<T>` - 线程安全(使用 parking_lot::Mutex)
- `RcStatefulMutator<T>` - 单线程(使用 RefCell)

### 9. Supplier - 共享接收者值提供者

无参数,每次调用 `get` 都返回一个 `T`; 以 `&self` 调用(对应
`Fn() -> T`),因此调用不需要 `&mut self`; 仍可使用内部可变性或产生外部副作用。

**Trait**: `Supplier<T>`
**核心方法**: `get(&self) -> T`
**等价闭包**: `Fn() -> T`

**实现类型**:
- `BoxSupplier<T>` - 单一所有权,无锁
- `ArcSupplier<T>` - 线程安全,无锁
- `RcSupplier<T>` - 单线程

**示例**:
```rust
use qubit_function::{Supplier, BoxSupplier};

let factory = BoxSupplier::new(|| String::from("你好"));
assert_eq!(factory.get(), "你好");
```

### 10. SupplierOnce - 一次性值提供者

无参数,仅能调用一次 `get` 以返回一个 `T`(对应 `FnOnce() -> T`)。

**Trait**: `SupplierOnce<T>`
**核心方法**: `get(self) -> T`
**等价闭包**: `FnOnce() -> T`

**实现类型**:
- `BoxSupplierOnce<T>` - 单一所有权,一次性使用

### 11. Callable - 可复用可失败计算

无参数,可多次执行计算,并返回成功值或错误(对应
`FnMut() -> Result<R, E>`)。

**Trait**: `Callable<R, E>`
**核心方法**: `call(&mut self) -> Result<R, E>`
**等价闭包**: `FnMut() -> Result<R, E>`

**实现类型**:
- `BoxCallable<R, E>` - 面向执行器任务的可复用 `Send` 单一所有权
- `LocalBoxCallable<R, E>` - 支持非 `Send` 捕获的本地可复用所有权
- `RcCallable<R, E>` - 可复用单线程共享所有权
- `ArcCallable<R, E>` - 可复用线程安全共享所有权

**示例**:
```rust
use qubit_function::{Callable, BoxCallable};

let mut task = BoxCallable::new(|| Ok::<i32, String>(42));
assert_eq!(task.call(), Ok(42));
```

### 12. Runnable - 可复用可失败动作

无参数,可重复执行动作,并报告成功或失败(对应
`FnMut() -> Result<(), E>`)。

**Trait**: `Runnable<E>`
**核心方法**: `run(&mut self) -> Result<(), E>`
**等价闭包**: `FnMut() -> Result<(), E>`

**实现类型**:
- `BoxRunnable<E>` - 面向执行器任务的可复用 `Send` 单一所有权
- `LocalBoxRunnable<E>` - 支持非 `Send` 捕获的本地可复用所有权
- `RcRunnable<E>` - 可复用单线程共享所有权
- `ArcRunnable<E>` - 可复用线程安全共享所有权

**示例**:
```rust
use qubit_function::{Runnable, BoxRunnable};

let mut task = BoxRunnable::new(|| Ok::<(), String>(()));
assert_eq!(task.run(), Ok(()));
```

### 13. CallableWith - 可复用可失败可变输入计算

接收调用方提供的可变输入并执行计算,返回成功值或错误(对应
`FnMut(&mut T) -> Result<R, E>`)。

**Trait**: `CallableWith<T, R, E>`
**核心方法**: `call_with(&mut self, input: &mut T) -> Result<R, E>`
**等价闭包**: `FnMut(&mut T) -> Result<R, E>`

**实现类型**:
- `BoxCallableWith<T, R, E>` - 面向执行器任务的可复用 `Send` 所有权
- `LocalBoxCallableWith<T, R, E>` - 支持非 `Send` 捕获的本地可复用所有权
- `RcCallableWith<T, R, E>` - 可复用单线程共享所有权
- `ArcCallableWith<T, R, E>` - 可复用线程安全共享所有权

**示例**:
```rust
use qubit_function::{CallableWith, BoxCallableWith};

let mut value = 40;
let mut task = BoxCallableWith::new(|input: &mut i32| {
    *input += 2;
    Ok::<i32, String>(*input)
});
assert_eq!(task.call_with(&mut value), Ok(42));
```

### 14. RunnableWith - 可复用可失败可变输入动作

接收调用方提供的可变输入并执行动作,只报告成功或失败(对应
`FnMut(&mut T) -> Result<(), E>`)。

**Trait**: `RunnableWith<T, E>`
**核心方法**: `run_with(&mut self, input: &mut T) -> Result<(), E>`
**等价闭包**: `FnMut(&mut T) -> Result<(), E>`

**实现类型**:
- `BoxRunnableWith<T, E>` - 面向执行器任务的可复用 `Send` 所有权
- `LocalBoxRunnableWith<T, E>` - 支持非 `Send` 捕获的本地可复用所有权
- `RcRunnableWith<T, E>` - 可复用单线程共享所有权
- `ArcRunnableWith<T, E>` - 可复用线程安全共享所有权

**示例**:
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

### 15. CallableOnce - 一次性可失败计算

无参数,仅执行一次计算,并返回成功值或错误(对应
`FnOnce() -> Result<R, E>`)。

**Trait**: `CallableOnce<R, E>`
**核心方法**: `call_once(self) -> Result<R, E>`
**等价闭包**: `FnOnce() -> Result<R, E>`

**实现类型**:
- `BoxCallableOnce<R, E>` - 可跨线程移动的单一所有权一次性任务
- `LocalBoxCallableOnce<R, E>` - 支持非 `Send` 捕获的本地一次性任务

**示例**:
```rust
use qubit_function::{BoxCallableOnce, CallableOnce};

let task = BoxCallableOnce::new(|| Ok::<i32, String>(42));
assert_eq!(task.call_once(), Ok(42));
```

### 16. RunnableOnce - 一次性可失败动作

无参数,仅执行一次动作,并报告成功或失败(对应
`FnOnce() -> Result<(), E>`)。

**Trait**: `RunnableOnce<E>`
**核心方法**: `run_once(self) -> Result<(), E>`
**等价闭包**: `FnOnce() -> Result<(), E>`

**实现类型**:
- `BoxRunnableOnce<E>` - 可跨线程移动的单一所有权一次性任务
- `LocalBoxRunnableOnce<E>` - 支持非 `Send` 捕获的本地一次性任务

**示例**:
```rust
use qubit_function::{BoxRunnableOnce, RunnableOnce};

let task = BoxRunnableOnce::new(|| Ok::<(), String>(()));
assert_eq!(task.run_once(), Ok(()));
```

### 17. StatefulSupplier - 有状态值提供者

在可变内部状态下返回 `T`; 多次 `get` 的结果可以不同(对应
`FnMut() -> T`)。

**Trait**: `StatefulSupplier<T>`
**核心方法**: `get(&mut self) -> T`
**等价闭包**: `FnMut() -> T`

**实现类型**:
- `BoxStatefulSupplier<T>` - 单一所有权
- `ArcStatefulSupplier<T>` - 线程安全(使用 parking_lot::Mutex)
- `RcStatefulSupplier<T>` - 单线程(使用 RefCell)

**示例**:
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

### 18. Function - 借用输入函数

基于借用输入计算结果,不消耗输入。

**Trait**: `Function<T, R>`
**核心方法**: `apply(&self, input: &T) -> R`
**等价闭包**: `Fn(&T) -> R`

**实现类型**:
- `BoxFunction<T, R>` - 单一所有权
- `ArcFunction<T, R>` - 线程安全
- `RcFunction<T, R>` - 单线程

**示例**:
```rust
use qubit_function::{Function, BoxFunction};

let to_string = BoxFunction::new(|x: &i32| format!("值: {}", x));
assert_eq!(to_string.apply(&42), "值: 42");
```

### 19. FunctionOnce - 一次性借用输入函数

基于借用输入计算一次结果。

**Trait**: `FunctionOnce<T, R>`
**核心方法**: `apply(self, input: &T) -> R`
**等价闭包**: `FnOnce(&T) -> R`

**实现类型**:
- `BoxFunctionOnce<T, R>` - 单一所有权,一次性使用

### 20. StatefulFunction - 有状态借用输入函数

基于借用输入计算结果,并允许修改内部状态。

**Trait**: `StatefulFunction<T, R>`
**核心方法**: `apply(&mut self, input: &T) -> R`
**等价闭包**: `FnMut(&T) -> R`

**实现类型**:
- `BoxStatefulFunction<T, R>` - 单一所有权
- `ArcStatefulFunction<T, R>` - 线程安全(使用 parking_lot::Mutex)
- `RcStatefulFunction<T, R>` - 单线程(使用 RefCell)

### 额外 Function 变体

Function 家族还包含借用双输入和可变输入形式：

| Trait | 核心方法签名 | 等价闭包类型 |
|-------|------------|-------------|
| `BiFunction<T, U, R>` | `apply(&self, first: &T, second: &U) -> R` | `Fn(&T, &U) -> R` |
| `BiFunctionOnce<T, U, R>` | `apply(self, first: &T, second: &U) -> R` | `FnOnce(&T, &U) -> R` |
| `MutatingFunction<T, R>` | `apply(&self, value: &mut T) -> R` | `Fn(&mut T) -> R` |
| `MutatingFunctionOnce<T, R>` | `apply(self, value: &mut T) -> R` | `FnOnce(&mut T) -> R` |
| `StatefulMutatingFunction<T, R>` | `apply(&mut self, value: &mut T) -> R` | `FnMut(&mut T) -> R` |
| `BiMutatingFunction<T, U, R>` | `apply(&self, first: &mut T, second: &mut U) -> R` | `Fn(&mut T, &mut U) -> R` |
| `BiMutatingFunctionOnce<T, U, R>` | `apply(self, first: &mut T, second: &mut U) -> R` | `FnOnce(&mut T, &mut U) -> R` |

### 21. Transformer - 值转换器

取得输入值的所有权,并将类型 `T` 的值转换为类型 `R` 的值。

**Trait**: `Transformer<T, R>`
**核心方法**: `apply(&self, input: T) -> R`
**等价闭包**: `Fn(T) -> R`

**实现类型**:
- `BoxTransformer<T, R>` - 单一所有权
- `ArcTransformer<T, R>` - 线程安全
- `RcTransformer<T, R>` - 单线程

**运算符标记 trait 与别名**: `UnaryOperator<T>` 是
`Transformer<T, T>` 的标记 trait。`BoxUnaryOperator<T>`、
`ArcUnaryOperator<T>` 和 `RcUnaryOperator<T>` 是同输入/输出类型
transformer 实现的别名。

**示例**:
```rust
use qubit_function::{Transformer, BoxTransformer};

let parse = BoxTransformer::new(|s: String| s.parse::<i32>().unwrap_or(0));
assert_eq!(parse.apply("42".to_string()), 42);
```

### 22. TransformerOnce - 一次性值转换器

一次性取得输入值的所有权,并将其转换为类型 `R` 的值。

**Trait**: `TransformerOnce<T, R>`
**核心方法**: `apply(self, input: T) -> R`
**等价闭包**: `FnOnce(T) -> R`

**实现类型**:
- `BoxTransformerOnce<T, R>` - 单一所有权,一次性使用

**运算符标记 trait 与别名**: `UnaryOperatorOnce<T>` 是
`TransformerOnce<T, T>` 的标记 trait。`BoxUnaryOperatorOnce<T>` 是
`BoxTransformerOnce<T, T>` 的别名。

### 23. StatefulTransformer - 有状态值转换器

取得输入值的所有权并完成转换,同时允许修改内部状态。

**Trait**: `StatefulTransformer<T, R>`
**核心方法**: `apply(&mut self, input: T) -> R`
**等价闭包**: `FnMut(T) -> R`

**实现类型**:
- `BoxStatefulTransformer<T, R>` - 单一所有权
- `ArcStatefulTransformer<T, R>` - 线程安全(使用 parking_lot::Mutex)
- `RcStatefulTransformer<T, R>` - 单线程(使用 RefCell)

### 24. BiTransformer - 双参数值转换器

取得两个输入值的所有权,并将其转换为结果。

**Trait**: `BiTransformer<T, U, R>`
**核心方法**: `apply(&self, first: T, second: U) -> R`
**等价闭包**: `Fn(T, U) -> R`

**实现类型**:
- `BoxBiTransformer<T, U, R>` - 单一所有权
- `ArcBiTransformer<T, U, R>` - 线程安全
- `RcBiTransformer<T, U, R>` - 单线程

**运算符标记 trait 与别名**: `BinaryOperator<T>` 是
`BiTransformer<T, T, T>` 的标记 trait。`BoxBinaryOperator<T>`、
`ArcBinaryOperator<T>` 和 `RcBinaryOperator<T>` 是同类型二元
transformer 实现的别名。

**示例**:
```rust
use qubit_function::{BiTransformer, BoxBiTransformer};

let add = BoxBiTransformer::new(|x: i32, y: i32| x + y);
assert_eq!(add.apply(10, 20), 30);
```

### 25. StatefulBiTransformer - 有状态双参数值转换器

取得两个输入值的所有权并完成转换,同时允许修改内部状态。

**Trait**: `StatefulBiTransformer<T, U, R>`
**核心方法**: `apply(&mut self, first: T, second: U) -> R`
**等价闭包**: `FnMut(T, U) -> R`

**实现类型**:
- `BoxStatefulBiTransformer<T, U, R>` - 单一所有权
- `ArcStatefulBiTransformer<T, U, R>` - 线程安全(使用 parking_lot::Mutex)
- `RcStatefulBiTransformer<T, U, R>` - 单线程(使用 RefCell)

**有状态运算符标记 trait 与别名**:
- `StatefulBinaryOperator<T>` 是 `StatefulBiTransformer<T, T, T>` 的标记 trait
- `BoxStatefulBinaryOperator<T>`、`ArcStatefulBinaryOperator<T>`、`RcStatefulBinaryOperator<T>`

### 26. BiTransformerOnce - 一次性双参数值转换器

一次性取得两个输入值的所有权,并将其转换为结果。

**Trait**: `BiTransformerOnce<T, U, R>`
**核心方法**: `apply(self, first: T, second: U) -> R`
**等价闭包**: `FnOnce(T, U) -> R`

**实现类型**:
- `BoxBiTransformerOnce<T, U, R>` - 单一所有权,一次性使用

**运算符标记 trait 与别名**: `BinaryOperatorOnce<T>` 是
`BiTransformerOnce<T, T, T>` 的标记 trait。`BoxBinaryOperatorOnce<T>`
是 `BoxBiTransformerOnce<T, T, T>` 的别名。

### 27. StatefulConsumer - 有状态消费者

接受值引用并执行带副作用的操作,同时允许修改内部状态。

**Trait**: `StatefulConsumer<T>`
**核心方法**: `accept(&mut self, value: &T)`
**等价闭包**: `FnMut(&T)`

**实现类型**:
- `BoxStatefulConsumer<T>` - 单一所有权
- `ArcStatefulConsumer<T>` - 线程安全(使用 parking_lot::Mutex)
- `RcStatefulConsumer<T>` - 单线程(使用 RefCell)

### 28. StatefulBiConsumer - 有状态双参数消费者

接受两个值引用并执行带副作用的操作,同时允许修改内部状态。

**Trait**: `StatefulBiConsumer<T, U>`
**核心方法**: `accept(&mut self, first: &T, second: &U)`
**等价闭包**: `FnMut(&T, &U)`

**实现类型**:
- `BoxStatefulBiConsumer<T, U>` - 单一所有权
- `ArcStatefulBiConsumer<T, U>` - 线程安全(使用 parking_lot::Mutex)
- `RcStatefulBiConsumer<T, U>` - 单线程(使用 RefCell)

### 29. Comparator - 排序比较器

比较两个值并返回 `Ordering`。

**Trait**: `Comparator<T>`
**核心方法**: `compare(&self, a: &T, b: &T) -> Ordering`
**等价闭包**: `Fn(&T, &T) -> Ordering`

**实现类型**:
- `BoxComparator<T>` - 单一所有权
- `ArcComparator<T>` - 线程安全
- `RcComparator<T>` - 单线程

**示例**:
```rust
use qubit_function::{Comparator, BoxComparator};
use std::cmp::Ordering;

let cmp = BoxComparator::new(|a: &i32, b: &i32| a.cmp(b));
assert_eq!(cmp.compare(&5, &3), Ordering::Greater);
```

### 30. Tester - 无参条件判定器

在不接收参数的前提下,判断某一状态或条件是否成立。

**Trait**: `Tester`
**核心方法**: `test(&self) -> bool`
**等价闭包**: `Fn() -> bool`

**实现类型**:
- `BoxTester` - 单一所有权
- `ArcTester` - 线程安全
- `RcTester` - 单线程

**示例**:
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

#### StatefulTester - 有状态无参条件判定器

当无参条件判断需要原生 `FnMut() -> bool` 语义,并且在判断时更新自身状态,
使用 `StatefulTester`。

**Trait**: `StatefulTester`
**核心方法**: `test(&mut self) -> bool`
**等价闭包**: `FnMut() -> bool`

**实现类型**:
- `BoxStatefulTester` - 单一所有权
- `ArcStatefulTester` - 线程安全(使用 parking_lot::Mutex)
- `RcStatefulTester` - 单线程(使用 RefCell)

**示例**:
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

## Trait 与闭包对应表

| Trait | 核心方法签名 | 等价闭包类型 |
|-------|------------|-------------|
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

## 实现类型对比

每个 trait 基于所有权模型都有多种实现:

| Trait | Box(单一所有权) | Arc(线程安全) | Rc(单线程) |
|-------|----------------|--------------|-----------|
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

**图例**:
- **Box**: 单一所有权和动态分发；任务 Box 包装器实现 `Send`
- **LocalBox**: 用于捕获非 `Send` 数据的单一所有权任务回调
- **Arc**: 共享所有权,线程安全,可克隆
- **Rc**: 共享所有权,单线程,可克隆
- **-**: 不适用(Once 类型不需要共享)

## 设计理念

本 crate 采用 **Trait + 多实现** 模式:

1. **统一接口**: 每个函数式类型都有一个定义核心行为的 trait
2. **专门实现**: 针对不同场景优化的多个具体类型
3. **所有权感知的组合**: 适用的组合方法返回相同包装器家族
4. **所有权灵活性**: 在单一所有权、线程安全共享或单线程共享之间选择
5. **线程安全回调**: 有状态 Arc 适配器通过互斥锁串行调用；执行回调期间会持有锁
6. **易用 API**: 自然的方法链式调用和函数组合

有状态 Rc 包装器的克隆共享同一个基于 `RefCell` 的回调，并在用户代码执行期间持有
可变借用；同步重入会 panic。有状态 Arc 包装器同样共享一个基于
`parking_lot::Mutex` 的回调，并在用户代码执行期间持锁；同步重入会死锁。
panic 不会回滚发生在 panic 前的状态修改，且 `parking_lot::Mutex` 不会中毒。

## 示例

`examples/` 目录包含每个主要抽象家族的演示。运行示例:

```bash
cargo run --features full --example predicate_demo
cargo run --features full --example consumer_demo
cargo run --features full --example function_family_demo
cargo run --features full --example transformer_demo
cargo run --features full --example task_demo
cargo run --features full --example comparator_demo
cargo run --features full --example tester_demo
```

## 测试

```bash
# 使用默认 feature 集运行测试
cargo test

# 使用项目声明的全部 feature 运行测试
cargo test --all-features

# 运行项目 CI 检查
./ci-check.sh

# 检查代码覆盖率
./coverage.sh
```

## 许可证

Copyright (c) 2025 - 2026. Haixing Hu. All rights reserved.

本项目基于 Apache License 2.0 授权。完整许可证文本请参阅
[LICENSE](LICENSE)。

## 贡献

欢迎贡献。请遵循 Rust API 指南，及时更新公共 API 文档与测试，并在提交
Pull Request 前运行 `./align-ci.sh`格式化代码，运行`./ci-check.sh`对齐CI要求。

## 作者

**Haixing Hu** - *Qubit Co. Ltd.*

仓库地址：[https://github.com/qubit-ltd/rs-function](https://github.com/qubit-ltd/rs-function)
