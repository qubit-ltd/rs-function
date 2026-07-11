# rs-function 设计与实现评审

评审日期：2026-07-11

评审版本：`qubit-function 0.15.1`

## 1. 评审结论

`rs-function` 的核心抽象有真实价值，当前实现质量和测试质量也较高。它已经成为 Qubit Rust 基础库中 callback、task、executor 和拦截器等边界的公共类型语言，不适合删除，也没有必要整体重写。

当前主要问题不是运行时正确性，而是公开 API 的增长已经超过了下游真实需求：相近语义 trait、调用形式、可变性、参数个数、所有权容器和组合器被做成了近乎完整的笛卡尔积。结果是维护面、学习成本、编译成本和方法解析冲突同步增加。

建议将后续方向从“继续补齐函数接口矩阵”调整为“稳定并收缩 Qubit 下游真正需要的 callback/task 核心”：

1. 保留现有核心 trait 和已被下游使用的 `Box`/`Arc` 适配器。
2. 冻结新的平行 family 和组合矩阵，不再追求形式上的完整性。
3. 先用文档、显式构造器和 feature 分层做非破坏性治理。
4. 在下一个允许破坏兼容性的版本中，将转换与组合方法移出核心 trait，缩小默认公开面。

综合评分：**6.5/10**。其中核心抽象约 **8/10**，实现与测试约 **8/10**，公开 API 设计约 **5.5/10**。

## 2. 评审依据

### 2.1 代码与验证

当前 crate 约有 5 万行库代码、数百个 Rust 文件和两百余个公开 trait/结构体/类型别名。代码未使用 `unsafe`，不同函数语义、所有权模型和调用次数都有大量独立测试。

本轮评审执行过以下验证：

- `cargo +1.94.0 test --all-features --quiet`：测试及 doctest 共约 4,800 项通过。
- nightly `cargo clippy --all-targets --all-features -- -D warnings`：通过。
- `RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps`：通过。
- 12 个非 `rs-dcl` 直接下游 crate 的 all-target/all-feature check：通过。

这些结果说明当前实现总体稳定；下面提出的问题主要集中在 API 边界、语义清晰度和长期维护成本，而不是把大量已通过验证的实现判定为不可用。

### 2.2 下游实际使用

工作区内有 13 个 crate 直接依赖 `qubit-function`：

- `rs-batch`
- `rs-cas`
- `rs-dcl`
- `rs-execution-services`
- `rs-executor`
- `rs-http`
- `rs-progress`
- `rs-rayon-batch`
- `rs-rayon-executor`
- `rs-retry`
- `rs-task`
- `rs-thread-pool`
- `rs-tokio-executor`

排除主要承担组合试验性质的 `rs-dcl` 后，生产代码实际导入的公开符号集中在 13 个：

`ArcBiConsumer`、`ArcBiFunction`、`ArcConsumer`、`ArcMutatingFunction`、`BiConsumer`、`BiFunction`、`BiPredicate`、`BoxConsumer`、`Callable`、`Consumer`、`Function`、`MutatingFunction`、`Runnable`。

这组数据表明，下游主要需要的是：

- 可作为泛型边界的语义 trait；
- 可共享的类型擦除 callback；
- 可失败的 `Callable`/`Runnable` task 边界；
- 少量 `Box`/`Arc` 包装。

大量 `Rc`、stateful、conditional、once、二元、mutating、transformer、operator 及其组合变体目前没有形成与其维护成本相匹配的下游需求。

## 3. 值得保留的设计

### 3.1 核心调用语义清楚

`Function::apply`、`Consumer::accept`、`Predicate::test`、`Callable::call`、`Runnable::run` 等核心方法能够在泛型 API 上表达“这个参数在业务上承担什么角色”。与直接暴露复杂 `Fn` 约束相比，这在 executor、retry、HTTP interceptor 和 batch API 中提高了签名可读性。

### 3.2 `Fn`、`FnMut`、`FnOnce` 的区分是必要的

