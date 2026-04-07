# Mapper 设计方案分析

## 概述

本文档分析是否需要实现一个 `Mapper` trait（等价于 `FnMut(T) -> R`），探讨其语义定位、与现有抽象的关系，以及是否应该将其添加到 `qubit-atomic` 库中。

Mapper 的核心功能是**将一个类型的值转换为另一个类型的值，并在转换过程中可以修改自身状态**，类似于 Rust 标准库中的 `FnMut(T) -> R`。本文将深入分析 Mapper 的设计价值，并提出符合实际业务需求的解决方案。

---

## 一、Mapper 的本质语义

### 1.1 什么是 Mapper？

**Mapper（映射器）的核心语义**：

> **将一个类型的值转换为另一个类型的值，并且在转换过程中可以修改自己的内部状态。这是一个"有状态的转换"操作，消耗输入产生输出，同时可能改变自身状态。**

从函数签名看：

```rust
pub trait Mapper<T, R> {
    fn apply(&mut self, input: T) -> R;  // 消耗输入，可修改自己
}
```

这对应于 Rust 标准库中的 `FnMut(T) -> R` trait。

**对比其他函数式抽象**：

| 类型 | 输入 | 输出 | self 签名 | 消耗输入？ | 修改自己？ | 典型用途 |
|------|------|------|-----------|-----------|-----------|---------|
| **Transformer** | `T` | `R` | `&self` | ✅ | ❌ | 纯函数转换、类型转换 |
| **Mapper** | `T` | `R` | `&mut self` | ✅ | ✅ | 有状态转换、组合管道 |
| **Predicate** | `&T` | `bool` | `&self` | ❌ | ❌ | 过滤、验证 |
| **Consumer** | `&T` | `()` | `&mut self` | ❌ | ✅ | 观察、统计 |
| **Supplier** | 无 | `T` | `&mut self` | N/A | ✅ | 工厂、生成 |

**关键洞察**：

- Mapper 是 **Transformer** + **状态修改能力**
- Mapper 在输入/输出关系上类似 Transformer，但允许修改自身状态
- Mapper 与 Consumer 类似都可以修改自身，但 Mapper 产生输出而 Consumer 不产生

### 1.2 Mapper 的主要用途

| 用途 | 描述 | 示例 |
|------|------|------|
| **有状态转换** | 带计数器、缓存的转换 | 计数转换、记忆化 |
| **组合管道** | 预先组合多个有状态转换器 | `mapper1.and_then(mapper2)` |
| **策略模式** | 将转换逻辑作为可切换策略 | `mappers.get("strategy")` |
| **复用逻辑** | 保存为字段，多次使用 | `struct { pipeline: BoxMapper<T, R> }` |
| **动态构建** | 根据配置动态组合转换器 | `if config { mapper.and_then(step) }` |

### 1.3 Mapper 的核心价值

**Mapper vs 直接使用闭包**：

```rust
// ❌ 不需要 Mapper：简单的一次性转换
let mut counter = 0;
let results: Vec<_> = vec![1, 2, 3]
    .into_iter()
    .map(|x| {
        counter += 1;
        format!("Item #{}: {}", counter, x)
    })
    .collect();

// ✅ 需要 Mapper：预先组合复杂的有状态转换器
let mut counter1 = 0;
let mapper1 = BoxMapper::new(move |x: i32| {
    counter1 += 1;
    format!("Step1[{}]: {}", counter1, x)
});

let mut counter2 = 0;
let mapper2 = BoxMapper::new(move |s: String| {
    counter2 += 1;
    format!("{} -> Step2[{}]", s, counter2)
});

// 组合成一个管道
let mut pipeline = mapper1.and_then(mapper2);

// 可以保存并复用
let mut supplier = BoxSupplier::new(|| 10).map(pipeline);
```

**Mapper 的价值在于**：
1. **组合能力**：可以预先组合多个有状态的转换器
2. **复用性**：可以保存为结构体字段，多次使用
3. **策略模式**：可以动态选择和切换不同的 mapper
4. **统一接口**：为闭包、函数指针、Mapper 对象提供统一接口

---

## 二、核心设计决策

### 2.1 为什么需要 Mapper？Transformer + RefCell 不够吗？

这是设计 Mapper 时最关键的问题。让我们对比两种方案：

#### 方案 A：使用 Transformer + RefCell（内部可变性）

```rust
// 场景：带计数的转换器
use std::cell::RefCell;

let counter = RefCell::new(0);
let transformer = BoxTransformer::new(move |x: i32| {
    let mut c = counter.borrow_mut();
    *c += 1;
    format!("Item #{}: {}", *c, x)
});

// ✅ 用户代码不需要 mut
assert_eq!(transformer.apply(100), "Item #1: 100");
assert_eq!(transformer.apply(200), "Item #2: 200");
```

**优点**：
- ✅ 用户代码不需要 `mut`
- ✅ 可以在不可变上下文使用
- ✅ 符合纯函数的外观

