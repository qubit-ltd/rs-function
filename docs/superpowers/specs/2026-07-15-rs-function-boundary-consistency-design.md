# rs-function 边界一致性加固设计

## 1. 背景

`rs-function` 已完成 wrapper-first 组合接口与统一诊断命名。本轮不再调整公开
callback 家族，而是修复复核中确认的边界约束、名称传播和 StatefulSupplier
实现问题，并清理重构后遗留的宏文档漂移。

本设计延续
[`2026-07-15-callback-object-consistency-design.md`](2026-07-15-callback-object-consistency-design.md)
确定的原则：单源变换保留名称，多源组合清除名称；Arc 下的无状态回调需要
`Send + Sync`，由 Mutex 串行保护的 Stateful 回调只需要 `Send`。

## 2. 目标

1. 允许 Stateful Arc 组合器捕获 `Send + !Sync` 回调，同时保持 predicate 的
   `Send + Sync` 要求。
2. 让 Runnable 到 Callable 的跨类型顺序组合遵守“多源组合清除名称”契约。
3. 去除 Rc/Arc StatefulSupplier memoize 的重复同步层，并保留源 metadata。
4. 修正共享宏文档、Stateful Arc 锁模型注释和 blanket impl 的物理位置。
5. 通过编译边界测试、行为测试和完整 feature matrix 防止回归。

## 3. 非目标

- 不调整 `default`、`rc`、`once`、`stateful` 或 `full` feature。
- 不新增 `StatefulBiFunction` 等 callback 家族。
- 不改变 Box、Rc、Arc 的公开类型、方法名或所有权语义。
- 不改变 Conditional 中间对象的 metadata 设计。
- 不引入新的依赖、过程宏或代码生成系统。
- 不提交 Git commit；提交行为需要用户另行明确授权。

## 4. Stateful Arc 约束模型

### 4.1 根因

共享组合宏目前使用同一个 `$extra_bounds` 同时约束：

- `when` 接收的 predicate；
- `and_then` 接收的后续 callback。

对无状态 Arc，两者都必须为 `Send + Sync + 'static`。对 Stateful Arc，predicate
仍存储在无状态 `ArcPredicate` 中，因此必须为 `Send + Sync + 'static`；后续
callback 则被捕获到 `Arc<Mutex<dyn FnMut + Send>>` 中，只需要
`Send + 'static`。复用一个 bounds 参数导致后者被过度约束。

Supplier 的 `(arc)` 宏分支同样同时服务 `ArcSupplier` 和
`ArcStatefulSupplier`，因此错误地把无状态 Arc 的 `Sync` 要求带给了 Stateful
Arc 的 mapper、predicate 和 zipped supplier。

### 4.2 方案

Consumer、Function、Mutator 和 Transformer 的共享方法宏把单一 bounds 参数
拆成两个显式能力参数：

- predicate bounds；
- chained callback bounds。

调用策略如下：

| 包装器能力 | Predicate | Chained callback |
| --- | --- | --- |
| Rc stateless/stateful | `'static` | `'static` |
| Arc stateless | `Send + Sync + 'static` | `Send + Sync + 'static` |
| Arc stateful | `Send + Sync + 'static` | `Send + 'static` |

Conditional 共享宏约束额外 callback：Consumer/Mutator 包括 `and_then` 和
`or_else`，Function/Transformer 包括 `or_else`。Stateful Arc 调用点改为
`Send + 'static`，无状态 Arc 和 Rc 保持现状。

Supplier 宏增加明确的 Stateful Arc 能力分支，并复用同一方法主体：

| Supplier | Mapper / Predicate / Other supplier |
| --- | --- |
| Rc stateless/stateful | `'static` |
| Arc stateless | `Send + Sync + 'static` |
| Arc stateful | `Send + 'static` |

`ArcStatefulFunction::constant` 捕获的返回值位于 Mutex 内部，因此把返回值约束
从 `Clone + Send + Sync + 'static` 收紧为实际所需的
`Clone + Send + 'static`。

这些修改只放宽输入类型，不改变已有调用的运行时行为或二进制数据结构。

## 5. 任务名称传播

以下操作均执行两个独立 callback，属于多源顺序组合：

- `BoxRunnable::then_callable`；
- `BoxRunnableOnce::then_callable`；
- `LocalBoxRunnableOnce::then_callable`；
- `BoxRunnableWith::then_callable_with`。