crate 没有把 Rust 的调用次数和可变性差异抹平。一次性任务、可复用无状态 callback 和带内部状态 callback 的能力边界总体是明确的。这一部分应继续保留，不宜为了减少类型数量而重新混成一个大 trait。

### 3.3 `Box`、`Rc`、`Arc` 表达了真实所有权选择

对需要类型擦除的调用方，单所有者、单线程共享和跨线程共享确实是不同需求。尤其是 `ArcConsumer`、`ArcBiFunction` 等类型已经被进度上报、HTTP、重试和并发执行模块实际使用，属于 crate 的高价值部分。

### 3.4 闭包 blanket impl 与自定义实现可以共存

下游既可以直接传闭包，也可以为领域对象实现 `Runnable`、`Callable`、`BiFunction` 等 trait。这使简单调用保持轻量，同时允许复杂任务保留命名类型、状态和专门测试。

### 3.5 测试覆盖广，边界意识较强

当前测试覆盖了不同 ownership、调用次数、组合器、panic、错误类型和 doctest。测试规模本身不代表 API 一定合理，但它显著降低了收缩或迁移时误改既有行为的风险。

## 4. 主要问题

### 4.1 高优先级：重叠语义 trait 产生真实的方法歧义

多个 trait 可由同一个闭包同时实现，而这些 trait 又在核心 trait 上提供同名转换方法。例如返回 `bool` 的闭包既可视为 `Function<T, bool>`，也可视为 `Predicate<T>`：

```rust,compile_fail
use qubit_function::{Function, Predicate};

let positive = |value: &i32| *value > 0;
let _boxed = positive.into_box();
```

编译器会报告 E0034，因为两个 trait 都提供 `into_box`。类似问题还存在于：

- `Callable<(), E>` 与 `Runnable<E>`；
- `Function<T, ()>` 与 `Consumer<T>`；
- `MutatingFunction<T, ()>` 与 `Mutator<T>`；
- `Supplier<bool>` 与 `Tester`；
- `Function<T, R>` 与 `StatefulFunction<T, R>` 等普通、stateful family：实现
  `Fn` 的闭包同时也实现 `FnMut`，因此同样会获得两组同名转换方法。

这不是 IDE 展示过多，而是正常导入多个语义 trait 后会触发的源码级冲突。调用方只能使用 UFCS，例如 `Predicate::into_box(positive)`，或者改用具体包装类型构造器。

当前工作区的 13 个直接下游中尚未出现这类编译失败。现有 `into_box`、
`into_arc` 调用主要作用于只带一个语义 trait bound 的泛型参数，编译器可以据此
确定方法来源。但“同时导入 `Function` 和 `Predicate`，然后直接包装一个返回
`bool` 的闭包”是正常的外部使用方式，因此不能以当前下游尚未触发为由忽略该
问题。

根因是核心语义 trait 同时承担了两项职责：

1. 定义 callback 的业务语义和调用方法；
2. 提供一整套容器转换与组合便利方法。

建议：

- 核心 trait 最终只保留 `apply`、`accept`、`test`、`call`、`run` 等本征操作。
- 保留闭包到语义 trait 的 blanket impl。它使接受 `Predicate`、`Consumer` 等泛型
  边界的 API 可以直接接收闭包；删除它会迫使普通调用也提前创建动态 wrapper，
  混淆“业务语义”和“类型擦除”两个职责。
- `into_box`、`into_rc`、`into_arc`、`into_fn` 等从核心 trait 移除，不再把同名
  转换方法搬到另一组 blanket extension trait；后一种做法仍会产生相同歧义。
- 闭包或自定义 trait 实现到具体 wrapper，统一使用目标类型上的 `new` 构造器，
  例如 `BoxPredicate::new(predicate)`、`ArcFunction::new(function)`。
- `new` 直接接受对应语义 trait 的实现，例如
  `BoxPredicate::new<P: Predicate<T> + 'static>(predicate: P)`；`Arc` 版本再增加
  相应的 `Send`/`Sync` 约束。闭包通过 blanket impl 自动满足语义 trait，因此
  闭包和自定义实现不需要两套构造入口。