**缺点**：
- ❌ 需要理解 RefCell 的概念（学习成本）
- ❌ 运行时借用检查开销
- ❌ 可能 panic（borrow_mut 失败）
- ❌ **无法预先组合多个有状态转换器**（关键！）

#### 方案 B：使用 Mapper trait（外部可变性）

```rust
// 场景：带计数的转换器
let mut counter = 0;
let mut mapper = BoxMapper::new(move |x: i32| {
    counter += 1;
    format!("Item #{}: {}", counter, x)
});

assert_eq!(mapper.apply(100), "Item #1: 100");
assert_eq!(mapper.apply(200), "Item #2: 200");
```

**优点**：
- ✅ 简单直观，无需理解 RefCell
- ✅ 无运行时开销
- ✅ 不会 panic
- ✅ **可以预先组合多个有状态转换器**（关键！）

**缺点**：
- ⚠️ 用户代码需要 `mut`
- ⚠️ 需要可变性传播

### 2.2 Mapper 的真正价值：组合能力

**关键洞察**：Mapper 的核心价值不在于简单的有状态转换（这可以用 Transformer + RefCell），而在于**预先组合多个有状态转换器**。

#### 场景 1：预先组合有状态转换管道

```rust
// ❌ 使用 FnMut：无法预先组合
let mut counter1 = 0;
let mut counter2 = 0;

// 无法将这两个有状态闭包组合成一个对象
let mut supplier = BoxSupplier::new(|| 10)
    .map(|x| {
        counter1 += 1;
        format!("Step1[{}]: {}", counter1, x)
    })
    .map(|s| {
        counter2 += 1;
        format!("{} -> Step2[{}]", s, counter2)
    });

// 问题：counter1 和 counter2 在不同的闭包中，无法复用组合
```

```rust
// ✅ 使用 Mapper：可以预先组合
let mut counter1 = 0;
let mapper1 = BoxMapper::new(move |x: i32| {
    counter1 += 1;
    format!("Step1[{}]: {}", counter1, x)
});

let mut counter2 = 0;
let mapper2 = BoxMapper::new(move |s: String| {
    counter2 += 1;
    format!("{} -> Step2[{}]", s, counter2)
});

// ✅ 预先组合成一个管道
let mut combined = mapper1.and_then(mapper2);

// ✅ 可以直接传给 Supplier::map
let mut supplier = BoxSupplier::new(|| 10).map(combined);

assert_eq!(supplier.get(), "Step1[1]: 10 -> Step2[1]");
assert_eq!(supplier.get(), "Step1[2]: 10 -> Step2[2]");
```

#### 场景 2：可复用的有状态转换管道

```rust
// ✅ 使用 Mapper：可以预先构建复杂的有状态管道
struct DataProcessor {
    // 保存预先组合好的 mapper
    pipeline: BoxMapper<RawData, ProcessedData>,
}

impl DataProcessor {
    fn new() -> Self {
        // 组合多个有状态的转换步骤
        let step1 = BoxMapper::new(|data: RawData| {
            // 有状态的解析
            parse_with_cache(data)
        });

        let step2 = BoxMapper::new(|parsed: ParsedData| {
            // 有状态的验证
            validate_with_counter(parsed)
        });

        let step3 = BoxMapper::new(|validated: ValidatedData| {
            // 有状态的转换
            transform_with_history(validated)
        });

        // ✅ 预先组合成一个管道
        let pipeline = step1.and_then(step2).and_then(step3);

        DataProcessor { pipeline }
    }

    fn process(&mut self, data: RawData) -> ProcessedData {
        self.pipeline.map(data)
    }
}

// ✅ 可以直接用在 Supplier::map
let mut supplier = BoxSupplier::new(|| fetch_raw_data())
    .map(processor.pipeline);
```

#### 场景 3：动态选择和组合 Mapper