它们不再把 runnable 的 metadata 传给结果 callable，而是通过普通 `new`
构造未命名结果。执行顺序、错误短路、返回值、Send 边界和公开签名保持不变。
调用方仍可在组合结束后使用 `with_name` 显式命名最终任务。

## 6. StatefulSupplier memoize

### 6.1 状态模型

`BoxStatefulSupplier` 的 memoized closure 直接捕获 `Option<T>`。
`RcStatefulSupplier` 和 `ArcStatefulSupplier` 也采用相同模型：缓存只由返回包装器
内部 closure 访问，而该 closure 已分别由外层 `RefCell` 或 `Mutex` 保证独占
调用，因此无需第二层 `Rc<RefCell<Option<T>>>` 或
`Arc<Mutex<Option<T>>>`。

调用流程为：

1. 获取返回包装器的外层可变访问权；
2. 缓存为 `Some` 时 clone 并返回；
3. 缓存为 `None` 时调用共享的源 supplier 一次；
4. clone 一份写入缓存并返回原值。

首次计算发生 panic 时不会写入缓存，后续调用仍会重试；这与当前行为一致。

### 6.2 Metadata

memoize 是单源包装，结果保留源 metadata：

- Box 消费 `self.metadata`；
- Rc/Arc clone `self.metadata`，使源包装器仍可继续使用且后续改名互不影响。

## 7. 能力文档与代码组织

共享 Consumer、Function、Mutator、Transformer 和 Supplier 宏文档统一使用本设计
第 4 节的能力表，不再引用已经移除的 `into_arc`、`into_rc` 或
`$predicate_conversion` 参数。

Stateful Arc 类型的内部标题和公开 rustdoc 统一描述真实存储：
`Arc<Mutex<dyn FnMut + Send>>`。线程安全来自 Mutex 串行访问，不暗示用户 callback
自身实现 `Sync`。

`impl<F, T> StatefulPredicate<T> for F where F: FnMut(&T) -> bool` 从具体
`arc_stateful_predicate.rs` 移到定义 trait 的 `stateful_predicate.rs`。这不改变
coherence 或 feature 行为，只让 blanket impl 的可用性与具体 wrapper 解耦。

本轮不建立新的生成系统。宏调用点、宏文档能力表和 compile-pass 测试共同构成
可审查的能力契约，避免为追求矩阵对称继续扩展公开 API。

## 8. 测试策略

所有行为按 TDD 实施，先观察测试因现有约束或行为失败，再写最小实现。

### 8.1 编译边界

使用包含 `Cell` 的 `Send + !Sync` 测试 callback，覆盖每个共享宏族：

- Stateful Arc Consumer/BiConsumer 的 `and_then`；
- Stateful Arc Function/MutatingFunction 的 `and_then`；
- Stateful Arc Mutator 的 `and_then`；
- Stateful Arc Transformer/BiTransformer 的 `and_then`；
- 对应 Conditional 的 `or_else`；
- Stateful Arc Supplier 的 `map`、`filter` 和 `zip`；
- `ArcStatefulFunction::constant` 返回 `Send + !Sync` 值。

这些测试在修复前应以 E0277 的 `Sync` 缺失失败，修复后编译并验证调用结果。
无状态 Arc 的共享宏调用点与存储类型继续显式要求
`Send + Sync + 'static`；本次测试重点验证 Stateful Arc 的正向放宽。

### 8.2 名称传播

四个 Runnable 到 Callable 的组合分别从有名 runnable 构造结果，断言结果
`name() == None`，同时验证成功结果和前序错误短路。

### 8.3 Memoize

Box、Rc、Arc 分别验证：

- 多次调用只执行源 supplier 一次；
- 每次返回缓存值的 clone；
- memoized wrapper 保留源名称；
- Rc/Arc 源对象在 memoize 后仍可使用。

### 8.4 完整验证

依次运行精确测试、全部 feature 组合、Clippy、rustdoc、项目指定 rustfmt 和完整
测试套件。最终工作区只包含本设计批准的源码、测试和文档改动。

## 9. 实施顺序

1. 添加 Stateful Arc `Send + !Sync` 失败测试并修正共享 bounds。
2. 添加任务名称传播失败测试并清除跨类型 sequencing metadata。
3. 添加 memoize 失败测试并简化缓存、保留 metadata。
4. 更新能力表、宏文档、锁模型注释并移动 blanket impl。
5. 运行完整验证和 diff 自查。