- 不新增 `from`、`wrap`、`from_predicate` 等平行构造方法。标准库风格的
  `From<P>` 会在 wrapper 自身也实现对应语义 trait 时与
  `impl<T> From<T> for T` 产生 coherence 冲突；额外命名构造器也会让调用方重新
  判断入口，而没有引入新的真实语义。
- 兼容期先补充 compile-fail 文档、UFCS 和显式 `Type::new(...)` 示例；在下一个
  破坏性版本中再移除 trait 上的转换方法。

相关实现集中在 `src/functions/function.rs`、`src/predicates/predicate.rs`、`src/tasks/callable.rs` 和 `src/tasks/runnable.rs`。

### 4.2 高优先级：公开 API 呈笛卡尔积增长

当前 API 同时沿以下维度展开：

- 零、一、二参数；
- 借用输入、拥有输入、可变输入；
- `Fn`、`FnMut`、`FnOnce`；
- 普通、stateful、conditional；
- `Box`、`Rc`、`Arc`；
- trait、具体 wrapper、closure extension trait；
- `and_then`、`when`、逻辑运算、错误映射和各种转换。

单看每一类都能找到合理用例，但把所有维度全部公开组合后，API 规模远大于真实使用面。它带来四个直接后果：

1. 同一种语义有多个近似入口，调用方难以判断推荐路径。
2. 宏和模板式文件数量持续增长，review 很难验证所有变体保持一致。
3. rustdoc、IDE 补全和编译时间承受与下游收益不对称的成本。
4. 新需求容易被误判为“再补一组平行类型”，而不是先判断是否可以由闭包或现有适配器表达。

建议立即设立准入规则：只有出现至少两个独立生产下游且现有闭包/trait 无法清楚表达时，才新增公开 family。形式对称不应再作为新增 API 的充分理由。

现阶段保留全部已有 family，不启动删除或弃用；上述规则只冻结后续扩张。既有类型
以后仍可能形成真实下游需求，是否收缩应继续由使用证据和迁移成本决定。

### 4.3 中优先级：ownership 转换只改变最外层包装

部分 `Arc`/`Rc`/`Box` 相互转换通过创建新闭包并捕获旧 wrapper 完成。以 `Arc` 转 `Rc` 为例，得到的是一个 `Rc` 持有的适配闭包，但闭包内部仍保存原来的 `Arc`。因此：

- 原有原子引用计数并未消失；
- stateful wrapper 原有的锁也不会消失；
- 可能再增加一层动态分发和闭包调用；
- `to_rc` 之类名称容易让调用方误以为底层 ownership 已被真正转换。

从使用目的反推，`ArcFoo -> RcFoo` 只有几种表面理由：进入单线程阶段后希望去掉
原子引用计数、适配只接受 `RcFoo` 的 API，或者把值放入统一的本地容器。但当前
实现无法满足第一个目的；`ArcFoo` 本身可以在单线程中调用，无需为了第二个目的
降低能力；只接受具体 `RcFoo` 而不接受对应语义 trait 的 API 则是边界设计过窄。
第三种需求得到的也只是 `Rc<adapter<Arc<...>>>`，并没有形成统一的底层 ownership。

工作区 13 个直接下游中没有任何 `ArcFoo -> RcFoo` 或 `RcFoo -> ArcFoo` 调用；现有
相关调用只存在于 `rs-function` 自身为了验证转换矩阵完整性的测试和示例中。缺少
生产需求与上述语义缺陷相互印证：这种转换主要来自 API 对称性，而不是真实场景。

建议在下一个破坏性版本中彻底删除 `ArcFoo` 与 `RcFoo` 之间双向的 `into_*`、
`to_*` API，不为它们提供改名后的 adapter view。需要目标 ownership 时，应从
原始闭包或自定义实现直接调用 `ArcFoo::new(...)` 或 `RcFoo::new(...)`。API 若只
需要执行行为，应接受对应语义 trait，而不是要求某一种具体 wrapper。

其他容器转换也采用相同准入原则：只有能够真正转移底层存储、转换后的类型语义与
实际成本一致，并且有生产调用证据时才保留；不能满足这些条件的转换不因矩阵对称性
而存在。