```rust
// ✅ 使用 Mapper 的 when/or_else：根据条件选择不同的处理策略
fn build_mapper_with_condition(threshold: i32) -> BoxMapper<i32, String> {
    let mut high_counter = 0;
    let mut low_counter = 0;

    BoxMapper::new(move |x| {
        high_counter += 1;
        format!("High[{}]: {} * 2 = {}", high_counter, x, x * 2)
    })
    .when(|x: &i32| *x >= threshold)  // 当输入 >= threshold 时使用上面的 mapper
    .or_else(move |x| {                // 否则使用这个 mapper
        low_counter += 1;
        format!("Low[{}]: {} + 1 = {}", low_counter, x, x + 1)
    })
}

// ✅ 使用 when 和 or_else 实现策略模式
fn build_processing_mapper(mode: ProcessingMode) -> BoxMapper<Data, Result<Data, Error>> {
    let mut fast_count = 0;
    let mut slow_count = 0;

    BoxMapper::new(move |data| {
        fast_count += 1;
        fast_process(data, fast_count)
    })
    .when(move |_| mode == ProcessingMode::Fast)
    .or_else(move |data| {
        slow_count += 1;
        slow_but_accurate_process(data, slow_count)
    })
}

// ✅ 链式组合多个 when/or_else
fn build_validation_mapper() -> BoxMapper<Input, Output> {
    let mut valid_count = 0;
    let mut invalid_count = 0;
    let mut error_count = 0;

    // 首先检查是否有效
    BoxMapper::new(move |input| {
        valid_count += 1;
        process_valid(input, valid_count)
    })
    .when(|input: &Input| input.is_valid())
    .or_else(move |input| {
        // 如果无效，再检查是否可以修复
        BoxMapper::new(move |input| {
            invalid_count += 1;
            fix_and_process(input, invalid_count)
        })
        .when(|input: &Input| input.is_fixable())
        .or_else(move |input| {
            // 无法修复，返回错误
            error_count += 1;
            handle_error(input, error_count)
        })
        .map(input)
    })
}

// ✅ 使用
let mut mapper = build_mapper_with_condition(10);
assert_eq!(mapper.apply(15), "High[1]: 15 * 2 = 30");
assert_eq!(mapper.apply(5), "Low[1]: 5 + 1 = 6");
assert_eq!(mapper.apply(20), "High[2]: 20 * 2 = 40");

let mut supplier = BoxSupplier::new(|| get_input())
    .map(build_validation_mapper());
```

#### 场景 4：Mapper 作为策略对象

```rust
// ✅ 使用 Mapper：可以保存和切换策略
struct DataPipeline {
    mappers: HashMap<String, BoxMapper<Data, Data>>,
    current_strategy: String,
}

impl DataPipeline {
    fn set_strategy(&mut self, name: &str) {
        self.current_strategy = name.to_string();
    }

    fn process(&mut self, data: Data) -> Data {
        let mapper = self.mappers.get_mut(&self.current_strategy).unwrap();
        mapper.apply(data)
    }
}

// ✅ 可以在运行时切换不同的有状态 mapper
pipeline.set_strategy("aggressive");
let result1 = pipeline.process(data1);

pipeline.set_strategy("conservative");
let result2 = pipeline.process(data2);
```

### 2.3 Mapper vs Transformer + RefCell 的价值对比

| 能力 | Mapper trait | Transformer + RefCell | 优势方 |
|------|-------------|----------------------|--------|
| **简单闭包** | ✅ 可以 | ✅ 更简洁 | RefCell |
| **有状态闭包** | ✅ 可以 | ✅ 可以 | 平局 |
| **预先组合** | ✅ **可以** | ❌ **不可以** | **Mapper** |
| **动态组合** | ✅ **可以** | ❌ **不可以** | **Mapper** |
| **保存为字段** | ✅ **可以** | ⚠️ 困难 | **Mapper** |
| **策略模式** | ✅ **可以** | ❌ **不可以** | **Mapper** |
| **无运行时开销** | ✅ | ❌ RefCell 检查 | **Mapper** |
| **学习成本** | 🟡 需要理解 `&mut` | 🔴 需要理解 RefCell | **Mapper** |

**重要发现**：Mapper 在**组合**和**复用**方面有明显优势！

### 2.4 为什么 Supplier::map 应该使用 Mapper trait？

这是一个关键的设计决策：**Supplier::map 应该接受 `Mapper<T, U>` trait 还是 `FnMut(T) -> U`？**

#### 当前推荐：使用 Mapper trait

```rust
impl<T> BoxSupplier<T> {
    pub fn map<U, F>(mut self, mut mapper: F) -> BoxSupplier<U>
    where
        F: Mapper<T, U> + 'static,  // ✅ 使用 Mapper trait
        U: 'static,
    {
        BoxSupplier::new(move || mapper.apply(self.get()))
    }
}
```

**为什么用 Mapper 而不是 FnMut？**

1. **支持组合的 Mapper 对象**：可以传入预先组合好的 BoxMapper
2. **统一接口**：闭包自动实现 Mapper，无缝集成
3. **更好的类型表达**：Mapper 明确表达"有状态的映射"语义
4. **与标准库兼容**：通过为 FnMut 实现 Mapper 保持兼容

```rust
// ✅ 场景 1：简单闭包（自动实现 Mapper）
let mut supplier = BoxSupplier::new(|| 10)
    .map(|x| x * 2);  // 闭包自动实现 Mapper

// ✅ 场景 2：组合的 Mapper 对象
let mapper = BoxMapper::new(|x: i32| x * 2)
    .and_then(|x| x + 5)
    .and_then(|x| format!("Result: {}", x));

let mut supplier = BoxSupplier::new(|| 10)
    .map(mapper);  // 直接传入组合后的 Mapper

// ✅ 场景 3：有状态的组合
let mut counter1 = 0;
let mut counter2 = 0;

let mapper = BoxMapper::new(move |x: i32| {
    counter1 += 1;
    x + counter1
}).and_then(move |x| {
    counter2 += 1;
    x * counter2
});

let mut supplier = BoxSupplier::new(|| 10).map(mapper);

assert_eq!(supplier.get(), (10 + 1) * 1);  // 11
assert_eq!(supplier.get(), (10 + 2) * 2);  // 24
```

### 2.5 Mapper 与 Map-Reduce 模式的关系

**重要区分**：Mapper 不是 Map-Reduce 中的 Mapper！

在经典的 Map-Reduce 模式中，Mapper 应该是**无状态的纯函数**：

```rust
// Map-Reduce 的 Mapper：应该用 Transformer（Fn）
use rayon::prelude::*;

let results: Vec<_> = data.par_iter()
    .map(|x| transformer.apply(x))  // Transformer（纯函数）
    .collect();
```

我们的 Mapper trait 是用于**顺序执行的有状态转换**：

```rust
// 顺序执行的 Mapper：可以有状态
let mut counter = 0;
let mut mapper = BoxMapper::new(move |x: i32| {
    counter += 1;
    x + counter
});

// 顺序调用
assert_eq!(mapper.apply(10), 11);
assert_eq!(mapper.apply(10), 12);
```

| 场景 | 应该使用 | 原因 |
|------|---------|------|
| **Map-Reduce（并行）** | Transformer（`Fn`）| 无状态，可并行 |
| **顺序转换管道** | Mapper（`FnMut`）| 可以有状态，顺序执行 |
| **Supplier::map** | Mapper（`FnMut`）| 顺序执行，允许状态 |

---

## 三、实现方案：Trait 抽象 + 多种实现

参考 Transformer、Consumer、Supplier 的设计，采用统一的 Trait + 多种实现方案。

### 3.1 核心架构

```rust
// ============================================================================
// 1. 最小化的 Mapper trait
// ============================================================================

/// 映射器 - 有状态的值转换器（可重复调用）
pub trait Mapper<T, R> {
    /// 转换输入值（可修改自身状态）
    fn apply(&mut self, input: T) -> R;

    // 类型转换方法
    fn into_box(self) -> BoxMapper<T, R>
        where Self: Sized + 'static, T: 'static, R: 'static;
    fn into_rc(self) -> RcMapper<T, R>
        where Self: Sized + 'static, T: 'static, R: 'static;
    fn into_arc(self) -> ArcMapper<T, R>
        where Self: Sized + Send + 'static, T: Send + 'static, R: Send + 'static;
}

// ============================================================================
// 2. 为闭包实现 Mapper trait（关键！）
// ============================================================================

/// 为 FnMut 闭包实现 Mapper trait
impl<T, R, F> Mapper<T, R> for F
where
    F: FnMut(T) -> R,
{
    fn apply(&mut self, input: T) -> R {
        self(input)
    }

    fn into_box(self) -> BoxMapper<T, R>
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        BoxMapper::new(self)
    }

    // ... 其他转换方法
}

// ============================================================================
// 3. BoxMapper - 单一所有权实现
// ============================================================================

pub struct BoxMapper<T, R> {
    function: Box<dyn FnMut(T) -> R>,
}

impl<T, R> BoxMapper<T, R>
where
    T: 'static,
    R: 'static,
{
    pub fn new<F>(f: F) -> Self
    where
        F: FnMut(T) -> R + 'static,
    {
        BoxMapper { function: Box::new(f) }
    }

    /// 恒等映射
    pub fn identity() -> BoxMapper<T, T> {
        BoxMapper::new(|x| x)
    }

    /// 链式组合：self -> after
    pub fn and_then<S, F>(mut self, mut after: F) -> BoxMapper<T, S>
    where
        F: Mapper<R, S> + 'static,
        S: 'static,
    {
        BoxMapper::new(move |x: T| {
            let intermediate = self.map(x);
            after.map(intermediate)
        })
    }

    /// 反向组合：before -> self
    pub fn compose<S, F>(mut self, mut before: F) -> BoxMapper<S, R>
    where
        F: Mapper<S, T> + 'static,
        S: 'static,
    {
        BoxMapper::new(move |x: S| {
            let intermediate = before.map(x);
            self.map(intermediate)
        })
    }

    /// 条件组合：当谓词满足时应用此 mapper，否则使用 or_else 提供的 mapper
    ///
    /// # 参数
    ///
    /// * `predicate` - 用于判断是否应用此 mapper 的谓词
    ///
    /// # 返回值
    ///
    /// 返回 `BoxConditionalMapper<T, R>`，可以继续调用 `or_else` 方法
    ///
    /// # 示例
    ///
    /// ```rust
    /// let mut counter = 0;
    /// let mut mapper = BoxMapper::new(move |x: i32| {
    ///     counter += 1;
    ///     x * 2
    /// })
    /// .when(|x: &i32| *x > 10)
    /// .or_else(|x| x + 1);
    ///
    /// assert_eq!(mapper.apply(15), 30);  // 15 > 10，应用 * 2
    /// assert_eq!(mapper.apply(5), 6);    // 5 <= 10，应用 + 1
    /// ```
    pub fn when<P>(self, predicate: P) -> BoxConditionalMapper<T, R>
    where
        P: Predicate<T> + 'static,
    {
        BoxConditionalMapper {
            mapper: self,
            predicate: predicate.into_box(),
        }
    }
}