相关逻辑可见 `src/macros/arc_conversions.rs` 等转换宏。

### 4.4 中优先级：stateful 共享包装有隐含的可重入限制

`ArcStateful*` 使用 mutex 保护 `FnMut`，并在持锁期间执行用户 callback；`RcStateful*` 使用 `RefCell` 并在可变借用期间执行 callback。这是实现共享 `FnMut` 所需的自然手段，但它形成了必须公开说明的行为契约：

- 同一个 `ArcStateful*` 的并发调用会被完全串行化；
- callback 若同步重入同一个 `ArcStateful*`，会死锁；
- callback 若重入同一个 `RcStateful*`，会因重复可变借用而 panic；
- callback 执行时间等于临界区长度。

目前 `Stateful` 容易被理解为普通“带状态 callback”，不足以提示上述并发和重入边界。建议在所有共享 stateful 类型，以及同样通过 `Mutex`/`RefCell` 共享
`FnMut` 的 `ArcCallable`、`RcCallable`、`ArcRunnable`、`RcRunnable` 等类型级
rustdoc 中统一声明：底层同步原语、调用串行化、临界区长度、同步重入行为和
callback panic 后的状态语义。`parking_lot::Mutex` 不会 poisoning，但 callback
panic 前对捕获状态的修改不会自动回滚。文档还应给出不应在 callback 中同步回调
同一实例的反例。

相关实现可见 `src/consumers/stateful_consumer/arc_stateful_consumer.rs` 及对应 `Rc` 变体。

### 4.5 中优先级：默认依赖和编译面未按真实使用分层

`parking_lot` 与宏辅助依赖目前是无条件依赖，大量实现和示例也进入同一个默认 crate。多数下游只使用少量无状态 trait 和 `Arc`/`Box` callback，却要承担完整 family 的解析、类型检查和文档面。

feature 应隔离显著成本或能力边界，而不是机械映射所有语义 family。建议采用：

- baseline 始终编译，不设置名为 `core` 的开关：核心 trait、闭包 blanket impl、
  高频 `Box` 和无状态 `Arc` wrapper；
- `rc`：单线程共享 wrapper；
- `once`：`FnOnce` family；
- `stateful`：显式 stateful family、共享 `FnMut` wrapper，并启用可选的
  `parking_lot` 依赖；
- `combinators`：conditional wrapper、`when`、`and_then`、逻辑组合和相关
  extension trait；
- `full`：聚合启用 `rc`、`once`、`stateful`、`combinators`。

建议的 Cargo 关系为：

```toml
[features]
default = []
rc = []
once = []
stateful = ["dep:parking_lot"]
combinators = []
full = ["rc", "once", "stateful", "combinators"]

[dependencies]
parking_lot = { version = "0.12", optional = true }
```

`default = []` 不代表空 crate；baseline API 始终存在，使
`--no-default-features` 仍然得到可用的核心抽象。`Callable`、`Runnable` trait 和
本地 `Box` wrapper 可留在 baseline；需要 `Mutex`/`RefCell` 的共享 `FnMut`
wrapper 分别受 `stateful`、`stateful + rc` 控制。

当前闭包 blanket impl 位于部分 `arc_*` 实现文件中，分层前必须将它移回核心 trait
模块，否则关闭 wrapper feature 会意外改变闭包是否实现语义 trait。4.1 的转换
方法收缩完成后，blanket impl 宏不再需要根据 trait 名拼接 wrapper 名；若
`pastey` 不再有其他用途，应直接移除，而不是再增加一个 feature。

暂不按 `functions`、`predicates`、`suppliers` 等语义 family 切 feature。这些不是
依赖或平台边界，继续细分会重新产生 `family × ownership × mutability` 的 feature
矩阵。

### 4.6 低优先级：性能宣传强于现有证据

README 使用了 “High-Performance Concurrency” 和 “Zero-Cost Abstractions”等表述，但 crate 广泛使用堆分配、动态分发、引用计数和 mutex，也没有提供可复现 benchmark 来界定这些成本。