impl<T, R> Mapper<T, R> for BoxMapper<T, R> {
    fn apply(&mut self, input: T) -> R {
        (self.function)(input)
    }
    // ...
}

// ============================================================================
// 3.5. BoxConditionalMapper - 条件映射器
// ============================================================================

pub struct BoxConditionalMapper<T, R> {
    mapper: BoxMapper<T, R>,
    predicate: BoxPredicate<T>,
}

impl<T, R> BoxConditionalMapper<T, R>
where
    T: Clone + 'static,
    R: 'static,
{
    /// 提供 else 分支的 mapper
    ///
    /// 如果谓词满足，使用 when 中的 mapper；否则使用此方法提供的 mapper
    ///
    /// # 参数
    ///
    /// * `else_mapper` - 当谓词不满足时使用的 mapper
    ///
    /// # 返回值
    ///
    /// 返回一个新的 `BoxMapper<T, R>`
    ///
    /// # 示例
    ///
    /// ```rust
    /// let mut counter = 0;
    /// let mut mapper = BoxMapper::new(move |x: i32| {
    ///     counter += 1;
    ///     format!("Even[{}]: {}", counter, x)
    /// })
    /// .when(|x: &i32| x % 2 == 0)
    /// .or_else(move |x| {
    ///     counter += 1;
    ///     format!("Odd[{}]: {}", counter, x)
    /// });
    ///
    /// assert_eq!(mapper.apply(10), "Even[1]: 10");
    /// assert_eq!(mapper.apply(11), "Odd[2]: 11");
    /// ```
    pub fn or_else<F>(mut self, mut else_mapper: F) -> BoxMapper<T, R>
    where
        F: Mapper<T, R> + 'static,
    {
        let pred = self.predicate;
        let mut then_mapper = self.mapper;
        BoxMapper::new(move |t: T| {
            if pred.test(&t) {
                then_mapper.apply(t)
            } else {
                else_mapper.apply(t)
            }
        })
    }
}

// ============================================================================
// 4. ArcMapper - 线程安全共享实现
// ============================================================================

pub struct ArcMapper<T, R> {
    function: Arc<Mutex<dyn FnMut(T) -> R + Send>>,
}

impl<T, R> ArcMapper<T, R>
where
    T: Send + 'static,
    R: Send + 'static,
{
    pub fn new<F>(f: F) -> Self
    where
        F: FnMut(T) -> R + Send + 'static,
    {
        ArcMapper {
            function: Arc::new(Mutex::new(f)),
        }
    }

    /// 链式组合（使用 &self，不消耗）
    pub fn and_then<S, F>(&self, after: F) -> ArcMapper<T, S>
    where
        F: Mapper<R, S> + Send + 'static,
        S: Send + 'static,
    {
        let self_fn = Arc::clone(&self.function);
        let after = Arc::new(Mutex::new(after));
        ArcMapper {
            function: Arc::new(Mutex::new(move |x: T| {
                let intermediate = self_fn.lock().unwrap()(x);
                after.lock().unwrap().map(intermediate)
            })),
        }
    }
}

impl<T, R> Mapper<T, R> for ArcMapper<T, R> {
    fn apply(&mut self, input: T) -> R {
        (self.function.lock().unwrap())(input)
    }
    // ...
}

impl<T, R> Clone for ArcMapper<T, R> {
    fn clone(&self) -> Self {
        ArcMapper {
            function: Arc::clone(&self.function),
        }
    }
}

// ============================================================================
// 5. RcMapper - 单线程共享实现
// ============================================================================

pub struct RcMapper<T, R> {
    function: Rc<RefCell<dyn FnMut(T) -> R>>,
}

// 类似 ArcMapper 的实现...

// ============================================================================
// 6. Supplier::map 使用 Mapper trait
// ============================================================================

impl<T> BoxSupplier<T> {
    pub fn map<U, F>(mut self, mut mapper: F) -> BoxSupplier<U>
    where
        F: Mapper<T, U> + 'static,  // ✅ 使用 Mapper trait
        U: 'static,
    {
        BoxSupplier::new(move || mapper.apply(self.get()))
    }
}
```

### 3.2 使用示例

```rust
// ============================================================================
// 1. 闭包自动拥有 Mapper 能力
// ============================================================================

let mut counter = 0;
let mut mapper = |x: i32| {
    counter += 1;
    format!("Item #{}: {}", counter, x)
};

// 闭包自动实现 Mapper
assert_eq!(mapper.apply(100), "Item #1: 100");
assert_eq!(mapper.apply(200), "Item #2: 200");

// ============================================================================
// 2. BoxMapper - 可重复调用，单一所有权
// ============================================================================

let mut counter = 0;
let mut mapper = BoxMapper::new(move |x: i32| {
    counter += 1;
    format!("Item #{}: {}", counter, x)
});

// ✅ 可以多次调用
assert_eq!(mapper.apply(100), "Item #1: 100");
assert_eq!(mapper.apply(200), "Item #2: 200");

// 方法链
let mut pipeline = BoxMapper::new(|x: i32| x * 2)
    .and_then(|x| x + 5)
    .and_then(|x| format!("Result: {}", x));

assert_eq!(pipeline.map(10), "Result: 25");

// ============================================================================
// 3. ArcMapper - 多线程共享
// ============================================================================

use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0));
let mapper = ArcMapper::new(move |x: i32| {
    let mut c = counter.lock().unwrap();
    *c += 1;
    format!("Item #{}: {}", *c, x)
});

// ✅ 可以克隆
let mut mapper_clone = mapper.clone();

// ✅ 可以跨线程使用
use std::thread;
let handle = thread::spawn(move || {
    mapper_clone.map(100)
});
assert_eq!(handle.join().unwrap(), "Item #1: 100");

// ============================================================================
// 4. RcMapper - 单线程复用
// ============================================================================

let counter = Rc::new(RefCell::new(0));
let mapper = RcMapper::new(move |x: i32| {
    let mut c = counter.borrow_mut();
    *c += 1;
    format!("Item #{}: {}", *c, x)
});

// ✅ 可以克隆
let mut mapper_clone = mapper.clone();

assert_eq!(mapper_clone.map(100), "Item #1: 100");
assert_eq!(mapper_clone.map(200), "Item #2: 200");

// ============================================================================
// 5. 与 Supplier::map 集成
// ============================================================================

// ✅ 简单闭包
let mut supplier = BoxSupplier::new(|| 10)
    .map(|x| x * 2);
assert_eq!(supplier.get(), 20);

// ✅ 组合的 Mapper 对象
let mapper = BoxMapper::new(|x: i32| x * 2)
    .and_then(|x| x + 5);

let mut supplier = BoxSupplier::new(|| 10).map(mapper);
assert_eq!(supplier.get(), 25);

// ✅ 有状态的组合
let mut counter1 = 0;
let mut counter2 = 0;

let mapper = BoxMapper::new(move |x: i32| {
    counter1 += 1;
    x + counter1
}).and_then(move |x| {
    counter2 += 1;
    x * counter2
});

let mut supplier = BoxSupplier::new(|| 10).map(mapper);
assert_eq!(supplier.get(), (10 + 1) * 1);  // 11
assert_eq!(supplier.get(), (10 + 2) * 2);  // 24

// ============================================================================
// 6. 统一的接口 - 泛型编程
// ============================================================================

fn transform_vec<T, R, F>(mapper: &mut F, vec: Vec<T>) -> Vec<R>
where
    F: Mapper<T, R>,
{
    vec.into_iter().map(|x| mapper.apply(x)).collect()
}

let mut counter = 0;
let mut mapper = BoxMapper::new(move |x: i32| {
    counter += 1;
    x + counter
});

let results = transform_vec(&mut mapper, vec![10, 20, 30]);
assert_eq!(results, vec![11, 22, 33]);
```

### 3.3 类型选择指南

| 需求 | 推荐类型 | 理由 |
|------|---------|------|
| 可重复调用，单一所有权 | `BoxMapper` | 单一所有权，可多次调用 |
| 多线程共享 | `ArcMapper` | 线程安全，可克隆 |
| 单线程复用 | `RcMapper` | 无原子操作，性能更好 |
| 简单一次性使用 | 直接用闭包 | 无需包装，简洁 |
| 组合管道 | `BoxMapper::and_then` | 预先组合，可复用 |

---

## 四、与其他函数式抽象的对比

### 4.1 核心差异

| | Transformer | Mapper | Predicate | Consumer | Supplier |
|---|---|---|---|---|---|
| **输入** | `T` | `T` | `&T` | `&T` | 无 |
| **输出** | `R` | `R` | `bool` | `()` | `T` |
| **self 签名** | `&self` | `&mut self` | `&self` | `&mut self` | `&mut self` |
| **消耗输入** | ✅ | ✅ | ❌ | ❌ | N/A |
| **修改自己** | ❌（内部可变性）| ✅ | ❌（内部可变性）| ✅ | ✅ |
| **核心用途** | 纯函数转换 | 有状态转换、组合 | 过滤、验证 | 观察、累积 | 工厂、生成 |

### 4.2 为什么 Mapper 需要 `&mut self` 而 Transformer 不需要？

| | Transformer | Mapper |
|---|---|---|
| **语义** | "纯函数转换" | "有状态的映射" |
| **典型场景** | 类型转换、数据映射 | 计数转换、组合管道 |
| **状态需求** | 次要（可用 RefCell）| 核心（状态是主要特性）|
| **组合能力** | 组合纯函数 | 组合有状态转换器 |
| **所有权** | 不消耗 self | 消耗 self（BoxMapper）|

**关键区别**：

```rust
// Transformer：纯函数转换（状态是次要的）
let transformer = BoxTransformer::new(|x: i32| x * 2);
transformer.apply(21);  // 不需要 mut