这些机制本身没有问题，问题在于“zero-cost”会给出错误预期。建议将
“High-Performance Concurrency”改为“Thread-safe callback adapters”，将
“Zero-Cost Abstractions”改为“Ergonomic callback abstractions”或
“Low-boilerplate type-erased callbacks”，并明确静态闭包通常可内联，而
`Box`、`Rc`、`Arc`、mutex wrapper 分别可能引入堆分配、动态分发、引用计数和
串行化成本。

### 4.7 低优先级：实现模块全部公开，扩大了路径兼容负担

crate root 已经重导出主要公开类型，但 `comparator`、`consumers`、`functions`、`mutators`、`predicates`、`suppliers`、`tasks`、`testers`、`transformers` 仍全部是公开模块，其内部实现模块也继续公开。调用方可以依赖深层实现路径，使未来重组文件或收缩 family 变成额外的兼容负担。

工作区外部下游目前全部使用 crate root 路径，没有生产代码依赖这些深层路径。
建议在下一个破坏性版本中采用三级路径策略：

1. crate root 是主要稳定入口，只重导出核心 trait 和高频 wrapper；
2. `functions`、`consumers`、`predicates`、`tasks` 等一级语义模块是稳定的补充
   命名空间，用于导出启用 feature 后的完整公开 API；
3. `functions::function::arc_function` 一类物理实现模块、宏模块和文件布局全部私有。

不提供包含全部 trait 的 `prelude::*`。在转换方法收缩前它会放大 4.1 的冲突；
收缩后也没有足够收益抵消隐式导入大量语义 trait 的可读性成本。

## 5. 建议的演进顺序

### 阶段一：当前兼容版本

1. 冻结新 family、新 ownership 组合和新的同名转换方法。
2. 在 README 和 rustdoc 中给出“推荐核心 API”清单。
3. 为方法歧义增加 compile-fail 示例，并给出具体 wrapper 的 `Type::new(...)`
   构造器和 UFCS 过渡方案。
4. 为所有共享 stateful 类型补齐串行化、重入、panic 和临界区文档。
5. 修正未经 benchmark 支撑的性能表述。
6. 统计 baseline、`rc`、`once`、`stateful`、`combinators` 各组合的编译时间和
   制品影响，验证已确定的分层边界。

### 阶段二：下一个破坏性版本

1. 核心 trait 仅保留本征调用方法。
2. 弃用或移除核心 trait 上通用的 `into_*`、`to_*` 和 `*_fn` 方法。
3. 扩展 `BoxPredicate::new(...)`、`ArcFunction::new(...)` 等关联构造器，使其统一
   接受对应语义 trait 的实现。
4. 删除 `ArcFoo` 与 `RcFoo` 间的双向转换，以及其他没有真实存储转换语义和生产
   证据的跨容器 API。
5. 将组合器放入按语义命名、按需导入的 extension trait。
6. 按 baseline + `rc`/`once`/`stateful`/`combinators`/`full` 落地 feature。
7. 默认公开面收缩为 baseline + 高频 `Box`/`Arc` adapter。
8. 明确 root 和一级语义模块为稳定路径，隐藏更深层实现模块。

### 阶段三：按下游证据维护

1. 以生产下游的真实使用统计决定哪些 family 继续演进。
2. 对长期无人使用且可由闭包直接替代的矩阵分支进入弃用流程。
3. 为高频 adapter 建立静态闭包、`Box`、`Arc`、stateful wrapper 的基准对比。
4. 将编译时间、rustdoc 规模和 breaking migration 成本纳入版本评审。

## 6. 最终意见

`rs-function` 应继续作为 Qubit Rust 体系的 callback/task 基础 crate，但它的成功标准不应再是“覆盖尽可能完整的 Java 风格函数接口”，而应是“用最小、清楚、稳定的 Rust API 支撑下游 callback 和 executor 边界”。

当前最合适的动作是**保留核心、冻结扩张、逐步收缩**，而不是删除、重写或继续补齐矩阵。