// Mapper：有状态转换（状态是核心）
let mut mapper = BoxMapper::new(move |x: i32| {
    counter += 1;
    x + counter
});
mapper.apply(10);  // 需要 mut
```

### 4.3 为什么 Supplier 需要 `&mut self`？

与 Mapper 类似，Supplier 的状态变化是核心语义：

```rust
pub trait Supplier<T> {
    fn get(&mut self) -> T;  // ✅ 合理
}

// 典型场景：计数器、序列生成器
let mut counter = 0;
let mut supplier = BoxSupplier::new(move || {
    counter += 1;
    counter
});

assert_eq!(supplier.get(), 1);
assert_eq!(supplier.get(), 2);
assert_eq!(supplier.get(), 3);
```

**为什么 Supplier 需要 `&mut self`？**

1. **没有输入，状态是核心**：Supplier 的输出完全依赖内部状态
2. **状态递增是主要用途**：计数器、ID 生成器、序列生成器
3. **无法用内部可变性替代**：因为整个闭包需要是 `FnMut()`

**结论**：Supplier 和 Mapper 的 `&mut self` 都是必要的，因为状态变化是其核心语义。

### 4.4 设计一致性

所有函数式抽象遵循统一的设计模式：

1. **统一的 trait 接口**：每种抽象都有核心 trait
2. **三种实现**：Box（单一）、Arc（共享+线程安全）、Rc（共享+单线程）
3. **类型保持的方法链**：组合方法返回相同类型
4. **闭包自动实现 trait**：无缝集成
5. **为 FnMut 实现 Mapper**：保持与标准库的兼容性

---

## 五、真实业务场景示例

### 5.1 数据转换管道

```rust
// 构建复杂的有状态数据处理管道
let mut counter = 0;
let mut pipeline = BoxMapper::new(move |raw: String| {
    counter += 1;
    format!("[{}] {}", counter, raw.trim())
})
.and_then(|s| s.parse::<i32>().ok())
.and_then(|opt| opt.unwrap_or(0))
.and_then(|x| x * 2)
.and_then(|x| format!("Result: {}", x));

let result = pipeline.map("  42  ".to_string());
assert_eq!(result, "Result: 84");
```

### 5.2 配置转换器

```rust
use std::collections::HashMap;

struct ConfigManager {
    transformers: HashMap<String, BoxMapper<String, String>>,
}

impl ConfigManager {
    fn new() -> Self {
        let mut transformers = HashMap::new();

        // 注册各种有状态转换器
        let mut counter = 0;
        transformers.insert(
            "with_counter".to_string(),
            BoxMapper::new(move |s: String| {
                counter += 1;
                format!("[{}] {}", counter, s)
            }),
        );

        transformers.insert(
            "uppercase".to_string(),
            BoxMapper::new(|s: String| s.to_uppercase()),
        );

        ConfigManager { transformers }
    }

    fn transform(&mut self, key: &str, value: String) -> String {
        if let Some(transformer) = self.transformers.get_mut(key) {
            transformer.map(value)
        } else {
            value
        }
    }
}
```

### 5.3 多线程数据处理

```rust
use std::thread;
use std::sync::{Arc, Mutex};

// 创建可在多线程间共享的有状态转换器
let counter = Arc::new(Mutex::new(0));
let mapper = ArcMapper::new(move |data: Vec<u8>| {
    let mut c = counter.lock().unwrap();
    *c += 1;
    let count = *c;
    data.into_iter()
        .map(|b| b.wrapping_mul(count as u8))
        .collect::<Vec<_>>()
});

let mut handles = vec![];

for i in 0..4 {
    let mut mapper_clone = mapper.clone();
    let handle = thread::spawn(move || {
        let data = vec![i; 100];
        mapper_clone.map(data)
    });
    handles.push(handle);
}

let results: Vec<_> = handles.into_iter()
    .map(|h| h.join().unwrap())
    .collect();
```

### 5.4 动态管道构建

```rust
// 根据配置动态构建有状态转换管道
struct PipelineBuilder {
    config: Config,
}

impl PipelineBuilder {
    fn build(&self) -> BoxMapper<Input, Output> {
        let mut mapper = BoxMapper::identity();

        if self.config.enable_logging {
            let mut log_count = 0;
            mapper = mapper.and_then(move |x| {
                log_count += 1;
                println!("[{}] Processing: {:?}", log_count, x);
                x
            });
        }

        if self.config.enable_validation {
            let mut valid_count = 0;
            let mut invalid_count = 0;
            mapper = mapper.and_then(move |x| {
                if validate(&x) {
                    valid_count += 1;
                    println!("Valid count: {}", valid_count);
                    x
                } else {
                    invalid_count += 1;
                    println!("Invalid count: {}", invalid_count);
                    default_value()
                }
            });
        }

        if self.config.enable_transformation {
            let mut transform_count = 0;
            mapper = mapper.and_then(move |x| {
                transform_count += 1;
                transform_with_history(x, transform_count)
            });
        }

        mapper
    }
}
```

---

## 六、总结

### 6.1 核心设计原则

1. **Mapper 消耗输入 `T`**：符合转换语义，与 Transformer 一致
2. **Mapper 返回所有权 `R`**：避免生命周期问题，语义明确
3. **Mapper 使用 `&mut self`**：状态变化是核心特性，不是次要的
4. **为 FnMut 实现 Mapper**：保持与标准库的兼容性
5. **提供组合方法**：and_then、compose 等
6. **类型名称语义明确**：Box/Arc/Rc 表达所有权模型

### 6.2 为什么这个设计最好？

**与 Transformer + RefCell 的对比**：

| | Transformer + RefCell | Mapper trait（推荐）|
|---|---|---|
| **简单场景** | 🟡 需要 RefCell | ✅ 直观简单 |
| **组合能力** | ❌ 无法预先组合 | ✅ **可以预先组合** |
| **复用性** | ⚠️ 困难 | ✅ **保存为字段** |
| **策略模式** | ❌ 不支持 | ✅ **动态选择** |
| **运行时开销** | ❌ RefCell 检查 | ✅ 无开销 |
| **学习成本** | 🔴 需要理解 RefCell | 🟡 需要理解 `&mut` |
| **panic 风险** | ❌ borrow_mut 可能 panic | ✅ 不会 panic |

**与其他模块设计的一致性**：

- Consumer **观察**输入（`&T`），**可修改**自己（累积）
- Predicate **判断**输入（`&T`），**不修改**自己（纯函数）
- Transformer **转换**输入（`T`），**不修改**自己（纯函数）
- Mapper **转换**输入（`T`），**可修改**自己（有状态）
- Supplier **生成**输出（无输入），**可修改**自己（状态递增）

### 6.3 最终结论

**✅ 应该实现 Mapper trait！**

经过深入分析，Mapper trait 具有明确的价值，特别是在**组合**和**复用**场景中。

**核心价值**：

1. ✅ **组合能力**：预先组合多个有状态转换器
2. ✅ **复用性**：保存为字段，多次使用
3. ✅ **策略模式**：动态选择不同的 mapper
4. ✅ **统一接口**：闭包、函数指针、对象统一
5. ✅ **管道构建**：构建复杂的数据处理管道
6. ✅ **无运行时开销**：不需要 RefCell 的借用检查
7. ✅ **不会 panic**：没有运行时借用检查失败的风险

**设计方案**：

```rust
// 1. 定义 Mapper trait（基于 FnMut）
pub trait Mapper<T, R> {
    fn apply(&mut self, input: T) -> R;
    // 转换方法...
}

// 2. 为 FnMut 实现 Mapper（关键！）
impl<T, R, F> Mapper<T, R> for F
where
    F: FnMut(T) -> R,
{
    fn apply(&mut self, input: T) -> R {
        self(input)
    }
}

// 3. 提供三种实现
- BoxMapper<T, R>   // 单一所有权
- ArcMapper<T, R>   // 线程安全共享
- RcMapper<T, R>    // 单线程共享

// 4. Supplier::map 使用 Mapper trait
impl<T> BoxSupplier<T> {
    pub fn map<U, F>(mut self, mut mapper: F) -> BoxSupplier<U>
    where
        F: Mapper<T, U> + 'static,  // ✅ 使用 Mapper
    {
        BoxSupplier::new(move || mapper.apply(self.get()))
    }
}
```

**使用场景对比**：

| 场景 | 使用方案 | 示例 |
|------|---------|------|
| **简单闭包** | 直接用闭包 | `.map(\|x\| x * 2)` |
| **组合转换** | BoxMapper | `mapper1.and_then(mapper2)` |
| **保存为字段** | BoxMapper | `struct { pipeline: BoxMapper<T, R> }` |
| **策略模式** | HashMap<String, BoxMapper> | `mappers.get("strategy")` |
| **动态构建** | 条件组合 | `if config { mapper.and_then(step) }` |
| **Map-Reduce** | Transformer | 并行场景用纯函数 |

**这是一个平衡了简洁性和功能性的设计，既支持简单场景（闭包），也支持复杂场景（组合、复用）。**
